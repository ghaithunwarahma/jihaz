use std::sync::{Arc, Mutex};
use parley::fontique::{Collection, CollectionOptions};
use serde::{Deserialize, Serialize};
use crate::preferences::PreferenceReach;

use super::{
    item_ids::{MAIN_BUTTON, MAIN_LABEL, SMALL_BUTTON, SMALL_LABEL, STATUS_BAR_ITEM}, 
    multi_script_font_preview_text, CowFontFamily, CowStyleProperty, 
    GeneralTextStyles, Language, LanguageTextStyles, SdGeneralTextStyles, 
    TextRelated
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SdTextRelated {
    pub language: Language,
    /// Styles that apply globally, unless overridding by priority styles
    pub global_styles: SdLanguageTextStyles,
    /// These are used to resolve the styles applied for items related to certain identifier.
    /// 
    /// The resolved styles inherit `priority styles` first, then inherits those
    /// of `global styles`
    pub priority_styles: Vec<(String, SdLanguageTextStyles)>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SdLanguageTextStyles(pub Vec<(Language, SdGeneralTextStyles)>);

impl From<&TextRelated> for SdTextRelated {
    fn from(value: &TextRelated) -> Self {
        Self {
            language: value.language,
            global_styles: (&value.global_styles).into(),
            priority_styles: value.priority_styles
                .iter()
                .map(|(id, styles)| {
                    (id.to_string(), styles.into())
                })
                .collect(),
        }
    }
}

impl From<SdTextRelated> for TextRelated {
    fn from(value: SdTextRelated) -> Self {
        Self {
            language: value.language,
            global_styles: value.global_styles.into(),
            priority_styles: Arc::new(value.priority_styles
                .into_iter()
                .map(|(id, styles)| {
                    (Arc::from(id), styles.into())
                })
                .collect()),
            selected_reach: PreferenceReach::Global,
            font_preview_text: multi_script_font_preview_text(value.language).into(),
            show_styling_preferences: false,
            font_collection: Arc::new(Mutex::new(Collection::new(CollectionOptions {
                system_fonts: true,
                ..Default::default()
            }))),
        }
    }
}

impl Default for SdTextRelated {
    fn default() -> Self {
        const LARGE_TEXT: f32 = 16.0;
        const VERY_SMALL_TEXT: f32 = 10.0;
        const SMALL_TEXT: f32 = 12.0;
        Self {
            language: Language::English,
            global_styles: SdLanguageTextStyles(vec![
                (Language::Arabic, (&GeneralTextStyles::default_styles()
                    .with_style(CowFontFamily::Named("Wafeq".into()))
                ).into()),
                (Language::English, (&GeneralTextStyles::default_styles()
                ).into()),
            ]),
            priority_styles: vec![
                (MAIN_BUTTON.to_string(), SdLanguageTextStyles(vec![
                        (Language::Arabic, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(LARGE_TEXT))
                        ).into()),
                        (Language::English, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(LARGE_TEXT))
                        ).into()),
                ])),
                (MAIN_LABEL.to_string(), SdLanguageTextStyles(vec![
                        (Language::Arabic, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(LARGE_TEXT))
                        ).into()),
                        (Language::English, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(LARGE_TEXT))
                        ).into()),
                ])),
                (SMALL_BUTTON.to_string(), SdLanguageTextStyles(vec![
                        (Language::Arabic, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(SMALL_TEXT))
                        ).into()),
                        (Language::English, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(SMALL_TEXT))
                        ).into()),
                ])),
                (SMALL_LABEL.to_string(), SdLanguageTextStyles(vec![
                        (Language::Arabic, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(LARGE_TEXT))
                        ).into()),
                        (Language::English, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(LARGE_TEXT))
                        ).into()),
                ])),
                (STATUS_BAR_ITEM.to_string(), SdLanguageTextStyles(vec![
                        (Language::Arabic, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(VERY_SMALL_TEXT))
                        ).into()),
                        (Language::English, (&GeneralTextStyles::empty()
                            .with_style(CowStyleProperty::FontSize(VERY_SMALL_TEXT))
                        ).into()),
                ])),
            ]
        }
    }
}

impl From<&LanguageTextStyles> for SdLanguageTextStyles {
    fn from(value: &LanguageTextStyles) -> Self {
        Self(value.0
            .iter()
            .map(|(lang, styles)| {
                (*lang, styles.into())
            })
            .collect()
        )
    }
}

impl From<SdLanguageTextStyles> for LanguageTextStyles {
    fn from(value: SdLanguageTextStyles) -> Self {
        Self(Arc::new(value.0
            .into_iter()
            .map(|(lang, styles)| {
                (lang, styles.into())
            })
            .collect()),
        )
    }
}