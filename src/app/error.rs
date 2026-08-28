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
    // Ok
    #[error("DB")]
    Db(#[from] sea_orm::DbErr),
    #[error("VALIDATION")]
    Validation,
    // #[error("NOT_FOUND")]
    // NotFound,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("UNREACHABLE")]
    Unreachable,
}

impl From<validator::ValidationErrors> for AppError {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::Validation
    }
}
