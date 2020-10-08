mod active;
mod flag;
mod forward_flow;
mod packet;
mod statistic;
mod sub_flow;

use crate::flow::active::Active;
use crate::flow::forward_flow::ForwardFlow;
use crate::flow::statistic::Statistic;
use crate::flow::sub_flow::SubFlow;
use libpcap_tools;
use std::cmp;

#[derive(Debug, Default)]
pub struct Flow<'p> {
    flow: libpcap_tools::Flow,
    num_bytes: usize,
    num_packets: usize,
    activity_timeout: u64,
    idle: Statistic,
    packet_length: Statistic,
    iat: Statistic,
    bidirectional: bool,

    active: Active,

    forward: ForwardFlow<'p>,

    backward: SubFlow<'p>,
}

impl<'p> Flow<'p> {
    pub fn new() -> Self {
        Flow { ..Default::default() }
    }
}

impl<'p> cmp::PartialEq for Flow<'p> {
    fn eq(&self, other: &Self) -> bool {
        self.flow == other.flow
    }
}

#[test]
fn test_new() {
    let flow = Flow::new();
    assert_eq!(Flow { ..Default::default() }, flow)
}
