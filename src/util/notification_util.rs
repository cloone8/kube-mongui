use notify_rust::{Notification, Timeout};

pub fn notify_node_problem<'a>(node_name: &str, problems: impl Iterator<Item = &'a String>) {
    log::info!("Notifying user of node problem for node {}", node_name);

    let problems_fmt = problems
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    let notif_result = Notification::new()
        .summary(format!("{} has a problem", node_name).as_str())
        .body(format!("{} has the following problems:\n{}", node_name, problems_fmt).as_str())
        .timeout(Timeout::Default)
        .show();

        match notif_result {
            Ok(_) => {},
            Err(_) => {
                log::warn!("Failed to show notification for node {} with problems: {}", node_name, problems_fmt);
            },
        }
}
