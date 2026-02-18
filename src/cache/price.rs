// Price cache implementation

use std::time::{SystemTime, Duration};
use tokio::sync::RwLock;

#[allow(dead_code)]
pub struct PriceCacheEntry {
    pub price: f64,
    pub ethbtc: f64,
    pub timestamp: SystemTime,
}

pub struct PriceCache {
    data: RwLock<Option<PriceCacheEntry>>,
    duration: Duration,
}

impl PriceCache {
    pub const fn new(duration_secs: u64) -> Self {
        Self {
            data: RwLock::const_new(None),
            duration: Duration::from_secs(duration_secs),
        }
    }

    pub async fn get(&self) -> Option<(f64, f64)> {
        let cache = self.data.read().await;
        if let Some(ref cached) = *cache {
            if let Ok(elapsed) = cached.timestamp.elapsed() {
                if elapsed < self.duration {
                    return Some((cached.price, cached.ethbtc));
                }
            }
        }
        None
    }

    pub async fn set(&self, price: f64, ethbtc: f64) {
        let mut cache = self.data.write().await;
        *cache = Some(PriceCacheEntry {
            price,
            ethbtc,
            timestamp: SystemTime::now(),
        });
    }

    #[allow(dead_code)]
    pub async fn invalidate(&self) {
        let mut cache = self.data.write().await;
        *cache = None;
    }
}

// Global cache instance
use crate::config::constants::PRICE_CACHE_DURATION_SECS;
pub static GLOBAL_PRICE_CACHE: PriceCache = PriceCache::new(PRICE_CACHE_DURATION_SECS);
