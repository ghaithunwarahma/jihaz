use std::sync::Arc;

use masonry::core::WidgetId;
use xilem::{view::{sized_box, textbox, FlexExt, MainAxisAlignment}, LineBreaking, WidgetView};
use xilem_core::{MessageResult, ViewId};

use crate::{
    preferences::{BasicPreferences, PreferenceReach, PreferencesKind, PreferencesMessageResult}, 
    text::{
        item_ids::{MAIN_BUTTON, SMALL_LABEL}, CowFontFamily, CowStyleProperty, Language, TextRelated
    }, theme::color
};
use super::{
    action_button, expand_to_parent_width, flex_col, flex_col_consumed_fully, flex_row_consumed_fully, font_preview, label, portal_consumed_fully, OverseeExt
};

pub fn preferences_view(state: &mut BasicPreferences) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    flex_row_consumed_fully((
        preferences_list(state),
        preferences_kind_view(state),
    ))
    .with_rtl(state.text_related.language.is_rtl())
}

pub fn preferences_list(state: &mut BasicPreferences) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    let sls = state.text_related.resolved_styles(SMALL_LABEL);

    flex_col((
        action_button(
            label(IN::PreferencesKind(PreferencesKind::General).to_string(state.text_related.language), sls.clone()), 
            PreferencesMessageResult::SetPreferencesKind(PreferencesKind::General)
        ),
        action_button(
            label(IN::PreferencesKind(PreferencesKind::TextStyling).to_string(state.text_related.language), sls.clone()), 
            PreferencesMessageResult::SetPreferencesKind(PreferencesKind::TextStyling)
        )
    ))
}

pub fn preferences_kind_view(state: &mut BasicPreferences) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    match state.visible_kind {
        PreferencesKind::General => general_preferences_view(state),
        PreferencesKind::TextStyling => text_styling_preferences_view(&mut state.text_related),
    }
}

pub fn general_preferences_view(state: &mut BasicPreferences) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    let sls = state.text_related.resolved_styles(SMALL_LABEL);
    flex_col((
        label(IN::Language.to_string(state.text_related.language), sls.clone()),
        action_button(label(state.text_related.language.to_str(), sls.clone()), PreferencesMessageResult::ChangeLanguage)
    ))
}

pub fn text_styling_preferences_view(state: &mut TextRelated) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    let mut preferences = vec![
        text_styling_preferences_for_reach(state, PreferenceReach::Global)
    ];
    for name in state.priority_styles.clone().keys() {
        preferences.push(
            text_styling_preferences_for_reach(state, PreferenceReach::Priority(name.clone()))
        );
    }
    flex_col(preferences)
}

// pub fn text_styling_preferences_list(state: &mut TextRelated) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
//     let mbs = state.resolved_styles(MAIN_BUTTON);
//     let mut preference_reaches = vec![
//         action_button(label(
//             IN::PreferenceReach(PreferenceReach::Global).to_string(state.language), mbs.clone()), 
//             PreferencesMessageResult::SetPreferencesReach(PreferenceReach::Global)
//         )
//     ];
//     let mut named_priority_reaches: Vec<_> = state.priority_styles
//         .keys()
//         .map(|name| {
//             action_button(label(
//                 IN::PreferenceReach(PreferenceReach::Priority(name.clone())).to_string(state.language), mbs.clone()), 
//                 PreferencesMessageResult::SetPreferencesReach(PreferenceReach::Priority(name.clone()))
//             )
//         })
//         .collect();
//     preference_reaches.append(&mut named_priority_reaches);
//     flex_col(preference_reaches)
// }

fn text_styling_preferences_for_reach(state: &mut TextRelated, reach: PreferenceReach) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    let sls = state.resolved_styles(SMALL_LABEL);
    let mbs = state.resolved_styles(MAIN_BUTTON);
    let mut font_name = state
        .reachd_styles(&reach)
        .get_first_font_family_name()
        .unwrap();
    if let Some(new) = font_name.strip_prefix("\"") {
        font_name = new.to_string();
    }
    if let Some(new) = font_name.strip_suffix("\"") {
        font_name = new.to_string();
    }
    let reach2 = reach.clone();
    flex_row_consumed_fully((
        // font name
        flex_col((
            label(IN::FontFamilyName.to_string(state.language), sls.clone()),
            action_button(label(font_name, mbs.clone()), PreferencesMessageResult::ShowFontList(reach2.clone()))
        )),
        // font size
        flex_col((
            label(IN::FontSize.to_string(state.language), sls.clone()),
            textbox(
                state.reachd_styles(&reach2.clone()).get_font_size().as_ref().map(ToString::to_string).unwrap_or_default(),
                |_state: &mut TextRelated, _content| {
                    PreferencesMessageResult::Handled
                }
            ).on_enter(move |_state: &mut TextRelated, content| {
                if let Ok(size) = content.parse::<f32>() {
                    PreferencesMessageResult::SetStyleProperty {
                        property: CowStyleProperty::FontSize(size), 
                        reach: reach2.clone(),
                    }
                } else {
                    PreferencesMessageResult::Handled
                }
            })
        )),
    ))
    .main_axis_alignment(state.flex_main_axis_alignment())
    .with_rtl(state.language.is_rtl())
}

pub fn fonts_selection_view(
    state: &mut TextRelated,
    styling_preferences_widget_id: WidgetId,
    styling_preferences_view_id: ViewId,
    reach: PreferenceReach,
    problem_fonts: &[&str],
) -> impl WidgetView<BasicPreferences, PreferencesMessageResult> {
    let lbm = LineBreaking::WordWrap;
    let mbs = state.resolved_styles(MAIN_BUTTON);
    let sls = state.resolved_styles(SMALL_LABEL);
    let mut selected_general_font = state
        .reachd_styles(&reach)
        .get_first_font_family_name()
        .unwrap();
    if let Some(new) = selected_general_font.strip_prefix("\"") {
        selected_general_font = new.to_string();
    }
    if let Some(new) = selected_general_font.strip_suffix("\"") {
        selected_general_font = new.to_string();
    }
    let preview = &state.font_preview_text;
    let collection = state.font_collection.clone();
    let mut fonts_list: Vec<_> = collection
        .lock()
        .unwrap()
        .family_names()
        .filter(|name| {
            // !PROBLEM_FONTS.contains(name)
            !problem_fonts.iter().any(|n| name.contains(n))
        })
        .map(Arc::from)
        .collect();
    // sorting the fonts
    fonts_list.sort_unstable();
    let fonts_list: Vec<_> = fonts_list
        .into_iter()
        .map(|name| {
            // // Debugging mods from AI
            // // Create a base style for the preview that uses SystemUI font and explicit sizing
            let preview_styles = mbs.clone();
            // // Set SystemUI as default font family for safe rendering
            // preview_styles.insert(CowStyleProperty::FontStack(CowFontStack::Single(CowFontFamily::Named("SystemUI".into()))));
            // // Add explicit font size for consistent rendering
            // preview_styles.insert(CowStyleProperty::FontSize(14.0));
            // // Add explicit line height for stable layout
            // preview_styles.insert(CowStyleProperty::LineHeight(1.5));
            
            let reach = reach.clone();
            let mut font_preview = font_preview(
                name, preview.clone(), sls.clone(), preview_styles, 
                move |_, name| {
                    PreferencesMessageResult::SelectFont {
                        name: name.clone(),
                        reach: reach.clone(),
                    }
                }
            );
            font_preview.name = font_preview.name.line_break_mode(lbm).alignment(parley::Alignment::Start);
            font_preview.preview = font_preview.preview.line_break_mode(lbm);
            expand_to_parent_width(font_preview)
        })
        .collect();

    sized_box(
        flex_col_consumed_fully((
            expand_to_parent_width(label(selected_general_font.clone(), sls)),
            textbox(state.font_preview_text.to_string(), |state: &mut BasicPreferences, content| {
                state.text_related.font_preview_text = content.as_str().into();
                PreferencesMessageResult::ModifiedFontPreviewText
            }),
            portal_consumed_fully(
                flex_col_consumed_fully(fonts_list)
            )
            .with_rtl(state.language.is_rtl())
            .flex(1.0)
        )
    )
        .cross_axis_alignment(state.flex_cross_axis_alignment())
    )
    .background(color::default_dark::BACKGROUND_2X.with_alpha(0.809))
    .oversee(styling_preferences_view_id, styling_preferences_widget_id, ())
    .on_build(move |_, _| collection.clone().into())
    .on_action_message_result(move |collection, action: PreferencesMessageResult, state: &mut BasicPreferences| {
        match action {
            PreferencesMessageResult::ShowFontList(reach) => {
                state.text_related.selected_reach = reach;
                crate::preferences::handled_action()
            }
            PreferencesMessageResult::SetPreferencesReach(reach) => {
                state.text_related.selected_reach = reach;
                crate::preferences::handled_action()
            }
            PreferencesMessageResult::SetPreferencesKind(kind) => {
                state.visible_kind = kind;
                crate::preferences::handled_action()
            }
            PreferencesMessageResult::SetStyleProperty { property, reach } => {
                let styles_mut = state.text_related.styles_mut_for_reach(&reach);
                if styles_mut.insert_was_different_or_empty(property) {
                    crate::preferences::handled_action()
                } else {
                    MessageResult::Nop
                }
            }
            PreferencesMessageResult::SelectFont { mut name, reach } => {
                if let Some(new) = name.strip_prefix("\"") {
                    name = new.into();
                }
                if let Some(new) = name.strip_suffix("\"") {
                    name = new.into();
                }
                let collection = collection.unwrap();
                if let Some(_family_info) = collection.clone().lock().unwrap().family_by_name(&name) {
                    tracing::debug!("selected font {name:?} maps to family_info");
                } else {
                    tracing::error!("selected font {name:?}, but it does not map to family_info in the font collection");
                }
                let style = CowFontFamily::Named(name.clone());
                let styles_mut = state.text_related.styles_mut_for_reach(&reach);
                if styles_mut.insert_was_different_or_empty(style.into()) {
                    crate::preferences::handled_action()
                } else {
                    MessageResult::Nop
                }
            }
            PreferencesMessageResult::ModifiedFontPreviewText => {
                crate::preferences::handled_action()
            }
            PreferencesMessageResult::ChangeLanguage => unimplemented!(),
            PreferencesMessageResult::DependentWidgetsNeedsUpdating => unimplemented!(),
            PreferencesMessageResult::Handled => crate::preferences::handled_action(),
        }
    })
}

/// Localised item names, for the preferences view
#[derive(Clone, Debug)]
pub enum IN {
    /// Preference reach button items
    PreferenceReach(PreferenceReach),
    /// Font family name item
    FontFamilyName,
    /// Font size item
    FontSize,
    /// Preferences kind
    PreferencesKind(PreferencesKind),
    ///
    Language
}

impl IN {
    pub fn to_string(&self, language: Language) -> String {
        match language {
            Language::Arabic => match self {
                IN::PreferenceReach(reach) => match reach {
                    PreferenceReach::Global => "التصميم العام".into(),
                    PreferenceReach::Priority(item_name) => {
                        format!("تصميم {}", item_name)
                    }
                }
                IN::FontFamilyName => "إسم عائلة الخط".into(),
                IN::FontSize => "حجم الخط".into(),
                IN::PreferencesKind(kind) => match kind {
                    PreferencesKind::General => "عام".into(),
                    PreferencesKind::TextStyling => "تصميم النص".into(),
                }
                IN::Language => "اللغة".into(),
            }
            Language::English => match self {
                IN::PreferenceReach(reach) => match reach {
                    PreferenceReach::Global => "Global styles".into(),
                    PreferenceReach::Priority(item_name) => {
                        format!("{} styles", item_name)
                    }
                }
                IN::FontFamilyName => "Font family name".into(),
                IN::FontSize => "Font size".into(),
                IN::PreferencesKind(kind) => match kind {
                    PreferencesKind::General => "General".into(),
                    PreferencesKind::TextStyling => "Text styling".into(),
                }
                IN::Language => "Language".into(),
            }
        }
    }
}

// pub fn main_content_zstack<OuterState, Action, Context, Message, StateF, InnerView, Component>(
//     component: Component,
//     state: &mut OuterState,
//     // This parameter ordering does run into https://github.com/rust-lang/rustfmt/issues/3605
//     // Our general advice is to make sure that the lens arguments are short enough...
//     map: StateF,
// ) -> impl WidgetView<OuterState, Action>
// where
//     StateF: Fn(&mut OuterState) -> &mut TextRelated + Send + Sync + 'static,
//     Component: FnOnce(&mut TextRelated) -> InnerView,
//     InnerView: View<TextRelated, Action, Context, Message>,
//     Context: ViewPathTracker,
// {
//     let mapped = map(state);
//     let view = component(mapped);
//     // MapState {
//     //     child: view,
//     //     map_state: map,
//     //     phantom: PhantomData,
//     // }
//     let inner_state = map(state);
//     if inner_state.show_styling_preferences {
//         (main_content, Some(
//             lens(fonts_list, state, |state| &mut state.text_related)
//             .map_message_result(|message_result| {
//                 message_result.map(|_| AppMessageResult::Handled)
//             })
//         ))
//     } else {
//         (main_content, None)
//     }
// }