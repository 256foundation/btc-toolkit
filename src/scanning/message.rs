use asic_rs::data::miner::MinerData;

#[derive(Debug, Clone)]
pub enum ScanningMessage {
    MinerFound {
        group_name: String,
        miner: MinerData,
    },
    GroupTabSelected(String),
    GroupCompleted(String),
    GroupError {
        group_name: String,
        error: String,
    },
    AllScansCompleted,
    StopScan,
    BackToDashboard,
    OpenIpInBrowser(std::net::Ipv4Addr),
}
