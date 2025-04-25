//! Serializable and serliazable style properties.
//! 
//! Eqivalent to the reference counterpart from [`Parley's`](parley)
//! crate.
use std::sync::Arc;

use masonry::core::BrushIndex;
// using peniko directly to enable serde feature
// use peniko::Brush;
use parley::{FontStyle, FontWeight, FontWidth, GenericFamily};
use serde::{Deserialize, Serialize};
use swash::Setting;

use super::{CowFontFamily, CowFontSettings, CowFontStack, CowStyleProperty};

/// Properties that define a style.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SdStyleProperty {
    /// Font family stack.
    FontStack(SdFontStack),
    /// Font size.
    FontSize(f32),
    /// Font width.
    FontWidth(f32),
    /// Font style.
    FontStyle(SdFontStyle),
    /// Font weight.
    FontWeight(f32),
    /// Font variation settings.
    FontVariations(SdFontSettings<SdFontVariation>),
    /// Font feature settings.
    FontFeatures(SdFontSettings<SdFontFeature>),
    /// Locale.
    Locale(Option<String>),
    /// Brush for rendering text.
    Brush(usize),
    /// Underline decoration.
    Underline(bool),
    /// Offset of the underline decoration.
    UnderlineOffset(Option<f32>),
    /// Size of the underline decoration.
    UnderlineSize(Option<f32>),
    /// Brush for rendering the underline decoration.
    UnderlineBrush(Option<usize>),
    /// Strikethrough decoration.
    Strikethrough(bool),
    /// Offset of the strikethrough decoration.
    StrikethroughOffset(Option<f32>),
    /// Size of the strikethrough decoration.
    StrikethroughSize(Option<f32>),
    /// Brush for rendering the strikethrough decoration.
    StrikethroughBrush(Option<usize>),
    /// Line height multiplier.
    LineHeight(f32),
    /// Extra spacing between words.
    WordSpacing(f32),
    /// Extra spacing between letters.
    LetterSpacing(f32),
}

/// Prioritized sequence of font families.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-family>
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SdFontStack {
    /// Font family list in CSS format.
    Source(String),
    /// Single font family.
    Single(SdFontFamily),
    /// Ordered list of font families.
    List(Vec<SdFontFamily>),
}

/// Named or generic font family.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-family>
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SdFontFamily {
    /// Named font family.
    Named(String),
    /// Generic font family.
    Generic(SdGenericFamily),
}

/// Describes a generic font family.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum SdGenericFamily {
    /// Glyphs have finishing strokes, flared or tapering ends, or have actual
    /// serifed endings.
    Serif = 0,
    /// Glyphs have stroke endings that are plain.
    SansSerif = 1,
    /// All glyphs have the same fixed width.
    Monospace = 2,
    /// Glyphs in cursive fonts generally have either joining strokes or other
    /// cursive characteristics beyond those of italic typefaces. The glyphs
    /// are partially or completely connected, and the result looks more like
    /// handwritten pen or brush writing than printed letter work.
    Cursive = 3,
    /// Fantasy fonts are primarily decorative fonts that contain playful
    /// representations of characters
    Fantasy = 4,
    /// Glyphs are taken from the default user interface font on a given
    /// platform.
    SystemUi = 5,
    /// The default user interface serif font.
    UiSerif = 6,
    /// The default user interface sans-serif font.
    UiSansSerif = 7,
    /// The default user interface monospace font.
    UiMonospace = 8,
    /// The default user interface font that has rounded features.
    UiRounded = 9,
    /// Fonts that are specifically designed to render emoji.
    Emoji = 10,
    /// This is for the particular stylistic concerns of representing
    /// mathematics: superscript and subscript, brackets that cross several
    /// lines, nesting expressions, and double struck glyphs with distinct
    /// meanings.
    Math = 11,
    /// A particular style of Chinese characters that are between serif-style
    /// Song and cursive-style Kai forms. This style is often used for
    /// government documents.
    FangSong = 12,
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the bytemuck impl.
}

/// Font settings that can be supplied as a raw source string or
/// a parsed slice.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SdFontSettings<T> {
    /// Setting source in CSS format.
    Source(String),
    /// List of settings.
    List(Vec<T>),
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SdFontWidth(f32);

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum SdFontStyle {
    /// An upright or "roman" style.
    Normal,
    /// Generally a slanted style, originally based on semi-cursive forms.
    /// This often has a different structure from the normal style.
    Italic,
    /// Oblique (or slanted) style with an optional angle in degrees,
    /// counter-clockwise from the vertical.
    Oblique(Option<f32>),
}

/// Setting for a font variation.
pub type SdFontVariation = SdSetting<f32>;

/// Setting for a font feature.
pub type SdFontFeature = SdSetting<u16>;

/// Setting combining a tag and a value for features and variations.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct SdSetting<T> {
    /// The tag that identifies the setting.
    pub tag: u32,
    /// The value for the setting.
    pub value: T,
}

impl From<&CowStyleProperty<BrushIndex>> for SdStyleProperty {
    fn from(value: &CowStyleProperty<BrushIndex>) -> Self {
        match value {
            CowStyleProperty::FontStack(v) => SdStyleProperty::FontStack(v.into()),
            CowStyleProperty::FontSize(v) => SdStyleProperty::FontSize(v.clone()),
            CowStyleProperty::FontWidth(v) => SdStyleProperty::FontWidth(v.ratio()),
            CowStyleProperty::FontStyle(v) => SdStyleProperty::FontStyle(v.into()),
            CowStyleProperty::FontWeight(v) => SdStyleProperty::FontWeight(v.value()),
            CowStyleProperty::FontVariations(v) => SdStyleProperty::FontVariations(v.into()),
            CowStyleProperty::FontFeatures(v) => SdStyleProperty::FontFeatures(v.into()),
            CowStyleProperty::Locale(v) => SdStyleProperty::Locale(v.as_ref().map(ToString::to_string)),
            CowStyleProperty::Brush(v) => SdStyleProperty::Brush(v.clone().0),
            CowStyleProperty::Underline(v) => SdStyleProperty::Underline(v.clone()),
            CowStyleProperty::UnderlineOffset(v) => SdStyleProperty::UnderlineOffset(v.clone()),
            CowStyleProperty::UnderlineSize(v) => SdStyleProperty::UnderlineSize(v.clone()),
            CowStyleProperty::UnderlineBrush(v) => SdStyleProperty::UnderlineBrush(v.clone().map(|v| v.0)),
            CowStyleProperty::Strikethrough(v) => SdStyleProperty::Strikethrough(v.clone()),
            CowStyleProperty::StrikethroughOffset(v) => SdStyleProperty::StrikethroughOffset(v.clone()),
            CowStyleProperty::StrikethroughSize(v) => SdStyleProperty::StrikethroughSize(v.clone()),
            CowStyleProperty::StrikethroughBrush(v) => SdStyleProperty::StrikethroughBrush(v.clone().map(|v| v.0)),
            CowStyleProperty::LineHeight(v) => SdStyleProperty::LineHeight(v.clone()),
            CowStyleProperty::WordSpacing(v) => SdStyleProperty::WordSpacing(v.clone()),
            CowStyleProperty::LetterSpacing(v) => SdStyleProperty::LetterSpacing(v.clone()),
        }
    }
}

impl From<SdStyleProperty> for CowStyleProperty<BrushIndex> {
    fn from(value: SdStyleProperty) -> Self {
        match value {
            SdStyleProperty::FontStack(v) => CowStyleProperty::FontStack(v.into()),
            SdStyleProperty::FontSize(v) => CowStyleProperty::FontSize(v.clone()),
            SdStyleProperty::FontWidth(v) => CowStyleProperty::FontWidth(FontWidth::from_ratio(v)),
            SdStyleProperty::FontStyle(v) => CowStyleProperty::FontStyle(v.into()),
            SdStyleProperty::FontWeight(v) => CowStyleProperty::FontWeight(FontWeight::new(v)),
            SdStyleProperty::FontVariations(v) => CowStyleProperty::FontVariations(v.into()),
            SdStyleProperty::FontFeatures(v) => CowStyleProperty::FontFeatures(v.into()),
            SdStyleProperty::Locale(v) => CowStyleProperty::Locale(v.map(Arc::from)),
            SdStyleProperty::Brush(v) => CowStyleProperty::Brush(BrushIndex(v)),
            SdStyleProperty::Underline(v) => CowStyleProperty::Underline(v.clone()),
            SdStyleProperty::UnderlineOffset(v) => CowStyleProperty::UnderlineOffset(v.clone()),
            SdStyleProperty::UnderlineSize(v) => CowStyleProperty::UnderlineSize(v.clone()),
            SdStyleProperty::UnderlineBrush(v) => CowStyleProperty::UnderlineBrush(v.map(|v| BrushIndex(v))),
            SdStyleProperty::Strikethrough(v) => CowStyleProperty::Strikethrough(v.clone()),
            SdStyleProperty::StrikethroughOffset(v) => CowStyleProperty::StrikethroughOffset(v.clone()),
            SdStyleProperty::StrikethroughSize(v) => CowStyleProperty::StrikethroughSize(v.clone()),
            SdStyleProperty::StrikethroughBrush(v) => CowStyleProperty::StrikethroughBrush(v.map(|v| BrushIndex(v))),
            SdStyleProperty::LineHeight(v) => CowStyleProperty::LineHeight(v.clone()),
            SdStyleProperty::WordSpacing(v) => CowStyleProperty::WordSpacing(v.clone()),
            SdStyleProperty::LetterSpacing(v) => CowStyleProperty::LetterSpacing(v.clone()),
        }
    }
}

impl From<&CowFontStack> for SdFontStack {
    fn from(value: &CowFontStack) -> Self {
        match value {
            CowFontStack::Source(v) => SdFontStack::Source(v.to_string()),
            CowFontStack::Single(v) => SdFontStack::Single(v.into()),
            CowFontStack::List(v) => SdFontStack::List(v.iter().map(SdFontFamily::from).collect()),
        }
    }
}

impl From<SdFontStack> for CowFontStack {
    fn from(value: SdFontStack) -> Self {
        match value {
            SdFontStack::Source(v) => CowFontStack::Source(v.into()),
            SdFontStack::Single(v) => CowFontStack::Single(v.into()),
            SdFontStack::List(v) => {
                let vec: Vec<CowFontFamily> = v.into_iter().map(CowFontFamily::from).collect();
                CowFontStack::List(Arc::from(vec))
            },
        }
    }
}

impl From<&CowFontFamily> for SdFontFamily {
    fn from(value: &CowFontFamily) -> Self {
        match value {
            CowFontFamily::Named(v) => SdFontFamily::Named(v.to_string()),
            CowFontFamily::Generic(v) => SdFontFamily::Generic(v.into()),
        }
    }
}

impl From<SdFontFamily> for CowFontFamily {
    fn from(value: SdFontFamily) -> Self {
        match value {
            SdFontFamily::Named(v) => CowFontFamily::Named(v.into()),
            SdFontFamily::Generic(v) => CowFontFamily::Generic(v.into()),
        }
    }
}

impl From<&GenericFamily> for SdGenericFamily {
    fn from(value: &GenericFamily) -> Self {
        match value {
            GenericFamily::Serif => SdGenericFamily::Serif,
            GenericFamily::SansSerif => SdGenericFamily::SansSerif,
            GenericFamily::Monospace => SdGenericFamily::Monospace,
            GenericFamily::Cursive => SdGenericFamily::Cursive,
            GenericFamily::Fantasy => SdGenericFamily::Fantasy,
            GenericFamily::SystemUi => SdGenericFamily::SystemUi,
            GenericFamily::UiSerif => SdGenericFamily::UiSerif,
            GenericFamily::UiSansSerif => SdGenericFamily::UiSansSerif,
            GenericFamily::UiMonospace => SdGenericFamily::UiMonospace,
            GenericFamily::UiRounded => SdGenericFamily::UiRounded,
            GenericFamily::Emoji => SdGenericFamily::Emoji,
            GenericFamily::Math => SdGenericFamily::Math,
            GenericFamily::FangSong => SdGenericFamily::FangSong,
        }
    }
}

impl From<SdGenericFamily> for GenericFamily {
    fn from(value: SdGenericFamily) -> Self {
        match value {
            SdGenericFamily::Serif => GenericFamily::Serif,
            SdGenericFamily::SansSerif => GenericFamily::SansSerif,
            SdGenericFamily::Monospace => GenericFamily::Monospace,
            SdGenericFamily::Cursive => GenericFamily::Cursive,
            SdGenericFamily::Fantasy => GenericFamily::Fantasy,
            SdGenericFamily::SystemUi => GenericFamily::SystemUi,
            SdGenericFamily::UiSerif => GenericFamily::UiSerif,
            SdGenericFamily::UiSansSerif => GenericFamily::UiSansSerif,
            SdGenericFamily::UiMonospace => GenericFamily::UiMonospace,
            SdGenericFamily::UiRounded => GenericFamily::UiRounded,
            SdGenericFamily::Emoji => GenericFamily::Emoji,
            SdGenericFamily::Math => GenericFamily::Math,
            SdGenericFamily::FangSong => GenericFamily::FangSong,
        }
    }
}

impl From<&FontStyle> for SdFontStyle {
    fn from(value: &FontStyle) -> Self {
        match value {
            FontStyle::Normal => SdFontStyle::Normal,
            FontStyle::Italic => SdFontStyle::Italic,
            FontStyle::Oblique(v) => SdFontStyle::Oblique(v.clone()),
        }
    }
}

impl From<SdFontStyle> for FontStyle {
    fn from(value: SdFontStyle) -> Self {
        match value {
            SdFontStyle::Normal => FontStyle::Normal,
            SdFontStyle::Italic => FontStyle::Italic,
            SdFontStyle::Oblique(v) => FontStyle::Oblique(v.clone()),
        }
    }
}

impl<T: Clone + core::fmt::Debug + PartialEq> From<&CowFontSettings<Setting<T>>> for SdFontSettings<SdSetting<T>> {
    fn from(value: &CowFontSettings<Setting<T>>) -> Self {
        match value {
            CowFontSettings::Source(v) => SdFontSettings::Source(v.to_string()),
            CowFontSettings::List(v) => SdFontSettings::List(v.iter().map(SdSetting::from).collect()),
        }
    }
}

impl<T: Clone + core::fmt::Debug + PartialEq> From<SdFontSettings<SdSetting<T>>> for CowFontSettings<Setting<T>> {
    fn from(value: SdFontSettings<SdSetting<T>>) -> Self {
        match value {
            SdFontSettings::Source(v) => CowFontSettings::Source(Arc::from(v)),
            SdFontSettings::List(v) => {
                let vec: Vec<Setting<T>> = v.into_iter().map(Setting::from).collect();
                CowFontSettings::List(Arc::from(vec))
            }
        }
    }
}

// impl From<&CowFontSettings<u16>> for SdFontSettings<u16> {
//     fn from(value: &CowFontSettings<u16>) -> Self {
//         match value {
//             CowFontSettings::Source(v) => SdFontSettings::Source(v.to_string()),
//             CowFontSettings::List(v) => SdFontSettings::List(v.to_vec()),
//         }
//     }
// }

// impl From<SdFontSettings<f32>> for CowFontSettings<f32> {
//     fn from(value: SdFontSettings<f32>) -> Self {
//         match value {
//             SdFontSettings::Source(v) => CowFontSettings::Source(Arc::from(v)),
//             SdFontSettings::List(v) => CowFontSettings::List(Arc::from(v)),
//         }
//     }
// }

impl<T: Clone> From<&Setting<T>> for SdSetting<T> {
    fn from(value: &Setting<T>) -> Self {
        Self { tag: value.tag, value: value.value.clone() }
    }
}

impl<T: Clone> From<SdSetting<T>> for Setting<T> {
    fn from(value: SdSetting<T>) -> Self {
        Self { tag: value.tag, value: value.value }
    }
}