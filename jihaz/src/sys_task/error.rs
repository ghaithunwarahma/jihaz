// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Errors at the application shell level.

use std::fmt;
use std::sync::Arc;

use crate::backend::error as backend;

/// Shell errors.
#[derive(Debug, Clone)]
pub enum Error {
    /// The SystemTaskHandle instance has already been created.
    SystemTaskHandleAlreadyExists,
    /// Platform specific error.
    Platform(backend::Error),
    /// Other miscellaneous error.
    Other(Arc<anyhow::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::SystemTaskHandleAlreadyExists => {
                write!(f, "A SystemTaskHandle instance has already been created.")
            }
            Error::Platform(err) => fmt::Display::fmt(err, f),
            Error::Other(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<anyhow::Error> for Error {
    fn from(src: anyhow::Error) -> Error {
        Error::Other(Arc::new(src))
    }
}

impl From<backend::Error> for Error {
    fn from(src: backend::Error) -> Error {
        Error::Platform(src)
    }
}
