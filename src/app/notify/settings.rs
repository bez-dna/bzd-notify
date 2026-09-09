use bzd_lib::settings::{NATSConsumerSettings, NATSProducerSettings};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NotifySettings {
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub notify: NATSProducerSettings,
    pub messages: NATSConsumerSettings,
}
