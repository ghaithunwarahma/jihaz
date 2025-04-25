
use std::{mem::Discriminant, ops::Deref, sync::Arc};
use hashbrown::HashMap;
use masonry::core::BrushIndex;
use parley::{GenericFamily, StyleProperty};
use super::{cow_style::CowStyleProperty, CowFontFamily, CowFontStack};

/// An owned, copy-on-write collection of [`CowStyleProperties`](super::CowStyleProperty), 
/// containing at most one of each property. 
/// With a wrapping [`Arc`] to enable copy-on-write and Sync.
/// It can also be cheaply compared to other [`GeneralTextStyles`] instances to see if
/// they point to the same inner data.
///
/// This is used by [`PlainEditor`](crate::editor::PlainEditor) to provide a reasonably ergonomic
/// mutable API for styles applied to all text managed by the editor.
/// This can be accessed using [`PlainEditor::edit_styles`](crate::editor::PlainEditor::edit_styles).
///
/// These styles apply to the text generally.
/// They are sufficient for rich text, as they do not have a corresponding range.
/// 
/// This type is an similar to [`parley::StyleSet`].
#[derive(Clone, PartialEq, Debug)]
pub struct GeneralTextStyles<B: parley::Brush>(
    Arc<HashMap<Discriminant<CowStyleProperty<B>>, CowStyleProperty<B>>>,
);

// --- MARK: GeneralTextStyles impl ---
impl<B: parley::Brush> GeneralTextStyles<B> {
    
    pub fn empty() -> Self {
        Self(Default::default())
    }

    /// Create a new collection of styles.
    ///
    /// The font size will be `font_size`, and can be overwritten at runtime by
    /// [inserting](Self::insert) a new [`FontSize`](crate::CowStyleProperty::FontSize).
    pub fn new(font_size: f32) -> Self {
        let mut this = Self(Default::default());
        this.insert(CowStyleProperty::FontSize(font_size));
        this
    }

    /// Add `style` to this collection, returning any overwritten value.
    ///
    /// Note: Adding a [font stack](crate::CowStyleProperty::FontStack) to this collection is not
    /// additive, and instead overwrites any previously added font stack.
    pub fn insert(&mut self, style: CowStyleProperty<B>) -> Option<CowStyleProperty<B>> {
        let discriminant = core::mem::discriminant(&style);
        Arc::make_mut(&mut self.0).insert(discriminant, style)
    }

    /// Add `style` to this collection, returning true if it was different
    /// than the overritten value.
    ///
    /// Note: Adding a [font stack](crate::CowStyleProperty::FontStack) to this collection is not
    /// additive, and instead overwrites any previously added font stack.
    pub fn insert_was_different(&mut self, style: CowStyleProperty<B>) -> bool {
        let new = style.clone();
        self.insert(style).map(|old| old != new).unwrap_or(false)
    }

    /// Add `style` to this collection, returning true if it was different
    /// than the overritten value, or if the added property did not exist in self before.
    ///
    /// Note: Adding a [font stack](crate::CowStyleProperty::FontStack) to this collection is not
    /// additive, and instead overwrites any previously added font stack.
    pub fn insert_was_different_or_empty(&mut self, style: CowStyleProperty<B>) -> bool {
        let new = style.clone();
        self.insert(style).map(|old| old != new).unwrap_or(true)
    }

    /// Inherit from the given `styles properties`, keeping the properties of self that
    /// don't exist in the given styles
    /// 
    /// Only returns true if atleast one `property` in the given `styles` has had its `style` updated,
    /// or didn't have a given property before.
    pub fn inherit(&mut self, given_styles: &GeneralTextStyles<B>) -> bool {
        let mut changed = false;
        for (_, style) in given_styles.inner() {
            changed |= self.insert_was_different_or_empty(style.clone());
        }
        changed
    }

    /// [Retain](std::vec::Vec::retain) only the styles for which `f` returns true.
    ///
    /// Styles which are removed return to their default values.
    ///
    /// Removing the [font size](crate::CowStyleProperty::FontSize) is not recommended, as an unspecified
    /// fallback font size will be used.
    pub fn retain(&mut self, mut f: impl FnMut(&CowStyleProperty<B>) -> bool) {
        Arc::make_mut(&mut self.0).retain(|_, v| f(v));
    }

    /// Remove the style with the discriminant `property`.
    ///
    /// Styles which are removed return to their default values.
    ///
    /// To get the discriminant requires constructing a valid `CowStyleProperty` for the
    /// the desired property and passing it to [`core::mem::discriminant`].
    /// Getting this discriminant is usually possible in a `const` context.
    ///
    /// Removing the [font size](crate::CowStyleProperty::FontSize) is not recommended, as an unspecified
    /// fallback font size will be used.
    pub fn remove(
        &mut self,
        property: Discriminant<CowStyleProperty<B>>,
    ) -> Option<CowStyleProperty<B>> {
        Arc::make_mut(&mut self.0).remove(&property)
    }

    /// Read the raw underlying storage of this.
    ///
    /// Write access is not provided due to the invariant that keys
    /// are the discriminant of their corresponding value.
    pub fn inner(&self) -> &HashMap<Discriminant<CowStyleProperty<B>>, CowStyleProperty<B>> {
        &self.0
    }

    pub fn iterate_styles<'a>(&'a self) -> impl Iterator<Item = StyleProperty<'a, B>> {
        self.0.values().map(StyleProperty::from)
    }
}

impl GeneralTextStyles<BrushIndex> {
    pub fn default_styles() -> Self {
        let mut styles = Self::new(
            // This is size 15.0
            masonry::theme::TEXT_SIZE_NORMAL
        );
        styles.insert(CowStyleProperty::LineHeight(1.2));
        styles.insert(GenericFamily::SystemUi.into());
        styles
    }

    pub fn with_style(mut self, style: impl Into<CowStyleProperty<BrushIndex>>) -> Self {
        self.insert(style.into());
        self
    }
}

// --- MARK: Getters ---
impl<B: parley::Brush> GeneralTextStyles<B> {
    /// Get the style of a `property`. Note that the give property style value doesn't matter.
    pub fn get(&self, style: CowStyleProperty<B>) -> Option<&CowStyleProperty<B>> {
        let discriminant = core::mem::discriminant(&style);
        self.0.get(&discriminant)
    }

    /// Get the font stack style property.
    pub fn get_font_stack(&self) -> Option<&CowFontStack> {
        // This is just to get the discriminant, its value is meaningless
        let property = CowStyleProperty::FontStack(CowFontStack::Single(CowFontFamily::Generic(GenericFamily::Serif)));
        let discriminant = core::mem::discriminant(&property);
        self.0.get(&discriminant).map(|style| {
            let CowStyleProperty::FontStack(stack) = style else { unreachable!() };
            stack
        })
    }

    /// Get the font family style property.
    pub fn get_font_families(&self) -> Option<Vec<CowFontFamily>> {
        self.get_font_stack().map(|stack| {
            match stack {
                CowFontStack::Source(source) => CowFontFamily::parse_list(source).collect(),
                CowFontStack::Single(family) => vec![family.clone()],
                CowFontStack::List(items) => items.deref().to_vec(),
            }
        })
    }

    /// Get the font family name style property.
    pub fn get_first_font_family_name(&self) -> Option<String> {
        self.get_font_stack().map(|stack| {
            match stack {
                CowFontStack::Source(source) => CowFontFamily::parse_list(source).next().unwrap().to_string(),
                CowFontStack::Single(family) => family.to_string(),
                CowFontStack::List(items) => items.first().unwrap().to_string(),
            }
        })
    }

    /// Get the font size style property.
    pub fn get_font_size(&self) -> Option<f32> {
        // This is just to get the discriminant, its value is meaningless
        let property = CowStyleProperty::FontSize(15.0);
        let discriminant = core::mem::discriminant(&property);
        self.0.get(&discriminant).map(|style| {
            let CowStyleProperty::FontSize(size) = style else { unreachable!() };
            *size
        })
    }
}