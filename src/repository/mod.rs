use std::time::Duration;

use super::media::Media;
use crate::configurations::DatabaseConfig;
use deadpool::unmanaged::PoolError;
use deadpool_postgres::Pool;
use log::{error, info};
use tokio_postgres::Error;

#[derive(Debug, Clone)]
pub struct MediaRepository {
    pub pool: Pool,
    pub db_name: String,
    pub db_table: String,
}

impl MediaRepository {
    pub fn new(pool: Pool, db_config: DatabaseConfig) -> Self {
        MediaRepository {
            pool,
            db_name: db_config.db,
            db_table: db_config.table,
        }
    }

    pub async fn search(&mut self, id: String) -> Result<Media, Box<dyn std::error::Error>> {
        let query_string = format!(
            "SELECT id, path FROM {} WHERE id = '{}';",
            self.db_table, id
        );
        info!("Postgres: processing query to database: {}", &query_string);

        let client = self.pool.get().await?;
        let row = client.query_one(query_string.as_str(), &[]).await?;

        let id: String = row.get("id");
        let path: String = row.get("path");
        let media = Media { id, path };

        info!("Postgres: row returned correctly: {}", media);
        Ok(media)
    }

    /// check if the connection with the database is still alive.
    pub fn check_db_connection(&self) {
        let pool2 = self.pool.clone();
        tokio::spawn(async move {
            
            println!("Connection with database is still up!.");
            loop {
                let result = pool2.get().await;
                match result {
                    Ok(con) => {
                        if !con.is_closed() {
                            println!("Connection with database is still up!.");
                        } else {
                            error!("Connection with database is down");
                        }
                    }
                    Err(e) => error!("Error getting connection: {}", e),
                };

                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        });
    }

    ///Save a media object to the database
    pub async fn save(&self, media: Media) -> Result<String, Box<dyn std::error::Error>> {
        let query_string = format!(
            "INSERT INTO {} (id, path) VALUES ('{}', '{}');",
            self.db_table, media.id, media.path
        );

        info!("Postgres: processing query to database: {}", &query_string);

        let client = self.pool.get().await;
        let client = match client {
            Ok(o) => o,
            Err(e) => panic!("Error: {}", e),
        };
        let rows = client.execute(&query_string, &[]).await?;
        info!(
            "Postgres: Multimedia data saved in database: {}. Rows modified: {}",
            media, rows
        );
        Ok(media.id)
    }
}
