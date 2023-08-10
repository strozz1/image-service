use crate::media::Media;

use super::broker::Broker;
use super::configurations::AppConfig;
use super::error::Error;
use super::error::ErrorType;
use super::get_app_config;
use super::repository::MediaRepository;
use super::response::Response;

use actix_files::NamedFile;
use actix_web::web;
use actix_web::HttpRequest;
use image::{DynamicImage, ImageError};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::iter;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct Params {
    id: String,
}
#[derive(Debug)]
pub struct Service {
    pub config: AppConfig,
    pub repo: MediaRepository,
}

impl Service {
    pub async fn new(config: AppConfig) -> Result<Self, postgres::Error> {
        //create connection to db and create storage folders
        let repo = Service::create_db_connection(config.clone()).await?;

        Ok(Service { config, repo })
    }

    pub async fn initialize_broker(&self) {
        let broker = Broker::new(self.config.broker_config.clone(), 5);
        println!("Broker initilized");
        let conn = if let Ok(conn) = broker.connect().await {
            conn
        } else {
            panic!("Cant connect to the message Queue");
        };

        println!("Connection stablished with the messageQ");
        println!("Generating consumers...");
        //broker.process_method(self, upload);
        let generate = broker.generate_consumers(&conn).await;
        match generate {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }

    ///upload an image data to the database and saves it in the file system
    pub async fn upload(&self, buffer: &Vec<u8>) -> Result<Response, Error> {
        //TODO refactor code
        let start_time = Instant::now();
        let id = self.generate_id();
        
        let image: Option<DynamicImage> = match image::load_from_memory(&buffer) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
        let result: DynamicImage = match image {
            Some(im) => im,
            None => return Err(Error::from(ErrorType::ErrorParsingFile)),
        };
        
        let extention = self.generate_extention(&result);
        let path = format!("{}/images/{}.{}", self.config.storage_path, &id, extention);
        println!("Saving file in path: {}.", &path);


        let save = self.repo.save(Media { id:id.clone(), path: path.clone() }).await;
        match save{
            Ok(idi)=>println!("ID new media: {}",idi),
            Err(e)=> println!("Error {}",e)
        };
        let res: Result<(), ImageError> = result.save(&path);

        let end_time = Instant::now();
        let duration = (end_time - start_time).as_millis();

        match res {
            Ok(_) => Ok(Response {
                image_id: id,
                path,
                message: "File uploaded successfully to the server".to_string(),
                duration,
            }),
            Err(err) => Err(Error {
                code: 5,
                reason: format!("{:?}", err),
            }),
        }
    }
    pub async fn get_media(&self, request: &HttpRequest) -> Result<NamedFile, Error> {
        let value = web::Query::<Params>::from_query(request.query_string());

        match value {
            Ok(query) => {
                let id = query.0;
                let path = format!("{}/{}", self.get_path(), id.id);
                let file_result = NamedFile::open_async(path).await;

                match file_result {
                    Ok(file) => Ok(file),
                    Err(err) => Err(Error {
                        code: 5,
                        reason: err.to_string(),
                    }),
                }
            }
            Err(e) => Err(Error {
                code: 5,
                reason: e.to_string(),
            }),
        }
    }

    pub fn get_path(&self) -> String {
        //TODO
        let root = self.config.storage_path.clone();
        root + "/images"
    }

    fn generate_id(&self) -> String {
        let mut rng = rand::thread_rng();
        let rand_string: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char) // Convert u8 to char
            .take(20)
            .collect();
        rand_string
    }

    fn generate_extention(&self, image: &DynamicImage) -> String {
        let format = image.color();

        match format {
            image::ColorType::Rgb8 => "jpg".to_string(),
            image::ColorType::Rgba8 => "png".to_string(),
            _ => "png".to_string(),
        }
    }
    pub fn create_storage_folders(&self) {
        let path = self.get_path();
        match fs::create_dir(&path) {
            Ok(_) => {
                println!("Folder '{}' created successfully.", path);
                println!(
                    "The data will be stored in the next path: {}.\nCreating folders...",
                    &path
                );
            }
            Err(err) => eprintln!("Error creating folder: {:?}", err),
        }
    }

    /// Connects to the database
    async fn create_db_connection(config: AppConfig) -> Result<MediaRepository, postgres::Error> {
        let db_conf = config.db_config;

        let repository = MediaRepository::new(
            db_conf.host,
            db_conf.port,
            db_conf.user,
            db_conf.password,
            db_conf.db,
            db_conf.table,
        )
        .await?;
        println!("DATABASE: Connected to the database successfuly");
        println!("DATABASE: Database ready for receiving querys");
        Ok(repository)
    }

    // Crea una instancia estática de Database utilizando lazy_static
}
