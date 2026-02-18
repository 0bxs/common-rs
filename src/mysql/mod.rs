use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::sync::OnceLock;
use std::time::Duration;
use tracing::log;

static MYSQL: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(conf: Mysql) -> Result<(), DbErr> {
    let url = format!(
        "mysql://{}:{}@{}/{}",
        conf.username, conf.password, conf.host, conf.database
    );
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(conf.max_connection)
        .min_connections(conf.min_connection)
        .connect_timeout(conf.connect_timeout)
        .acquire_timeout(conf.acquire_timeout)
        .idle_timeout(conf.idle_timeout)
        .max_lifetime(conf.max_lifetime)
        .sqlx_logging(conf.show_sqlx)
        .sqlx_logging_level(log::LevelFilter::Info);
    MYSQL.set(Database::connect(opt).await?).unwrap();
    mysql().ping().await?;
    Ok(())
}

pub fn mysql() -> &'static DatabaseConnection {
    MYSQL.get().unwrap()
}

#[derive(Debug, Clone)]
pub struct Mysql {
    pub host: String,
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
    // 设置连接超时时间
    pub connect_timeout: Duration,
    // 等待获取链接超时时间
    pub acquire_timeout: Duration,
    pub show_sqlx: bool,
}
