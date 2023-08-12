use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq,Clone)]
pub struct Media {
    pub id: String,
    pub path: String,
}

impl Media {
    pub fn new(id: String, path: String) -> Self {
        Media { id, path }
    }
}

impl Display for Media {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Media {{id: \"{}\", path: \"{}\"}}", self.id, self.path)
    }
}
