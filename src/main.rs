use std::{env, sync::Arc};

use anyhow::Context;

mod text;

fn main() {
    dotenv::dotenv().ok();
    init_logger();
    let sentry_dsn = env::var("SENTRY_DSN").unwrap();
    let _guard = sentry::init(sentry_dsn);
    let token = env::var("BOT_TOKEN").unwrap();
    let mut bot = tbot::Bot::new(token.clone()).event_loop();

    let text_handler = Arc::new(text::Handler::new());

    bot.text(move |ctx| {
        let text_handler = text_handler.clone();
        async move {
            if let Err(error) = text_handler
                .handle(ctx.clone())
                .await
                .with_context(|| format!("Error in text: {:?}", *ctx))
            {
                sentry_anyhow::capture_anyhow(&error);
            }
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

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(webhook).unwrap();
}

fn init_logger() {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();
}
