use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeInfo {
    pub name: String,
    pub hardware: HardwareDetail,
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
pub(crate) struct HardwareDetail {
    pub arch: String,
    pub capacity: Option<Hardware>,
    pub allocatable: Option<Hardware>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Hardware {
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub pods: Option<String>,
    pub ephemeral_storage: Option<String>,
    pub hugepages_2_mi: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeCondition {
    pub condition_type: String,
    pub status: Option<bool>,
    pub last_heartbeat_time: Option<DateTime<Utc>>,
    pub last_transition_time: Option<DateTime<Utc>>,
    pub reason: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Address {
    pub address_type: String,
    pub address: String,
}
