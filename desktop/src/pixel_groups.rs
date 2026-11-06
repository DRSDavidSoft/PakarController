//! Board-owned PHYSICAL group geometry. Logical mappings never reorder overrides.
use crate::{DeviceStatus, peer_http::Pin};
use serde::{Deserialize, Serialize};
use std::time::Instant;

pub const PATH: &str = "/api/pixels/groups";
pub const FEATURE: &str = "pixel-group-layout";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Override {
    pub group: u8,
    pub count: u8,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layout {
    pub revision: u32,
    pub default_count: u8,
    pub overrides: Vec<Override>,
    pub counts: Vec<u8>,
    pub starts: Vec<u16>,
    pub used_pixels: u16,
    pub capacity: u16,
    pub group_count: u8,
    pub index_base: u8,
}
impl Layout {
    pub fn uniform(capacity: u16, default_count: u8) -> Option<Self> {
        if default_count == 0 || capacity == 0 || capacity % u16::from(default_count) != 0 { return None; }
        let group_count = u8::try_from(capacity / u16::from(default_count)).ok()?;
        Some(Self { revision: 0, default_count, overrides: vec![],
            counts: vec![default_count; usize::from(group_count)],
            starts: (0..u16::from(group_count)).map(|g| g * u16::from(default_count)).collect(),
            used_pixels: capacity, capacity, group_count, index_base: 0 })
    }
    pub fn parse(value: &serde_json::Value) -> Result<Self, String> {
        let mut value = value.clone();
        if value["partial"] == true {
            // Explicit compact USB projection omits only redundant offsets and
            // overrides. Prefix sums come from observed counts, never defaults.
            let counts = value["counts"].as_array().ok_or("Compact group counts missing")?;
            let default = value["default_count"].as_u64().ok_or("Default group size missing")?;
            let mut next = 0_u64;
            let mut starts = Vec::new(); let mut overrides = Vec::new();
            for (index, count) in counts.iter().enumerate() {
                let count = count.as_u64().filter(|n| (1..=255).contains(n)).ok_or("Invalid compact group count")?;
                starts.push(next); next = next.checked_add(count).ok_or("Group size overflow")?;
                if count != default { overrides.push(serde_json::json!({"group":index+1,"count":count})); }
            }
            if value.get("starts").is_none() { value["starts"] = serde_json::json!(starts); }
            if value.get("overrides").is_none() { value["overrides"] = serde_json::json!(overrides); }
        }
        let layout: Self = serde_json::from_value(value).map_err(|_| "Invalid physical group layout")?;
        layout.validate()?;
        Ok(layout)
    }
    pub fn validate(&self) -> Result<(), String> {
        if !matches!((self.capacity, self.group_count, self.default_count), (480, 80, 6) | (216, 18, 12))
            || self.index_base != 0 || self.counts.len() != usize::from(self.group_count)
            || self.starts.len() != self.counts.len() || self.overrides.len() > self.counts.len() {
            return Err("Physical group layout does not match the board capacity".into());
        }
        let (counts, starts, total) = self.calculate()?;
        if counts != self.counts || starts != self.starts || total != self.used_pixels {
            return Err("Physical group counts and offsets are inconsistent".into());
        }
        Ok(())
    }
    fn calculate(&self) -> Result<(Vec<u8>, Vec<u16>, u16), String> {
        let mut counts = vec![self.default_count; usize::from(self.group_count)];
        let mut seen = std::collections::BTreeSet::new();
        for item in &self.overrides {
            if item.group == 0 || item.group > self.group_count || !seen.insert(item.group) {
                return Err("Physical group overrides must be unique and in range".into());
            }
            counts[usize::from(item.group - 1)] = if item.count == 0 { self.default_count } else { item.count };
        }
        let mut starts = Vec::with_capacity(counts.len());
        let mut total = 0_u16;
        for &count in &counts {
            if count == 0 { return Err("Each physical group requires at least one pixel".into()); }
            starts.push(total);
            total = total.checked_add(u16::from(count)).ok_or("Physical group size overflow")?;
        }
        if total > self.capacity { return Err(format!("{total} pixels exceed this board's {}-pixel capacity", self.capacity)); }
        Ok((counts, starts, total))
    }
    pub fn draft_total(&self) -> Result<u16, String> { self.calculate().map(|(_, _, total)| total) }
    pub fn set_override(&mut self, group: u8, count: u8) {
        self.overrides.retain(|item| item.group != group);
        if count != 0 && count != self.default_count { self.overrides.push(Override { group, count }); }
        self.overrides.sort_by_key(|item| item.group);
    }
    pub fn span(&self, group: u8) -> Option<(u16, u8)> {
        let at = usize::from(group.checked_sub(1)?);
        Some((*self.starts.get(at)?, *self.counts.get(at)?))
    }
    pub fn containing(&self, pixel: u16) -> Option<u8> {
        self.starts.iter().zip(&self.counts).position(|(&start, &count)|
            pixel >= start && pixel < start + u16::from(count)).and_then(|i| u8::try_from(i + 1).ok())
    }
    pub fn request(&self) -> Result<serde_json::Value, String> {
        self.draft_total()?;
        Ok(serde_json::json!({"revision":self.revision,"overrides":self.overrides}))
    }
    pub fn verifies(&self, actual: &Self) -> bool {
        let Ok((counts, starts, used)) = self.calculate() else { return false; };
        actual.validate().is_ok() && actual.revision >= self.revision && actual.capacity == self.capacity
            && actual.default_count == self.default_count && actual.counts == counts
            && actual.starts == starts && actual.used_pixels == used
    }
    pub fn receipt(value: &serde_json::Value, peer: &Pin) -> Result<Self, String> {
        if !peer.matches_identity(value["board_id"].as_str().unwrap_or(""), value["boot_id"].as_str().unwrap_or("")) {
            return Err("Physical group readback belongs to a different board session".into());
        }
        if value["partial"] == true { return Err("Editable group configuration is incomplete".into()); }
        Self::parse(value)
    }
}
pub fn advertised(status: &DeviceStatus) -> bool {
    status.capabilities.as_ref().is_some_and(|c| c.has(FEATURE))
}
pub fn for_status(status: &DeviceStatus) -> Option<Layout> {
    let (capacity, default_count) = crate::pixel_diagnostic::profile(status)?;
    if advertised(status) {
        let layout = Layout::parse(&status.pixel_groups).ok()?;
        (layout.capacity == capacity && layout.default_count == default_count).then_some(layout)
    } else { Layout::uniform(capacity, default_count) }
}
#[derive(Default)]
pub struct Editor {
    pub owner: Option<Pin>,
    pub value: Option<Layout>,
    pub dirty: bool,
    pub message: String,
    pub pending: Option<Instant>,
    pub expected: Option<Layout>,
    pub open: bool,
}
pub enum Purpose {
    Maintenance { id:u64, operation:crate::maintenance::Operation },
    Minigame(crate::minigame::UsbRequest),
    Groups,
    Frame { revision: u32, layout_revision: u32, at: Instant },
    Diagnostic { id: u64, probe: crate::pixel_diagnostic::Probe, off: bool, request_id: String, previous: DeviceStatus },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pixel_groups_validate_physical_offsets_capacity_and_revision() {
        let mut draft = Layout::uniform(480, 6).unwrap(); draft.revision = 7;
        draft.set_override(1, 5); draft.set_override(80, 7);
        let (counts, starts, used) = draft.calculate().unwrap();
        let mut actual = draft.clone(); actual.revision = 8; actual.counts = counts;
        actual.starts = starts; actual.used_pixels = used;
        assert!(draft.verifies(&actual)); assert_eq!(actual.span(2), Some((5, 6)));
        assert_eq!(actual.span(80), Some((473, 7))); assert_eq!(actual.containing(479), Some(80));
        actual.starts[1] = 6; assert!(actual.validate().is_err());
        draft.set_override(1, 7); assert!(draft.request().is_err());
    }
    #[test]
    fn pixel_groups_inheritance_and_duplicates_are_not_guessed() {
        let mut layout = Layout::uniform(216, 12).unwrap();
        layout.overrides.push(Override { group: 2, count: 0 }); assert!(layout.validate().is_ok());
        layout.overrides.push(Override { group: 2, count: 11 }); assert!(layout.validate().is_err());
        layout.overrides.clear(); layout.set_override(2, 11);
        assert_eq!(layout.draft_total().unwrap(), 215);
        assert_eq!(layout.span(18), Some((204, 12)), "editing must not mutate last readback offsets");
        let unchanged=Layout::uniform(216,12).unwrap();
        assert!(unchanged.verifies(&unchanged),"explicit no-op receipt may keep its revision");
    }
    #[test]
    fn pixel_groups_partial_metadata_never_conceals_inconsistent_fields_or_foreign_receipts() {
        let mut layout = Layout::uniform(480,6).unwrap();
        layout.set_override(1,5); layout.counts[0]=5;layout.used_pixels=479;
        for start in &mut layout.starts[1..] {*start-=1;}
        let mut value=serde_json::to_value(layout).unwrap(); value["partial"]=true.into();
        assert!(Layout::parse(&value).is_ok());
        value["starts"][1]=6.into();assert!(Layout::parse(&value).is_err());
        value.as_object_mut().unwrap().remove("starts");
        value["overrides"]=serde_json::json!([]);assert!(Layout::parse(&value).is_err());
        value.as_object_mut().unwrap().remove("overrides");assert!(Layout::parse(&value).is_ok());
        let peer=Pin::from_identity("fixture",&"a".repeat(32)).unwrap();
        let mut receipt=serde_json::to_value(Layout::uniform(480,6).unwrap()).unwrap();
        receipt["board_id"]="fixture".into();receipt["boot_id"]="a".repeat(32).into();
        assert!(Layout::receipt(&receipt,&peer).is_ok());
        receipt["boot_id"]="b".repeat(32).into();assert!(Layout::receipt(&receipt,&peer).is_err());
    }
}
