use redis::ConnectionAddr::Tcp;
use redis::aio::MultiplexedConnection;
use redis::{
    AsyncTypedCommands, Client, ConnectionInfo, ProtocolVersion, RedisConnectionInfo, RedisResult,
};
use std::error::Error;
use std::sync::OnceLock;
use tracing::info;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn init(conf: Redis) -> Result<(), Box<dyn Error>> {
    let c = Client::open(ConnectionInfo {
        addr: Tcp(conf.addr.clone(), conf.port),
        redis: RedisConnectionInfo {
            db: conf.db,
            username: conf.username.clone(),
            password: conf.password.clone(),
            protocol: ProtocolVersion::RESP3,
        },
    })?;
    CLIENT.set(c).unwrap();
    info!("Connected to Redis：{}", redis().await?.ping().await?);
    Ok(())
}

fn client() -> &'static Client {
    CLIENT.get().unwrap()
}

pub async fn redis() -> RedisResult<MultiplexedConnection> {
    client().get_multiplexed_tokio_connection().await
}

pub struct Redis {
    pub addr: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: u16,
    pub db: i64,
}
