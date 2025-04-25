//! Support and utilities for other Takarum projects.
mod backend;
pub mod bits;
pub mod convert;
pub mod string;
pub mod sys_task;
pub mod time;
pub mod collection;

// use bits::*;

#[cfg(feature = "scraper")]
pub mod scraper;

// re-exports

pub use jihaz_macros as macros;
pub use jihaz_primal as primal;
pub use jihaz_primal::*;

use std::path::Path;
use std::{io, fs};

/// Copies a folder recursively
// From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}