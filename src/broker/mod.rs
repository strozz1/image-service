use crate::{get_app_config, service::Service};

use super::configurations::BrokerConfig;
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties, ConsumerDelegate,
};
use std::{sync::Arc, time::Duration};
use subscriber::ServiceConsumer;

mod subscriber;
pub struct Broker {
    pub config: BrokerConfig,
    pub connection_properties: ConnectionProperties,
    pub consume_options: BasicConsumeOptions,
    pub num_consumers: u16,
}
impl Broker {
    ///Create new broker with broker config values and start consumer number.
    pub fn new(config: BrokerConfig, num_consumers: u16) -> Self {
        Broker {
            config,
            connection_properties: ConnectionProperties::default(),
            consume_options: BasicConsumeOptions::default(),
            num_consumers,
        }
    }
    /// Connects to the MessageQ service and returns the connection.
    pub async fn connect(&self) -> Result<Connection, lapin::Error> {
        let conf = &self.config;
        let uri = format!(
            "amqp://{}:{}@{}:{}",
            conf.user, conf.user, conf.host, conf.port
        );

        let con = Connection::connect(&uri, self.connection_properties.clone()).await?;
        Ok(con)
    }
    ///generate self.num_consumers for receiving the data
    pub async fn generate_consumers(&self, con: &Connection) -> Result<(), lapin::Error> {
        for _ in 0..self.num_consumers {
            let handler = move |delivery_result| {
                let service_consumer = ServiceConsumer;
                service_consumer.on_new_delivery(delivery_result)
            };
            self.create_consumer(con, QueueDeclareOptions::default(), handler)
                .await?;
        }
        //Add time for the workers initilize
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn create_consumer<F>(
        &self,
        con: &Connection,
        queue_options: QueueDeclareOptions,
        handler: F,
    ) -> Result<(), lapin::Error>
    where
        F: Fn(DeliveryResult) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + ConsumerDelegate
            + 'static,
    {
        let channel = con.create_channel().await?;
        let queue_name = self.config.queue.clone();
        let _ = channel
            .queue_declare(&queue_name, queue_options, FieldTable::default())
            .await?;

        let options = self.consume_options.clone();

        let consumer = channel
            .basic_consume(queue_name.as_str(), "", options, FieldTable::default())
            .await
            .unwrap();
        println!(
            "Consumer listening queue '{}' with tag: '{}' created. State: {:?}",
            &queue_name,
            consumer.tag(),
            consumer.state()
        );
        //delegate action to handler
        consumer.set_delegate(handler);
        let service = Arc::new(Service::new(get_app_config()).await);
        let service_clone = Arc::clone(&service);
        consumer.set_delegate(
            move |delivery: Result<Option<lapin::message::Delivery>, lapin::Error>| {
                let service = Arc::clone(&service_clone); // Clonamos nuevamente para el cierre

                async move {
                    match delivery {
                        Ok(Some(delivery)) => {
                            let response= (*service).as_ref().unwrap().upload(&delivery.data).await;
                            match response{
                                Ok(_)=> {
                                    println!("Subscriber: Message processing complete",);
                                    delivery.ack(BasicAckOptions::default()).await.unwrap();
                                },
                                Err(e) =>{
                                    println!("Error on delivery: [code: {} reason: {}]",e.code,e.reason)
                                    //TODO  !ack
                                }
                            }
                        }
                        Ok(None) => {
                            println!("Subscriber: No message available");
                        }
                        Err(e) => {
                            println!("Subscriber: Error reading message: {}", e);
                        }
                    }
                }
            },
        );
        Ok(())
    }
}
