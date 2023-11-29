use salvo::http::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("http::ParseError:`{0}`")]
    ParseError(#[from] ParseError),
    #[error("sqlx::Error:`{0}`")]
    SqlxError(#[from] sqlx::Error),
    #[error("ValidationError:`{0}`")]
    ValidationError(#[from] validator::ValidationErrors),
}
