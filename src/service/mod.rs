use std::time::Instant;

use super::error::Error;
use super::error::ErrorType;
use super::response::Response;
use rand::{distributions::Alphanumeric, Rng};
use std::iter;
use image::{DynamicImage,ImageError};


pub fn upload(buffer: &Vec<u8>,root: String) -> Result<Response, Error> {
    
    let start_time = Instant::now();

   let image: Option<DynamicImage>= match image::load_from_memory(&buffer) {
        Ok(image) => Some(image),
        Err(_) => None,
    };
    let result: DynamicImage= match image{
        Some(im) => im,
        None => return Err(Error::from(ErrorType::ErrorParsingFile)),
    };
    

    let id = generate_id();
    let extention= generate_extention(&result);
    let path= format!("{}/images/{}.{}",root,&id,extention);
    println!("Saving file in path: {}.",&path);

    let res: Result<(), ImageError> = result.save(&path);

    let end_time = Instant::now();
    let duration = (end_time - start_time).as_millis();

    match res{
        Ok(_)=> Ok(Response {
            image_id: id,
            path,
            message: "File uploaded successfully to the server".to_string(),
            duration
        }),
        Err(err)=> Err(Error { code: 5, reason: format!("{:?}",err) })
    } 
}


fn generate_id() -> String{
    let mut rng = rand::thread_rng();
    let rand_string: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char) // Convert u8 to char
        .take(20)
        .collect();
    rand_string
}

fn generate_extention(image: &DynamicImage) -> String{
    let format = image.color();

    match format {
        image::ColorType::Rgb8 => "jpg".to_string(),
        image::ColorType::Rgba8 => "png".to_string(),
        _ => "png".to_string(),
    }
}
