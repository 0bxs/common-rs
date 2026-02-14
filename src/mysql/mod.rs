use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{Connection, MySql, Pool};
use std::error::Error;
use std::sync::OnceLock;
use std::time::Duration;

static MYSQL: OnceLock<Pool<MySql>> = OnceLock::new();

pub async fn init(conf: Mysql) -> Result<(), Box<dyn Error>> {
    let pool = MySqlPoolOptions::new()
        .max_connections(conf.max_connection)
        .min_connections(conf.min_connection)
        .max_lifetime(conf.max_lifetime)
        .idle_timeout(conf.idle_timeout)
        .connect_lazy_with(
            MySqlConnectOptions::new()
                .host(&conf.host)
                .port(conf.port)
                .username(&conf.username)
                .password(&conf.password)
                .database(&conf.database),
        );
    MYSQL.set(pool).unwrap();
    Ok(mysql().acquire().await?.ping().await?)
}

pub fn mysql() -> &'static Pool<MySql> {
    MYSQL.get().unwrap()
}

#[derive(Debug)]
pub struct Mysql {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    // 设置最大连接数
    pub min_connection: u32,
    // 设置最小连接数为
    pub max_connection: u32,
    // 设置连接的最大生命周期
    pub max_lifetime: Duration,
    // 设置空闲超时时间
    pub idle_timeout: Duration,
}
