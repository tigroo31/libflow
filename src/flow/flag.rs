#[derive(Debug)]
pub(crate) enum Flag {
    FIN,
    PSH,
    URG,
    ECE,
    SYN,
    ACK,
    CWR,
    RST,
}
