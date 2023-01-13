mod macos;

mod platform_specific {
    #[cfg(target_os = "macos")]
    pub use super::macos::*;

    #[cfg(target_os = "linux")]
    pub use super::linux::*;

    #[cfg(target_os = "windows")]
    pub use super::windows::*;

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    pub use super::fallback::*;
}

pub fn init_notifications() {
    platform_specific::init_notifications();
}

pub fn send_notification(summary: &str, body: &str) -> Result<(), ()> {
    platform_specific::send_notification(summary, body)
}
