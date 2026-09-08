use crate::device::discovery::DeviceDiscovery;
use crate::device::identity::DeviceIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceDiff {
    Connected(DeviceIdentity),
    Disconnected(String), // Path of removed device
    NoChange,
}

pub struct ReconnectHandler;

impl ReconnectHandler {
    /// Compares current physical devices against a previous snapshot to detect connect/disconnect events.
    pub fn diff(previous: &[DeviceIdentity]) -> (Vec<DeviceIdentity>, Vec<DeviceDiff>) {
        let current = DeviceDiscovery::enumerate_all();
        let mut diffs = Vec::new();

        // Check for new arrivals
        for dev in &current {
            if !previous.iter().any(|p| p.path == dev.path) {
                diffs.push(DeviceDiff::Connected(dev.clone()));
            }
        }

        // Check for removals
        for prev in previous {
            if !current.iter().any(|c| c.path == prev.path) {
                diffs.push(DeviceDiff::Disconnected(prev.path.clone()));
            }
        }

        (current, diffs)
    }
}
