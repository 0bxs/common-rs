use crate::no_rdb::redis;
use crate::utils::set::to_bytes;
use crate::utils::str::to_set;
use moka::future::Cache;
use redis::{AsyncTypedCommands, RedisError};
use std::collections::HashSet;
use std::sync::OnceLock;
use std::time::Duration;

static CACHE: OnceLock<Cache<i64, HashSet<i16>>> = OnceLock::new();
static KEY: OnceLock<String> = OnceLock::new();

pub async fn init(cap: u64, exp: Duration) {
    let cache = Cache::builder().max_capacity(cap).time_to_idle(exp).build();
    CACHE.set(cache).unwrap();
}

fn cache() -> &'static Cache<i64, HashSet<i16>> {
    CACHE.get().unwrap()
}

fn key0() -> &'static String {
    KEY.get().unwrap()
}

fn key(id: i64) -> String {
    format!("{}{}", key0(), id)
}

pub async fn get(id: i64) -> Result<Option<HashSet<i16>>, RedisError> {
    if let Some(v) = cache().get(&id).await {
        Ok(Some(v))
    } else {
        if let Some(old) = redis().await?.get(key(id)).await? {
            let set = to_set(old);
            cache().insert(id, set.clone()).await;
            return Ok(Some(set));
        }
        Ok(None)
    }
}

pub async fn set(id: i64, set: HashSet<i16>) -> Result<(), RedisError> {
    redis().await?.set(key(id), to_bytes(set.clone())).await?;
    cache().insert(id, set).await;
    Ok(())
}

pub async fn del(id: i64) -> Result<(), RedisError> {
    redis().await?.del(key(id)).await?;
    cache().remove(&id).await;
    Ok(())
}
