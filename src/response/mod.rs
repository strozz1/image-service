
use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize)]
pub struct Response{
    pub image_id: String,
    pub path: String,
    pub message: String,
    pub duration: u128,
}