use core::fmt;
use std::sync::Arc;

use parley::GenericFamily;

/// Prioritized sequence of font families.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-family>
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CowFontStack {
    /// Font family list in CSS format.
    Source(Arc<str>),
    /// Single font family.
    Single(CowFontFamily),
    /// Ordered list of font families.
    List(Arc<[CowFontFamily]>),
}

/// Named or generic font family.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-family>
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CowFontFamily {
    /// Named font family.
    Named(Arc<str>),
    /// Generic font family.
    Generic(GenericFamily),
}


impl CowFontFamily {
    /// Parses a font family containing a name or a generic family.
    ///
    /// # Example
    /// ```
    /// # extern crate alloc;
    /// use alloc::borrow::Cow;
    /// use parley::style::CowFontFamily::{self, *};
    /// use parley::style::GenericFamily::*;
    ///
    /// assert_eq!(CowFontFamily::parse("Palatino Linotype"), Some(Named(Arc::from("Palatino Linotype"))));
    /// assert_eq!(CowFontFamily::parse("monospace"), Some(Generic(Monospace)));
    ///
    /// // Note that you can quote a generic family to capture it as a named family:
    ///
    /// assert_eq!(CowFontFamily::parse("'monospace'"), Some(Named(Arc::from("monospace"))));
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        Self::parse_list(s).next()
    }

    /// Parses a comma separated list of font families.
    ///
    /// # Example
    /// ```
    /// # extern crate alloc;
    /// use alloc::borrow::Cow;
    /// use parley::style::CowFontFamily::{self, *};
    /// use parley::style::GenericFamily::*;
    ///
    /// let source = "Arial, 'Times New Roman', serif";
    ///
    /// let parsed_families = CowFontFamily::parse_list(source).collect::<Vec<_>>();
    /// let families = vec![Named(Arc::from("Arial")), Named(Arc::from("Times New Roman")), Generic(Serif)];
    ///
    /// assert_eq!(parsed_families, families);
    /// ```
    pub fn parse_list<'a>(s: &'a str) -> impl Iterator<Item = CowFontFamily> + 'a + Clone {
        ParseList {
            source: s.as_bytes(),
            len: s.len(),
            pos: 0,
        }
    }
}

impl From<GenericFamily> for CowFontFamily {
    fn from(f: GenericFamily) -> Self {
        CowFontFamily::Generic(f)
    }
}

impl From<GenericFamily> for CowFontStack {
    fn from(f: GenericFamily) -> Self {
        CowFontStack::Single(f.into())
    }
}

impl From<CowFontFamily> for CowFontStack {
    fn from(f: CowFontFamily) -> Self {
        CowFontStack::Single(f)
    }
}

impl<'a> From<&'a str> for CowFontStack{
    fn from(s: &'a str) -> Self {
        CowFontStack::Source(Arc::from(s))
    }
}

impl<'a> From<&'a [CowFontFamily]> for CowFontStack {
    fn from(fs: &'a [CowFontFamily]) -> Self {
        CowFontStack::List(Arc::from(fs))
    }
}

impl fmt::Display for CowFontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => write!(f, "{:?}", name),
            Self::Generic(family) => write!(f, "{}", family),
        }
    }
}

#[derive(Clone)]
struct ParseList<'a> {
    source: &'a [u8],
    len: usize,
    pos: usize,
}

impl<'a> Iterator for ParseList<'a> {
    type Item = CowFontFamily;

    fn next(&mut self) -> Option<Self::Item> {
        let mut quote = None;
        let mut pos = self.pos;
        while pos < self.len && {
            let ch = self.source[pos];
            ch.is_ascii_whitespace() || ch == b','
        } {
            pos += 1;
        }
        self.pos = pos;
        if pos >= self.len {
            return None;
        }
        let first = self.source[pos];
        let mut start = pos;
        match first {
            b'"' | b'\'' => {
                quote = Some(first);
                pos += 1;
                start += 1;
            }
            _ => {}
        }
        if let Some(quote) = quote {
            while pos < self.len {
                if self.source[pos] == quote {
                    self.pos = pos + 1;
                    return Some(CowFontFamily::Named(Arc::from(
                        core::str::from_utf8(self.source.get(start..pos)?)
                            .ok()?
                            .trim(),
                    )));
                }
                pos += 1;
            }
            self.pos = pos;
            return Some(CowFontFamily::Named(Arc::from(
                core::str::from_utf8(self.source.get(start..pos)?)
                    .ok()?
                    .trim(),
            )));
        }
        let mut end = start;
        while pos < self.len {
            if self.source[pos] == b',' {
                pos += 1;
                break;
            }
            pos += 1;
            end += 1;
        }
        self.pos = pos;
        let name = core::str::from_utf8(self.source.get(start..end)?)
            .ok()?
            .trim();
        Some(match GenericFamily::parse(name) {
            Some(family) => CowFontFamily::Generic(family),
            _ => CowFontFamily::Named(Arc::from(name)),
        })
    }
}

/// Font settings that can be supplied as a raw source string or
/// a parsed slice.
#[derive(Clone, PartialEq, Debug)]
pub enum CowFontSettings<T>
where
    [T]: ToOwned,
    <[T] as ToOwned>::Owned: fmt::Debug + PartialEq + Clone,
{
    /// Setting source in CSS format.
    Source(Arc<str>),
    /// List of settings.
    List(Arc<[T]>),
}



impl<'a, T> From<&'a str> for CowFontSettings<T>
where
    [T]: ToOwned,
    <[T] as ToOwned>::Owned: fmt::Debug + PartialEq + Clone,
{
    fn from(value: &'a str) -> Self {
        Self::Source(Arc::from(value))
    }
}

// impl<'a, T> From<&'a [T]> for CowFontSettings<T>
// where
//     [T]: ToOwned,
//     <[T] as ToOwned>::Owned: fmt::Debug + PartialEq + Clone,
// {
//     fn from(value: &'a [T]) -> Self {
//         Self::List(Arc::from(value))
//     }
// }