use std::collections::HashSet;
use std::time::Duration;

use serde::Serialize;

use crate::flag::Flag;

#[derive(Debug, Default, Serialize)]
pub struct Packet {
    // number of bytes
    pub length: u64,
    /// timestamp
    pub timestamp: Duration,
    /// list of flags
    pub flag_list: HashSet<Flag>,
    /// layer 3 protocol (e.g IPv4, IPv6)
    pub network_protocol: u16,
    // position into the set considered
    pub position: usize,
}

impl Packet {
    /// Provide the default structure for now.
    pub fn new() -> Self {
        Packet { ..Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::Duration;

    use crate::packet::Packet;

    #[test]
    fn test_default() {
        let default = Packet::default();
        assert_eq!(default.length, 0);
        assert_eq!(default.timestamp, Duration::default());
        assert_eq!(default.flag_list, HashSet::default());
        assert_eq!(default.network_protocol, 0);
        assert_eq!(default.position, 0);
    }

    #[test]
    fn test_new() {
        let new = Packet::new();
        assert_eq!(new.length, 0);
        assert_eq!(new.timestamp, Duration::default());
        assert_eq!(new.flag_list, HashSet::default());
        assert_eq!(new.network_protocol, 0);
        assert_eq!(new.position, 0);
    }
}
