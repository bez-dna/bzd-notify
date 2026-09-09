use crate::app::{clients::ClientsState, mess::MessState, notify::settings::NotifySettings};

#[derive(Clone)]
pub struct NotifyState {
    pub clients: ClientsState,
    pub settings: NotifySettings,
    pub mess: MessState,
}
