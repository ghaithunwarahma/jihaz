use std::path::{Path, PathBuf};

/// Convenience PathBuf to String & str conversions.
pub trait PathBufToString {
    /// Yields a <code>String</code> if the `OsStr` is valid Unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    fn to_string(&self) -> Option<String>;
    /// Yields a <code>&str</code> if the `OsStr` is valid Unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    fn to_str(&self) -> Option<&str>;
}

impl PathBufToString for &Path {
    fn to_string(&self) -> Option<String> {
        self.as_os_str()
            .to_str()
            .map(ToString::to_string)
    }
    fn to_str(&self) -> Option<&str> {
        self.as_os_str().to_str()
    }
}

impl PathBufToString for PathBuf {
    fn to_string(&self) -> Option<String> {
        self.as_path().to_string()
    }
    fn to_str(&self) -> Option<&str> {
        self.as_path().to_str()
    }
}

impl PathBufToString for &PathBuf {
    fn to_string(&self) -> Option<String> {
        self.as_path().to_string()
    }
    fn to_str(&self) -> Option<&str> {
        self.as_path().to_str()
    }
}

impl<T: PathBufToString> PathBufToString for Option<T> {
    fn to_string(&self) -> Option<String> {
        self.as_ref()
            .and_then(|path| path.to_string())
    }
    fn to_str(&self) -> Option<&str> {
        self.as_ref()
            .and_then(|path| path.to_str())
    }
}