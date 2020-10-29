use crate::flow::sub_flow::SubFlow;
use std::fmt;

#[derive(Default)]
pub(crate) struct ForwardFlow<'p> {
    // TODO extend SubFlow using Object trait through the methods to mask substructure
    sub_flow: SubFlow<'p>,

    act_data_pkt: Option<pcap_parser::PcapBlock<'p>>,
    min_seg_size: u64,
}

impl<'p> fmt::Debug for ForwardFlow<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.sub_flow, self.min_seg_size)
    }
}
