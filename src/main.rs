use command::Command;
use frankenstein::{Api, ChatId, Message, SendMessageParams, TelegramApi, Update};
use log::{debug, error};
use settings::Settings;
use tiny_http::{Method, Response, Server};

mod command;
mod error;
mod settings;
use error::TelegramError;

fn main() {
    let settings = Settings::new().expect("error in settings");
    pretty_env_logger::formatted_timed_builder()
        .filter_level(settings.log.level)
        .init();

    let api = Api::new(&settings.bot.token);

    let server = Server::http((settings.server.bind_address, settings.server.port))
        .expect("cannot start webhook server");

    for mut request in server.incoming_requests() {
        if *request.method() == Method::Post {
            let mut reader = request.as_reader();
            let update: Result<Update, _> = serde_json::from_reader(&mut reader);
            if let Ok(update) = update {
                let response = if let Err(e) = on_update(&api, update) {
                    error!("error processing update: {}", e);
                    Response::empty(500)
                } else {
                    Response::empty(200)
                };
                if let Err(e) = request.respond(response) {
                    error!("cannot send a response: {}", e);
                }
            }
        }
    }
}

fn on_update(api: &Api, update: Update) -> eyre::Result<()> {
    if let Some(message) = &update.message {
        on_message(api, message)?;
    }
    Ok(())
}

fn on_message(api: &Api, message: &Message) -> eyre::Result<()> {
    debug!(
        "message from chat #{} and user #{:?}",
        message.chat.id,
        message.from().map(|u| u.id)
    );
    if let Some(text) = message.text.as_ref().or_else(|| message.caption.as_ref()) {
        {
            let prev_len = text.len();
            let maybe_command = text.trim_start_matches('/');
            if maybe_command.len() < prev_len {
                return on_command(api, message, Command::new(maybe_command));
            }
        }
        let mut send = SendMessageParams::new(ChatId::Integer(message.chat.id), text.clone());
        send.set_reply_to_message_id(Some(message.message_id));
        api.send_message(&send).map_err(TelegramError)?;
    }
    Ok(())
}

fn on_command(api: &Api, message: &Message, command: Command) -> eyre::Result<()> {
    if command.label == "start" {
        let mut send = SendMessageParams::new(
            ChatId::Integer(message.chat.id),
            "This is an example echo bot.".into(),
        );
        send.set_reply_to_message_id(Some(message.message_id));
        api.send_message(&send).map_err(TelegramError)?;
    }
    Ok(())
}
