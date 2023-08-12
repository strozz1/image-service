use super::super::{configurations::AppConfig, repository::MediaRepository};
use rdkafka::{error::KafkaError, producer::BaseProducer, ClientConfig};

pub struct Producer {
    pub repository: MediaRepository,
    pub producer: BaseProducer,
    pub config: AppConfig,
}

impl Producer {
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
        let producer: BaseProducer = client_config.create()?;
        Ok(Producer {
            repository,
            producer,
            config,
        })
    }
}
