use std::sync::Arc;

use masonry::core::BrushIndex;
use text::{CowStyleProperty, TextRelated};
use xilem_core::MessageResult;

pub mod text;

pub struct BasicPreferences {
    pub text_related: TextRelated,
    /// The preferences that are currently showing
    pub visible_kind: PreferencesKind,
}

/// The reach of the preference change
#[derive(Clone, Debug)]
pub enum PreferenceReach {
    /// Preferences changes will apply globally
    Global,
    /// Preferences changes for specific kinds of items
    Priority(Arc<str>),
}

/// The kind of the preferences
#[derive(Clone, Copy, Debug)]
pub enum PreferencesKind {
    /// General preferences
    General,
    /// Text styling preferences
    TextStyling,
}

#[derive(Default, Clone, Debug)]
pub enum PreferencesMessageResult {
    /// Show font list to modify font for the given preferences reach
    ShowFontList(PreferenceReach),
    /// Select a font
    SelectFont {
        name: Arc<str>,
        reach: PreferenceReach,
    },
    SetStyleProperty {
        property: CowStyleProperty<BrushIndex>,
        reach: PreferenceReach,
    },
    /// Sets the preferences reach for the preferences view
    SetPreferencesReach(PreferenceReach),
    /// Set the view preferences kind
    SetPreferencesKind(PreferencesKind),
    /// Change language
    ChangeLanguage,
    /// Modified font preview text
    ModifiedFontPreviewText,
    /// When data that is represented in multiple widgets
    /// is changed by one of the widgets, we request the
    /// rebuilding of other dependent widgets to make them up to date.
    DependentWidgetsNeedsUpdating,
    /// The action has been handled and can now be returned to the Xilem driver
    /// to issue a [`xilem::View::rebuild`].
    #[allow(unused)]
    #[default]
    Handled,
}

pub fn handled_action() -> MessageResult<PreferencesMessageResult> {
    MessageResult::Action(PreferencesMessageResult::Handled)
}