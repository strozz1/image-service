use std::fs;
use std::time::Instant;

use super::error::Error;
use super::error::ErrorType;
use super::response::Response;
use actix_web::web;
use actix_web::HttpRequest;
use image::{DynamicImage, ImageError};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use serde::Serialize;
use std::iter;

use actix_files::NamedFile;

pub fn upload(buffer: &Vec<u8>, root: String) -> Result<Response, Error> {
    let start_time = Instant::now();

    let image: Option<DynamicImage> = match image::load_from_memory(&buffer) {
        Ok(image) => Some(image),
        Err(_) => None,
    };
    let result: DynamicImage = match image {
        Some(im) => im,
        None => return Err(Error::from(ErrorType::ErrorParsingFile)),
    };

    let id = generate_id();
    let extention = generate_extention(&result);
    let path = format!("{}/images/{}.{}", root, &id, extention);
    println!("Saving file in path: {}.", &path);

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

#[derive(Serialize, Deserialize)]
pub struct Params {
    id: String,
}

pub async fn get_media(request: &HttpRequest) -> Result<NamedFile, Error> {
    let value = web::Query::<Params>::from_query(request.query_string());

    match value {
        Ok(query) => {
            let id = query.0;
            let path = format!("{}/{}", get_path(), id.id);
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

fn get_path() -> String {
    let root = std::env::var("ROOT").expect("Environment variable 'ROOT' not found!.");
    root + "/images"
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let rand_string: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char) // Convert u8 to char
        .take(20)
        .collect();
    rand_string
}

fn generate_extention(image: &DynamicImage) -> String {
    let format = image.color();

    match format {
        image::ColorType::Rgb8 => "jpg".to_string(),
        image::ColorType::Rgba8 => "png".to_string(),
        _ => "png".to_string(),
    }
}
pub fn create_folders(data_root: &String) {
    let root = format!("{}/images", data_root);
    match fs::create_dir(&root) {
        Ok(_) => {
            println!("Folder '{}' created successfully.", root);
            println!("The data will be stored in the next path: {}.\nCreating folders...",&data_root);
        }
        Err(err) => eprintln!("Error creating folder: {:?}", err),
    }
}
