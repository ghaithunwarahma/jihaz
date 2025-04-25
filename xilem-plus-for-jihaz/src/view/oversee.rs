use std::marker::PhantomData;
use masonry::core::{WidgetId, WidgetMut};
use xilem::{Pod, ViewCtx, WidgetView};
use xilem_core::{
    DynMessage, MessageResult, Mut, View, ViewId, ViewMarker, ViewPathTracker,
};
use crate::widget;

pub struct Oversee<ChildView, State, Action, OwnViewState = (), ViewInput = ()>
where
    ChildView: WidgetView<State, Action>
{
    /// We set a ViewId for the [`Oversee`] view, it helps as [`Oversee`] can receive
    /// messages directly and handle them via the on_message closure call.
    /// 
    /// It's also helpful in routing messages down the view tree, as sometimes
    /// the [`Oversee`] view can be a member of a view sequence or a siblings of views
    /// in the view tree, and may contain a child view that receives messages.
    view_id: ViewId,

    /// We give the [`Oversee`] widget a reserved [`WidgetId`] to set it as an action widget.
    /// This lets us use the [`Oversee`] widget ID to send a 
    /// [`masonry::app::MasonryUserEvent::Action`] event that gets directed to the [`Oversee`] view.
    ///
    /// This also makes the [`Oversee`] view able to receive [`Oversee`] widget actions.
    ///
    /// This can be done by creating an application type that implements a [`winit::application::ApplicationHandler`],
    /// and then calling a [`winit::application::ApplicationHandler::user_event`] to send the action.
    /// 
    /// The action is then forwarded as a [`xilem::core::DynMessage`] on the [`xilem::MasonryDriver`]'s on_action call,
    /// and the [`WidgetId`] will be used to route the message to the [`Oversee`] view via the [`xilem::View::message`] call.
    widget_id: Option<WidgetId>,
    /// The child view
    child: ChildView,

    /// View input data for the [`Oversee`] view, the default is an empty tuple.
    /// 
    /// Helpful in rebuild comparisons,
    /// and is given as an argument for the on_rebuild closure function call.
    view_input: ViewInput,

    /// A caller on [`View::rebuild`]. It can create an optional own view state for the [`Oversee`] view.
    /// The default type for the own view state is an as empty typle.
    on_build: Option<Box<dyn Fn(
        &mut ViewCtx, &mut widget::Oversee<ChildView::Widget>
    ) -> Option<OwnViewState> + Send + Sync + 'static>>,

    /// A caller on [`View::rebuild`]
    on_rebuild: Option<Box<dyn Fn(
        &ViewInput, &ViewInput, Option<&mut OwnViewState>, &mut ViewCtx, &mut WidgetMut<'_, widget::Oversee<ChildView::Widget>>
    ) + Send + Sync + 'static>>,

    /// A caller on [`View::teardown`]
    on_teardown: Option<Box<dyn Fn(
        &ViewInput, Option<&mut OwnViewState>, &mut ViewCtx, &mut WidgetMut<'_, widget::Oversee<ChildView::Widget>>
    ) + Send + Sync + 'static>>,

    /// A caller on [`View::message`], that handles the [`DynMessage`] targetting to [`Oversee`] view
    on_message: Option<Box<dyn Fn(
        Option<&mut OwnViewState>, DynMessage, &mut State
    ) -> MessageResult<Action> + Send + Sync + 'static>>,

    /// A caller on [`View::message`], that handles the [`MessageResult::Action`] varient
    on_action_message_result: Option<Box<dyn Fn(
        Option<&mut OwnViewState>, Action, &mut State
    ) -> MessageResult<Action> + Send + Sync + 'static>>,

    phantom: PhantomData<(State, Action, OwnViewState)>,
}

pub struct OverseeViewState<ChildVS, OwnViewState> {
    child_view_state: ChildVS,
    own_view_state: Option<OwnViewState>,
}

impl<ChildVS, OwnViewState> OverseeViewState<ChildVS, OwnViewState> {
    pub fn child(&mut self) -> &mut ChildVS {
        &mut self.child_view_state
    }

    pub fn own(&mut self) -> Option<&mut OwnViewState> {
        self.own_view_state.as_mut()
    }
}

// --- MARK: OverseeExt ---
/// A trait that extends a [`WidgetView`] with a method to wrap it into an
/// [`Oversee`] view.
pub trait OverseeExt<State, Action, OwnViewState = (), ViewInput = ()>: WidgetView<State, Action> {
    fn oversee(
        self,
        view_id: ViewId,
        widget_id: impl Into<Option<WidgetId>>,
        view_input: ViewInput,
    ) -> Oversee<Self, State, Action, OwnViewState, ViewInput>
    where
        State: 'static,
        Action: 'static,
        OwnViewState: 'static,
        ViewInput: 'static,
        Self: Sized,    
    {
        Oversee {
            child: self,
            view_id,
            widget_id: widget_id.into(),
            view_input,
            on_build: None,
            on_rebuild: None,
            on_teardown: None,
            on_message: None,
            on_action_message_result: None,
            phantom: PhantomData,
        }
    }
}

impl<State, Action, OwnViewState, ViewInput, V: WidgetView<State, Action>>
    OverseeExt<State, Action, OwnViewState, ViewInput> for V {}

// --- MARK: BUILDERS ---
impl<ChildView, State, Action, OwnViewState, ViewInput>
    Oversee<ChildView, State, Action, OwnViewState, ViewInput> 
where
    ChildView: WidgetView<State, Action>,    
    OwnViewState: 'static,
    ViewInput: 'static,
    State: 'static,
    Action: 'static,
{
    /// This method needs to be called along with [`Oversee::new`] and the other [`Oversee`] builder methods 
    /// to help rust do type inference.
    /// 
    /// If you don't want to add an on_build fn, you can call the methods
    /// you want to use and the ones you don't want to use like so:
    ///
    /// # Example
    /// ```
    /// use xilem::view::{flex, button};
    /// use xilem_core::{MessageResult, ViewId};
    /// use xilem_plus_for_jihaz::view::Oversee;
    /// 
    /// struct AppState {
    ///     count: usize
    /// }
    /// 
    /// const WINDOW_VIEW_ID: ViewId = ViewId::new(0);
    /// 
    /// enum AppMsgResult {
    ///     Plus
    ///     Ninus
    /// }
    /// 
    /// Oversee::<_, _, _, _, _, _, AppState, AppMsgResult>::new(
    ///     WINDOW_VIEW_ID,
    ///     None,
    ///     flex((
    ///         button(label("+"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Plus)
    ///         })),
    ///         button(label("-"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Minus)
    ///         })),
    ///     ))
    ///     (),
    /// )
    /// .on_build(|_, _| {})
    /// .on_rebuild(|_, _, _, _, _| {})
    /// .on_teardown(|_, _, _, _| {})
    /// .on_message(|_, _, _| MessageResult::Nop)
    /// .on_action_message_result(|_, action, state| {
    ///     match action {
    ///         AppMsgResult::Plus => {
    ///             state.count += 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///         AppMsgResult::Minus => {
    ///             state.count -= 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///     }
    /// })
    /// ```
    pub fn on_build(
        mut self, 
        on_build: impl Fn(
            &mut ViewCtx, &mut widget::Oversee<ChildView::Widget>
        ) -> Option<OwnViewState> + Send + Sync + 'static,
    ) -> Self {
        self.on_build = Some(Box::new(on_build));
        self
    }

    /// This method needs to be called along with [`Oversee::new`] and the other [`Oversee`] builder methods 
    /// to help rust do type inference.
    /// 
    /// If you don't want to add an on_rebuild fn, you can call the methods
    /// you want to use and the ones you don't want to use like so:
    ///
    /// # Example
    /// ```
    /// use xilem::view::{flex, button};
    /// use xilem_core::{MessageResult, ViewId};
    /// use xilem_plus_for_jihaz::view::Oversee;
    /// 
    /// struct AppState {
    ///     count: usize
    /// }
    /// 
    /// const WINDOW_VIEW_ID: ViewId = ViewId::new(0);
    /// 
    /// enum AppMsgResult {
    ///     Plus
    ///     Ninus
    /// }
    /// 
    /// Oversee::<_, _, _, _, _, _, AppState, AppMsgResult>::new(
    ///     WINDOW_VIEW_ID,
    ///     None,
    ///     flex((
    ///         button(label("+"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Plus)
    ///         })),
    ///         button(label("-"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Minus)
    ///         })),
    ///     ))
    ///     (),
    /// )
    /// .on_build(|_, _| {})
    /// .on_rebuild(|_, _, _, _, _| {})
    /// .on_teardown(|_, _, _, _| {})
    /// .on_message(|_, _, _| MessageResult::Nop)
    /// .on_action_message_result(|_, action, state| {
    ///     match action {
    ///         AppMsgResult::Plus => {
    ///             state.count += 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///         AppMsgResult::Minus => {
    ///             state.count -= 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///     }
    /// })
    /// ```
    pub fn on_rebuild(
        mut self,
        on_rebuild: impl Fn(
            &ViewInput, &ViewInput, Option<&mut OwnViewState>, &mut ViewCtx, &mut WidgetMut<'_, widget::Oversee<ChildView::Widget>>
        ) + Send + Sync + 'static,
    ) -> Self {
        self.on_rebuild = Some(Box::new(on_rebuild));
        self
    }
    
    /// This method needs to be called along with [`Oversee::new`] and the other [`Oversee`] builder methods 
    /// to help rust do type inference.
    /// 
    /// If you don't want to add an on_teardown fn, you can call the methods
    /// you want to use and the ones you don't want to use like so:
    ///
    /// # Example
    /// ```
    /// use xilem::view::{flex, button};
    /// use xilem_core::{MessageResult, ViewId};
    /// use xilem_plus_for_jihaz::view::Oversee;
    /// 
    /// struct AppState {
    ///     count: usize
    /// }
    /// 
    /// const WINDOW_VIEW_ID: ViewId = ViewId::new(0);
    /// 
    /// enum AppMsgResult {
    ///     Plus
    ///     Ninus
    /// }
    /// 
    /// Oversee::<_, _, _, _, _, _, AppState, AppMsgResult>::new(
    ///     WINDOW_VIEW_ID,
    ///     None,
    ///     flex((
    ///         button(label("+"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Plus)
    ///         })),
    ///         button(label("-"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Minus)
    ///         })),
    ///     ))
    ///     (),
    /// )
    /// .on_build(|_, _| {})
    /// .on_rebuild(|_, _, _, _, _| {})
    /// .on_teardown(|_, _, _, _| {})
    /// .on_message(|_, _, _| MessageResult::Nop)
    /// .on_action_message_result(|_, action, state| {
    ///     match action {
    ///         AppMsgResult::Plus => {
    ///             state.count += 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///         AppMsgResult::Minus => {
    ///             state.count -= 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///     }
    /// })
    /// ```
    pub fn on_teardown(
        mut self, 
        on_teardown: impl Fn(
            &ViewInput, Option<&mut OwnViewState>, &mut ViewCtx, &mut WidgetMut<'_, widget::Oversee<ChildView::Widget>>
        ) + Send + Sync + 'static,
    ) -> Self {
        self.on_teardown = Some(Box::new(on_teardown));
        self
    }

    /// This method needs to be called along with [`Oversee::new`] and the other [`Oversee`] builder methods 
    /// to help rust do type inference.
    /// 
    /// If you don't want to add an on_message fn, you can call the methods
    /// you want to use and the ones you don't want to use like so:
    ///
    /// # Example
    /// ```
    /// use xilem::view::{flex, button};
    /// use xilem_core::{MessageResult, ViewId};
    /// use xilem_plus_for_jihaz::view::Oversee;
    /// 
    /// struct AppState {
    ///     count: usize
    /// }
    /// 
    /// const WINDOW_VIEW_ID: ViewId = ViewId::new(0);
    /// 
    /// enum AppMsgResult {
    ///     Plus
    ///     Ninus
    /// }
    /// 
    /// Oversee::<_, _, _, _, _, _, AppState, AppMsgResult>::new(
    ///     WINDOW_VIEW_ID,
    ///     None,
    ///     flex((
    ///         button(label("+"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Plus)
    ///         })),
    ///         button(label("-"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Minus)
    ///         })),
    ///     ))
    ///     (),
    /// )
    /// .on_build(|_, _| {})
    /// .on_rebuild(|_, _, _, _, _| {})
    /// .on_teardown(|_, _, _, _| {})
    /// .on_message(|_, _, _| MessageResult::Nop)
    /// .on_action_message_result(|_, action, state| {
    ///     match action {
    ///         AppMsgResult::Plus => {
    ///             state.count += 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///         AppMsgResult::Minus => {
    ///             state.count -= 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///     }
    /// })
    /// ```
    pub fn on_message(
        mut self, 
        on_message: impl Fn(
            Option<&mut OwnViewState>, DynMessage, &mut State
        ) -> MessageResult<Action> + Send + Sync + 'static,
    ) -> Self {
        self.on_message = Some(Box::new(on_message));
        self
    }

    /// This method needs to be called along with [`Oversee::new`] and the other [`Oversee`] builder methods 
    /// to help rust do type inference.
    /// 
    /// If you don't want to add an on_message fn, you can call the methods
    /// you want to use and the ones you don't want to use like so:
    ///
    /// # Example
    /// ```
    /// use xilem::view::{flex, button};
    /// use xilem_core::{MessageResult, ViewId};
    /// use xilem_plus_for_jihaz::view::Oversee;
    /// 
    /// struct AppState {
    ///     count: usize
    /// }
    /// 
    /// const WINDOW_VIEW_ID: ViewId = ViewId::new(0);
    /// 
    /// enum AppMsgResult {
    ///     Plus
    ///     Ninus
    /// }
    /// 
    /// Oversee::<_, _, _, _, _, _, AppState, AppMsgResult>::new(
    ///     WINDOW_VIEW_ID,
    ///     None,
    ///     flex((
    ///         button(label("+"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Plus)
    ///         })),
    ///         button(label("-"), |_state: &mut AppState, _| {
    ///             MessageResult::Action(AppMsgResult::Minus)
    ///         })),
    ///     ))
    ///     (),
    /// )
    /// .on_build(|_, _| {})
    /// .on_rebuild(|_, _, _, _, _| {})
    /// .on_teardown(|_, _, _, _| {})
    /// .on_message(|_, _, _| MessageResult::Nop)
    /// .on_action_message_result(|_, action, state| {
    ///     match action {
    ///         AppMsgResult::Plus => {
    ///             state.count += 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///         AppMsgResult::Minus => {
    ///             state.count -= 1;
    ///             MessageResult::RequestRebuild,
    ///         }
    ///     }
    /// })
    /// ```
    pub fn on_action_message_result(
        mut self, 
        on_action_message_result: impl Fn(
            Option<&mut OwnViewState>, Action, &mut State
        ) -> MessageResult<Action> + Send + Sync + 'static,
    ) -> Self {
        self.on_action_message_result = Some(Box::new(on_action_message_result));
        self
    }
}

// --- MARK: IMPL VIEW ---
impl<ChildView, State, Action, OwnViewState, ViewInput> ViewMarker 
    for Oversee<ChildView, State, Action, OwnViewState, ViewInput> 
where ChildView: WidgetView<State, Action> {}

impl<ChildView, State, Action, OwnViewState, ViewInput>
    View<State, Action, ViewCtx>
for Oversee<ChildView, State, Action, OwnViewState, ViewInput>
where
    ChildView: WidgetView<State, Action>,    
    OwnViewState: 'static,
    ViewInput: 'static,
    State: 'static,
    Action: 'static,
{
    type Element = Pod<widget::Oversee<ChildView::Widget>>;

    type ViewState = OverseeViewState<ChildView::ViewState, OwnViewState>;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = ctx
            .with_id(self.view_id, |ctx| self.child.build(ctx));
        
        let mut widget = widget::Oversee::new_pod(child.into_widget_pod());

        let own_view_state = self.on_build
            .as_ref()
            .and_then(|on_build| on_build(ctx, &mut widget));

        let pod = ctx.new_pod(widget);
        if let Some(id) = self.widget_id {
            ctx.record_action(id);
        }
        
        let view_state = OverseeViewState {
            child_view_state: child_state,
            own_view_state
        };
        (pod, view_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        if let Some(on_rebuild) = self.on_rebuild.as_ref() {
            on_rebuild(&self.view_input, &prev.view_input, view_state.own(), ctx, &mut element);
        }
        let child_element = widget::Oversee::child_mut(&mut element);
        ctx.with_id(self.view_id, |ctx| {
            self.child
                .rebuild(&prev.child, &mut view_state.child_view_state, ctx, child_element)
        });
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        if let Some(on_teardown) = self.on_teardown.as_ref() {
            on_teardown(&self.view_input, view_state.own(), ctx, &mut element);
        }
        let child_element = widget::Oversee::child_mut(&mut element);
        ctx.with_id(self.view_id, |ctx| {
            self.child
                .teardown(&mut view_state.child_view_state, ctx, child_element)
        });
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State
    ) -> MessageResult<Action> {
        
        match id_path.split_first() {
            Some((start, rest)) if start == &self.view_id => {

                let message_result = self
                    .child
                    .message(&mut view_state.child_view_state, rest, message, app_state);

                let result = match (message_result, self.on_action_message_result.as_ref()) {
                    (MessageResult::Action(action), Some(on_action_message_result)) => {
                        on_action_message_result(view_state.own_view_state.as_mut(), action, app_state)
                    }
                    (result, _) => result,
                };
                if let MessageResult::Stale(_) = &result {
                    tracing::debug!("Received a stale message result in Oversee, id path is {:?}", &id_path);
                }
                result
            }
            None => match self.on_message.as_ref() {
                // A message was directed to Oversee
                Some(on_message) => {
                    let result = on_message(
                        view_state.own_view_state.as_mut(), message, app_state
                    );
                    if let MessageResult::Stale(message) = result {
                        tracing::error!("
                            Wrong message type in Oversee::message: {message:?}. 
                            Message types directed to Oversee should be known and should be handled."
                        );
                        MessageResult::Stale(message)
                    } else {
                        result
                    }
                }
                None => {
                    tracing::error!("Received a message directed to Oversee, but Oversee has no on_message caller");
                    MessageResult::Nop
                }
            }
            Some((_, _)) => {
                tracing::warn!("Got unexpected id path in FontPreview::message");
                MessageResult::Stale(message)
            }
        }
    }
}