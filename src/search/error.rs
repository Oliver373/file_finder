use tokio::sync::AcquireError;
use tokio::sync::mpsc::error::SendError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SearchError {
    IoError(io::Error),
    AcquireError(AcquireError),
    SendError(SendError<PathBuf>),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SearchError::IoError(err) => write!(f, "IO error: {}", err),
            SearchError::AcquireError(err) => write!(f, "Semaphore acquire error: {}", err),
            SearchError::SendError(err) => write!(f, "Channel send error: {}", err),

        }
    }
}

impl From<io::Error> for SearchError {
    fn from(err: io::Error) -> Self {
        SearchError::IoError(err)
    }
}

impl From<AcquireError> for SearchError {
    fn from(err: AcquireError) -> Self {
        SearchError::AcquireError(err)
    }
}

impl From<SendError<PathBuf>> for SearchError {
    fn from(err: SendError<PathBuf>) -> Self {
        SearchError::SendError(err)
    }
}
