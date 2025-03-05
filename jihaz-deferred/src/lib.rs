pub mod icons;
pub mod message;
pub mod message_receiver;
pub mod plist;

pub fn sleep_for(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}
