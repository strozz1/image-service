use crate::{configurations::AppConfig, media::Media, service::Service};
use image::{DynamicImage, ImageError};
use log::{error, info};
use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    error::KafkaError,
    ClientConfig, Message,
};

use super::super::repository::MediaRepository;

pub struct Subscriber {
    pub repository: MediaRepository,
    pub consumer: BaseConsumer,
    pub config: AppConfig,
}

impl Subscriber {
    //Create basic subscriber and connects to the server
    pub fn new(repository: MediaRepository, config: AppConfig) -> Result<Self, KafkaError> {
        let mut client_config = ClientConfig::new();
        client_config
            .set("group.id", config.broker_config.group.clone())
            .set(
                "bootstrap.servers",
                format!(
                    "{}:{}",
                    config.broker_config.host, config.broker_config.port
                ),
            );
        let consumer: BaseConsumer = client_config.create()?;
        Ok(Subscriber {
            repository,
            consumer,
            config,
        })
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), KafkaError> {
        self.consumer.subscribe(&[topic])?;

        Ok(())
    }

    pub fn unsubscribe(&self) {
        self.consumer.unsubscribe();
    }

    ///consume de payload message
    pub async fn consume(&self) {
        loop {
            if let Some(result) = self.consumer.poll(None) {
                match result {
                    Ok(borrow) => {
                        if let Some(payload) = borrow.payload() {
                            if let Some(key) = borrow.key() {
                                info!("Received payload, key: {:?}, payload: {:?}", key, payload);
                                let process_result = self.process_data(payload).await;
                                match process_result {
                                    Ok(id) => {
                                        println!("ID: {}", id)
                                    }
                                    Err(e) => error!("Error saving the data: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => error!("Error reading message from queue: {}", e),
                }
            }
        }
    }

    async fn process_data(&self, payload: &[u8]) -> Result<String, ImageError> {
        let id = Service::generate_id();
        let image = image::load_from_memory(&payload)?;
        let file_ext = Service::generate_extention(&image.color());
        let path = Service::parse_path(
            id.as_str(),
            &self.config.storage_path.as_str(),
            file_ext.as_str(),
        );

        let media = Media::new(id.clone(), path.clone());
        let save_op = self.repository.save(media.clone()).await;
        match save_op {
            Ok(_) => info!("Multimedia data saved in database."),
            Err(e) => error!("Error saving file to database: {}", e),
        };

        self.store(path, image)?;
        info!("Multimedia file saved in storage.");
        Ok(id)
    }

    ///stores the file into the local storage
    fn store(&self, path: String, image: DynamicImage) -> Result<(), ImageError> {
        image.save(path)?;
        Ok(())
    }
}
