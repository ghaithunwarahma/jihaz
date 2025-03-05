use xilem::{Pod, ViewCtx};
use xilem_core::{MessageResult, Mut, View, ViewId, ViewMarker};
use crate::{
    app::AppState, widget::window::WindowWi, app_helpers::EmptyMessageResult
};

use super::tablet::{TabletVi, TabletViState};

/// Represents the viewed spread of body or a page of the Quran
pub struct WindowVi {
    pub tablet: TabletVi,
}

/// The ViewState is retained, unlike View, in which a new instance is created
/// whenever app_logic is ran.
/// 
/// Since message occurs in the async AppTask task, and the rebuild occurs in
/// the synchronious App task, at the moment I think loading work should be
/// done in the message method and not in the rebuild method.
/// 
/// Also either I 
pub struct WindowViState {
    pub tablet: TabletViState,
}
impl ViewMarker for WindowVi {}
impl View<AppState, (), ViewCtx> for WindowVi {
    type Element = Pod<WindowWi>;
    type ViewState = WindowViState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        // println!(" @@@ ±±± @@@ called build");
        // eprintln!("is_two_leaf: {}", self.child_leaves_seq.is_two_leaf);

        let (child_element, child_state) = self.tablet.build(ctx);

        let element = ctx.with_action_widget(|ctx| {
            ctx.new_pod(WindowWi::new(child_element))
        });
        let state = WindowViState { tablet: child_state };
        (element, state)
    }
    
    fn rebuild<'el>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        let child_element = element.ctx.get_mut(&mut element.widget.tablet.inner);
        self.tablet.rebuild(&prev.tablet, &mut view_state.tablet, ctx, child_element);
        element
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: xilem_core::Mut<'_, Self::Element>,
    ) {
        let child_element = element.ctx.get_mut(&mut element.widget.tablet.inner);
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

// When the state changes, xilem automatically calls rebuild
impl WindowVi {
    pub fn new(tablet: TabletVi) -> WindowVi {
        WindowVi {
            tablet,
        }
    }
}