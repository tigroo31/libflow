use std::collections::HashMap;
use std::io::Read;

use crate::flow_id::FlowId;
use crate::flow_information::FlowInformation;

pub struct Generator {}

impl<'p> Generator {
    pub fn load<R>(_reader: R) -> HashMap<FlowId, FlowInformation>
    where
        R: Read,
    {
        // TODO read the file
        HashMap::new()
    }
}
