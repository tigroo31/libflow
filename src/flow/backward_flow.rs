use crate::flow::sub_flow::SubFlow;

pub(crate) struct BackwardFlow<'p> {
    // TODO extend SubFlow using Object trait through the methods to mask substructure
    sub_flow: SubFlow<'p>,
}