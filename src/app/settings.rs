use bzd_lib::settings::ClientSettings;
use bzd_lib::settings::DBSettings;
use bzd_lib::settings::NATSSettings;
use bzd_lib::settings::Settings;

use bzd_lib::settings::HttpSettings;
use serde::Deserialize;

use crate::app::notify::settings::NotifySettings;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub http: HttpSettings,
    pub db: DBSettings,
    pub nats: NATSSettings,
    pub notify: NotifySettings,
    pub clients: ClientsSettings,
}

#[derive(Deserialize, Clone)]
pub struct ClientsSettings {
    pub users: ClientSettings,
}

impl Settings<AppSettings> for AppSettings {}
