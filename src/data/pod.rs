use std::cmp::Ordering;

use super::container::ContainerInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PodInfo {
    pub name: String,
    pub containers: Vec<ContainerInfo>
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
