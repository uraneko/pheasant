use crate::{ErrorStatus, StatusLiterals};
use std::format;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageError {
    FailedToReadRequestBytes,
    FailedToParseRequest,
    RequestFailedScrutinyTests,
    ResourceNotFound,
    // FaildToGenerateRespond
    RespondFailedScrutinyTests,
    FailedToParseRespond,
    FailedToWriteRespondBytes,
    Other,
}

impl From<ErrorStatus> for MessageError {
    fn from(err: ErrorStatus) -> MessageError {
        match err.code() {
            400 => MessageError::FailedToParseRequest,
            404 => MessageError::ResourceNotFound,
            code if code > 400 && code < 500 => Self::RequestFailedScrutinyTests,
            500 => Self::FailedToWriteRespondBytes,
            501 => MessageError::FailedToParseRespond,
            code if code > 500 && code <= 599 => Self::RespondFailedScrutinyTests,
            _ => MessageError::Other,
        }
    }
}

impl From<MessageError> for std::io::Error {
    fn from(err: MessageError) -> Self {
        Self::other(format!("{:?}", err))
    }
}

impl From<ErrorStatus> for std::io::Error {
    fn from(err: ErrorStatus) -> Self {
        MessageError::from(err).into()
    }
}
