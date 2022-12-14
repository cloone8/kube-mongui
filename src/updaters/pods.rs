use std::{thread::{self, sleep}, time::Duration};

use crate::{util::request_util, data::{pod::PodInfo, container::ContainerInfo}};

pub(crate) fn start(ui_info: &mut crate::KubeMonGUI) -> Result<(), ()> {
    let selected_namespace = ui_info.selected_namespace.clone();
    let ip = ui_info.proxy.listen_addr.ip();
    let port = ui_info.proxy.listen_addr.port();
    let pods = ui_info.pods.clone();

    thread::spawn(move || {
        loop {
            let url = {
                let locked_namespace = selected_namespace.lock();

                locked_namespace.as_ref().map(|ns| {
                    format!(
                        "http://{}:{}/api/v1/namespaces/{}/pods",
                        ip,
                        port,
                        ns
                    )
                })
            };

            if let Some(url) = url {
                if let Ok(json_response) = request_util::get_json_from_url(url.as_str()) {
                    let new_pods_iter = json_response["items"].as_array().unwrap().iter()
                        .map(|pod| {
                            let name = pod["metadata"]["name"].as_str().unwrap().to_string();
                            let containers: Vec<ContainerInfo> = pod["spec"]["containers"].as_array().unwrap().iter()
                                .map(|container| {
                                    let name = container["name"].as_str().unwrap().to_string();
                                    let image = container["image"].as_str().unwrap().to_string();

                                    ContainerInfo {
                                        name,
                                        image,
                                    }
                                })
                                .collect();

                            PodInfo {
                                name,
                                containers
                            }
                        });

                    let mut pods_locked = pods.lock();

                    pods_locked.clear();
                    pods_locked.extend(new_pods_iter);
                    pods_locked.sort();
                }
            }

            sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}
