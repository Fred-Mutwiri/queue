use chrono::{DateTime, Duration, Utc};

/// Returns the current UTC timestamp.
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Calculate exponential backoff.
/// increases wait time each time a job fails (like retrying after 2s, 4s, 8s, …).
/// Example: retry 1 → +2s, retry 2 → +4s, retry 3 → +8s.
pub fn exponential_backoff(retries: u32) -> Duration {
    let secs = 2u64.pow(retries.min(10)); // cap at 2^10 = 1024s
    Duration::seconds(secs as i64)
}
