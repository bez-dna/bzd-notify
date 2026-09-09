use bzd_lib::error::Error;
use tokio_stream::StreamExt as _;
use tracing::error;

use crate::app::notify::state::NotifyState;

pub async fn messages(state: NotifyState) -> Result<(), Error> {
    let consumer = messages::consumer(&state.mess, &state.settings).await?;
    let mut messages = consumer.messages().await?;

    while let Some(message) = messages.next().await {
        if let Err(err) = messages::handler(&state, message?).await {
            error!("{}", err);
        }
    }

    Ok(())
}

mod messages {
    use async_nats::jetstream::{
        self,
        consumer::{Consumer, pull::Config},
    };
    use bzd_lib::error::Error;
    use bzd_messages_api::events;
    use prost::Message as _;

    use crate::app::{
        error::AppError,
        mess::MessState,
        notify::{
            service::{self, notify_message},
            settings::NotifySettings,
            state::NotifyState,
        },
    };

    pub async fn consumer(
        mess: &MessState,
        settings: &NotifySettings,
    ) -> Result<Consumer<Config>, Error> {
        Ok(mess
            .js
            .create_consumer_on_stream(
                Config {
                    durable_name: Some(settings.messaging.messages.consumer.clone()),
                    filter_subjects: settings.messaging.messages.subjects.clone(),
                    ..Default::default()
                },
                mess.settings.stream.clone(),
            )
            .await?)
    }

    pub async fn handler(state: &NotifyState, message: jetstream::Message) -> Result<(), AppError> {
        let NotifyState { mess, settings, .. } = state;

        service::notify_message(&mess.js, &settings, (&message).try_into()?).await?;

        message.ack().await?;

        Ok(())
    }

    impl TryFrom<&jetstream::Message> for notify_message::Request {
        type Error = AppError;

        fn try_from(message: &jetstream::Message) -> Result<Self, Self::Error> {
            let message = events::Message::decode(message.payload.clone())?;

            Ok(Self { message })
        }
    }
}
