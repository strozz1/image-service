use crate::media::Media;

use super::configurations::AppConfig;
use super::repository::MediaRepository;
use super::broker::Broker;
use actix_files::NamedFile;
use actix_web::web;
use actix_web::HttpRequest;
use image::ColorType;
use log::{error, info};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rdkafka::error::KafkaError;
use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fs;
use std::iter;


///Parameters from the url
#[derive(Serialize, Deserialize)]
pub struct Params {
    id: String,
}
#[derive(Debug, Clone)]
pub struct Service {
    pub config: AppConfig,
    pub repository: MediaRepository,
    pub broker: Broker,
}

impl Service {
    pub async fn new(config: AppConfig, repository: MediaRepository) -> Self {
        let broker = Broker::new(repository.clone(), config.clone(), 5);
        Service {
            config,
            repository,
            broker,
        }
    }

    ///Initialize broker and create subscribers for incoming messages
    pub fn initialize_broker(&self) -> Result<(), KafkaError> {
        self.broker.generate_subscribers()?;
        Ok(())
    }

    pub async fn get_media(
        &self,
        request: &HttpRequest,
    ) -> Result<Media, Box<dyn error::Error>> {
        let query = web::Query::<Params>::from_query(request.query_string())?;

        //TODO error handle
        let id = query.0;
        let media = self.repository.clone().search(id.id).await?;

        Ok(media)
    }

    pub fn get_path(&self) -> String {
        //TODO
        let root = self.config.storage_path.clone();
        root
    }

    pub fn create_storage_folders(&self) {
        let path = self.get_path();
        match fs::create_dir(&path) {
            Ok(_) => {
                info!("Folder '{}' created successfully.", path);
                info!(
                    "The data will be stored in the next path: {}.\nCreating folders...",
                    &path
                );
            }
            Err(err) => error!("Error creating folder: {:?}", err),
        }
    }

    pub fn parse_path(id: &str, storage_path: &str, file_ext: &str) -> String {
        format!("{}/{}.{}", storage_path, id, file_ext)
    }
    ///generate extention for file
    pub fn generate_extention(format: &ColorType) -> String {
        match format {
            //TODO better extentions
            image::ColorType::Rgb8 => "jpg".to_string(),
            image::ColorType::Rgba8 => "png".to_string(),
            _ => "png".to_string(),
        }
    }

    ///generate random Id for multimedia file
    pub fn generate_id() -> String {
        let mut rng = rand::thread_rng();
        let rand_string: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char) // Convert u8 to char
            .take(20)
            .collect();
        rand_string
    }

    // Crea una instancia estática de Database utilizando lazy_static
}
