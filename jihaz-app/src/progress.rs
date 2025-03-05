use anyhow::Error;
use jihaz_deferred::message::DeferredTaskMessage;
use core::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ProgressMessage {

    // - - - Progress - - -

    BeganProducingIconsAndGeneratingPackages {
        app_name_lowercase: String,
        target_packages_directory_path: String,
    },
    WrittenExecutable {
        executable_path: String,
        is_main: bool,
    },
    FinishedProducingIconsAndGeneratingPackages {
        app_name_lowercase: String,
        target_packages_directory_path: String,
    },

    // - - - jihaz_deferred Task Messages
    DefferedTask(DeferredTaskMessage),


    // - - - Errors - - -
    
    NotAllFieldsAreFilled,
    ExecutablePathDoesNotPointToAFile,
    SourceIconPathDoesNotPointToAFile,
    TargetDirectoryPathIsNotADirectory,
    OtherError(String),

    /// Empty message
    Nop,
}

impl From<Error> for ProgressMessage {
    fn from(error: Error) -> Self {
        ProgressMessage::OtherError(error.to_string())
    }
}

impl From<DeferredTaskMessage> for ProgressMessage {
    fn from(value: DeferredTaskMessage) -> Self {
        ProgressMessage::DefferedTask(value)
    }
}

impl ProgressMessage {
    pub(crate) fn began_production(
        app_name_lowercase: String,
        target_packages_directory_path: String,
    ) -> ProgressMessage {
        ProgressMessage::BeganProducingIconsAndGeneratingPackages { 
            app_name_lowercase, target_packages_directory_path 
        }
    }

    pub(crate) fn written_executable(
        executable_path: String,
        is_main: bool,
    ) -> ProgressMessage {
        ProgressMessage::WrittenExecutable { executable_path, is_main }
    }

    pub(crate) fn finished(
        app_name_lowercase: String,
        target_packages_directory_path: String,
    ) -> ProgressMessage {
        ProgressMessage::FinishedProducingIconsAndGeneratingPackages {
            app_name_lowercase, target_packages_directory_path 
        } 
    }

    pub(crate) fn set(
        &mut self, 
        new_progress_message: ProgressMessage,
    ) {
        *self = new_progress_message;
    }
}

impl Display for ProgressMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {

            ProgressMessage::BeganProducingIconsAndGeneratingPackages {
                app_name_lowercase, target_packages_directory_path
            } => {
                f.write_fmt(format_args!("Began producing icons and generating packages for app {app_name_lowercase}. Placing in {target_packages_directory_path}."))
            }

            ProgressMessage::WrittenExecutable { executable_path, is_main } => {
                let kind = if *is_main { "main" } else { "secondary" };
                f.write_fmt(format_args!("Written {kind} executable file for the app at {executable_path}"))
            }

            ProgressMessage::FinishedProducingIconsAndGeneratingPackages {
                app_name_lowercase, target_packages_directory_path
            } => {
                f.write_fmt(format_args!("Finished producing packages for app: {app_name_lowercase}.. Placed in {target_packages_directory_path}."))
            }

            ProgressMessage::DefferedTask(message) => {
                std::fmt::Display::fmt(message, f)
            }

            ProgressMessage::NotAllFieldsAreFilled => {
                write!(f, "Can't generate packages. Not all of the needed fields are populated.")
            }

            ProgressMessage::ExecutablePathDoesNotPointToAFile => {
                write!(f, "Can't generated packages. Executable path does not point to a file.")
            }

            ProgressMessage::SourceIconPathDoesNotPointToAFile => {
                write!(f, "Can't generated packages. Source icon path does not point to a file.")
            }

            ProgressMessage::TargetDirectoryPathIsNotADirectory => {
                write!(f, "Can't generated packages. Target directory path is not a directory.")
            }

            ProgressMessage::OtherError(error) => {
                std::fmt::Display::fmt(error, f)
            },

            ProgressMessage::Nop => Ok(()),
        }
    }
}

/// A Progress stream that can be shared between threads and doesn't require mutable access.
mod sync_stream {
    use core::fmt::Debug;
    use parking_lot::RwLock;
    use xilem_core::Message;
    use std::{collections::VecDeque, sync::Arc};
    use super::ProgressMessage;

    /// A way to show progress. This is temporary until we have a proxy or asynchronous tasks
    /// that return progress messages.
    #[derive(Default, Clone)]
    pub struct ProgressMessageStream {
        messages: Arc<RwLock<VecDeque<ProgressMessage>>>
    }

    impl Debug for ProgressMessageStream {
        // This needs revision, largly copied from xilem_core/src/message.rs
        // the blanket implementation of xilem_core::Message for AppMessageResult
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let inner = self.dyn_debug();
            f.debug_tuple("Message").field(&inner).finish()
        }
    }

    impl ProgressMessageStream {
        pub fn new() -> ProgressMessageStream {
            ProgressMessageStream { messages: Arc::new(RwLock::new(VecDeque::new())) }
        }

        pub fn push_message(&self, message: ProgressMessage) {
            self.messages.write().push_back(message);
        }

        pub fn next_message(&self) -> Option<ProgressMessage> {
            self.messages.write().pop_front()
        }

        /// Takes a `VecDeque` of items that implement `Into<ProgressMessage>` and appends it
        /// to the progress message stream
        pub fn append_more_messages(&self, mut more_messages: VecDeque<impl Into<ProgressMessage>>) {
            let mut more_messages = more_messages
                .drain(..)
                .map(|message| message.into())
                .collect();
            self.messages.write().append(&mut more_messages);
        }

        pub fn has_messages(&self) -> bool {
            self.messages.read().len() > 0
        }
    }
}