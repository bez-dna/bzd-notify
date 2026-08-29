use sea_orm::DbConn;

use crate::app::{
    error::AppError,
    tokens::repo::{self, TokenModel},
};

pub async fn create_token(
    db: &DbConn,
    req: create_token::Request,
) -> Result<create_token::Response, AppError> {
    let current_user_id = req.current_user.get_user_id(AppError::Forbidden)?;

    let token = repo::create_stream(db, TokenModel::new(req.token, current_user_id)).await?;

    Ok(create_token::Response { token })
}

pub mod create_token {
    use bzd_lib::current_user::CurrentUser;
    use validator::Validate;

    use crate::app::tokens::repo::TokenModel;

    #[derive(Validate, Debug)]
    pub struct Request {
        pub current_user: CurrentUser,
        #[validate(length(min = 2))]
        pub token: Vec<u8>,
    }

    pub struct Response {
        pub token: TokenModel,
    }
}
