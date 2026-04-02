//! Async runtime for Taurine
//!
//! This module provides async/await support using Tokio runtime.
//! It allows non-blocking I/O operations and concurrent execution.

#[cfg(feature = "async")]
use tokio::runtime::Runtime;
#[cfg(feature = "async")]
use std::sync::Arc;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

/// Async runtime handle
#[cfg(feature = "async")]
pub struct AsyncRuntime {
    runtime: Arc<Runtime>,
    task_count: Arc<Mutex<usize>>,
}

#[cfg(feature = "async")]
impl AsyncRuntime {
    /// Create a new async runtime
    pub fn new() -> Result<Self, String> {
        let runtime = Runtime::new()
            .map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;
        
        Ok(Self {
            runtime: Arc::new(runtime),
            task_count: Arc::new(Mutex::new(0)),
        })
    }

    /// Get the Tokio runtime
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// Spawn a task
    pub fn spawn<F, T>(&self, future: F) -> tokio::task::JoinHandle<T>
    where
        F: futures::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    /// Run a future to completion
    pub fn block_on<F, T>(&self, future: F) -> T
    where
        F: futures::Future<Output = T>,
    {
        self.runtime.block_on(future)
    }

    /// Get task count
    pub async fn get_task_count(&self) -> usize {
        *self.task_count.lock().await
    }

    /// Increment task count
    pub async fn increment_task_count(&self) {
        let mut count = self.task_count.lock().await;
        *count += 1;
    }

    /// Decrement task count
    pub async fn decrement_task_count(&self) {
        let mut count = self.task_count.lock().await;
        *count = count.saturating_sub(1);
    }
}

#[cfg(feature = "async")]
impl Clone for AsyncRuntime {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            task_count: self.task_count.clone(),
        }
    }
}

#[cfg(feature = "async")]
impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create async runtime")
    }
}

/// Future value wrapper
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct FutureValue {
    inner: Arc<tokio::sync::broadcast::Sender<crate::value::Value>>,
}

#[cfg(feature = "async")]
impl FutureValue {
    /// Create a new future value
    pub fn new(value: crate::value::Value) -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(16);
        let _ = tx.send(value);
        Self {
            inner: Arc::new(tx),
        }
    }

    /// Get the value (blocks until available)
    pub async fn await_value(&self) -> crate::value::Value {
        let mut rx = self.inner.subscribe();
        rx.recv().await.unwrap_or(crate::value::Value::Nil)
    }

    /// Check if value is ready
    pub fn is_ready(&self) -> bool {
        true  // Simplified for now
    }
}

/// Async sleep function
#[cfg(feature = "async")]
pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

/// Async sleep function (seconds)
#[cfg(feature = "async")]
pub async fn sleep_secs(secs: f64) {
    tokio::time::sleep(tokio::time::Duration::from_secs_f64(secs)).await;
}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
    use super::*;

    #[test]
    fn test_async_runtime_creation() {
        let runtime = AsyncRuntime::new();
        assert!(runtime.is_ok());
    }

    #[tokio::test]
    async fn test_sleep() {
        let start = std::time::Instant::now();
        sleep_ms(100).await;
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 90);
    }
}
