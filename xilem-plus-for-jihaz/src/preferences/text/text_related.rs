use std::sync::{Arc, Mutex};
use hashbrown::HashMap;
use masonry::core::BrushIndex;
use parley::fontique::Collection;
use xilem::view::{CrossAxisAlignment, MainAxisAlignment};
use xilem_core::MessageResult;
use crate::preferences::PreferenceReach;
use super::{CowStyleProperty, GeneralTextStyles, Language};

/// useful identifiers for saving item specific text style information
pub mod item_ids {
    pub const MAIN_BUTTON: &'static str = "main_button";
    pub const MAIN_LABEL: &'static str = "main_label";
    pub const SMALL_BUTTON: &'static str = "small_button";
    pub const SMALL_LABEL: &'static str = "small_label";
    pub const STATUS_BAR_ITEM: &'static str = "status_bar_item";
}

// --- MARK: Styling ---
/// A type that has information used for dynamically styling text.
/// 
/// Contains a list of preferred font family names, for
/// a given language.
/// 
/// It implements [`MakeStyles`] to allow this type
/// to be used with other [`xilem_plus_for_jihaz`] dynamic styling widgets.
/// 
/// A styling type that makes [`parley::StylePropery`]
/// 
#[derive(Clone)]
pub struct TextRelated {
    pub language: Language,
    /// Styles that apply globally, unless overridding by priority styles
    pub global_styles: LanguageTextStyles,
    /// These are used to resolve the styles applied for items related to certain identifier.
    /// 
    /// The resolved styles inherit `priority styles` first, then inherits those
    /// of `global styles`
    pub priority_styles: Arc<HashMap<Arc<str>, LanguageTextStyles>>,

    // -- Runtime fields
    pub selected_reach: PreferenceReach,
    pub font_preview_text: Arc<str>,
    pub show_styling_preferences: bool,
    pub font_collection: Arc<Mutex<Collection>>,
}

/// General styles that apply per named language
#[derive(Clone, Debug)]
pub struct LanguageTextStyles(pub Arc<Vec<(Language, GeneralTextStyles<BrushIndex>)>>);

impl TextRelated {
    /// Gets a copy of the resolved general styles for the given identifier and the active language.
    pub fn resolved_styles(&self, id: impl AsRef<str>) -> GeneralTextStyles<BrushIndex> {
        let mut styles = self.global_styles();

        if let Some(priorities) = self.priority_styles.get(id.as_ref()) {
            let priority = priorities.general_styles(self.language);
            styles.inherit(&priority);
        }
        styles
    }

    /// Gets a copy of the general styles for the given preference reach and the active language.
    pub fn reachd_styles(&self, reach: &PreferenceReach) -> GeneralTextStyles<BrushIndex> {
        match reach {
            PreferenceReach::Global => self.global_styles(),
            PreferenceReach::Priority(id) => self.priority_styles(id.as_ref()),
        }
    }

    /// Gets a mutable reference to the general styles for the given preference reach and the active language.
    /// 
    /// This creates a new owned [`GeneralTextStyles`] if there's more than one strong [`Arc`] pointer.
    pub fn styles_mut_for_reach(&mut self, reach: &PreferenceReach) -> &mut GeneralTextStyles<BrushIndex> {
        match reach {
            PreferenceReach::Global => self.global_styles_mut(),
            PreferenceReach::Priority(id) => self.priority_styles_mut(id.as_ref()),
        }
    }

    /// Gets a copy of the priority general styles for the given identifier and the active language.
    pub fn priority_styles(&self, id: impl AsRef<str>) -> GeneralTextStyles<BrushIndex> {
        self.priority_styles
            .get(id.as_ref())
            .map(|languages| languages.general_styles(self.language))
            .expect("The given styles identifier must point to a profile already.")
    }

    /// Gets a mutable reference to the priority general styles for the given identifier and the active language.
    /// 
    /// This creates a new owned [`GeneralTextStyles`] if there's more than one strong [`Arc`] pointer.
    pub fn priority_styles_mut(&mut self, id: impl AsRef<str>) -> &mut GeneralTextStyles<BrushIndex> {
        Arc::make_mut(&mut self.priority_styles)
            .get_mut(id.as_ref())
            .map(|languages| languages.general_styles_mut(self.language))
            .expect("The given styles identifier must point to a profile already.")
    }

    /// Gets a copy of the global general styles for the active language.
    pub fn global_styles(&self) -> GeneralTextStyles<BrushIndex> {
        self.global_styles.general_styles(self.language)
    }

    /// Gets a mutable reference to the global general styles for the active language.
    /// 
    /// This creates a new owned [`GeneralTextStyles`] if there's more than one strong [`Arc`] pointer.
    pub fn global_styles_mut(&mut self) -> &mut GeneralTextStyles<BrushIndex> {
        self.global_styles.general_styles_mut(self.language)
    }

    pub fn change_language(&mut self) {
        self.language = match self.language {
            Language::English => Language::Arabic,
            Language::Arabic => Language::English,
        };
        self.font_preview_text = multi_script_font_preview_text(self.language).into();
    }
}

// --- MARK: Xilem widget related ---
impl TextRelated {
    pub fn flex_cross_axis_alignment(&self) -> CrossAxisAlignment {
        match self.language.is_rtl() {
            true => CrossAxisAlignment::End,
            false => CrossAxisAlignment::Start,
        }
    }

    pub fn flex_main_axis_alignment(&self) -> MainAxisAlignment {
        match self.language.is_rtl() {
            true => MainAxisAlignment::End,
            false => MainAxisAlignment::Start,
        }
    }
}

impl LanguageTextStyles {
    /// Gets a copy of the general styles for the given language.
    pub fn general_styles(&self, language: Language) -> GeneralTextStyles<BrushIndex> {
        for (lang, general_styles) in self.0.iter() {
            if *lang == language {
                return general_styles.clone().into();
            }
        }
        panic!("Any selected Language must have a GeneralTextStyles profile already created.");
    }

    /// Gets a mutable reference to the general styles for the given language.
    /// 
    /// This creates a new owned [`GeneralTextStyles`] if there's more than one strong [`Arc`] pointer
    pub fn general_styles_mut(&mut self, language: Language) -> &mut GeneralTextStyles<BrushIndex> {
        for (lang, general_styles) in Arc::make_mut(&mut self.0).iter_mut() {
            if *lang == language {
                return general_styles;
            }
        }
        panic!("Any selected Language must have a GeneralTextStyles profile already created.");
    }
}

pub const fn font_preview_text(language: Language) -> &'static str {
    match language {
        Language::Arabic => "نص لمعاينة الخط المختار",
        Language::English => "A text for previewing the selected font",
    }
}

pub fn multi_script_font_preview_text(language: Language) -> String {
    match language {
        Language::Arabic => [
            font_preview_text(Language::Arabic), font_preview_text(Language::English), "漢字"
        ].join(" "),
        Language::English => [
            font_preview_text(Language::English), "漢字", font_preview_text(Language::Arabic)
        ].join(" "),
    }
}