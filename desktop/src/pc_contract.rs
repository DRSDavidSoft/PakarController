use reqwest::blocking::Client;
use serde_json::Value;
use std::time::{Duration, Instant};
use crate::protocol_opcodes::controller as wire;

pub const PROFILE: u8 = 0x50;
pub fn expected_response(opcode: u8, payload: &[u8]) -> Option<u8> {
    match opcode {
        wire::HELLO => Some(wire::HELLO_RESPONSE),
        wire::GET_STATUS => Some(wire::STATUS_RESPONSE),
        wire::GET_SETTINGS => Some(wire::SETTINGS_RESPONSE),
        wire::TEMPERATURE_LIST => Some(wire::TEMPERATURE_LIST_RESPONSE),
        wire::PWM_GET => Some(wire::PWM_VALUES_RESPONSE),
        wire::STATUS_PROFILE_GET => Some(wire::STATUS_PROFILE_RESPONSE),
        wire::RADIO_LEARN_LIST => Some(wire::RADIO_LEARN_LIST_RESPONSE),
        wire::I2C_TRANSFER => Some(wire::I2C_TRANSFER_RESPONSE),
        wire::FRONT_PANEL_GET => Some(wire::FRONT_PANEL_RESPONSE),
        wire::MENU_LIST => Some(wire::MENU_LIST_RESPONSE),
        wire::MENU_LAYOUT_GET => Some(wire::MENU_LAYOUT_RESPONSE),
        wire::HOST_MENU_STATE_GET => Some(wire::HOST_MENU_STATE_RESPONSE),
        wire::AUTOMATION_LIST => Some(wire::AUTOMATION_LIST_RESPONSE),
        wire::AUTOMATION_PUT => Some(wire::AUTOMATION_RECORD_RESPONSE),
        wire::MACRO_STEP if matches!(payload.first().copied(), Some(2 | 5)) => Some(wire::MACRO_STATUS_RESPONSE),
        wire::SET_STREAM_PERIOD | wire::SET_SETTINGS | wire::BUZZER | wire::PWM_SET | wire::PWM_ALL_OFF | wire::STATUS_RGB | wire::ADDRESSABLE_LED | wire::STATUS_EFFECT | wire::STATUS_PROFILE_SET | wire::RADIO_TRANSMIT | wire::RADIO_LEARN_START | wire::RADIO_LEARN_CANCEL | wire::RADIO_LEARN_CLEAR | wire::RADIO_LEARN_REMOVE | wire::MENU_ACTION | wire::RELAY_SET | wire::RELAY_SIDE | wire::RELAY_ALL_OFF | wire::RELAY_TEST | wire::RESET | wire::MENU_SET_PAGE | wire::DISPLAY_TEXT | wire::MACRO_START | wire::MACRO_CANCEL | wire::MACRO_STEP | wire::REMOTE_KEY_GESTURE | wire::RADIO_LEARN_REPLACE | wire::MENU_LAYOUT_SET | wire::HOST_MENU_DIRECTORY | wire::HOST_MENU_CONTENT | wire::PROGRAM_STATE | wire::AUTOMATION_REMOVE | wire::AUTOMATION_CLEAR => Some(wire::ACK),
        _ => None,
    }
}

pub fn decode_hex(input: &str) -> Result<Vec<u8>, String> {
    if input.len() > 96 || input.len() % 2 != 0 || !input.is_ascii() {
        return Err("Payload must contain at most 48 bytes of even-length hexadecimal".into());
    }
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let digit = |c: u8| (c as char).to_digit(16).map(|n| n as u8);
            Ok((digit(pair[0]).ok_or("Invalid hexadecimal payload")? << 4)
                | digit(pair[1]).ok_or("Invalid hexadecimal payload")?)
        })
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn opcode(input: &str) -> Result<u8, String> {
    let registry: Value = serde_json::from_str(include_str!(
        "../../docs/pccontroller/ProtocolContract.json"
    ))
    .map_err(|e| e.to_string())?;
    let numeric = if let Some(hex) = input.strip_prefix("0x") {
        u8::from_str_radix(hex, 16).ok()
    } else {
        input.parse::<u8>().ok()
    };
    registry["opcodes"]
        .as_array()
        .ok_or("Missing donor registry")?
        .iter()
        .find_map(|entry| {
            let value = entry["value"].as_u64()? as u8;
            (value < 128
                && (numeric == Some(value)
                    || entry["wireName"]
                        .as_str()
                        .is_some_and(|name| name.eq_ignore_ascii_case(input))))
            .then_some(value)
        })
        .ok_or_else(|| {
            "Unknown PCController request opcode; reserved/response values are not requests".into()
        })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub bytes: [u8; 21],
    pub persisted: bool,
}
impl Settings {
    pub fn parse(payload: &str) -> Result<Self, String> {
        let bytes = decode_hex(payload)?;
        if bytes.len() != 22 || bytes[0] != PROFILE || bytes[21] > 1 {
            return Err("AVR did not return Pakar settings profile 0x50".into());
        }
        let mut value = Self {
            bytes: bytes[..21].try_into().unwrap(),
            persisted: bytes[21] != 0,
        };
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&mut self) -> Result<(), String> {
        let p = &self.bytes;
        let period = u16::from_le_bytes([p[3], p[4]]);
        let threshold = u16::from_le_bytes([p[5], p[6]]);
        if p[0] != PROFILE
            || p[1] & !7 != 0
            || p[2] > 7
            || (period != 0 && period < 100)
            || !(1..=1023).contains(&threshold)
            || p[7] >= 6
            || p[8] == 0
            || p[8] & !63 != 0
            || p[8] & (1 << p[7]) == 0
            || p[12] > 8
        {
            return Err("Invalid AVR settings values".into());
        }
        let mut seen = 0u8;
        for rank in 0..6 {
            let page = (p[9 + rank / 2] >> (4 * (rank % 2))) & 15;
            if page >= 6 || seen & (1 << page) != 0 {
                return Err("Invalid AVR menu permutation".into());
            }
            seen |= 1 << page;
        }
        if p[13..].iter().enumerate().any(|(i, b)| {
            if i < p[12] as usize {
                !(0x20..=0x7e).contains(b)
            } else {
                *b != 0
            }
        }) {
            return Err("Invalid AVR board name".into());
        }
        Ok(())
    }
    pub fn set(&mut self, key: &str, input: &str) -> Result<(), String> {
        let boolean = || match input {
            "on" | "true" => Ok(true),
            "off" | "false" => Ok(false),
            _ => Err("Use on or off".to_owned()),
        };
        let number = || {
            input
                .parse::<u16>()
                .map_err(|_| "Expected an unsigned integer".to_owned())
        };
        match key {
            "silent" | "welcome" | "save-last-page" => {
                let bit = if key == "silent" {
                    1
                } else if key == "welcome" {
                    2
                } else {
                    4
                };
                self.bytes[1] = (self.bytes[1] & !bit) | if boolean()? { bit } else { 0 };
            }
            "display-brightness" => {
                let n = number()?;
                if n > 7 {
                    return Err("Display brightness must be 0..7".into());
                }
                self.bytes[2] = n as u8;
            }
            "stream-period" => {
                let n = number()?;
                if n != 0 && n < 100 {
                    return Err("Stream period must be 0 or 100..65535 ms".into());
                }
                self.bytes[3..5].copy_from_slice(&n.to_le_bytes());
            }
            "button9-threshold" => {
                let n = number()?;
                if !(1..=1023).contains(&n) {
                    return Err("ADC threshold must be 1..1023".into());
                }
                self.bytes[5..7].copy_from_slice(&n.to_le_bytes());
            }
            "default-page" => {
                let n = number()?;
                if n >= 6 {
                    return Err("Default page must be 0..5".into());
                }
                self.bytes[7] = n as u8;
            }
            "board-name" => {
                if input.len() > 8 || !input.bytes().all(|b| (0x20..=0x7e).contains(&b)) {
                    return Err(
                        "AVR board name must be at most eight printable ASCII characters".into(),
                    );
                }
                self.bytes[12] = input.len() as u8;
                self.bytes[13..].fill(0);
                self.bytes[13..13 + input.len()].copy_from_slice(input.as_bytes());
            }
            _ => return Err("Unknown AVR parameter".into()),
        }
        self.validate()
    }
    pub fn json(&self) -> Value {
        serde_json::json!({
            "silent": self.bytes[1]&1!=0, "welcome":self.bytes[1]&2!=0,
            "save_last_page": self.bytes[1]&4!=0,
            "display_brightness":self.bytes[2],
            "stream_period_ms":u16::from_le_bytes([self.bytes[3],self.bytes[4]]),
            "button9_threshold":u16::from_le_bytes([self.bytes[5],self.bytes[6]]),
            "default_page":self.bytes[7], "visible_pages":self.bytes[8],
            "board_name":String::from_utf8_lossy(&self.bytes[13..13+self.bytes[12] as usize]),
            "persisted":self.persisted
        })
    }
}

pub fn get(
    client: &Client,
    base: &str,
    user: &str,
    password: &str,
    path: &str,
) -> Result<Value, String> {
    client
        .get(format!("{}{path}", base.trim_end_matches('/')))
        .basic_auth(user, Some(password))
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())
}
fn consume(
    client: &Client,
    base: &str,
    user: &str,
    password: &str,
    id: u64,
    boot: &str,
    opcode: u8,
) -> bool {
    client
        .delete(format!(
            "{}/api/avr/contracts/result",
            base.trim_end_matches('/')
        ))
        .basic_auth(user, Some(password))
        .query(&[
            ("id", id.to_string()),
            ("boot_id", boot.to_owned()),
            ("opcode", opcode.to_string()),
        ])
        .send()
        .ok()
        .and_then(|r| r.error_for_status().ok())
        .and_then(|r| r.json::<Value>().ok())
        .is_some_and(|v| v["consumed"].as_bool() == Some(true))
}
pub fn request(
    client: &Client,
    base: &str,
    user: &str,
    password: &str,
    opcode: u8,
    payload: &[u8],
) -> Result<Value, String> {
    if payload.len() > 48 {
        return Err("PCController payload exceeds 48 bytes".into());
    }
    let expected = u64::from(expected_response(opcode, payload)
        .ok_or("Unknown controller response contract; nothing sent, no fallback")?);
    let value: Value = client
        .post(format!("{}/api/avr/contracts", base.trim_end_matches('/')))
        .basic_auth(user, Some(password))
        .form(&[
            ("opcode", opcode.to_string()),
            ("payload_hex", encode_hex(payload)),
        ])
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    let id = value["request_id"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("Missing contract request ID")?;
    let boot = value["boot_id"]
        .as_str()
        .ok_or("Missing device boot identity")?
        .to_owned();
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        // Only read the existing receipt. Never re-POST an uncertain mutation.
        let receipt = get(
            client,
            base,
            user,
            password,
            &format!("/api/avr/contracts/result?id={id}"),
        )?;
        if receipt["request_id"].as_u64() != Some(id)
            || receipt["boot_id"].as_str() != Some(&boot)
            || receipt["request_opcode"].as_u64() != Some(opcode as u64)
        {
            return Err("Contract receipt identity changed; operation outcome is unknown".into());
        }
        match receipt["state"].as_str() {
            Some("responded") => {
                let response = receipt["response_opcode"]
                    .as_u64()
                    .ok_or("Missing response opcode")?;
                let payload = decode_hex(
                    receipt["payload_hex"]
                        .as_str()
                        .ok_or("Missing response payload")?,
                )?;
                if response == u64::from(wire::ERROR_RESPONSE) {
                    if payload.len() != 2 || payload[0] != opcode || !(1..=6).contains(&payload[1])
                    {
                        return Err(
                            "Malformed device rejection; operation outcome is unknown".into()
                        );
                    }
                    // Only our validated terminal receipt; never delete a
                    // queued/awaiting or identity-mismatched transaction.
                    let _ = consume(client, base, user, password, id, &boot, opcode);
                    return Err(format!(
                        "AVR rejected opcode 0x{opcode:02x}: {payload:02x?}"
                    ));
                }
                if response != expected || (response == u64::from(wire::ACK) && payload != [opcode, 0]) {
                    return Err(
                        "Response does not match the request; operation outcome is unknown".into(),
                    );
                }
                let mut receipt = receipt;
                receipt["receipt_consumed"] =
                    Value::Bool(consume(client, base, user, password, id, &boot, opcode));
                return Ok(receipt); // Cleanup failure does not undo real ACK.
            }
            Some("outcome_unknown") => {
                return Err("Operation outcome is unknown; it was not retried".into());
            }
            Some("queued" | "awaiting") if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(40))
            }
            _ => return Err("Operation outcome is unknown; it was not retried".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opcode_namespace_has_explicit_response_contract_for_every_registered_request() {
        let registry: Value = serde_json::from_str(include_str!("../../docs/pccontroller/ProtocolContract.json")).unwrap();
        for opcode in 0..=255_u8 {
            let request = registry["opcodes"].as_array().unwrap().iter().any(|row|
                row["value"].as_u64() == Some(u64::from(opcode)) && opcode < 128);
            assert_eq!(expected_response(opcode, &[]).is_some(), request);
        }
        assert_eq!(expected_response(wire::MACRO_STEP, &[2]), Some(wire::MACRO_STATUS_RESPONSE));
        assert_eq!(expected_response(wire::MACRO_STEP, &[5]), Some(wire::MACRO_STATUS_RESPONSE));
        assert_eq!(expected_response(wire::MACRO_STEP, &[0]), Some(wire::ACK));
        assert_eq!(expected_response(0xfc, &[]), None);
    }
    #[test]
    fn actual_donor_registry_has_no_pwm_audio_alias() {
        assert_eq!(opcode("PWM_SET").unwrap(), 0x11);
        assert_eq!(opcode("DISPLAY_TEXT").unwrap(), 0x38);
        assert_eq!(opcode("AUTOMATION_PUT").unwrap(), 0x47);
        assert!(opcode("0x13").is_err());
        assert!(opcode("0x91").is_err());
    }
    #[test]
    fn hex_bounds_and_non_ascii_are_rejected() {
        assert_eq!(decode_hex("00ff").unwrap(), vec![0, 255]);
        for bad in ["0", "xx", " 00", "é0"] {
            assert!(decode_hex(bad).is_err());
        }
        assert!(decode_hex(&"00".repeat(48)).is_ok());
        assert!(decode_hex(&"00".repeat(49)).is_err());
    }
    #[test]
    fn settings_are_peer_validated_and_preserve_untouched_fields() {
        let mut raw = vec![0x50, 2, 5, 0, 0, 0, 2, 0, 63, 0x10, 0x32, 0x54, 0];
        raw.extend_from_slice(&[0; 8]);
        raw.push(1);
        let mut s = Settings::parse(&encode_hex(&raw)).unwrap();
        s.set("display-brightness", "0").unwrap();
        assert_eq!(s.bytes[2], 0);
        s.set("silent", "on").unwrap();
        assert_eq!(s.bytes[1], 3);
        s.set("board-name", "AVR").unwrap();
        assert_eq!(s.json()["board_name"], "AVR");
        assert!(s.set("button9-threshold", "1024").is_err());
        assert!(Settings::parse("03").is_err());
    }
}
