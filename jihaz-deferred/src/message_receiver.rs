/// A trait for a type that receives messages. It helps when calling functions need to
/// be generic over the message receiver type.
/// 
/// The calling function can also be a deferred task running on a different thread,
/// as the type implements both Send and Sync.
pub trait MessageReceiver<FinalMessage = RecDynMessage, InitMessage = FinalMessage>: Send + Sync + 'static {
    fn send(&self, message: InitMessage);
}

/// A type that allows for sending dynamic messages to the receiver
pub type RecDynMessage = Box<dyn Message>;

/// A trait to be implemented for the types that can be sent to the MessageReceiver.
/// 
/// This to ensure the message can be sent across threads.
pub trait Message: 'static + Send {}