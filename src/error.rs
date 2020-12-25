use serde_derive::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if let Some(e) = err.find::<CustomError>() {
        match e {
            CustomError::TokenIsNotFound => (StatusCode::UNAUTHORIZED, "Token Not Found"),
            CustomError::TokenIsInvalid => (StatusCode::UNAUTHORIZED, "Token Is Invalid"),
            CustomError::AccountIsNotFound => (StatusCode::BAD_REQUEST, "Account Not Found"),
            CustomError::ReadingFileError => (StatusCode::BAD_REQUEST, "Reading File Error"),
            CustomError::WritingFileError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Writing File Error")
            }
            CustomError::DirectoryCouldNotCreated => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Upload Failed")
            }
            CustomError::FileIsNotComplete => (StatusCode::OK, "File Is Not Complete"),
            CustomError::SaveIsNotComplete => (StatusCode::OK, "Save Is Not Complete"),
        }
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, "Invalid Body")
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (StatusCode::UNAUTHORIZED, "Method Not Allowed")
    } else {
        eprintln!("unhandled error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}

#[derive(Debug)]
pub enum CustomError {
    TokenIsNotFound,
    TokenIsInvalid,
    AccountIsNotFound,
    ReadingFileError,
    WritingFileError,
    DirectoryCouldNotCreated,
    FileIsNotComplete,
    SaveIsNotComplete,
}
impl CustomError {
    pub fn rejection(self) -> Rejection {
        warp::reject::custom(self)
    }
}

impl warp::reject::Reject for CustomError {}
