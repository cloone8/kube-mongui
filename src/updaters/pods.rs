use std::{
    str::FromStr,
    thread::{self, sleep},
    time::Duration,
};

use k8s_openapi::{
    api::core::v1::{Container, Pod},
    ListResponse,
};

use crate::{
    data::{
        container::{ContainerInfo, ContainerResources, ContainerStatus},
        pod::{PodInfo, PodStatus, QoSClass},
    },
    util::request_util,
};

fn get_container_info(container: &Container, pod: &Pod) -> ContainerInfo {
    let name = &container.name;
    let image = container.image.as_ref().unwrap();

    let my_status_field = match pod.status.as_ref() {
        Some(status) => match status.container_statuses.as_ref() {
            Some(cont_statuses) => cont_statuses
                .iter()
                .find(|cnt_status| &cnt_status.name == name),
            None => None,
        },
        None => None,
    };

    let status = match my_status_field {
        Some(status_field) => match status_field.state.as_ref() {
            Some(state) => {
                if state.running.is_some() {
                    ContainerStatus::Running
                } else if state.terminated.is_some() {
                    ContainerStatus::Terminated
                } else {
                    ContainerStatus::Waiting
                }
            }
            None => ContainerStatus::Waiting,
        },
        None => ContainerStatus::Waiting,
    };

    let ready = match my_status_field {
        Some(status_field) => status_field.ready,
        None => false,
    };

    let resources = container.resources.as_ref().map(ContainerResources::from);

    ContainerInfo {
        name: name.clone(),
        image: image.clone(),
        status,
        ready,
        resources,
    }
}

fn get_pod_info(pod: &Pod) -> PodInfo {
    let name = pod.metadata.name.as_ref().unwrap();

    let containers: Vec<ContainerInfo> = pod.spec.as_ref().map_or_else(Vec::new, |s| {
        s.containers
            .iter()
            .map(|container| get_container_info(container, pod))
            .collect()
    });

    let qos_class = match pod.status.as_ref() {
        Some(s) => s
            .qos_class
            .as_ref()
            .map(|q| QoSClass::from_str(q.as_str()).unwrap()),
        None => None,
    };

    let status = match pod.status.as_ref() {
        Some(s) => s
            .phase
            .as_ref()
            .map(|p| PodStatus::from_str(p.as_str()).unwrap()),
        None => None,
    };

    PodInfo {
        name: name.clone(),
        containers,
        qos_class: qos_class.unwrap(),
        status: status.unwrap(),
    }
}

pub(crate) fn start(ui_info: &mut crate::KubeMonGUI) -> Result<(), ()> {
    let selected_namespace = ui_info.selected_namespace.clone();
    let ip = ui_info.proxy.listen_addr.ip();
    let port = ui_info.proxy.listen_addr.port();
    let pods = ui_info.pods.clone();

    thread::spawn(move || loop {
        let url = {
            let locked_namespace = selected_namespace.lock();

            locked_namespace
                .as_ref()
                .map(|ns| format!("http://{}:{}/api/v1/namespaces/{}/pods", ip, port, ns))
        };

        if let Some(url) = url {
            let response = request_util::get_response_from_url::<ListResponse<Pod>>(url.as_str());

            if let Ok(ListResponse::Ok(response)) = response {
                let mut new_pods: Vec<PodInfo> = response.items.iter().map(get_pod_info).collect();

                let mut pods_locked = pods.lock();

                pods_locked.clear();
                pods_locked.append(&mut new_pods);
            }
        }

        sleep(Duration::from_secs(1));
    });

    Ok(())
}
