use crate::flow::packet::Packet;
use crate::flow::statistic::Statistic;

pub(crate) struct SubFlow<'p> {
    flow: libpcap_tools::Flow,
    packet_list: Vec<Packet<'p>>,

    iat: Statistic,
    init_win_bytes: u32,
    urg_counter: u32,
    psh_counter: u32,
}