use super::configurations::DatabaseConfig;
use deadpool_postgres::{BuildError, Manager, ManagerConfig, Pool, RecyclingMethod};
use std::time::Duration;
use tokio_postgres::{Config, NoTls};

pub struct PoolManager;
impl PoolManager {
    pub async fn get_db_manager(db_config: DatabaseConfig) -> Manager {
        let mut config = Config::new();
        config
            .host(db_config.host.as_str())
            .port(db_config.port)
            .user(db_config.user.as_str())
            .password(db_config.password.as_str())
            .dbname(db_config.db.as_str())
            .connect_timeout(Duration::from_secs(5));

        let manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        Manager::from_config(config, NoTls, manager_config)
    }

    pub fn get_pool(manager: Manager) -> Result<Pool, BuildError> {
        Pool::builder(manager).max_size(16).build()
    }
}
