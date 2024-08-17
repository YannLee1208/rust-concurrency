use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex, RwLock},
};

use anyhow::{anyhow, Result};
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

#[derive(Debug, Clone)]
pub struct RwLockMetrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

pub struct DashMapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn desc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        Ok(data.clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RwLockMetrics {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn desc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self.data.read().map_err(|e| anyhow!(e.to_string()))?;
        Ok(data.clone())
    }
}

impl Display for RwLockMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().unwrap();
        write!(f, "{:?}", data)
    }
}

impl DashMapMetrics {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
    #[allow(dead_code)]

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn desc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl Display for DashMapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            write!(f, "{}:{} ", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metrics_new() {
        let metrics = Metrics::new();
        assert_eq!(metrics.snapshot().unwrap().len(), 0);
    }

    #[test]
    fn test_metrics_inc() {
        let metrics = Metrics::new();
        metrics.inc("key1").unwrap();
        metrics.inc("key1").unwrap();
        metrics.inc("key2").unwrap();
    }

    #[test]
    fn test_metrics_desc() {
        let metrics = Metrics::new();
        metrics.inc("key1").unwrap();
        metrics.desc("key1").unwrap();
        assert_eq!(metrics.snapshot().unwrap().get("key1"), Some(&0));
    }

    #[test]
    fn test_metrics_snapshot() {
        let metrics = Metrics::new();
        metrics.inc("key1").unwrap();
        metrics.inc("key1").unwrap();
        metrics.inc("key2").unwrap();
        let snapshot = metrics.snapshot().unwrap();
        assert_eq!(snapshot.get("key1"), Some(&2));
        assert_eq!(snapshot.get("key2"), Some(&1));
    }
}

#[cfg(test)]
mod test_rw_lock_metrics {
    use super::*;

    #[test]
    fn test_metrics_new() {
        let metrics = RwLockMetrics::new();
        assert_eq!(metrics.snapshot().unwrap().len(), 0);
    }

    #[test]
    fn test_metrics_inc() {
        let metrics = RwLockMetrics::new();
        metrics.inc("key1").unwrap();
        metrics.inc("key1").unwrap();
        metrics.inc("key2").unwrap();
    }

    #[test]
    fn test_metrics_desc() {
        let metrics = RwLockMetrics::new();
        metrics.inc("key1").unwrap();
        metrics.desc("key1").unwrap();
        assert_eq!(metrics.snapshot().unwrap().get("key1"), Some(&0));
    }

    #[test]
    fn test_metrics_snapshot() {
        let metrics = RwLockMetrics::new();
        metrics.inc("key1").unwrap();
        metrics.inc("key1").unwrap();
        metrics.inc("key2").unwrap();
        let snapshot = metrics.snapshot().unwrap();
        assert_eq!(snapshot.get("key1"), Some(&2));
        assert_eq!(snapshot.get("key2"), Some(&1));
    }

    #[test]
    fn test_metrics_display() {
        let metrics = RwLockMetrics::new();
        metrics.inc("key1").unwrap();
        metrics.inc("key1").unwrap();
        metrics.inc("key2").unwrap();
        println!("{}", metrics);
    }
}
