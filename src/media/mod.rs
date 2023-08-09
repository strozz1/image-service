

use core::fmt;
use std::fmt::Display;

use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,Debug, PartialEq, Eq)]
pub struct Media{
    pub id: String,
    pub path: String,

}

impl Display for Media{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Media {{id: '{}', path: '{}'}}",self.id,self.path)
    }
}