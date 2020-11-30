use crate::flow::flow_id::FlowId;
use crate::flow::information::Information;
use std::collections::HashMap;
use std::io::Read;

pub struct Generator {}

impl<'p> Generator {
    pub fn load<R>(_reader: R) -> HashMap<FlowId, Information>
    where
        R: Read,
    {
        // TODO read the file
        HashMap::new()
    }
}
