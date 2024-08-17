use std::{
    collections::HashMap,
    fmt::Display,
    sync::{atomic::AtomicI64, Arc},
};

use anyhow::Result;

pub struct AtomicMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AtomicMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl AtomicMetrics {
    pub fn new(keys: &[&'static str]) -> Self {
        let map = keys.iter().map(|&name| (name, AtomicI64::new(0))).collect();
        Self {
            data: Arc::new(map),
        }
    }

    pub fn incr(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        if let Some(counter) = self.data.get(key) {
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn desc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        if let Some(counter) = self.data.get(key) {
            counter.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
        Ok(())
    }
}

impl Display for AtomicMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            write!(
                f,
                "{}: {}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
