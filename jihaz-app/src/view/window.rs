use xilem::{view::PointerButton, Pod, ViewCtx};
use xilem_core::{MessageResult, Mut, View, ViewId, ViewMarker};
use crate::{
    app::AppState, app_helpers::EmptyMessageResult, app_message::AppMessageResult, widget
};

use super::tablet::{Tablet, TabletState};

/// Represents the viewed spread of body
pub struct Window<F> {
    pub tablet: Tablet<F>,
}

/// The ViewState is retained, unlike View, in which a new instance is created
/// whenever app_logic is ran.
/// 
/// Since message occurs in the async AppTask task, and the rebuild occurs in
/// the synchronious App task, at the moment I think loading work should be
/// done in the message method and not in the rebuild method.
/// 
/// Also either I 
pub struct WindowState {
    pub tablet: TabletState,
}

// When the state changes, xilem automatically calls rebuild
impl<F> Window<F> {
    pub fn new(tablet: Tablet<F>) -> Window<F> {
        Window {
            tablet,
        }
    }
}

impl<F> ViewMarker for Window<F> {}

impl<F> View<AppState, (), ViewCtx> for Window<F>
where
    F: Fn(&mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + Sync + 'static,
{
    type Element = Pod<widget::Window>;
    type ViewState = WindowState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        // println!(" @@@ ±±± @@@ called build");
        // eprintln!("is_two_leaf: {}", self.child_leaves_seq.is_two_leaf);

        let (child_element, child_state) = self.tablet.build(ctx);

        let element = ctx.with_action_widget(|ctx| {
            ctx.new_pod(widget::Window::new(child_element.into_widget_pod()))
        });
        let state = WindowState { tablet: child_state };
        (element, state)
    }
    
    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        let child_element = widget::Window::child_mut(&mut element);
        self.tablet.rebuild(&prev.tablet, &mut view_state.tablet, ctx, child_element);
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: xilem_core::Mut<'_, Self::Element>,
    ) {
        let child_element = widget::Window::child_mut(&mut element);
        self.tablet.teardown(&mut view_state.tablet, ctx, child_element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: xilem_core::DynMessage,
        app_state: &mut AppState,
    ) -> MessageResult<()> {
        self.tablet
            .message(&mut view_state.tablet, id_path, message, app_state)
            .empty()
    }
}