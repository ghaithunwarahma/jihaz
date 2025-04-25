use std::marker::PhantomData;

use xilem::{Pod, ViewCtx, WidgetView};
use xilem_core::{
    DynMessage, MessageResult, Mut, View, ViewId, ViewMarker,
};
use crate::widget;

/// A trait that extends a [`WidgetView`] with methods to wrap it in a Debug view
/// to paint its size in a background color.
pub trait DebugExt<State, Action>: WidgetView<State, Action> {
    /// Maps the [`MessageResult<Action>`] type of this view,
    /// converting [`MessageResult<Action>`] type into a [`MessageResult<OuterAction>`].
    fn debug(self) -> Debug<Self, State, Action>
    where
        State: 'static,
        Action: 'static,
        Self: Sized
    {
        Debug { child: self, phantom: PhantomData }
    }
}

impl<State, Action, V: WidgetView<State, Action>> DebugExt<State, Action> for V {}


/// A font preview element, with the preview text presented in the given font.
/// # Example
///
/// ```ignore
/// use xilem::view::label;
/// use xilem_plus_for_jihaz::view::Debug;
///
/// let child = label("Calibri");
/// debug(child, preview, child_styles, preview_styles)
/// ```
pub fn debug<Child, State, Action>(child: Child) -> Debug<Child, State, Action> {
    Debug {
        child,
        phantom: PhantomData,
    }
}

pub struct Debug<V, State, Action> {
    pub child: V,
    phantom: PhantomData<(State, Action)>,
}

impl<V, State, Action> ViewMarker for Debug<V, State, Action> {}

impl<V, State, Action> View<State, Action, ViewCtx> for Debug<V, State, Action>
where
    V: WidgetView<State, Action>,
    State: 'static,
    Action: 'static,
{
    type Element = Pod<widget::Debug<V::Widget>>;

    type ViewState = V::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = self.child.build(ctx);
        let pod = ctx.new_pod(widget::Debug::new_pod(
            child.into_widget_pod()
        ));
        (pod, child_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        let child_child = widget::Debug::child_mut(&mut element);
        self.child.rebuild(&prev.child, view_state, ctx, child_child);
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        let child_child = widget::Debug::child_mut(&mut element);
        self.child.teardown(view_state, ctx, child_child);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State) -> MessageResult<Action>
    {
        self.child.message(view_state, id_path, message, app_state)
    }
}
