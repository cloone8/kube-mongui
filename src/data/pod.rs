use std::{cmp::Ordering, str::FromStr};

use super::container::ContainerInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PodInfo {
    pub name: String,
    pub status: PodStatus,
    pub containers: Vec<ContainerInfo>,
    pub qos_class: QoSClass
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PodStatus {
    Running,
    Pending,
    Succeeded,
    Failed,
    Unknown
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum QoSClass {
    Guaranteed,
    Burstable,
    BestEffort
}

impl PartialOrd for PodInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PodInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl FromStr for PodStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Running" => Ok(PodStatus::Running),
            "Pending" => Ok(PodStatus::Pending),
            "Succeeded" => Ok(PodStatus::Succeeded),
            "Failed" => Ok(PodStatus::Failed),
            "Unknown" => Ok(PodStatus::Unknown),
            _ => Err(())
        }
    }
}

impl FromStr for QoSClass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Guaranteed" => Ok(QoSClass::Guaranteed),
            "Burstable" => Ok(QoSClass::Burstable),
            "BestEffort" => Ok(QoSClass::BestEffort),
            _ => Err(())
        }
    }
}
