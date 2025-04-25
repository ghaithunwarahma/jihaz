// Copyright 2020 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Web implementation of features at the application scope.
//! 
//! These are copied from Druid's implementation.

use crate::application::AppHandler;

use super::clipboard::Clipboard;
use super::error::Error;

#[derive(Clone)]
pub(crate) struct SystemTaskHandle;

impl SystemTaskHandle {
    pub fn new() -> Result<SystemTaskHandle, Error> {
        Ok(SystemTaskHandle)
    }

    pub fn run(self, _handler: Option<Box<dyn AppHandler>>) {}

    pub fn quit(&self) {}

    pub fn clipboard(&self) -> Clipboard {
        Clipboard
    }

    pub fn get_locale() -> String {
        web_sys::window()
            .and_then(|w| w.navigator().language())
            .unwrap_or_else(|| "en-US".into())
    }
}
