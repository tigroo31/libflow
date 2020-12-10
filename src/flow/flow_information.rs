use crate::flow::information::Information;
use serde::Serialize;

/// The flow information.
/// It contains forward and backward information's.
#[derive(Debug, Default, Serialize)]
pub struct FlowInformation {
    /// forward information
    pub forward_information: Information,
    /// backward information
    pub backward_information: Information,
}
