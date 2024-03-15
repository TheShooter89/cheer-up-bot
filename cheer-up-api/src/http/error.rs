use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use sqlx::error::Error as SqlxError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("requested entity not found")]
    NotFound,
    #[error("an error occurred with the database")]
    Sqlx(#[from] SqlxError),
    #[error("internal server error")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(ref e) => {
                // TODO: USE TRACER
                log::error!("SQLx error: {:?}", e);
            }
            Self::Anyhow(ref e) => {
                // TODO: USE TRACER
                log::error!("Generic error: {:?}", e);
            }
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
