use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use config::{Config, FileFormat};
use configurations::AppConfig;
use core::panic;
use image::EncodableLayout;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use service::Service;
use std::sync::Mutex;
mod broker;
mod configurations;
mod error;
mod media;
mod repository;
mod response;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //get config from .toml file
    let app_config = get_app_config();

    //create new service with the config imported
    let service_result = Service::new(app_config.clone()).await;
    let service = match service_result {
        Ok(service) => service,
        Err(e) => panic!("Error creating the media service: {}", e),
    };

    let address = format!("{}:{}", &app_config.host, &app_config.port);
    println!("Media-service started at address {}.", &address);

    service.create_storage_folders();
    service.initialize_broker().await;

    let data = Data::new(Mutex::new(service));
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(root)
            .service(get_media)
            .service(publish)
            .app_data(web::PayloadConfig::new(50_242_880))
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "Couldnt start the service at address: {}, Error-> {:?}",
            &address, err
        )
    })
    .run()
    .await
}

#[get("/api")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("root directory")
}

/// Retrieves a media file from the server by id param
#[get("api/get")]
async fn get_media(request: HttpRequest, service: Data<Mutex<Service>>) -> impl Responder {
    let service = service.lock().unwrap(); // unwrap service
    let result: Result<NamedFile, error::Error> = service.get_media(&request).await;

    match result {
        Ok(file) => file.into_response(&request),
        //TODO error types
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

/// Uploads an image to the server. This method listens to a messageQ waiting for new insert or update requests
// async fn upload_image(media: web::Bytes, service: Data<Mutex<Service>>) -> impl Responder {
//     let service = service.lock().unwrap(); // unwrap service
    
//     let result = service.upload(&media.to_vec());
//     match result {
//         Ok(response) => HttpResponse::Ok().json(response),
//         Err(error) => HttpResponse::NotAcceptable().json(error),
//     }
// }

async fn tmp() -> Result<(), lapin::Error> {
    let address = "amqp://guest:guest@localhost:5672";
    let con = Connection::connect(&address, ConnectionProperties::default()).await?;


    let file_data = std::fs::read("E:/root/images/3rst24mFeWHDWkpzcl1X.jpg")?;
    let channel = con.create_channel().await?;

    let _ = channel
        .queue_declare(
            "queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .basic_publish(
            "",
            "queue",
            BasicPublishOptions::default(),
            file_data.as_bytes(),
            BasicProperties::default(),
        )
        .await?;
    con.close(0, "Cierre de la conexión").await?;

    println!("Archivo enviado a la cola con éxito!");
    Ok(())
}

///Tries deserilizing the app configuration struct from the file specified in the add_source method and returns a AppConf struct with all the app configuration params.
/// Panics if error occurred.
pub fn get_app_config() -> AppConfig {
    let config: Config = Config::builder()
        .add_source(config::File::new("app-config", FileFormat::Toml))
        .build()
        .expect("Error loading the app configuration");
    let app_config = config
        .try_deserialize::<AppConfig>()
        .expect("Error deserializing the app configuration.");
    app_config
}



#[get("/api/publish")]
async fn publish() -> impl Responder {
    let res = tmp().await;
    match res {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    HttpResponse::Ok().body("published")
}
