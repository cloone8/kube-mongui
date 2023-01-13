fn send_notification(_: &str, _: &str) -> Result<(), ()> {
    log::error!("Unsupported OS for notifications: {}", std::env::consts::OS);
    Err(())
}

pub fn init_notifications() {

}
