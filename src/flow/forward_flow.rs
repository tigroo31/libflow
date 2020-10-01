use crate::flow::sub_flow::SubFlow;

pub(crate) struct ForwardFlow<'p> {
    // TODO extend SubFlow using Object trait through the methods to mask substructure
    sub_flow: SubFlow<'p>,

    act_data_pkt: pcap_parser::PcapBlock<'p>,
    min_seg_size: u64,
}