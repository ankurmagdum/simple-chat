use super::client::Client;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::sync::{MutexGuard, PoisonError};

#[derive(Debug)]
pub enum ServerError {
    IoError(IoError),
    MutexError(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IoError(e) => write!(f, "Io Error: {e:?}"),
            Self::MutexError(e) => write!(f, "Mutex Error: {e}"),
        }
    }
}

impl From<IoError> for ServerError {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<PoisonError<MutexGuard<'_, Vec<Client>>>> for ServerError {
    fn from(e: PoisonError<MutexGuard<'_, Vec<Client>>>) -> Self {
        Self::MutexError(e.to_string())
    }
}
