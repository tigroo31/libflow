use std::collections::BTreeSet;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::flag::Flag;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Packet {
    // number of bytes (potentially just the assembly segment)
    pub length: u64,
    // window size value
    pub window: Option<u16>,
    /// timestamp
    pub timestamp: Duration,
    /// list of flags
    pub flag_list: BTreeSet<Flag>,
    /// layer 3 protocol (e.g IPv4, IPv6)
    pub network_protocol: u16,
    /// layer 3 header size (number of bytes)
    pub network_header_length: Option<usize>,
    /// layer 3 payload size (number of bytes)
    pub network_payload_length: Option<usize>,
    // position into the set considered
    pub position: usize,
}

impl Packet {
    /// Provide the default structure for now.
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::time::Duration;

    use crate::packet::Packet;

    fn remove_whitespace(s: &str) -> String {
        s.split_whitespace().collect()
    }

    fn basic_packet() -> &'static str {
        return r#"
{
  "length": 66,
  "timestamp": {
    "secs": 1595325117,
    "nanos": 502092000
  },
  "flag_list": [],
  "network_protocol": 34525,
  "network_header_length": 5,
  "network_payload_length": 106,
  "position": 28456
}
"#;
    }

    fn complete_packet() -> &'static str {
        return r#"
{
  "length": 55,
  "window": 2893,
  "timestamp": {
    "secs": 1595325118,
    "nanos": 502092010
  },
  "flag_list": ["ACK","CWR","ECE","FIN","NS","PSH","RST","SYN","URG"],
  "network_protocol": 17,
  "network_header_length": 5,
  "network_payload_length": 105,
  "position": 1234
}
"#;
    }

    fn bad_packet_with_string_network_protocol() -> &'static str {
        return r#"
{
  "length": 44,
  "windows": 2882,
  "timestamp": {
    "secs": 1595325119,
    "nanos": 502092020
  },
  "flag_list": [],
  "network_protocol": "6",
  "network_header_length": 10,
  "network_payload_length": 104,
  "position": 2345
}
"#;
    }

    fn bad_packet_with_u32_position() -> &'static str {
        return r#"
{
  "length": 33,
  "windows": 2871,
  "timestamp": {
    "secs": 1595325120,
    "nanos": 502092030
  },
  "flag_list": [],
  "network_protocol": 6,
  "network_header_length": 7,
  "network_payload_length": 103,
  "position": 42424242424242424242
}
"#;
    }

    fn bad_packet_without_flag_list() -> &'static str {
        return r#"
{
  "length": 23,
  "windows": 2860,
  "timestamp": {
    "secs": 1595325130,
    "nanos": 502092031
  },
  "network_protocol": 6,
  "network_header_length": 7,
  "network_payload_length": 53,
  "position": 42
}
"#;
    }

    #[test]
    fn test_default() {
        let default = Packet::default();
        assert_eq!(default.length, 0);
        assert_eq!(default.window, None);
        assert_eq!(default.timestamp, Duration::default());
        assert_eq!(default.flag_list, BTreeSet::default());
        assert_eq!(default.network_protocol, 0);
        assert_eq!(default.network_payload_length, None);
        assert_eq!(default.network_header_length, None);
        assert_eq!(default.position, 0);
    }

    #[test]
    fn test_new() {
        let new = Packet::new();
        assert_eq!(new.length, 0);
        assert_eq!(new.window, None);
        assert_eq!(new.timestamp, Duration::default());
        assert_eq!(new.flag_list, BTreeSet::default());
        assert_eq!(new.network_protocol, 0);
        assert_eq!(new.network_payload_length, None);
        assert_eq!(new.network_header_length, None);
        assert_eq!(new.position, 0);
    }

    #[test]
    fn it_can_deserialize_then_serialize_a_basic_packet() {
        let json = basic_packet();
        let packet: Packet = serde_json::from_str(json).unwrap();
        assert_eq!(packet.timestamp.as_nanos(), 1595325117502092000);
        assert_eq!(serde_json::to_string(&packet).unwrap(), remove_whitespace(json));
    }

    #[test]
    fn it_can_deserialize_then_serialize_a_complete_packet() {
        let json = complete_packet();
        let packet: Packet = serde_json::from_str(json).unwrap();
        assert_eq!(packet.timestamp.as_nanos(), 1595325118502092010);
        assert_eq!(serde_json::to_string(&packet).unwrap(), remove_whitespace(json));
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_deserializing_a_packet_with_u32_position() {
        let json = bad_packet_with_u32_position();
        let _: Packet = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_deserializing_a_packet_with_string_network() {
        let json = bad_packet_with_string_network_protocol();
        let _: Packet = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_deserializing_a_packet_without_flag_list() {
        let json = bad_packet_without_flag_list();
        let _: Packet = serde_json::from_str(json).unwrap();
    }
}
