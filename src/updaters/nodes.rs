use std::{thread::{self, sleep}, time::Duration};

use k8s_openapi::{api::core::v1::Node, ListResponse};

use crate::{util::request_util, data::node::{NodeInfo, OsDetail}};

fn get_node_info(node: &Node) -> NodeInfo {
    let node_name = node.metadata.name.clone().unwrap();
    let arch = node.status.as_ref().unwrap().node_info.as_ref().unwrap().architecture.clone();

    let os_detail = OsDetail {
        name: node.status.as_ref().unwrap().node_info.as_ref().unwrap().operating_system.clone(),
        kernel_version: Some(node.status.as_ref().unwrap().node_info.as_ref().unwrap().kernel_version.clone()),
        image: Some(node.status.as_ref().unwrap().node_info.as_ref().unwrap().os_image.clone()),
    };

    NodeInfo {
        name: node_name,
        arch: arch,
        os: os_detail,
        conditions: Vec::new(),
        addresses: Vec::new(),
    }
}

pub(crate) fn start(ui_info: &mut crate::KubeMonGUI) -> Result<(), ()> {
    let ip = ui_info.proxy.listen_addr.ip();
    let port = ui_info.proxy.listen_addr.port();
    let url = format!("http://{}:{}/api/v1/nodes", ip, port);

    let nodes = ui_info.nodes.clone();

    thread::spawn(move || loop {
        let response = request_util::get_response_from_url::<ListResponse<Node>>(url.as_str());

        if let Ok(ListResponse::Ok(response)) = response {
            let new_nodes: Vec<NodeInfo> = response.items.iter()
                .map(get_node_info)
                .collect();

            let mut nodes_locked = nodes.lock();
            nodes_locked.clear();
            nodes_locked.extend(new_nodes);
        }

        sleep(Duration::from_secs(1));
    });

    Ok(())
}
