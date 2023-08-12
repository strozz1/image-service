use self::subscriber::Subscriber;
use super::configurations::AppConfig;
use super::repository::MediaRepository;
use log::info;
use rdkafka::error::KafkaError;
pub mod subscriber;
pub mod producer;

#[derive(Debug, Clone)]
pub struct Broker {
    repository: MediaRepository,
    config: AppConfig,
    num_subscribers: u16,
}

impl Broker {
    pub fn new(repository: MediaRepository, config: AppConfig, num_subscribers: u16) -> Self {
        Broker {
            repository,
            config,
            num_subscribers,
        }
    }

    //Generate num_consumers number of consumers on their own thread.
    pub fn generate_subscribers(&self) -> Result<(), KafkaError> {
        for i in 0..self.num_subscribers {
            let sub = Subscriber::new(self.repository.clone(), self.config.clone())?;
            
            tokio::spawn(async move {
                sub.subscribe(&sub.config.broker_config.topic).unwrap();
                sub.consume().await;
            });
            info!("Created consumer n:{}", i);
        }
        Ok(())
    }
}
