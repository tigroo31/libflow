use std::cmp::Ordering::{Equal, Greater, Less};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use serde::Serialize;
use std::fmt;

/// The flow unique identifier.
/// A flow id is equal to
/// another flow with the same
/// transport protocol, src IP and port and dest IP and port.
/// A flow id is also equal to
/// another flow with the same
/// transport protocol, and the src IP and port from one
/// equal to the dest IP and port of the other.
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

impl FlowId {
    /// Create a Flow Id with IP addresses for src and dest provided as string.
    pub fn new(transport_protocol: u8, src: &str, dst: &str, src_port: u16, dst_port: u16) -> Self {
        let src_ip_addr = IpAddr::from_str(src).unwrap();
        let dst_ip_addr = IpAddr::from_str(dst).unwrap();

        FlowId {
            transport_protocol,
            src: src_ip_addr,
            dst: dst_ip_addr,
            src_port,
            dst_port,
        }
    }
}

impl Default for FlowId {
    /// Create a Flow Id with defaults values
    /// and "0.0.0.0" IP addresses for src and dest.
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
            ((self.src == other.src && self.src_port == other.src_port && self.dst == other.dst && self.dst_port == other.dst_port) ||
                // same reverse source and destination IP and port (forward), so equal too
                (self.src == other.dst && self.src_port == other.dst_port && self.dst == other.src && self.dst_port == other.src_port))
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

impl fmt::Display for FlowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}-{}",
            self.src, self.dst, self.src_port, self.dst_port, self.transport_protocol
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::net::{IpAddr, Ipv4Addr};

    use crate::flow_id::FlowId;

    fn build_local_flow_id() -> FlowId {
        FlowId::new(
            17, // UDP
            "127.0.0.1",
            "192.168.0.1",
            8001,
            8002,
        )
    }

    #[test]
    fn test_default() {
        let default = FlowId::default();
        assert_eq!(default.transport_protocol, 0);
        assert_eq!(default.src, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(default.dst, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(default.src_port, 0);
        assert_eq!(default.dst_port, 0);
    }

    #[test]
    fn test_local_new() {
        let new = build_local_flow_id();
        assert_eq!(new.transport_protocol, 17);
        assert_eq!(new.src, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(new.dst, IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(new.src_port, 8001);
        assert_eq!(new.dst_port, 8002);
    }

    #[test]
    #[should_panic]
    fn test_bad_src_new() {
        FlowId::new(
            17, // UDP
            "bad",
            "192.168.0.1",
            8001,
            8002,
        );
    }

    #[test]
    #[should_panic]
    fn test_bad_dst_new() {
        FlowId::new(
            17, // UDP
            "127.0.0.1",
            "192.168.0.KK",
            8001,
            8002,
        );
    }

    #[test]
    fn test_forward_eq() {
        let flow1 = build_local_flow_id();
        // the same
        let flow2 = build_local_flow_id();
        assert_eq!(flow1, flow2)
    }

    #[test]
    fn test_backward_eq() {
        let flow1 = build_local_flow_id();
        // the reverse
        let flow2 = FlowId::new(
            17, // UDP
            "192.168.0.1",
            "127.0.0.1",
            8002,
            8001,
        );
        assert_eq!(flow1, flow2)
    }

    #[test]
    fn test_forward_hash() {
        let mut hasher1 = DefaultHasher::new();
        let flow1 = build_local_flow_id();
        flow1.hash(&mut hasher1);
        // the same
        let mut hasher2 = DefaultHasher::new();
        let flow2 = build_local_flow_id();
        flow2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_backward_hash() {
        let mut hasher1 = DefaultHasher::new();
        let flow1 = build_local_flow_id();
        flow1.hash(&mut hasher1);
        // the reverse
        let mut hasher2 = DefaultHasher::new();
        let flow2 = FlowId::new(
            17, // UDP
            "192.168.0.1",
            "127.0.0.1",
            8002,
            8001,
        );
        flow2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_display() {
        let flow = build_local_flow_id();
        assert_eq!(flow.to_string(), "127.0.0.1-192.168.0.1-8001-8002-17")
    }
}
