use crate::input::normalize::{InputEvent, RemapAction};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventFilter {
    All,
    PhysicalOnly,
    InjectedOnly,
    RemappedOnly,
}

pub struct EventMonitor {
    buffer: VecDeque<InputEvent>,
    max_capacity: usize,
    pub is_paused: bool,
    pub filter: EventFilter,
}

impl EventMonitor {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_capacity),
            max_capacity,
            is_paused: false,
            filter: EventFilter::All,
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        if self.is_paused {
            return;
        }
        if self.buffer.len() >= self.max_capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(event);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn filtered_events(&self) -> Vec<&InputEvent> {
        self.buffer
            .iter()
            .filter(|ev| match self.filter {
                EventFilter::All => true,
                EventFilter::PhysicalOnly => !ev.is_injected(),
                EventFilter::InjectedOnly => ev.is_injected(),
                EventFilter::RemappedOnly => matches!(ev.action, RemapAction::Remapped { .. }),
            })
            .collect()
    }

    pub fn to_clipboard_text(&self) -> String {
        let mut out = String::new();
        out.push_str("Timestamp\tState\tVirtualKey\tScanCode\tOrigin\tAction\tDeviceHandle\n");
        for ev in &self.buffer {
            out.push_str(&format!(
                "{}\t{:?}\t{:?} (0x{:02X})\t0x{:04X}\t{:?}\t{:?}\t0x{:X}\n",
                ev.timestamp.format("%H:%M:%S%.3f"),
                ev.state,
                ev.vkey,
                ev.raw_vk,
                ev.scan_code,
                ev.origin,
                ev.action,
                ev.device_handle_raw
            ));
        }
        out
    }
}
