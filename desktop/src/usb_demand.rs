//! Consumer leases, adapted from PCController's shared status subscription hub:
//! no subscribers means no full snapshots; the one physical owner coalesces all readers.
use crate::protocol_opcodes::pakar as wire;
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
pub const REQUEST: u8 = wire::HOST_DEMAND;
pub const RESPONSE: u8 = wire::HOST_DEMAND_RESPONSE;
pub const STATUS: u16 = 1;
pub const PANEL: u16 = 2;
pub const PERIPHERAL: u16 = 4;
pub const PIXELS: u16 = 8;
pub const LEASE_MS: u16 = 3000;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Demand {
    pub flags: u16,
    pub table: bool,
}
impl Demand {
    pub fn metadata() -> Self {
        Self {
            flags: STATUS,
            table: true,
        }
    }
    pub fn merge(self, other: Self) -> Self {
        Self {
            flags: self.flags | other.flags,
            table: self.table || other.table,
        }
    }
}
pub fn visible_demand(
    sample: (Demand, Option<Instant>),
    visible: bool,
    minimized: bool,
    occluded: bool,
    exiting: bool,
) -> Demand {
    if visible
        && !minimized
        && !occluded
        && !exiting
        && sample
            .1
            .is_some_and(|at| at.elapsed() < Duration::from_secs(1))
    {
        sample.0
    } else {
        Demand::default()
    }
}
#[derive(Default)]
pub struct Watchers(Mutex<Vec<(Instant, Demand)>>);
impl Watchers {
    pub fn touch(&self, demand: Demand) {
        if let Ok(mut readers) = self.0.lock() {
            readers.retain(|(until, _)| *until > Instant::now());
            // At most one lease per distinct demand mask, not per poll/client.
            let until = Instant::now() + Duration::from_secs(3);
            if let Some(reader) = readers.iter_mut().find(|(_, old)| *old == demand) {
                reader.0 = until;
            } else if readers.len() < 16 {
                readers.push((until, demand));
            }
        }
    }
    pub fn demand(&self) -> Demand {
        self.at(Instant::now())
    }
    fn at(&self, now: Instant) -> Demand {
        self.0
            .lock()
            .map(|mut readers| {
                readers.retain(|(until, _)| *until > now);
                readers
                    .iter()
                    .fold(Demand::default(), |all, (_, d)| all.merge(*d))
            })
            .unwrap_or_default()
    }
}
#[derive(Default)]
pub struct Lease {
    desired: u16,
    pending: Option<(u8, u16, Instant)>,
    sent: Option<Instant>,
}
impl Lease {
    pub fn desired(&mut self, flags: u16) {
        if self.desired != flags {
            self.desired = flags;
            self.pending = None;
            self.sent = None;
        }
    }
    pub fn request(&mut self, sequence: u8) -> Option<[u8; 4]> {
        if self
            .pending
            .is_some_and(|(_, _, at)| at.elapsed() < Duration::from_secs(1))
        {
            return None;
        }
        if self
            .sent
            .is_some_and(|at| at.elapsed() < Duration::from_secs(1))
        {
            return None;
        }
        // A disabled lease needs one packet, never recurring zero traffic.
        if self.desired == 0 && self.sent.is_some() {
            return None;
        }
        let now = Instant::now();
        self.pending = Some((sequence, self.desired, now));
        self.sent = Some(now);
        let lease = if self.desired == 0 { 0 } else { LEASE_MS };
        let flags = self.desired.to_le_bytes();
        let lease = lease.to_le_bytes();
        Some([flags[0], flags[1], lease[0], lease[1]])
    }
    pub fn accept(&mut self, sequence: u8, payload: &[u8]) -> bool {
        let Some((expected, flags, at)) = self.pending else {
            return false;
        };
        if sequence != expected || payload.len() != 4 || at.elapsed() > Duration::from_secs(1) {
            return false;
        }
        let effective = u16::from_le_bytes([payload[0], payload[1]]);
        let remaining = u16::from_le_bytes([payload[2], payload[3]]);
        if flags != self.desired
            || effective != flags
            || (flags == 0 && remaining != 0)
            || (flags != 0 && !(1..=LEASE_MS).contains(&remaining))
        {
            return false;
        }
        self.pending = None;
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn usb_demand_consumers_expire_and_rgb_is_explicit() {
        let readers = Watchers::default();
        assert_eq!(readers.demand(), Demand::default());
        readers.touch(Demand::metadata());
        assert_eq!(readers.demand().flags, STATUS);
        readers.touch(Demand {
            flags: PIXELS,
            table: true,
        });
        assert_eq!(readers.demand().flags, STATUS | PIXELS);
        assert_eq!(
            readers.at(Instant::now() + Duration::from_secs(4)),
            Demand::default()
        );
    }
    #[test]
    fn usb_demand_late_ack_cannot_restore_retired_pixels() {
        let mut lease = Lease::default();
        lease.desired(PIXELS);
        assert!(lease.request(7).is_some());
        lease.desired(0);
        assert!(!lease.accept(7, &[8, 0, 0xb8, 0x0b]));
        assert_eq!(lease.request(8), Some([0, 0, 0, 0]));
        assert!(lease.accept(8, &[0, 0, 0, 0]));
        assert_eq!(lease.request(9), None);
    }
    #[test]
    fn usb_demand_hidden_closed_or_minimized_is_not_a_pixel_consumer() {
        let sample = (
            Demand {
                flags: PIXELS,
                table: true,
            },
            Some(Instant::now()),
        );
        assert_eq!(
            visible_demand(sample, true, false, false, false).flags,
            PIXELS
        );
        for (visible, minimized, occluded, exiting) in [
            (false, false, false, false),
            (true, true, false, false),
            (true, false, true, false),
            (true, false, false, true),
        ] {
            assert_eq!(
                visible_demand(sample, visible, minimized, occluded, exiting),
                Demand::default()
            );
        }
    }
}
