pub mod message_receiver;

pub fn sleep_for(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}