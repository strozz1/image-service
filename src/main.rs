use std::fs;

use actix_files::NamedFile;
use actix_files::Files;
use actix_web::http::header::USER_AGENT;
use actix_web::http::uri::Port;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, get, HttpRequest, http::header::{ContentType, ContentDisposition}};
use repository::MediaRepository;
use serde_json::to_string;
mod error;
mod response;
mod service;
mod repository;
mod media;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env variables
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));



    let port = std::env::var("PORT").expect("Environment variable 'PORT' not found!.");
    let host = std::env::var("HOST").expect("Environment variable 'HOST' not found!.");
    let data_root = std::env::var("ROOT").expect("Environment variable 'ROOT' not found!.");
    let address = format!("{}:{}", host, port);

    service::create_folders(&data_root);
    
    let media_repo = connect_database().await;
    

    println!("Media-service started at address {}.", &address);
    HttpServer::new(|| {
        App::new()
        .service(upload_image)
        .service(get_media)
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


/// Retrieves a media file from the server by id param
#[get("api/get")]
async fn get_media(request: HttpRequest) -> impl Responder{
       let result: Result<NamedFile, error::Error> = service::get_media(&request).await;
        match result{
            Ok(file) => {
                
                file.into_response(&request)
               
            },
            Err(err) => HttpResponse::BadRequest().json(err)
        }
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





async fn connect_database() -> MediaRepository {

    let host = std::env::var("DB_HOST").expect("Environment variable 'DB_HOST' not found!.");
    let name = std::env::var("DB_NAME").expect("Environment variable 'DB_NAME' not found!.");
    let table = std::env::var("DB_TABLE").expect("Environment variable 'DB_HOST' not found!.");
    let password = std::env::var("DB_PASSWORD").expect("Environment variable 'DB_PASSWORD' not found!.");
    let user = std::env::var("DB_USER").expect("Environment variable 'DB_USER' not found!.");
    let port = std::env::var("DB_PORT").expect("Environment variable 'DB_PORT' not found!.").parse::<u16>().expect("The port must be INT type");


    let result= MediaRepository::new(host,port,user,password,name,table).await;
    match result{
        Ok(mut media) =>{
            let is_alive= media.is_alive();
            
            match is_alive{
                true=>{
                    println!("DATABASE: the database is online and ready for receiving connections.");
                    media
                },
                false=>panic!("The database is not alive."),
            }
        },
        Err(error)=>panic!("{}",error),
    }
   
}

