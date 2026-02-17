use moka::Expiry;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Val<V> {
    pub v: V,
    pub ttl: Duration,
}

pub struct PerKeyExpiry;

impl<K, V> Expiry<K, Val<V>> for PerKeyExpiry {
    fn expire_after_create(&self, _key: &K, value: &Val<V>, _now: Instant) -> Option<Duration> {
        Some(value.ttl)
    }
}
