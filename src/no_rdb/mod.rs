use bb8::{Pool};
use redis::{AsyncTypedCommands, RedisResult};
use std::error::Error;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::info;
use urlencoding::encode;
use bb8_redis::RedisConnectionManager;


static REDIS: OnceLock<Pool<RedisConnectionManager>> = OnceLock::new();

pub async fn init(conf: Redis) -> Result<(), Box<dyn Error>> {
    let pool_client = pool(&conf).await?;
    REDIS.set(pool_client).unwrap();
    info!("Connected to Redis (bb8)：{}", redis().get().await?.ping().await?);
    Ok(())
}

pub fn redis() -> &'static Pool<RedisConnectionManager> {
    REDIS.get().unwrap()
}

#[derive(Debug, Clone)]
pub struct Redis {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub db: i64,
    // 最大连接数
    pub max_size: u32,
    // 最小空闲连接数
    pub min_idle: Option<u32>,
    // 连接池最大生存时间
    pub max_lifetime: Option<Duration>,
    // 空闲超时时间
    pub idle_timeout: Option<Duration>,
    // 连接超时时间
    pub connection_timeout: Option<Duration>,
}

async fn pool(conf: &Redis) -> RedisResult<Pool<RedisConnectionManager>> {
    Pool::builder()
        .max_size(conf.max_size)
        .min_idle(conf.min_idle)
        .max_lifetime(conf.max_lifetime)
        .idle_timeout(conf.idle_timeout)
        .connection_timeout(conf.connection_timeout.unwrap_or(Duration::from_secs(30)))
        .test_on_check_out(true)
        .retry_connection(true)
        .build(client(conf)?)
        .await
}

fn client(conf: &Redis) -> RedisResult<RedisConnectionManager> {
    if conf.username.is_empty() {
        RedisConnectionManager::new(format!(
            "redis://:{}@{}:{}/{}",
            encode(conf.password.as_str()),
            conf.addr,
            conf.port,
            conf.db
        ))
    } else {
        RedisConnectionManager::new(format!(
            "redis://{}:{}@{}:{}/{}",
            encode(conf.username.as_str()),
            encode(conf.password.as_str()),
            conf.addr,
            conf.port,
            conf.db
        ))
    }
}
