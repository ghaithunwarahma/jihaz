//! Jihaz Macros that provide support and utilities for other Takarum apps.
//! 

// Doing this to allow using external directories crate in the directory macros
// without the user needing to import the crate themselves
pub extern crate directories;

// pub mod bits;
pub mod directory;
pub mod util;

pub extern crate paste;