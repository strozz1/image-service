use serde::{Serialize, Deserialize};

///
/// Struct for handling the service errors.
/// Contains the error code, the reason
#[derive(Serialize,Deserialize)]
pub struct Error {
    pub code: u8,
    pub reason: String
}

//Codes
// 01 file to big
// 02 file not found
// 03 error parsing file
// 04 Format not supported
// 05 Unknown error

impl Error {
    pub fn from(error: ErrorType) -> Error {
        let (code, reason) = match error {
            ErrorType::FileTooBig => (1, "File is too big".to_string()),
            ErrorType::FileNotFound => (2, "File not found".to_string()),
            ErrorType::ErrorParsingFile => (3, "Error formating the image".to_string()),
            ErrorType::FormatNotSupported => (4, "Format not supported".to_string()),
            ErrorType::UnknownError => (5, "Unknown error".to_string()),
        };
        Error { code, reason }
    }
}

pub enum ErrorType {
    FileTooBig,
    FileNotFound,
    ErrorParsingFile,
    FormatNotSupported,
    UnknownError,
}
