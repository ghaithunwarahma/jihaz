//! An owned, copy-on-write and Sync style properties.
//! 
//! Eqivalent to the reference counterpart from [`Parley's`](parley)
//! crate.
use std::{borrow::Cow, sync::Arc};

use parley::{
    Brush, FontFamily, FontFeature, FontSettings, FontStack, FontStyle, FontVariation, FontWeight, FontWidth, GenericFamily, StyleProperty
};
use swash::Setting;

use super::{CowFontFamily, CowFontSettings, CowFontStack};

/// Properties that define a style.
#[derive(Clone, PartialEq, Debug)]
pub enum CowStyleProperty<B: Brush> {
    /// Font family stack.
    FontStack(CowFontStack),
    /// Font size.
    FontSize(f32),
    /// Font width.
    FontWidth(FontWidth),
    /// Font style.
    FontStyle(FontStyle),
    /// Font weight.
    FontWeight(FontWeight),
    /// Font variation settings.
    FontVariations(CowFontSettings<FontVariation>),
    /// Font feature settings.
    FontFeatures(CowFontSettings<FontFeature>),
    /// Locale.
    Locale(Option<Arc<str>>),
    /// Brush for rendering text.
    Brush(B),
    /// Underline decoration.
    Underline(bool),
    /// Offset of the underline decoration.
    UnderlineOffset(Option<f32>),
    /// Size of the underline decoration.
    UnderlineSize(Option<f32>),
    /// Brush for rendering the underline decoration.
    UnderlineBrush(Option<B>),
    /// Strikethrough decoration.
    Strikethrough(bool),
    /// Offset of the strikethrough decoration.
    StrikethroughOffset(Option<f32>),
    /// Size of the strikethrough decoration.
    StrikethroughSize(Option<f32>),
    /// Brush for rendering the strikethrough decoration.
    StrikethroughBrush(Option<B>),
    /// Line height multiplier.
    LineHeight(f32),
    /// Extra spacing between words.
    WordSpacing(f32),
    /// Extra spacing between letters.
    LetterSpacing(f32),
}

impl<B: Brush> From<CowFontStack> for CowStyleProperty<B> {
    fn from(fs: CowFontStack) -> Self {
        CowStyleProperty::FontStack(fs)
    }
}

impl<B: Brush> From<&[CowFontFamily]> for CowStyleProperty<B> {
    fn from(fs: &[CowFontFamily]) -> Self {
        CowStyleProperty::FontStack(fs.into())
    }
}

impl<B: Brush> From<CowFontFamily> for CowStyleProperty<B> {
    fn from(f: CowFontFamily) -> Self {
        CowStyleProperty::FontStack(CowFontStack::from(f))
    }
}

impl<B: Brush> From<GenericFamily> for CowStyleProperty<B> {
    fn from(f: GenericFamily) -> Self {
        CowStyleProperty::FontStack(f.into())
    }
}

// --- MARK: Cow to Ref ---

impl<'a, B: Brush> From<&'a CowStyleProperty<B>> for StyleProperty<'a, B> {
    fn from(value: &'a CowStyleProperty<B>) -> Self {
        match value {
            CowStyleProperty::FontStack(v) => StyleProperty::FontStack(v.into()),
            CowStyleProperty::FontSize(v) => StyleProperty::FontSize(v.clone()),
            CowStyleProperty::FontWidth(v) => StyleProperty::FontWidth(*v),
            CowStyleProperty::FontStyle(v) => StyleProperty::FontStyle(*v),
            CowStyleProperty::FontWeight(v) => StyleProperty::FontWeight(*v),
            CowStyleProperty::FontVariations(v) => StyleProperty::FontVariations(v.into()),
            CowStyleProperty::FontFeatures(v) => StyleProperty::FontFeatures(v.into()),
            CowStyleProperty::Locale(v) => StyleProperty::Locale(v.as_ref().map(Arc::as_ref)),
            CowStyleProperty::Brush(v) => StyleProperty::Brush(v.clone()),
            CowStyleProperty::Underline(v) => StyleProperty::Underline(v.clone()),
            CowStyleProperty::UnderlineOffset(v) => StyleProperty::UnderlineOffset(v.clone()),
            CowStyleProperty::UnderlineSize(v) => StyleProperty::UnderlineSize(v.clone()),
            CowStyleProperty::UnderlineBrush(v) => StyleProperty::UnderlineBrush(v.clone()),
            CowStyleProperty::Strikethrough(v) => StyleProperty::Strikethrough(v.clone()),
            CowStyleProperty::StrikethroughOffset(v) => StyleProperty::StrikethroughOffset(v.clone()),
            CowStyleProperty::StrikethroughSize(v) => StyleProperty::StrikethroughSize(v.clone()),
            CowStyleProperty::StrikethroughBrush(v) => StyleProperty::StrikethroughBrush(v.clone()),
            CowStyleProperty::LineHeight(v) => StyleProperty::LineHeight(v.clone()),
            CowStyleProperty::WordSpacing(v) => StyleProperty::WordSpacing(v.clone()),
            CowStyleProperty::LetterSpacing(v) => StyleProperty::LetterSpacing(v.clone()),
        }
    }
}

impl<'a> From<&'a CowFontStack> for FontStack<'a> {
    fn from(value: &'a CowFontStack) -> Self {
        match value {
            CowFontStack::Source(v) => FontStack::Source(Cow::from(v.as_ref())),
            CowFontStack::Single(v) => FontStack::Single(FontFamily::from(v)),
            CowFontStack::List(v) => FontStack::List(v.iter().map(FontFamily::from).collect()),
        }
    }
}

impl<'a> From<&'a CowFontFamily> for FontFamily<'a> {
    fn from(value: &'a CowFontFamily) -> Self {
        match value {
            CowFontFamily::Named(v) => FontFamily::Named(Cow::from(v.as_ref())),
            CowFontFamily::Generic(v) => FontFamily::Generic(*v),
        }
    }
}

impl<'a, T: Clone + core::fmt::Debug + PartialEq> From<&'a CowFontSettings<Setting<T>>> for FontSettings<'a, Setting<T>> {
    fn from(value: &'a CowFontSettings<Setting<T>>) -> Self {
        match value {
            CowFontSettings::Source(v) => FontSettings::Source(Cow::from(v.as_ref())),
            CowFontSettings::List(v) => FontSettings::List(Cow::from(v.as_ref())),
        }
    }
}