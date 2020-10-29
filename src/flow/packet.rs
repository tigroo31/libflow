use crate::flow::flag::Flag;
use pcap_parser;
use std::fmt;

pub(crate) struct Packet<'p> {
    packet: pcap_parser::PcapBlock<'p>,
    header: pcap_parser::PcapBlock<'p>,
    flag_list: Vec<Flag>,
    tcp_window: u32, // TODO challenge the type
    payload: u32,    // TODO challenge the type
}

impl<'p> fmt::Debug for Packet<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{},{}", self.flag_list, self.tcp_window, self.payload)
    }
}
