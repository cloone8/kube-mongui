#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeInfo {
    pub name: String,
    pub arch: String,
    pub os: OsDetail,
    pub conditions: Vec<NodeCondition>,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct OsDetail {
    pub name: String,
    pub image: Option<String>,
    pub kernel_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeCondition {
    pub condition_type: String,
    pub status: bool,
    pub last_heartbeat_time: String,
    pub last_transition_time: String,
    pub reason: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Address {
    pub address_type: String,
    pub address: String,
}
