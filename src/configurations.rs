use serde::Deserialize;

#[derive(Debug, Deserialize,Clone)]
pub struct AppConfig{
    pub host: String,
    pub port: u16,
    pub storage_path: String,
    pub broker_config: BrokerConfig,
    pub db_config: DatabaseConfig
}


#[derive(Debug, Deserialize,Clone)]
pub struct BrokerConfig{
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub queue: String
}


#[derive(Debug, Deserialize,Clone)]
pub struct DatabaseConfig{
    pub host: String,
    pub port: u16,
    pub db: String,
    pub table: String,
    pub user: String,
    pub password: String,
}

