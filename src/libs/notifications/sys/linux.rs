use notify_rust::{Timeout, Notification};

pub fn send_notification(summary: &str, body: &str) -> Result<(), ()> {
    let notif_result = Notification::new()
        .appname("kube-mongui")
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

}
