use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NodeInfo {
    pub name: String,
    pub os: OsDetail,
    pub hardware: HardwareDetail,
    pub usage: Option<NodeUsage>,
    pub conditions: Vec<NodeCondition>,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NodeUsage {
    pub cpu: Option<f64>,
    pub memory: Option<i64>,
    pub updated: DateTime<Utc>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct OsDetail {
    pub name: String,
    pub image: Option<String>,
    pub kernel_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HardwareDetail {
    pub arch: String,
    pub capacity: Option<Hardware>,
    pub allocatable: Option<Hardware>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Hardware {
    pub cpu: Option<f64>,
    pub memory: Option<i64>,
    pub pods: Option<i64>,
    pub ephemeral_storage: Option<i64>,
    pub hugepages_2_mi: Option<i64>,
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
