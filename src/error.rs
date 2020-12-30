use serde_derive::Serialize;
use std::convert::Infallible;
use std::string::FromUtf8Error;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message): (StatusCode, String) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".into())
    } else if let Some(e) = err.find::<HandleError>() {
        use HandleError::*;
        (
            match e {
                AuthorizationCodeIsNotFound => StatusCode::BAD_REQUEST,
                ReqwestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                GoogleResponseIsInvalid => StatusCode::INTERNAL_SERVER_ERROR,
                TokenIsInvalid => StatusCode::UNAUTHORIZED,
                AccountIsNotFound => StatusCode::BAD_REQUEST,
                AccountIsNotSelected => StatusCode::BAD_REQUEST,
                AccountSelectionIsInvalid => StatusCode::BAD_REQUEST,
                IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ReadingFileError => StatusCode::BAD_REQUEST,
                DirectoryCouldNotCreated => StatusCode::INTERNAL_SERVER_ERROR,
                FileIsNotFound => StatusCode::OK,
                SaveIsNotComplete => StatusCode::OK,
                FileIsNotDeleted => StatusCode::OK,
                FileIsInvalid => StatusCode::OK,
                OtherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            e.to_string(),
        )
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, "Invalid Body".into())
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (StatusCode::UNAUTHORIZED, "Method Not Allowed".into())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".into(),
        )
    };

    println!("{} {}", code, message);
    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("Code Is Not Found")]
    AuthorizationCodeIsNotFound,
    #[error("ReqwestError: {0:?}")]
    ReqwestError(reqwest::Error),
    #[error("Google End Point Is Down")]
    GoogleResponseIsInvalid,

    #[error("Token Is Invalid")]
    TokenIsInvalid,
    #[error("Account Not Found")]
    AccountIsNotFound,
    #[error("Account Is Not Selected")]
    AccountIsNotSelected,
    #[error("Account Selection Is Invalid")]
    AccountSelectionIsInvalid,

    #[error("IOError: {0:?}")]
    IOError(std::io::Error),

    #[error("Reading File Error")]
    ReadingFileError,

    #[error("Upload Failed")]
    DirectoryCouldNotCreated,
    #[error("File Is Not Found")]
    FileIsNotFound,
    #[error("Save Is Not Complete")]
    FileIsInvalid,
    #[error("File Is Not Deleted")]
    FileIsNotDeleted,
    #[error("File Is Invalid")]
    SaveIsNotComplete,
    #[error("FromUtf8 Error: {0:?}")]
    FromUtf8Error(FromUtf8Error),

    #[error("Other Error: {0}")]
    OtherError(anyhow::Error),
}
impl HandleError {
    pub fn rejection(self) -> Rejection {
        warp::reject::custom(self)
    }
}

impl warp::reject::Reject for HandleError {}
