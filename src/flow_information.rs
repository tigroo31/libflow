use serde::{Deserialize, Serialize};

use crate::packet::Packet;

/// The flow information.
/// It contains forward and backward packet's.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FlowInformation {
    /// backward packet list
    pub backward_packet_list: Vec<Packet>,
    /// forward packet list
    pub forward_packet_list: Vec<Packet>,
}

impl FlowInformation {
    /// Provide a flow information with empty packet lists for now.
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use crate::flow_information::FlowInformation;

    #[test]
    fn test_default() {
        let default = FlowInformation::default();
        assert!(default.backward_packet_list.is_empty());
        assert!(default.forward_packet_list.is_empty());
    }

    #[test]
    fn test_new() {
        let new = FlowInformation::new();
        assert!(new.backward_packet_list.is_empty());
        assert!(new.forward_packet_list.is_empty());
    }
}
