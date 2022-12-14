use std::{str::FromStr, collections::BTreeMap};

use k8s_openapi::{apimachinery::pkg::api::resource::Quantity, api::core::v1::ResourceRequirements};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ContainerInfo {
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub ready: bool,
    pub resources: Option<ContainerResources>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ContainerStatus {
    Running,
    Waiting,
    Terminated
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ContainerResources {
    pub requests: Option<ContainerResource>,
    pub limits: Option<ContainerResource>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ContainerResource {
    pub cpu: Option<String>,
    pub memory: Option<String>
}

impl FromStr for ContainerStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Running" => Ok(ContainerStatus::Running),
            "Waiting" => Ok(ContainerStatus::Waiting),
            "Terminated" => Ok(ContainerStatus::Terminated),
            _ => Err(())
        }
    }
}

impl From<&BTreeMap<String, Quantity>> for ContainerResource {
    fn from(r: &BTreeMap<String, Quantity>) -> Self {
        ContainerResource {
            cpu: r.get("cpu").map(|cpu| cpu.0.clone()),
            memory: r.get("memory").map(|memory| memory.0.clone()),
        }
    }
}

impl From<&ResourceRequirements> for ContainerResources {
    fn from(r_req: &ResourceRequirements) -> Self {
        ContainerResources {
            requests: r_req.requests.as_ref().map(ContainerResource::from),
            limits: r_req.limits.as_ref().map(ContainerResource::from),
        }
    }
}
