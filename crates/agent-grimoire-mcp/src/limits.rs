//! Limits on tool work: concurrency, rate and time (`MCP-SPEC`: servers MUST
//! rate-limit tool invocations), and output size.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::Semaphore;

/// Why a tool call was not run to completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// More calls arrived than the rate limit allows.
    RateLimited,
    /// Every worker slot is busy.
    Busy,
    /// The work took longer than the time limit.
    TimedOut,
    /// The work failed unexpectedly.
    Failed,
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Refused::RateLimited => "too many requests; wait a moment and try again",
            Refused::Busy => "the server is busy; try again shortly",
            Refused::TimedOut => {
                "the request took too long and was stopped; narrow it down and try again"
            }
            Refused::Failed => "the request failed unexpectedly",
        })
    }
}

/// Limit settings.
#[derive(Debug, Clone, Copy)]
pub struct LimitConfig {
    /// Calls that may run at once.
    pub max_concurrent: usize,
    /// Calls allowed per second, on average.
    pub per_second: u32,
    /// Calls allowed in a burst.
    pub burst: u32,
    /// The most time one call may take.
    pub timeout: Duration,
}

impl Default for LimitConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 8,
            per_second: 20,
            burst: 40,
            timeout: Duration::from_secs(10),
        }
    }
}

#[derive(Debug)]
struct Bucket {
    tokens: f64,
    last: Instant,
}

/// Runs blocking tool work within the limits. Shared by every call of a server.
#[derive(Debug, Clone)]
pub struct Limits {
    config: LimitConfig,
    permits: Arc<Semaphore>,
    bucket: Arc<Mutex<Bucket>>,
}

impl Limits {
    /// Limits with these settings.
    #[must_use]
    pub fn new(config: LimitConfig) -> Self {
        Self {
            permits: Arc::new(Semaphore::new(config.max_concurrent.max(1))),
            bucket: Arc::new(Mutex::new(Bucket {
                tokens: f64::from(config.burst.max(1)),
                last: Instant::now(),
            })),
            config,
        }
    }

    fn take_token(&self) -> bool {
        let Ok(mut bucket) = self.bucket.lock() else {
            return false;
        };
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last).as_secs_f64();
        bucket.last = now;
        let burst = f64::from(self.config.burst.max(1));
        bucket.tokens = (bucket.tokens + elapsed * f64::from(self.config.per_second)).min(burst);
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Runs `work` on the blocking pool if a slot and a rate token are free,
    /// and gives up waiting after the time limit. The slot stays taken until
    /// the work actually ends, so a timed-out call still counts.
    ///
    /// # Errors
    ///
    /// Returns why the work was refused or stopped.
    pub async fn run<T, F>(&self, work: F) -> Result<T, Refused>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let permit = Arc::clone(&self.permits)
            .try_acquire_owned()
            .map_err(|_| Refused::Busy)?;
        if !self.take_token() {
            return Err(Refused::RateLimited);
        }
        let task = tokio::task::spawn_blocking(move || {
            let result = work();
            drop(permit);
            result
        });
        match tokio::time::timeout(self.config.timeout, task).await {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(_)) => Err(Refused::Failed),
            Err(_) => Err(Refused::TimedOut),
        }
    }
}

/// `text` cut to at most `max_chars` characters, and whether it was cut.
#[must_use]
pub fn truncate(text: &str, max_chars: usize) -> (&str, bool) {
    match text.char_indices().nth(max_chars) {
        Some((i, _)) => (&text[..i], true),
        None => (text, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncation_counts_characters() {
        assert_eq!(truncate("héllo", 2), ("hé", true));
        assert_eq!(truncate("hé", 2), ("hé", false));
    }

    #[tokio::test]
    async fn the_rate_limit_and_the_timeout_apply() {
        let limits = Limits::new(LimitConfig {
            max_concurrent: 4,
            per_second: 0,
            burst: 2,
            timeout: Duration::from_millis(50),
        });
        assert_eq!(limits.run(|| 1).await, Ok(1));
        assert_eq!(
            limits
                .run(|| std::thread::sleep(Duration::from_millis(500)))
                .await,
            Err(Refused::TimedOut)
        );
        assert_eq!(limits.run(|| 1).await, Err(Refused::RateLimited));
    }

    #[tokio::test]
    async fn busy_slots_refuse_more_work() {
        let limits = Limits::new(LimitConfig {
            max_concurrent: 1,
            per_second: 100,
            burst: 100,
            timeout: Duration::from_secs(5),
        });
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        let slow = {
            let limits = limits.clone();
            tokio::spawn(async move { limits.run(move || rx.recv().ok()).await })
        };
        // Wait until the slow call holds the only slot.
        while limits.permits.available_permits() > 0 {
            tokio::task::yield_now().await;
        }
        assert_eq!(limits.run(|| 1).await, Err(Refused::Busy));
        tx.send(()).unwrap();
        assert_eq!(slow.await.unwrap(), Ok(Some(())));
    }
}
