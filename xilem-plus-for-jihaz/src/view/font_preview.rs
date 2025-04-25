use std::sync::Arc;

use masonry::core::{ArcStr, BrushIndex};
use xilem::{view::PointerButton, Pod, ViewCtx};
use xilem_core::{
    DynMessage, MessageResult, Mut, View, ViewId, ViewMarker, ViewPathTracker,
};
use crate::{text::{CowFontFamily, CowFontStack, CowStyleProperty, GeneralTextStyles}, widget};
use super::{label, Label};

/// A font preview element, with the preview text presented in the given font.
/// # Example
///
/// ```ignore
/// use xilem_plus_for_jihaz::text_related::{CowFontFamily, CowStyleProperty, GeneralTextStyles};
/// use xilem_plus_for_jihaz::view::FontPreview;
///
/// let name = "Calibri";
/// let preview = "A text sentence for previewing the font";
/// let name_styles = GeneralTextStyles::default_styles();
/// let mut preview_styles = name_styles.clone();
/// preview_styles.insert(CowStyleProperty::FontSize(14.0));
/// 
/// font_preview(name, preview, name_styles, preview_styles)
/// ```
pub fn font_preview<State, Action>(
    name: impl Into<ArcStr>,
    preview: impl Into<ArcStr>,
    name_styles: GeneralTextStyles<BrushIndex>,
    mut preview_styles: GeneralTextStyles<BrushIndex>,
    callback: impl Fn(
        &mut State, &Arc<str>
    ) -> Action + Send + Sync + 'static,
) -> FontPreview<State, Action> {
    // setting the font property of the preview label styles as
    // a named font family property that has its font name taken from the font name label.
    let name = name.into();

    // // Debugging mods from AI
    // // Create a base style for the preview that uses SystemUI font
    // let mut preview_styles = preview_styles;
    // // Use font stack with fallback to system UI
    // let preview_fonts = CowFontStack::List(vec![
    //     CowFontFamily::Named(name.clone()),
    //     CowFontFamily::Generic(parley::GenericFamily::SystemUi)
    // ].into());
    // preview_styles.insert(CowStyleProperty::FontStack(preview_fonts));
    // // Add explicit font size for consistent rendering
    // preview_styles.insert(CowStyleProperty::FontSize(14.0));
    // // Add explicit line height for stable layout
    // preview_styles.insert(CowStyleProperty::LineHeight(1.5));

    preview_styles.insert(CowFontFamily::Named(name.clone()).into());

    
    FontPreview {
        name: label(name, name_styles),
        preview: label(preview, preview_styles),
        callback: Box::new(move |state, name, button| match button {
            PointerButton::Primary => MessageResult::Action(callback(state, name)),
            _ => MessageResult::Nop,
        }),
    }
}

/// A font preview element, with the preview text presented in the given font.
/// # Example
///
/// ```ignore
/// use xilem_plus_for_jihaz::text_related::{CowFontFamily, CowStyleProperty, GeneralTextStyles};
/// use xilem_plus_for_jihaz::view::FontPreview;
///
/// let name = "Calibri";
/// let preview = "A text sentence for previewing the font";
/// let name_styles = GeneralTextStyles::default_styles();
/// let mut preview_styles = name_styles.clone();
/// preview_styles.insert(CowStyleProperty::FontSize(14.0));
/// 
/// FontPreview::new(name, preview, name_styles, preview_styles)
/// ```
pub fn font_preview_any_pointer<State, Action>(
    name: impl Into<ArcStr>,
    preview: impl Into<ArcStr>,
    name_styles: GeneralTextStyles<BrushIndex>,
    mut preview_styles: GeneralTextStyles<BrushIndex>,
    callback: impl Fn(
        &mut State, &Arc<str>, PointerButton
    ) -> Action + Send + Sync + 'static,
) -> FontPreview<State, Action> {
    // setting the font property of the preview label styles as
    // a named font family property that has its font name taken from the font name label.
    let name = name.into();
    // Create a base style for the preview that uses SystemUI font
    let mut preview_styles = preview_styles;
    // Use font stack with fallback to system UI
    let preview_fonts = CowFontStack::List(vec![
        CowFontFamily::Named(name.clone()),
        CowFontFamily::Generic(parley::GenericFamily::SystemUi)
    ].into());
    preview_styles.insert(CowStyleProperty::FontStack(preview_fonts));
    // Add explicit font size for consistent rendering
    preview_styles.insert(CowStyleProperty::FontSize(14.0));
    // Add explicit line height for stable layout
    preview_styles.insert(CowStyleProperty::LineHeight(1.5));

    FontPreview {
        name: label(name, name_styles),
        preview: label(preview, preview_styles),
        callback: Box::new(move |state, name, button| {
            MessageResult::Action(callback(state, name, button))
        }),
    }
}

pub struct FontPreview<State, Action> {
    pub name: Label,
    pub preview: Label,
    pub callback: Box<dyn Fn(
        &mut State, &Arc<str>, PointerButton
    ) -> MessageResult<Action> + Send + Sync + 'static>,
}

const NAME_VIEW_ID: ViewId = ViewId::new(0);
const PREVIEW_VIEW_ID: ViewId = ViewId::new(1);

impl<State, Action> ViewMarker for FontPreview<State, Action> {}

impl<State, Action> View<State, Action, ViewCtx> for FontPreview<State, Action>
where
    State: 'static,
    Action: 'static,
{
    type Element = Pod<widget::FontPreview>;

    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (name, ()) = ctx.with_id(NAME_VIEW_ID, |ctx| {
            View::<State, Action, _>::build(&self.name, ctx)
        });
        let (preview, ()) = ctx.with_id(PREVIEW_VIEW_ID, |ctx| {
            View::<State, Action, _>::build(&self.preview, ctx)
        });
        ctx.with_leaf_action_widget(|ctx| {
            ctx.new_pod(widget::FontPreview::from_label_pods(
                self.name.label.clone(),
                name.into_widget_pod(),
                preview.into_widget_pod()
            ))
        })
    }

    fn rebuild(
        &self,
        prev: &Self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        let child_name = widget::FontPreview::name_mut(&mut element);
        ctx.with_id(NAME_VIEW_ID, |ctx| {
            View::<State, Action, _>::rebuild(&self.name, &prev.name, &mut (), ctx, child_name);
        });
        let child_preview = widget::FontPreview::preview_mut(&mut element);
        ctx.with_id(PREVIEW_VIEW_ID, |ctx| {
            View::<State, Action, _>::rebuild(&self.preview, &prev.preview, &mut (), ctx, child_preview);
        });
    }

    fn teardown(
        &self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        let child_name = widget::FontPreview::name_mut(&mut element);
        ctx.with_id(NAME_VIEW_ID, |ctx| {
            View::<State, Action, _>::teardown(&self.name, &mut (), ctx, child_name);
        });
        let child_preview = widget::FontPreview::preview_mut(&mut element);
        ctx.with_id(PREVIEW_VIEW_ID, |ctx| {
            View::<State, Action, _>::teardown(&self.preview, &mut (), ctx, child_preview);
        });
    }

    fn message(
        &self,
        _: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State) -> MessageResult<Action>
    {
        match id_path.split_first() {
            Some((&NAME_VIEW_ID, rest)) => self.name.message(&mut (), rest, message, app_state),
            Some((&PREVIEW_VIEW_ID, rest)) => self.preview.message(&mut (), rest, message, app_state),
            None => match message.downcast::<masonry::core::Action>() {
                Ok(action) => {
                    if let masonry::core::Action::ButtonPressed(button) = *action {
                        (self.callback)(app_state, &self.name.label, button)
                    } else {
                        tracing::error!("Wrong action type in FontPreview::message: {action:?}");
                        MessageResult::Stale(action)
                    }
                }
                Err(message) => {
                    tracing::error!("Wrong message type in FontPreview::message: {message:?}");
                    MessageResult::Stale(message)
                }
            }
            _ => {
                tracing::warn!("Got unexpected id path in FontPreview::message");
                MessageResult::Stale(message)
            }
        }
    }
}
