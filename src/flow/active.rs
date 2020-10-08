use crate::flow::statistic::Statistic;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Active {
    active: Statistic,
    active_first_seen: libpcap_tools::Duration,
    active_last_seen: libpcap_tools::Duration,
}
