//! macOS implementation of features at the application scope.
//! 
//! These are copied from Druid's implementation.

#![allow(non_upper_case_globals)]

use super::clipboard::Clipboard;
use super::error::Error;
use super::util;

#[derive(Clone)]
pub(crate) struct SystemTaskHandle;

impl SystemTaskHandle {
    pub fn activate() -> Result<SystemTaskHandle, Error> {
        // from druid's SystemTaskHandle implementation, may not be relevant for the SystemTaskHandle:
        // macOS demands that we run not just on one thread,
        // but specifically the first thread of the app.
        util::assert_main_thread();
        Ok(SystemTaskHandle)
    }

    pub fn finished(self) {

    }

    pub fn quit(&self) {
        
    }

    pub fn clipboard(&self) -> Clipboard {
        Clipboard
    }
}
