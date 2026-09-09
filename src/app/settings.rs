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
}

impl Settings<AppSettings> for AppSettings {}
