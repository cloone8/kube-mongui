use crate::{KubeMonGUI, util::request_util};

use std::{thread::{self, sleep}, time::Duration};

pub(crate) fn start(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    let namespaces = ui_info.namespaces.clone();
    let selected_namespace = ui_info.selected_namespace.clone();

    let url = format!("http://{}:{}/api/v1/namespaces", ui_info.proxy.listen_addr.ip(), ui_info.proxy.listen_addr.port());

    thread::spawn(move || {
        loop {
            let json_response = request_util::get_json_from_url(url.as_str());

            if let Ok(response) = json_response { // Enter a new block/scope so we can ensure the mutexes are dropped before sleeping

                let mut namespaces = namespaces.lock();

                namespaces.clear();

                for retrieved_namespace in response["items"].as_array().unwrap() {
                    let name = retrieved_namespace["metadata"]["name"].as_str().unwrap().to_string();

                    namespaces.push(name);
                }

                let mut selected_namespace = selected_namespace.lock();

                match selected_namespace.as_ref() {
                    Some(ns) => {
                        if !namespaces.contains(ns) {
                            *selected_namespace = None;
                        }
                    },
                    None => (),
                };
            }

            sleep(Duration::from_secs(5));
        }
    });

    Ok(())
}
