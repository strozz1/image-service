use super::media::Media;
use core::panic;
use std::time::Duration;
use tokio_postgres::{Client, Config, Error, NoTls, row, Statement};
pub struct MediaRepository {
    pub client: Client,
    pub db_name: String,
    pub db_table: String,
}

impl MediaRepository {
    pub async fn new(
        db_host: String,
        port: u16,
        user: String,
        password: String,
        db_name: String,
        db_table: String,
    ) -> Result<Self, Error> {
        let mut config = Config::new();

        config
            .host(db_host.as_str())
            .port(port)
            .user(user.as_str())
            .password(password.as_str())
            .dbname(db_name.as_str())
            .connect_timeout(Duration::from_secs(5));

        let res = config.connect(NoTls).await; //TODO tls security

        match res {
            Ok((client, connection)) => Ok({
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                MediaRepository {
                    client,
                    db_name,
                    db_table,
                }
            }),
            Err(err) => Err(err),
        }
    }


    pub async fn search(&mut self, id: String) -> Result<Media, Error> {
        let query_string = format!("SELECT id, path FROM {} WHERE id = '{}';", self.db_table, id);

        println!("DATABASE: Processing query to database: {}",&query_string);
      
        
        let result = self.client.query_one(query_string.as_str(), &[]).await;

        match result {
            Ok(row) => {
                let id: String = row.get("id");
                let path: String = row.get("path");

                let media = Media { id, path };

                println!("DATABASE: row returned correctly: {}",media);
                Ok(media)
            }
            Err(err) => panic!("{}",err),
        }
    }

    /// check if the connection with the database is still alive.
    /// Timeout time is 5 secs
    pub  fn is_alive(&mut self) -> bool {
        let result = self.client.is_closed();
        match result {
            true => false,
            false => true,
        }
    }


    ///Save a media object to the database
    pub async fn save(&self, media: Media) -> Result<String,Error>{
        let query_string = format!("INSERT INTO {} (id, path) VALUES ('{}', '{}');",self.db_table,media.id,media.path);
        let result = self.client.execute(&query_string, &[]).await;
        match result{
            Ok(_) =>{
                //TODO what if rows == 0
                println!("DATABASE: Media saved in database: {}",media);
                Ok(media.id)
            },
            Err(err) =>Err(err),
        }
    } 
}
