use std::env;

use frankenstein::{Api, ChatId, Message, SendMessageParams, TelegramApi, Update};
use log::{debug, error};
use tiny_http::{Method, Response, Server};

mod error;
use error::TelegramError;

fn main() {
    pretty_env_logger::init();

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN is not present");
    let api = Api::new(&token);

    let port: u16 = env::var("PORT")
        .map(|port| port.parse().expect("PORT format is invalid"))
        .unwrap_or(8090);

    let bind_addr = env::var("BIND_ADDR").unwrap_or("0.0.0.0".into());

    let server = Server::http((bind_addr, port)).expect("cannot start webhook server");

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
    if let Some(message) = update.message() {
        on_message(api, message)?;
    }
    Ok(())
}

fn on_message(api: &Api, message: Message) -> eyre::Result<()> {
    debug!(
        "message from chat #{} and user #{:?}",
        message.chat().id(),
        message.from().map(|u| u.id())
    );
    let message = SendMessageParams::new(ChatId::Integer(message.chat().id()), "Hello!".into());
    api.send_message(&message).map_err(TelegramError)?;
    Ok(())
}
