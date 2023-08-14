use actix_files::Files;
use actix_web::{
    get,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use config::{Config, FileFormat};
use configurations::AppConfig;
use core::panic;
use log::{error, info};
use poolmanager::PoolManager;
use repository::MediaRepository;
use service::Service;
use std::sync::Mutex;

mod broker;
mod configurations;
mod media;
mod poolmanager;
mod repository;
mod response;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //get config from .toml file
    let app_config = get_app_config();

    let address = format!("{}:{}", &app_config.host, &app_config.port);
    info!("Multimedia-service started at address {}.", &address);

    //create new service with the config imported
    let manager = PoolManager::get_db_manager(app_config.db_config.clone()).await;
    let pool = PoolManager::get_pool(manager).unwrap();

    //init the repo and the service
    let repository = MediaRepository::new(pool, app_config.db_config.clone());


    let service = Service::new(app_config.clone(), repository.clone()).await;

    service.create_storage_folders();

    let init_result = service.initialize_broker();

    if let Err(e) = init_result {
        error!("Error initializing the broker: {}", e);
        panic!();
    } else {
        info!("Broker initilized correctly.");
    }

    let data = Data::new(Mutex::new(service.clone()));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(root)
            .service(get_media)
            .service(publish)
            .service(Files::new("/images", &app_config.storage_path).show_files_listing())
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
    let service = service.lock().expect("error"); // unwrap service

    let result = service.get_media(&request).await;

    match result {
        Ok(media) => HttpResponse::Ok().json(media),
        //TODO error types
        Err(err) => {
            if err.is::<tokio_postgres::Error>() {
                //TODO handle error
                let db_error: &tokio_postgres::Error =
                    err.downcast_ref::<tokio_postgres::Error>().unwrap();
                if let Some(_) = db_error.code() {
                    error!("Database error: {}", db_error);
                    HttpResponse::InternalServerError()
                        .json(format!("Database error: {}", db_error))
                } else {
                    HttpResponse::NotFound().json(format!("Database error: id not found"))
                }
            } else if err.is::<std::io::Error>() {
                let io_error: &std::io::Error = err.downcast_ref::<std::io::Error>().unwrap();
                HttpResponse::NotFound().json(format!("IO error: {}", io_error.to_string()))
            } else if err.is::<actix_web::error::QueryPayloadError>() {
                HttpResponse::BadRequest().json(format!("Query error: {}", err.to_string()))
            } else {
                error!("Unexpected error: {}", err);
                HttpResponse::InternalServerError()
                    .json(format!("Unexpected error: {}", err.to_string()))
            }
        }
    }
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
    HttpResponse::Ok().body("Publisher acted.")
}
