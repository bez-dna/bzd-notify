use crate::app::{mess::MessState, notify::settings::NotifySettings};

#[derive(Clone)]
pub struct NotifyState {
    pub settings: NotifySettings,
    pub mess: MessState,
}
