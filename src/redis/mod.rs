use redis::aio::MultiplexedConnection;
use redis::{AsyncTypedCommands, Client, RedisResult};
use std::error::Error;
use std::sync::OnceLock;
use tracing::info;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn init(conf: Redis) -> Result<(), Box<dyn Error>> {
    let c = Client::open(format!(
        "redis://{}:{}@{}:{}/{} ",
        conf.username, conf.password, conf.addr, conf.port, conf.db
    ))?;
    CLIENT.set(c).unwrap();
    info!("Connected to Redis：{}", redis().await?.ping().await?);
    Ok(())
}

fn client() -> &'static Client {
    CLIENT.get().unwrap()
}

pub async fn redis() -> RedisResult<MultiplexedConnection> {
    client().get_multiplexed_async_connection().await
}

#[derive(Debug, Clone)]
pub struct Redis {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub db: i64,
}
