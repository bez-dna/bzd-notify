use bzd_lib::internal_from;
use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            _ => Self::internal(error.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("VALIDATION")]
    Validation,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("INTERNAL")]
    Internal,
    #[error("UNREACHABLE")]
    Unreachable,
}

impl From<validator::ValidationErrors> for AppError {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::Validation
    }
}

internal_from!(
    AppError;
    sea_orm::DbErr,
    async_nats::Error,
    async_nats::error::Error<async_nats::jetstream::context::PublishErrorKind>,
    prost::DecodeError,
    prost::EncodeError,
);
