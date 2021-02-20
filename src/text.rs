use std::sync::Arc;

use fst::{automaton::Subsequence, IntoStreamer, Streamer};
use tbot::{
    contexts::{methods::ChatMethods, Text},
    types::chat::Action,
    util::ChatActionLoop,
};
use tokio::select;

#[derive(Clone)]
pub struct Handler {
    command_matcher: fst::Map<Vec<u8>>,
}

mod command {
    pub const START: u64 = 0;
}

impl Handler {
    pub fn new() -> Self {
        let mut matcher = fst::MapBuilder::memory();
        matcher.insert("start", command::START).unwrap();

        Self {
            command_matcher: matcher.into_map(),
        }
    }

    pub async fn handle(&self, ctx: Arc<Text>) -> anyhow::Result<()> {
        if ctx.text.value.starts_with("/") {
            let command_text = ctx.text.value[1..].to_string();
            let mut args = Args { s: &command_text };
            if let Some(label) = args.next() {
                self.command(ctx, label, args).await
            } else {
                Ok(())
            }
        } else {
            self.text(ctx).await
        }
    }

    pub async fn text(&self, ctx: Arc<Text>) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn command(&self, ctx: Arc<Text>, label: &str, args: Args<'_>) -> anyhow::Result<()> {
        let search = Subsequence::new(label);
        let mut stream = self.command_matcher.search(search).into_stream();

        if let Some((_, command)) = stream.next() {
            match command {
                command::START => {
                    let typing = ctx.send_chat_action_in_loop(Action::Typing);
                    let reply = ctx.send_message_in_reply("Hello!").call();
                    select! {
                        _ = reply => {}
                        _ = typing => {}
                        else => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub struct Args<'a> {
    pub s: &'a str,
}

impl<'a> Iterator for Args<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let end_index = self
            .s
            .char_indices()
            .find(|(_, letter)| letter.is_whitespace())
            .map_or(self.s.len(), |(index, _)| index);
        let (arg, inner) = self.s.split_at(end_index);
        self.s = inner.trim_start();
        if arg.is_empty() {
            None
        } else {
            Some(arg)
        }
    }
}

impl<'a> Args<'a> {
    pub fn rest(&self) -> &'a str {
        self.s
    }
}
