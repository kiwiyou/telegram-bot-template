# telegram-bot-template

Rust telegram bot template.

It uses [`frankenstein`](https://github.com/ayrat555/frankenstein) for minimal dependencies.

## Features

- Settings using config directory.

config/{default, development, testing, production}.toml is supported.

by RUST_ENV=&lt;config name&gt; you can choose what config file you will use.

- Parse commandline arguments.

with `Command` and `Args` you can easily process bot commands.

## How to use

install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate).

```bash
cargo generate --git https://github.com/kiwiyou/telegram-bot-template.git
```
