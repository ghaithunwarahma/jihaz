use std::fmt::Display;
use crate::icons::IconTaskMessage;

#[derive(Debug, Clone)]
pub enum DeferredTaskMessage {
    IconsMessage(IconTaskMessage),
}

impl Display for DeferredTaskMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeferredTaskMessage::IconsMessage(message) => {
                message.fmt(f)
            }
        }
    }
}