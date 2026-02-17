use crate::no_rdb::redis;
use moka::future::Cache;
use redis::{AsyncTypedCommands, RedisError};
use std::sync::OnceLock;
use std::time::Duration;

static CACHE: OnceLock<Cache<i64, i64>> = OnceLock::new();
static KEY: OnceLock<String> = OnceLock::new();
static EXP: OnceLock<u64> = OnceLock::new();

pub fn init(key: String, cap: u64, exp: Duration) {
    let cache = Cache::builder().max_capacity(cap).time_to_idle(exp).build();
    CACHE.set(cache).unwrap();
    KEY.set(key).unwrap();
    EXP.set(exp.as_millis() as u64).unwrap();
}

fn cache() -> &'static Cache<i64, i64> {
    CACHE.get().unwrap()
}

fn key0() -> &'static str {
    KEY.get().unwrap().as_str()
}

fn key(id: i64) -> String {
    format!("{}{}", key0(), id)
}

fn exp() -> &'static u64 {
    EXP.get().unwrap()
}

pub async fn get(id: i64) -> Result<Option<i64>, RedisError> {
    if let Some(old) = cache().get(&id).await {
        Ok(Some(old))
    } else {
        let result = redis().await?.get_int(key(id)).await?;
        if let Some(old0) = result {
            let old1 = old0 as i64;
            cache().insert(id, old1).await;
            Ok(Some(old1))
        } else {
            Ok(None)
        }
    }
}

pub async fn set(id: i64, exp0: i64) -> Result<(), RedisError> {
    cache().insert(id, exp0).await;
    redis().await?.pset_ex(key(id), exp0, exp().clone()).await?;
    Ok(())
}

pub async fn del(id: i64) -> Result<(), RedisError> {
    cache().remove(&id).await;
    redis().await?.del(key(id)).await?;
    Ok(())
}

pub async fn kick_out(ids: Vec<i64>) -> Result<(), RedisError> {
    let mut keys = Vec::new();
    for id in ids {
        cache().remove(&id).await;
        keys.push(key(id));
    }
    redis().await?.del(keys).await?;
    Ok(())
}
