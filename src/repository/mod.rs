use super::media::Media;
use crate::configurations::DatabaseConfig;
use deadpool_postgres::Pool;
use log::info;


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
            "SELECT id, url FROM {} WHERE id = '{}';",
            self.db_table, id
        );
        info!("Postgres: processing query to database: {}", &query_string);

        let client = self.pool.get().await?;
        let row = client.query_one(query_string.as_str(), &[]).await?;

        let id: String = row.get("id");
        let path: String = row.get("url");
        let media = Media { id, url: path };

        info!("Postgres: row returned correctly: {}", media);
        Ok(media)
    }

    ///Save a media object to the database
    pub async fn save(&self, media: Media) -> Result<String, Box<dyn std::error::Error>> {
        let query_string = format!(
            "INSERT INTO {} (id, url) VALUES ('{}', '{}');",
            self.db_table, media.id, media.url
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
