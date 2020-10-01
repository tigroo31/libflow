use pcap_parser;
use crate::flow::flag::Flag;

pub(crate) struct Packet<'p> {
    packet: pcap_parser::PcapBlock<'p>,
    header: pcap_parser::PcapBlock<'p>,
    flag_list: Vec<Flag>,
    tcp_window: u32, // TODO challenge the type
    payload: u32, // TODO challenge the type
}