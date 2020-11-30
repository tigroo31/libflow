use serde::Serialize;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Serialize)]
pub struct FlowId {
    /// Layer 4 protocol (e.g TCP, UDP, ICMP)
    pub transport_protocol: u8,
    /// Source IP address
    pub src: IpAddr,
    /// Destination IP address
    pub dst: IpAddr,
    /// Source port. 0 if not relevant for protocol
    pub src_port: u16,
    /// Destination port. 0 if not relevant for protocol
    pub dst_port: u16,
}

impl Default for FlowId {
    fn default() -> Self {
        Self {
            transport_protocol: 0,
            src: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            dst: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            src_port: 0,
            dst_port: 0,
        }
    }
}

impl PartialEq for FlowId {
    fn eq(&self, other: &Self) -> bool {
        // same protocol
        self.transport_protocol == other.transport_protocol &&
            // same source and destination IP and port (backward), so equal
            (self.src == other.src && self.src_port == other.src_port && self.dst == other.dst && self.dst_port == other.dst_port) ||
            // same reverse source and destination IP and port (forward), so equal too
            (self.src == other.dst && self.src_port == other.dst_port && self.dst == other.src && self.dst_port == other.dst_port)
    }
}

impl Eq for FlowId {}

impl Hash for FlowId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // protocol hash
        self.transport_protocol.hash(state);
        match (self.src.cmp(&self.dst), self.src_port.cmp(&self.dst_port)) {
            (Less, _) | (Equal, Less) | (Equal, Equal) => {
                // source then destination IP and port (backward) hash
                self.src.hash(state);
                self.src_port.hash(state);
                self.dst.hash(state);
                self.dst_port.hash(state);
            }
            (Greater, _) | (Equal, Greater) => {
                // destination then source IP and port (forward) hash
                self.dst.hash(state);
                self.dst_port.hash(state);
                self.src.hash(state);
                self.src_port.hash(state);
            }
        }
    }
}

impl FlowId {
    pub fn new() -> Self {
        FlowId { ..Default::default() }
    }
}

#[test]
fn test_new() {
    let information = FlowId::new();
    // TODO implement tests
    assert_eq!(FlowId { ..Default::default() }, information)
}
