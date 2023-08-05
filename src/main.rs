use std::fs;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use std::time::Instant;
mod error;
mod response;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env variables
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = std::env::var("PORT").expect("Environment variable 'PORT' not found!.");
    let host = std::env::var("HOST").expect("Environment variable 'HOST' not found!.");
    let data_root = std::env::var("ROOT").expect("Environment variable 'ROOT' not found!.");
    let address = format!("{}:{}", host, port);

    println!("Media-service started at address {}.", &address);
    println!("The data will be stored in the next path: {}.\nCreating folders...", &data_root);

    create_folders(&data_root);


    HttpServer::new(|| {
        App::new()
            .service(upload_image)
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


/// Uploads an image to the server
/// If upload was successfull returns a json Response object with image id
#[post("/api/upload")]
async fn upload_image(media: web::Bytes) -> impl Responder {
    let root = std::env::var("ROOT").expect("Environment variable 'ROOT' not found!.");
    let result = service::upload(&media.to_vec(),root);
    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::NotAcceptable().json(error),
    }
}



fn create_folders(data_root: &String) {
    //TODO create folders
    let root= format!("{}/images",data_root);
    match fs::create_dir(&root) {
        Ok(_) => println!("Folder '{}' created successfully.",root),
        Err(err) => eprintln!("Error creating folder: {:?}", err),
    }
}