use crate::flow::Flow;
use std::io::Read;

pub struct Generator {}

impl<'p> Generator {
    pub fn load<R>(reader: R) -> Vec<Flow<'p>>
    where
        R: Read,
    {
        // TODO read the file
        Vec::new()
    }
}
