mod sys;

pub fn init_notifications() {
    sys::init_notifications();
}

pub fn notify_node_problem<'a>(node_name: &str, problems: impl Iterator<Item = &'a String>) {
    log::info!("Notifying user of node problem for node {}", node_name);

    let problems_fmt = problems
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    let notif_result = sys::send_notification(
        format!("{} has a problem", node_name).as_str(),
        format!("{} has the following problems:\n{}", node_name, problems_fmt).as_str()
    );

    match notif_result {
        Ok(_) => {},
        Err(_) => {
            log::warn!("Failed to show notification for node {} with problems: {}", node_name, problems_fmt);
        },
    }
}
