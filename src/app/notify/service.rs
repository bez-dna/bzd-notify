use async_nats::jetstream::Context;
use bzd_notify_api::notify::Notify;
use prost::{Message as _, bytes::BytesMut};

use crate::app::{error::AppError, notify::settings::NotifySettings};

pub async fn notify_message(
    js: &Context,
    settings: &NotifySettings,
    req: notify_message::Request,
) -> Result<(), AppError> {
    let subject = settings.messaging.notify.subject.clone();
    let mut buf = BytesMut::new();
    let payload: Notify = req.into();
    payload.encode(&mut buf)?;

    js.publish(subject, buf.into()).await?;

    Ok(())
}

pub mod notify_message {
    use bzd_notify_api::notify::{Message, Notify, message::User, notify::Payload};

    pub struct Request {
        pub message: bzd_messages_api::events::Message,
    }

    impl From<Request> for Notify {
        fn from(req: Request) -> Self {
            Self {
                payload: Some(Payload::Message(Message {
                    message_id: req.message.message_id,
                    text: req.message.text,
                    code: req.message.code,
                    user_id: req.message.user_id,
                    order: req.message.order,
                    user: Some(User::default()),
                    message_ids: req.message.message_ids,
                    created_at: req.message.created_at,
                    updated_at: req.message.updated_at,
                })),
            }
        }
    }
}
