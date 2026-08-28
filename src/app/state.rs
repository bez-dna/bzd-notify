use bzd_lib::error::Error;

use crate::app::{db::DbState, settings::AppSettings, tokens::state::TokensState};

#[derive(Clone)]
pub struct AppState {
    pub tokens: TokensState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let db = DbState::new(&settings.db).await?;

        let tokens = TokensState { db };

        Ok(Self { tokens })
    }
}
