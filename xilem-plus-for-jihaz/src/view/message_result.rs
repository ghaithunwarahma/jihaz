use std::marker::PhantomData;
use xilem::{Pod, ViewCtx, WidgetView};
use xilem_core::{
    DynMessage, MessageResult, Mut, View, ViewId, ViewMarker
};

/// A trait that extends a [`WidgetView`] with methods to handle its [`MessageResult<Action>`] type.
pub trait MapMessageResultExt<State, Action>: WidgetView<State, Action> {
    /// Maps the [`MessageResult<Action>`] type of this view,
    /// converting [`MessageResult<Action>`] type into a [`MessageResult<OuterAction>`].
    fn map_message_result<OuterAction, F>(
        self, map_message_result: F
    ) -> MapMessageResult<Self, F, State, Action, OuterAction>
    where
        State: 'static,
        Action: 'static,
        OuterAction: 'static,
        F: Fn(MessageResult<Action>) -> MessageResult<OuterAction> + 'static,
        Self: Sized
    {
        MapMessageResult { child: self, map_message_result, phantom: PhantomData }
    }

    /// Maps the [`MessageResult<Action>`] type of this view,
    /// converting the [`MessageResult`] action type into an empty tuple.
    fn empty_message_result(
        self
    ) -> MapMessageResult<Self, impl Fn(MessageResult<Action>) -> MessageResult<()> + 'static , State, Action, ()>
    where
        State: 'static,
        Action: 'static,
        Self: Sized
    {
        MapMessageResult { child: self, map_message_result: EmptyMessageResult::empty, phantom: PhantomData }
    }
}

impl<State, Action, V: WidgetView<State, Action>> MapMessageResultExt<State, Action> for V {}

pub struct MapMessageResult<ChildView, F, State, Action, OuterAction> {
    child: ChildView,
    map_message_result: F,
    phantom: PhantomData<(State, Action, OuterAction)>,
}

impl<ChildView, F, State, Action, OuterAction> ViewMarker 
    for MapMessageResult<ChildView, F, State, Action, OuterAction> {}

impl<ChildView, F, State, Action, OuterAction> View<State, OuterAction, ViewCtx>
    for MapMessageResult<ChildView, F, State, Action, OuterAction>
where
    ChildView: WidgetView<State, Action>,
    F: Fn(MessageResult<Action>) -> MessageResult<OuterAction> + 'static,
    State: 'static,
    OuterAction: 'static,
    Action: 'static,
{
    type Element = Pod<ChildView::Widget>;

    type ViewState = ChildView::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = self.child.build(ctx);
        (child, child_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<Self::Element>,
    ) {
        self.child.rebuild(&prev.child, view_state, ctx, element);
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<Self::Element>,
    ) {
        self.child.teardown(view_state, ctx, element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State) -> MessageResult<OuterAction>
    {
        (self.map_message_result)(self.child.message(view_state, id_path, message, app_state))
    }
}

/// A convenience method to convert the [`MessageResult`] action type from `Action` into an
/// empty tuple, discarding the `Action` value.
pub trait EmptyMessageResult<Action> {
    /// A convenience method to convert the [`MessageResult`] action type from `Action` into an
    /// empty tuple, discarding the `Action` value.
    fn empty(self) -> MessageResult<()>;
}

impl<Action> EmptyMessageResult<Action> for MessageResult<Action> {
    fn empty(self) -> MessageResult<()> {
        self.map(|_| ())
    }
}