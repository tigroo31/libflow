use std::collections::HashSet;
use std::time::Duration;

use log::info;
use serde::Serialize;

/// The information.
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct Information {
    // number of packet
    pub nb_packet: usize,
    // number of packet bytes
    pub nb_packet_byte: usize,
    /// layer 3 protocol set (e.g IPv4, IPv6)
    pub network_protocol: HashSet<u16>,
    /// timestamp of first seen packet
    pub first_seen_packet: Duration,
    /// timestamp of last seen packet
    pub last_seen_packet: Duration,
    // TODO check if position is relevant
    // position into the set considered
    //pub position: usize,
}

impl Information {
    /// Provide the default structure for now.
    pub fn new() -> Self {
        Information { ..Default::default() }
    }

    /// Add a packet to the information:
    /// add the data len, increase the counter and store the discovered protocol.
    pub fn add_packet(&mut self, nb_packet_byte: usize, network_protocol: u16) {
        self.nb_packet_byte += nb_packet_byte;
        self.nb_packet += 1;
        self.network_protocol.insert(network_protocol);
    }

    /// Update the duration:
    /// put as first a duration if the date is older
    /// and / or as last duration if it is newer
    /// than the current one.
    pub fn update_timestamp(&mut self, first_seen_packet: Duration, last_seen_packet: Duration) {
        if self.first_seen_packet.as_nanos() == 0 || first_seen_packet < self.first_seen_packet {
            self.first_seen_packet = first_seen_packet;
        } else {
            info!(
                "the provided duration {:?} isn't older than the current {:?}: we ignore it",
                first_seen_packet, self.first_seen_packet
            );
        }
        if last_seen_packet > self.last_seen_packet {
            self.last_seen_packet = last_seen_packet;
        } else {
            info!(
                "the provided duration {:?} isn't newer than the current {:?}: we ignore it",
                last_seen_packet, self.last_seen_packet
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::Duration;

    use crate::flow::information::Information;

    fn build_information() -> Information {
        let mut network_protocol_list = HashSet::new();
        network_protocol_list.insert(34525);
        Information {
            nb_packet: 1,
            nb_packet_byte: 100,
            network_protocol: network_protocol_list,
            first_seen_packet: Duration::new(1607509522, 1),
            last_seen_packet: Duration::new(1607509532, 2),
        }
    }

    fn build_network_protocol_list() -> HashSet<u16> {
        let mut network_protocol_list = HashSet::new();
        network_protocol_list.insert(34525);
        network_protocol_list.insert(6);
        network_protocol_list
    }

    #[test]
    fn test_default() {
        let default = Information::default();
        assert_eq!(default.nb_packet, 0);
        assert_eq!(default.nb_packet_byte, 0);
        assert_eq!(default.network_protocol, HashSet::default());
        assert_eq!(default.first_seen_packet, Duration::default());
        assert_eq!(default.last_seen_packet, Duration::default());
    }

    #[test]
    fn test_new() {
        let new = Information::new();
        assert_eq!(new.nb_packet, 0);
        assert_eq!(new.nb_packet_byte, 0);
        assert_eq!(new.network_protocol, HashSet::default());
        assert_eq!(new.first_seen_packet, Duration::default());
        assert_eq!(new.last_seen_packet, Duration::default());
    }

    #[test]
    fn test_add_packet() {
        let mut information = build_information();
        let network_protocol_list = build_network_protocol_list();

        information.add_packet(50, 6);
        assert_eq!(information.nb_packet, 2);
        assert_eq!(information.nb_packet_byte, 150);
        assert_eq!(information.network_protocol, network_protocol_list);
        assert_eq!(information.first_seen_packet, Duration::new(1607509522, 1));
        assert_eq!(information.last_seen_packet, Duration::new(1607509532, 2));
    }

    #[test]
    fn test_first_seen_default_update_timestamp() {
        let mut information = Information::default();
        information.update_timestamp(Duration::new(1607509512, 3), Duration::new(1607509527, 4));
        assert_eq!(information.nb_packet, 0);
        assert_eq!(information.nb_packet_byte, 0);
        assert_eq!(information.network_protocol.len(), 0);
        assert_eq!(information.first_seen_packet, Duration::new(1607509512, 3));
        assert_eq!(information.last_seen_packet, Duration::new(1607509527, 4));
    }

    #[test]
    fn test_first_seen_update_timestamp() {
        let mut information = build_information();
        information.update_timestamp(Duration::new(1607509512, 3), Duration::new(1607509527, 4));
        assert_eq!(information.nb_packet, 1);
        assert_eq!(information.nb_packet_byte, 100);
        assert_eq!(information.network_protocol.len(), 1);
        assert_eq!(information.network_protocol.iter().next(), Some(34525 as u16).as_ref());
        assert_eq!(information.first_seen_packet, Duration::new(1607509512, 3));
        assert_eq!(information.last_seen_packet, Duration::new(1607509532, 2));
    }

    #[test]
    fn test_last_seen_update_timestamp() {
        let mut information = build_information();
        information.update_timestamp(Duration::new(1607509527, 3), Duration::new(1607509542, 4));
        assert_eq!(information.nb_packet, 1);
        assert_eq!(information.nb_packet_byte, 100);
        assert_eq!(information.network_protocol.len(), 1);
        assert_eq!(information.network_protocol.iter().next(), Some(34525).as_ref());
        assert_eq!(information.first_seen_packet, Duration::new(1607509522, 1));
        assert_eq!(information.last_seen_packet, Duration::new(1607509542, 4));
    }

    #[test]
    fn test_both_update_timestamp() {
        let mut information = build_information();
        information.update_timestamp(Duration::new(1607509512, 3), Duration::new(1607509542, 4));
        assert_eq!(information.nb_packet, 1);
        assert_eq!(information.nb_packet_byte, 100);
        assert_eq!(information.network_protocol.len(), 1);
        assert_eq!(information.network_protocol.iter().next(), Some(34525).as_ref());
        assert_eq!(information.first_seen_packet, Duration::new(1607509512, 3));
        assert_eq!(information.last_seen_packet, Duration::new(1607509542, 4));
    }
}
