// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Windows implementation of features at the SystemTaskHandle scope.
//! 
//! These are copied from Druid's implementation.

use super::clipboard::Clipboard;
use super::error::Error;

#[derive(Clone)]
pub(crate) struct SystemTaskHandle;

impl SystemTaskHandle {
    pub fn activate() -> Result<SystemTaskHandle, Error> {
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
