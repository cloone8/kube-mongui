use std::{thread::{self, sleep}, time::Duration};

use k8s_metrics::v1beta1::NodeMetrics;
use k8s_openapi::{api::core::v1::Node, ListResponse, List};

use crate::{util::request_util, data::node::{NodeInfo, OsDetail}};

fn get_node_info(node: &Node, _: Option<&NodeMetrics>) -> NodeInfo {

    // TODO: Clean up this disgusting mess
    let node_name = node.metadata.name.clone().unwrap();
    let arch = node.status.as_ref().unwrap().node_info.as_ref().unwrap().architecture.clone();

    let os_detail = OsDetail {
        name: node.status.as_ref().unwrap().node_info.as_ref().unwrap().operating_system.clone(),
        kernel_version: Some(node.status.as_ref().unwrap().node_info.as_ref().unwrap().kernel_version.clone()),
        image: Some(node.status.as_ref().unwrap().node_info.as_ref().unwrap().os_image.clone()),
    };

    // END TODO

    NodeInfo {
        name: node_name,
        arch: arch,
        os: os_detail,
        conditions: Vec::new(),
        addresses: Vec::new(),
    }
}

fn get_node_metric_pair<'a>(node: &'a Node, metrics: &'a Vec<NodeMetrics>) -> (&'a Node, Option<&'a NodeMetrics>) {
    for metric in metrics {
        if metric.metadata.name == node.metadata.name {
            return (node, Some(metric));
        }
    }

    log::debug!("No metrics found for node {}", node.metadata.name.as_ref().unwrap());

    (node, None)
}

fn get_new_nodes(node_response: List<Node>, metrics_response: Vec<NodeMetrics>) -> Vec<NodeInfo> {
    node_response.items.iter()
        .map(|node| get_node_metric_pair(node, &metrics_response))
        .map(|pair| get_node_info(pair.0, pair.1))
        .collect()
}

pub(crate) fn start(ui_info: &mut crate::KubeMonGUI) -> Result<(), ()> {
    let ip = ui_info.proxy.listen_addr.ip();
    let port = ui_info.proxy.listen_addr.port();

    let url = format!("http://{}:{}/api/v1/nodes", ip, port);
    let url_metrics = format!("http://{}:{}/apis/metrics.k8s.io/v1beta1/nodes", ip, port);

    let nodes = ui_info.nodes.clone();

    thread::spawn(move || loop {
        let response = request_util::get_response_from_url::<ListResponse<Node>>(url.as_str());
        let metrics_response = request_util::attempt_as_json::<NodeMetrics>(url_metrics.as_str());

        if let Ok(ListResponse::Ok(response)) = response {
            if let Ok(metrics_response) = metrics_response {

                let new_nodes: Vec<NodeInfo> = get_new_nodes(response, metrics_response);

                let mut nodes_locked = nodes.lock();

                nodes_locked.clear();
                nodes_locked.extend(new_nodes);
            }
        }

        sleep(Duration::from_secs(1));
    });

    Ok(())
}
