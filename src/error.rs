use std::fmt::Display;

#[derive(Debug)]
pub struct TelegramError(pub frankenstein::Error);

impl Display for TelegramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use frankenstein::Error;
        match &self.0 {
            Error::HttpError(http) => f.write_fmt(format_args!(
                "HTTP Request failed with code {} {}",
                http.code, http.message
            )),
            Error::ApiError(api) => f.write_fmt(format_args!(
                "Telegram API failed with code {} {}",
                api.error_code, api.description
            )),
        }
    }
}

impl std::error::Error for TelegramError {}
