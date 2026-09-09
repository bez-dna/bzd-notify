use async_nats::jetstream::Context;
use bzd_notify_api::notify::Notify;
use bzd_users_api::users::GetUserRequest;
use prost::{Message as _, bytes::BytesMut};

use crate::app::{clients::ClientsState, error::AppError, notify::settings::NotifySettings};

pub async fn notify_message(
    js: &Context,
    settings: &NotifySettings,
    ClientsState {
        users_service_client,
        ..
    }: &ClientsState,
    mut req: notify_message::Request,
) -> Result<(), AppError> {
    // Тут хочу перевести на nats request-reply, но пока синк вызов

    req.user = users_service_client
        .clone()
        .get_user(GetUserRequest {
            user_id: Some(req.message.user_id().into()),
        })
        .await?
        .into_inner()
        .user;

    let subject = settings.messaging.notify.subject.clone();
    let mut buf = BytesMut::new();
    let payload: Notify = req.try_into()?;

    payload.encode(&mut buf)?;

    js.publish(subject, buf.into()).await?;

    Ok(())
}

pub mod notify_message {
    use bzd_notify_api::notify::{Message, Notify, message::User, notify::Payload};
    use bzd_users_api::users::get_user_response;

    use crate::app::error::AppError;

    pub struct Request {
        pub message: bzd_messages_api::events::Message,
        pub user: Option<get_user_response::User>,
    }

    impl TryFrom<Request> for Notify {
        type Error = AppError;

        fn try_from(req: Request) -> Result<Self, Self::Error> {
            let user = req.user.ok_or(AppError::Unreachable)?;

            Ok(Self {
                payload: Some(Payload::Message(Message {
                    message_id: req.message.message_id,
                    text: req.message.text,
                    code: req.message.code,
                    user_id: req.message.user_id,
                    order: req.message.order,
                    user: Some(User {
                        user_id: user.user_id,
                        name: user.name,
                        abbr: user.abbr,
                        color: user.color,
                    }),
                    message_ids: req.message.message_ids,
                    created_at: req.message.created_at,
                    updated_at: req.message.updated_at,
                })),
            })
        }
    }
}
