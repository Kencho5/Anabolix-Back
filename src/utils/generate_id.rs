use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_id() -> String {
    // Get the current time in milliseconds since UNIX_EPOCH
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let millis = elapsed.as_millis() as u64;

    // Generate a pseudo-random number based on the current time
    let id = millis % 90_000_000 + 10_000_000;

    id.to_string()
}
