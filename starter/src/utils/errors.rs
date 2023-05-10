use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub struct MessagedError {
    pub message: String,
    pub error: Box<dyn Error>,
}

impl Debug for MessagedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for MessagedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.message, self.error))
    }
}

impl Error for MessagedError {}

pub trait ErrorExt {
    fn with_message(self, msg: &str) -> MessagedError;
}

impl ErrorExt for Box<dyn Error> {
    fn with_message(self, msg: &str) -> MessagedError {
        return MessagedError {
            message: msg.to_owned(),
            error: self,
        };
    }
}
