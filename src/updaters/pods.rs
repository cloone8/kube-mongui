use std::{thread::{self, sleep}, time::Duration};

use crate::util::request_util;

pub(crate) fn start(ui_info: &mut crate::KubeMonGUI) -> Result<(), ()> {
    let selected_namespace = ui_info.selected_namespace.clone();
    let ip = ui_info.proxy.listen_addr.ip();
    let port = ui_info.proxy.listen_addr.port();

    thread::spawn(move || {
        loop {
            let url = {
                let locked_namespace = selected_namespace.lock();

                match locked_namespace.as_ref() {
                    Some(ns) => Some(format!(
                        "http://{}:{}/api/v1/namespaces/{}/pods",
                        ip,
                        port,
                        ns
                    )),
                    None => None,
                }
            };

            if let Some(url) = url {
                let json_response = request_util::get_json_from_url(url.as_str());
            }

            sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}
