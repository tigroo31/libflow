use libpcap_tools::Duration;
use serde::Serialize;

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct Information {
    // number of packet
    pub nb_packet: usize,
    // number of packet bytes
    pub nb_packet_byte: usize,
    /// timestamp of first packet
    pub first_seen_packet: Duration,
    /// timestamp of last seen packet
    pub last_seen_packet: Duration,
    // true if backward, false if forward
    pub backward: bool,
    /// Layer 3 protocol (e.g IPv4, IPv6)
    pub network_protocol: u16,
}

impl Information {
    pub fn new() -> Self {
        Information { ..Default::default() }
    }
}

#[test]
fn test_new() {
    let information = Information::new();
    // TODO implement tests
    assert_eq!(Information { ..Default::default() }, information)
}
