use std::{
    collections::BTreeMap,
    thread::{self, sleep},
};

use k8s_metrics::v1beta1::NodeMetrics;
use k8s_openapi::{
    api::core::v1::Node, apimachinery::pkg::api::resource::Quantity, List, ListResponse,
};

use crate::{
    data::node::{Address, Hardware, HardwareDetail, NodeCondition, NodeInfo, OsDetail, NodeUsage},
    util::{request_util, k8s_openapi_util::{quantity_to_float, quantity_to_int}},
};

fn get_hardware_detail_instance(hardware_detail: &BTreeMap<String, Quantity>) -> Hardware {
    Hardware {
        cpu: hardware_detail.get("cpu").and_then(|x| quantity_to_float(x).ok()),
        memory: hardware_detail.get("memory").and_then(|x| quantity_to_int(x).ok()),
        pods: hardware_detail.get("pods").and_then(|x| quantity_to_int(x).ok()),
        ephemeral_storage: hardware_detail.get("ephemeral-storage").and_then(|x| quantity_to_int(x).ok()),
        hugepages_2_mi: hardware_detail.get("hugepages-2Mi").and_then(|x| quantity_to_int(x).ok()),
    }
}

fn get_condition(condition: &k8s_openapi::api::core::v1::NodeCondition) -> NodeCondition {
    NodeCondition {
        condition_type: condition.type_.clone(),
        status: match condition.status.as_str() {
            "True" => Some(true),
            "False" => Some(false),
            _ => None,
        },
        last_heartbeat_time: condition.last_heartbeat_time.as_ref().map(|x| x.0),
        last_transition_time: condition.last_transition_time.as_ref().map(|x| x.0),
        reason: condition.reason.clone(),
        message: condition.message.clone(),
    }
}

fn get_address(address: &k8s_openapi::api::core::v1::NodeAddress) -> Address {
    Address {
        address_type: address.type_.clone(),
        address: address.address.clone(),
    }
}

fn get_node_info(node: &Node, metrics: Option<&NodeMetrics>) -> NodeInfo {
    let node_status = node.status.as_ref().unwrap();
    let node_info = node_status.node_info.as_ref().unwrap();

    let node_name = node.metadata.name.clone().unwrap();

    let os_detail = OsDetail {
        name: node_info.operating_system.clone(),
        kernel_version: Some(node_info.kernel_version.clone()),
        image: Some(node_info.os_image.clone()),
    };

    let hardware_detail = HardwareDetail {
        arch: node_info.architecture.clone(),
        capacity: node_status
            .capacity
            .as_ref()
            .map(get_hardware_detail_instance),
        allocatable: node_status
            .allocatable
            .as_ref()
            .map(get_hardware_detail_instance),
    };

    let node_metrics: Option<NodeUsage> = metrics.map(|metrics| {
        let cpu_usage = quantity_to_float(&metrics.usage.cpu).ok();
        let memory_usage = quantity_to_int(&metrics.usage.memory).ok();

        NodeUsage {
            cpu: cpu_usage,
            memory: memory_usage,
            updated: metrics.timestamp.0,
        }
    });


    NodeInfo {
        name: node_name,
        os: os_detail,
        hardware: hardware_detail,
        usage: node_metrics,
        conditions: node_status
            .conditions
            .as_ref()
            .map_or(Vec::new(), |x| x.iter().map(get_condition).collect()),
        addresses: node_status
            .addresses
            .as_ref()
            .map_or(Vec::new(), |x| x.iter().map(get_address).collect()),
    }
}

fn get_node_metric_pair<'a>(
    node: &'a Node,
    metrics: &'a List<NodeMetrics>,
) -> (&'a Node, Option<&'a NodeMetrics>) {
    for metric in metrics.items.iter() {
        if metric.metadata.name == node.metadata.name {
            return (node, Some(metric));
        }
    }

    log::debug!(
        "No metrics found for node {}",
        node.metadata.name.as_ref().unwrap()
    );

    (node, None)
}

fn get_new_nodes(node_response: List<Node>, metrics_response: List<NodeMetrics>) -> Vec<NodeInfo> {
    node_response
        .items
        .iter()
        .map(|node| get_node_metric_pair(node, &metrics_response))
        .map(|pair| get_node_info(pair.0, pair.1))
        .collect()
}

pub(crate) fn start(ui_info: &mut crate::KubeMonGUI) -> Result<(), ()> {
    let kube_url = ui_info.k8s_api.get_url();

    let url = format!("{}/api/v1/nodes", kube_url);
    let url_metrics = format!("{}/apis/metrics.k8s.io/v1beta1/nodes", kube_url);

    let update_freq = ui_info.base_update_freq;
    let nodes = ui_info.nodes.clone();

    thread::spawn(move || loop {
        let response = request_util::get_response_from_url::<ListResponse<Node>>(url.as_str());
        let metrics_response = request_util::get_response_from_url::<ListResponse<NodeMetrics>>(url_metrics.as_str());

        if let Ok(ListResponse::Ok(response)) = response {
            if let Ok(ListResponse::Ok(metrics_response)) = metrics_response {
                let mut new_nodes: Vec<NodeInfo> = get_new_nodes(response, metrics_response);

                let mut nodes_locked = nodes.lock();

                nodes_locked.clear();
                nodes_locked.append(&mut new_nodes);
            }
        }

        sleep(update_freq);
    });

    Ok(())
}
