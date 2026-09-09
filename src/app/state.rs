use bzd_lib::error::Error;

use crate::app::{
    clients::ClientsState, db::DbState, mess::MessState, notify::state::NotifyState,
    settings::AppSettings, tokens::state::TokensState,
};

#[derive(Clone)]
pub struct AppState {
    pub tokens: TokensState,
    pub notify: NotifyState,
}

impl AppState {
    pub async fn new(settings: &AppSettings) -> Result<Self, Error> {
        let db = DbState::new(&settings.db).await?;
        let mess = MessState::new(&settings.nats).await?;
        let clients = ClientsState::new(&settings.clients)?;

        let tokens = TokensState { db };
        let notify = NotifyState {
            clients: clients.clone(),
            mess: mess.clone(),
            settings: settings.notify.clone(),
        };

        Ok(Self { tokens, notify })
    }
}
