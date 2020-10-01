mod packet;
mod flag;
mod statistic;
mod forward_flow;
mod backward_flow;
mod sub_flow;

use libpcap_tools;
use crate::flow::statistic::Statistic;
use crate::flow::forward_flow::ForwardFlow;
use crate::flow::backward_flow::BackwardFlow;

struct Flow<'p> {
    flow: libpcap_tools::Flow,
    num_bytes: usize,
    num_packets: usize,
    activity_timeout: u64,
    idle: Statistic,
    length: Statistic,
    iat: Statistic,
    bidirectional: bool,

    // TODO create an active struct
    active: Statistic,
    active_first_seen: libpcap_tools::Duration,
    active_last_seen: libpcap_tools::Duration,

    forward_flow: ForwardFlow<'p>,

    backward_flow: BackwardFlow<'p>,
}