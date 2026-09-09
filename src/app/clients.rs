use bzd_lib::error::Error;
use bzd_users_api::users::users_service_client::UsersServiceClient;
use tonic::transport::{Channel, Endpoint};

use crate::app::settings::ClientsSettings;

#[derive(Clone)]
pub struct ClientsState {
    pub users_service_client: UsersServiceClient<Channel>,
    pub _settings: ClientsSettings,
}

impl ClientsState {
    pub fn new(settings: &ClientsSettings) -> Result<Self, Error> {
        let users_service_client =
            UsersServiceClient::new(Endpoint::new(settings.users.endpoint.clone())?.connect_lazy());

        Ok(Self {
            users_service_client,
            _settings: settings.clone(),
        })
    }
}
