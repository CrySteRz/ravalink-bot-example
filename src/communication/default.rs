use std::fmt::Display;
use crate::communication::constants::*;
use std::error::Error;

pub enum RustyMessage {
    Ping,
}

impl Display for RustyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => f.write_str(RESP_PING),
            
        }
    }
}

pub enum RustyMessageError {
    GenericError { error: Box<dyn Error> },
    Ping,
}

impl Display for RustyMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => f.write_str(ERR_PING),
            Self::GenericError { error } => {
                f.write_str(&format!("**{}** {}", ERR_GENERIC, error))
            }
        }
    }
}

impl From<RustyMessageError> for String {
    fn from(error: RustyMessageError) -> Self {
        error.to_string()
    }
}
