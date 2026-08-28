use bzd_notify_api::tokens::{
    CreateTokenRequest, CreateTokenResponse, tokens_service_server::TokensService,
};
use tonic::{Request, Response, Status};

use crate::app::tokens::state::TokensState;

pub struct GrpcTokensService {
    pub state: TokensState,
}

impl GrpcTokensService {
    pub fn new(state: TokensState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TokensService for GrpcTokensService {
    async fn create_token(
        &self,
        req: Request<CreateTokenRequest>,
    ) -> Result<Response<CreateTokenResponse>, Status> {
        let res = create_token::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

mod create_token {
    use bzd_lib::current_user::CurrentUser;
    use bzd_notify_api::tokens::{CreateTokenRequest, CreateTokenResponse};
    use validator::Validate;

    use crate::app::{
        error::AppError,
        tokens::{
            service::{
                self,
                create_token::{Request, Response},
            },
            state::TokensState,
        },
    };

    pub async fn handler(
        TokensState { db, .. }: &TokensState,
        req: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, AppError> {
        let res = service::create_token(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<CreateTokenRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateTokenRequest) -> Result<Self, Self::Error> {
            let data = Self {
                current_user: CurrentUser::new(req.current_user_id()),
                token: req.token().into(),
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl From<Response> for CreateTokenResponse {
        fn from(res: Response) -> Self {
            Self {
                token_id: Some(res.token.token_id.into()),
            }
        }
    }
}
