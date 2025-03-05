#[cfg(feature = "xilem")]
use crate::{icons::IconTaskMessage, message::DeferredTaskMessage};
#[cfg(feature = "xilem")]
use jihaz_primal::error::print_error;
#[cfg(feature = "xilem")]
use xilem_core::MessageProxy;

/// A trait for a type that receives messages. It helps when calling functions need to
/// be generic over the message receiver type.
/// 
/// The calling function can also be a deferred task running on a different thread,
/// as the type implements both Send and Sync.
pub trait MessageReceiver<FinalMessage = DynMessage, InitMessage = FinalMessage>: Send + Sync + 'static {
    fn send(&self, message: InitMessage);
}

/// A type that allows for sending dynamic messages to the receiver
pub type DynMessage = Box<dyn Message>;

/// A trait to be implemented for the types that can be sent to the MessageReceiver.
/// 
/// This to ensure the message can be sent across threads.
pub trait Message: 'static + Send {}


#[cfg(feature = "xilem")]
impl MessageReceiver<DeferredTaskMessage, IconTaskMessage> for MessageProxy<DeferredTaskMessage> {
    fn send(&self, message: IconTaskMessage) {
        print_error(self.message(DeferredTaskMessage::IconsMessage(message)));
    }
}