use std::env;

use sentry::IntoDsn;
use tbot::{contexts::methods::ChatMethods, types::chat::Action, util::ChatActionLoop};

fn main() {
    dotenv::dotenv().ok();
    init_logger();
    let sentry_dsn = env::var("SENTRY_DSN").unwrap();
    let _guard = sentry::init(sentry::ClientOptions {
        dsn: sentry_dsn.into_dsn().unwrap(),
        release: sentry::release_name!(),
        ..Default::default()
    });
    let token = env::var("BOT_TOKEN").unwrap();
    let mut bot = tbot::Bot::new(token.clone()).event_loop();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(bot.fetch_username()).unwrap();

    bot.command("start", |text| async move {
        let action = text.send_chat_action_in_loop(Action::Typing);
        let reply = text.send_message_in_reply("Hello").call();
        tokio::select! {
            _ = action => {}
            _ = reply => {}
        }
    });

    let url = env::var("URL").unwrap();
    let bind_port = env::var("BIND_PORT").unwrap().parse().unwrap();
    let webhook_url = format!("{}/{}", &url, &token);
    let webhook = bot
        .webhook(&webhook_url, bind_port)
        .accept_updates_on(format!("/{}", &token))
        .ip("127.0.0.1".parse().unwrap())
        .http()
        .start();

    rt.block_on(webhook).unwrap();
}

fn init_logger() {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();
}
