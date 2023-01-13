use notify_rust::{Timeout, Notification, get_bundle_identifier_or_default, set_application};

pub fn send_notification(summary: &str, body: &str) -> Result<(), ()> {
    let notif_result = Notification::new()
        .summary(summary)
        .body(body)
        .timeout(Timeout::Default)
        .show();

    match notif_result {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn init_notifications() {
    // Try to find an application we can use for an icon.
    let bundle_ident = get_bundle_identifier_or_default("Terminal");

    match set_application(&bundle_ident) {
        Ok(_) => {},
        Err(e) => log::warn!("Could not set notification application origin: {}", e),
    }
}
