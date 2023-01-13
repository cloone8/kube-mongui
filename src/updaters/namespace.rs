use k8s_openapi::{ListResponse, api::core::v1::Namespace};

use crate::{KubeMonGUI, util::request_util};

use std::{thread::{self, sleep}};

pub(crate) fn start(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    let update_freq = ui_info.base_update_freq.mul_f64(5.0);
    let namespaces = ui_info.namespaces.clone();
    let selected_namespace = ui_info.selected_namespace.clone();
    let kube_url = ui_info.k8s_api.get_url();

    let url = format!("{}/api/v1/namespaces", kube_url);

    thread::spawn(move || {
        loop {
            let response = request_util::get_response_from_url::<ListResponse<Namespace>>(url.as_str());

            if let Ok(ListResponse::Ok(response)) = response { // Enter a new block/scope so we can ensure the mutexes are dropped before sleeping
                let mut namespaces = namespaces.lock();

                namespaces.clear();

                for retrieved_namespace in response.items.iter() {
                    let name = retrieved_namespace.metadata.name.as_ref().unwrap();

                    namespaces.push(name.clone());
                }

                let mut selected_namespace = selected_namespace.lock();

                if let Some(ns) = selected_namespace.as_ref() {
                    if !namespaces.contains(ns) {
                        *selected_namespace = None;
                    }
                };
            }

            sleep(update_freq);
        }
    });

    Ok(())
}
