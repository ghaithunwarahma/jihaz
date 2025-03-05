// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! macOS backend errors.
//! 
//! These are copied from Druid's implementation.

#[derive(Debug, Clone)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NSError")
    }
}

impl std::error::Error for Error {}
