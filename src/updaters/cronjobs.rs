use std::{thread::{self, sleep}, time::Duration};

use k8s_openapi::{ListResponse, api::batch::v1::CronJob};

use crate::{KubeMonGUI, util::request_util};

pub(crate) fn start(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    let namespaces = ui_info.namespaces.clone();
    let selected_namespace = ui_info.selected_namespace.clone();
    let kube_url = ui_info.k8s_api.get_url();

    let url = format!("{}/apis/batch/v1/cronjobs", kube_url);

    thread::spawn(move || {
        loop {
            let response = request_util::get_response_from_url::<ListResponse<CronJob>>(url.as_str());

            if let Ok(ListResponse::Ok(response)) = response {
            //     let mut namespaces = namespaces.lock();

            //     namespaces.clear();

            //     for retrieved_namespace in response.items.iter() {
            //         let name = retrieved_namespace.metadata.name.as_ref().unwrap();

            //         namespaces.push(name.clone());
            //     }

            //     let mut selected_namespace = selected_namespace.lock();

            //     if let Some(ns) = selected_namespace.as_ref() {
            //         if !namespaces.contains(ns) {
            //             *selected_namespace = None;
            //         }
            //     };
            }

            sleep(Duration::from_secs(5));
        }
    });

    Ok(())
}
