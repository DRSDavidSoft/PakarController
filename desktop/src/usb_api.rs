//! Bounded API transfer on the existing USB owner; execute once, never replay.
//! Uses the same open/append/execute/read/close contract as mobile UsbTransport.
use crate::protocol_opcodes::pakar as wire;
use crate::{DeviceStatus, usb_table};
use serde_json::Value;
use std::{sync::{mpsc::Sender, Arc, atomic::{AtomicBool, Ordering}}, time::{Duration, Instant}};
pub const OPCODE: u8 = wire::USB_API;
pub const RESPONSE: u8 = wire::USB_API_RESPONSE;

pub struct Work {
    pub admission: usb_table::Admission,
    pub transaction: Transaction,
    pub reply: Sender<Result<Value, String>>,
    pub wait: Option<(u8, Instant)>,
    pub revoked: Vec<Arc<AtomicBool>>,
}
impl Work {
    pub fn current(&self, session: u64, status: Option<&DeviceStatus>) -> bool {
        !self.revoked.iter().any(|flag| flag.load(Ordering::Acquire))
            && self.admission.current(session, status) && self.transaction.started.elapsed() < Duration::from_secs(9)
    }
    pub fn fail(self, message: impl Into<String>) { let _ = self.reply.send(Err(message.into())); }
}
pub struct Transaction {
    request: Vec<u8>,
    fence: Vec<u8>,
    token: u32,
    action: u8,
    sent: usize,
    total: usize,
    received: Vec<u8>,
    value: Option<Value>,
    pub started: Instant,
}
impl Transaction {
    pub fn new(status: &DeviceStatus, method: &str, path: &str, body: Value) -> Result<Self, String> {
        if !status.capabilities.as_ref().is_some_and(|c| c.has("usb-api"))
            || crate::peer_http::Pin::from_status(status).is_none()
            || !matches!(method, "GET" | "POST") || !body.is_object() {
            return Err("USB API capability or peer unavailable; nothing sent".into());
        }
        let request = serde_json::to_vec(&serde_json::json!({"method":method,"path":path,"body":body,
            "board_id":status.board_id,"boot_id":status.boot_id})).map_err(|e| e.to_string())?;
        if request.is_empty() || request.len() > 4096 { return Err("USB API request exceeds bounded transfer size".into()); }
        let fence = status.boot_id.as_bytes().chunks_exact(2).map(|pair|
            u8::from_str_radix(std::str::from_utf8(pair).unwrap_or(""), 16).map_err(|_| "Invalid USB boot identity"))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { request, fence, token: 0, action: 0, sent: 0, total: 0, received: vec![], value: None, started: Instant::now() })
    }
    pub fn request(&self) -> Result<Vec<u8>, String> {
        if self.action == 2 && self.started.elapsed() >= Duration::from_secs(7) {
            return Err("USB API admission expired before execute; no replay".into());
        }
        let mut bytes = vec![self.action];
        if self.action == 0 { bytes.extend(&self.fence); bytes.extend((self.request.len() as u16).to_le_bytes()); }
        else {
            bytes.extend(self.token.to_le_bytes());
            match self.action {
                1 => { bytes.extend((self.sent as u16).to_le_bytes()); bytes.extend(&self.request[self.sent..(self.sent + 41).min(self.request.len())]); }
                3 => bytes.extend((self.received.len() as u16).to_le_bytes()),
                _ => {}
            }
        }
        Ok(bytes)
    }
    pub fn cleanup(&self) -> Option<Vec<u8>> {
        (self.token != 0).then(|| { let mut bytes = vec![4]; bytes.extend(self.token.to_le_bytes()); bytes })
    }
    pub fn accept(&mut self, opcode: u8, bytes: &[u8]) -> Result<Option<Value>, String> {
        if self.action == 4 {
            return if opcode == wire::ACK && bytes == [OPCODE, 0] { Ok(self.value.take()) }
                else { Err("USB API close not acknowledged; result not replayed".into()) };
        }
        if opcode != RESPONSE || !(9..=48).contains(&bytes.len()) || bytes[0] != self.action {
            return Err("USB API rejected or invalid correlated response; no replay".into());
        }
        let token = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
        let total = usize::from(u16::from_le_bytes(bytes[5..7].try_into().unwrap()));
        let offset = usize::from(u16::from_le_bytes(bytes[7..9].try_into().unwrap()));
        if token == 0 || self.token != 0 && token != self.token
            || offset != if self.action == 3 { self.received.len() } else { 0 } {
            return Err("USB API token/offset changed; no replay".into());
        }
        self.token = token;
        match self.action {
            0 | 1 if total == 0 && bytes.len() == 9 => {
                if self.action == 1 { self.sent = (self.sent + 41).min(self.request.len()); }
                self.action = if self.sent == self.request.len() { 2 } else { 1 };
            }
            2 if (1..=8192).contains(&total) && bytes.len() == 9 => { self.total = total; self.action = 3; }
            3 if total == self.total && bytes.len() > 9 && self.received.len() + bytes.len() - 9 <= total => {
                self.received.extend(&bytes[9..]);
                if self.received.len() == total {
                    let value: Value = serde_json::from_slice(&self.received).map_err(|_| "Malformed USB API JSON receipt; no replay")?;
                    if !value.is_object() { return Err("USB API receipt is not an object".into()); }
                    self.value = Some(value); self.action = 4;
                }
            }
            _ => return Err("USB API response length/state mismatch; no replay".into()),
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn status() -> DeviceStatus { serde_json::from_value(serde_json::json!({"board_id":"fixture","boot_id":"a".repeat(32),"http_peer_fencing":true,
        "capabilities":{"board_kind":"table","acknowledged_features":["usb-api"]}})).unwrap() }
    fn response(action:u8, total:u16, offset:u16, tail:&[u8]) -> Vec<u8> {
        let mut p=vec![action];p.extend(7_u32.to_le_bytes());p.extend(total.to_le_bytes());p.extend(offset.to_le_bytes());p.extend(tail);p
    }
    #[test]
    fn pixel_groups_usb_api_executes_once_and_closes_after_complete_json() {
        let mut tx=Transaction::new(&status(),"GET",crate::pixel_groups::PATH,serde_json::json!({})).unwrap();
        assert_eq!(tx.request().unwrap().len(),19);
        tx.accept(RESPONSE,&response(0,0,0,&[])).unwrap();
        while tx.action==1 { assert!(tx.request().unwrap().len()<=48);tx.accept(RESPONSE,&response(1,0,0,&[])).unwrap(); }
        assert_eq!(tx.request().unwrap()[0],2);
        tx.accept(RESPONSE,&response(2,11,0,&[])).unwrap();
        tx.accept(RESPONSE,&response(3,11,0,b"{\"ok\":true}")).unwrap();
        assert_eq!(tx.request().unwrap()[0],4);
        assert_eq!(tx.accept(0x80,&[OPCODE,0]).unwrap(),Some(serde_json::json!({"ok":true})));
    }
    #[test]
    fn pixel_groups_usb_api_rejects_wrong_token_and_out_of_order_read() {
        let mut tx=Transaction::new(&status(),"GET",crate::pixel_groups::PATH,serde_json::json!({})).unwrap();
        tx.accept(RESPONSE,&response(0,0,0,&[])).unwrap();
        let mut bad=response(1,0,0,&[]);bad[1]=8;
        assert!(tx.accept(RESPONSE,&bad).is_err());
        assert!(tx.accept(RESPONSE,&response(1,0,1,&[])).is_err());
        assert!(tx.accept(0x82,&[OPCODE,6]).is_err());
    }
}
