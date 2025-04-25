//! Convenience Path and String conversions
use std::path::{Path, PathBuf};

/// Converting a [`String`] into an optional [`PathBuf`].
/// 
/// We use [`Option<PathBuf>`], because it is safer 
/// not to use an empty [`PathBuf`] as a representation for not having a path.
pub trait StringToOptionalPath {
    /// Creates an optional [`PathBuf`] from the string.
    /// 
    /// If the string is empty, it will return None.
    fn to_optional_path(self) -> Option<PathBuf>;
}

impl StringToOptionalPath for String {
    fn to_optional_path(self) -> Option<PathBuf> {
        (self.len() > 0).then(|| PathBuf::from(self))
    }
}

/// Helpful convenience methods for dealing with an optional paths.
/// 
/// We use [`Option<PathBuf>`], because it is safer 
/// not to use an empty [`PathBuf`] as a representation for not having a path.
pub trait OptionalPath {
    /// Converts an [`Option<PathBuf>`] into a [`String`].
    /// If the string is empty, it will return None.
    /// 
    /// Since this method entails an [`OsStr::to_str`] call, it may return None if
    /// the `OsStr` is not valid Unicode.
    fn to_string(&self) -> String;
    /// Returns true if the option has a path and the path exists.
    fn exists(&self) -> bool;
    /// Returns true if the option has a path and the path points to a directory.
    fn is_dir(&self) -> bool;
    /// Returns true if the option has a path and the path points to a file.
    fn is_file(&self) -> bool;
}

impl OptionalPath for Option<&PathBuf>  {
    fn to_string(&self) -> String {
        self.and_then(PathToStr::to_string).unwrap_or_default()
    }
    
    fn exists(&self) -> bool {
        self.map(PathBuf::as_path).map(Path::exists).unwrap_or(false)
    }
    
    fn is_dir(&self) -> bool {
        self.map(PathBuf::as_path).map(Path::is_dir).unwrap_or(false)
    }
    
    fn is_file(&self) -> bool {
        self.map(PathBuf::as_path).map(Path::is_file).unwrap_or(false)
    }
}

impl OptionalPath for Option<PathBuf>  {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
    
    fn exists(&self) -> bool {
        self.as_ref().exists()
    }
    
    fn is_dir(&self) -> bool {
        self.as_ref().is_dir()
    }
    
    fn is_file(&self) -> bool {
        self.as_ref().is_file()
    }
}

/// Convenience [`PathBuf`] and [`&Path`] to [`String`] and [`&str`] conversions
pub trait PathToStr {
    /// Yields a [`String`] if the `OsStr` is valid Unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    fn to_string(&self) -> Option<String>;
    
    /// Yields a [`&str`] if the `OsStr` is valid Unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    fn to_str(&self) -> Option<&str>;
}

/// Deprecated! Use [`PathToStr`] instead.
#[deprecated = "Renamed to `PathToStr`"]
pub trait PathBufToString: PathToStr {}


impl PathToStr for &Path {
    fn to_string(&self) -> Option<String> {
        self.as_os_str().to_str().map(ToString::to_string)
    }
    fn to_str(&self) -> Option<&str> {
        self.as_os_str().to_str()
    }
}

impl PathToStr for PathBuf {
    fn to_string(&self) -> Option<String> {
        self.as_path().to_string()
    }
    fn to_str(&self) -> Option<&str> {
        self.as_path().to_str()
    }
}

/// Convenience [`String`] and [`&str`] to [`PathBuf`] and [`&Path`] conversions
pub trait StrToPath {
    fn to_path(&self) -> &Path;

    fn to_path_buf(&self) -> PathBuf;
}

impl StrToPath for &str {
    fn to_path(&self) -> &Path {
        Path::new(self)
    }

    fn to_path_buf(&self) -> PathBuf {
        Path::new(self).into()
    }
}

impl StrToPath for String {
    fn to_path(&self) -> &Path {
        Path::new(self)
    }

    fn to_path_buf(&self) -> PathBuf {
        Path::new(self).into()
    }
}