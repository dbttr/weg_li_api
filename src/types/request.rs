#[derive(Debug, Clone)]
pub struct RetrySettings {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub backoff_multiplier: u64,
}
