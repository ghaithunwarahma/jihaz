use std::marker::PhantomData;
use xilem::{view::{flex_item, FlexItem, FlexParams}, Pod, ViewCtx, WidgetView};
use xilem_core::{
    DynMessage, MessageResult, Mut, View, ViewId, ViewMarker
};

use crate::widget;

/// A view that builds a child widget out of two choices for the child,
/// depending on a condition.
pub fn either_or<FirstChildView, SecondChildView, State, Action>(
    is_first: bool,
    first_child: FirstChildView,
    second_child: SecondChildView,
) -> EitherOr<FirstChildView, SecondChildView, State, Action> 
where
    FirstChildView: WidgetView<State, Action>,
    SecondChildView: WidgetView<State, Action>,
    State: 'static,
    Action: 'static,
{
    EitherOr {
        is_first,
        first_child,
        second_child,
        phantom: PhantomData,
    }
}

// --- MARK: EitherOr extenders ---
impl<FirstChildView, SecondChildView, State, Action> EitherOr<FirstChildView, SecondChildView, State, Action> 
where
    FirstChildView: WidgetView<State, Action>,
    SecondChildView: WidgetView<State, Action>,
    State: 'static + Send + Sync,
    Action: 'static + Send + Sync,
{
    /// Conditinally convert the EitherOr view into a FlexItem by applying a FlexExt::flex function on the View,
    /// and the flex function is fed different input types based on the current varient of the EitherOr View.
    pub fn cond_flex(self, first_value: impl Into<FlexParams>, second_value: impl Into<FlexParams>) -> FlexItem<Self, State, Action> {
        match self.is_first {
            true => flex_item(self, first_value),
            false => flex_item(self, second_value),
        }
    }
}

// --- MARK: CondConvert ---
pub trait CondConvert<OutView> {
    /// Conditinally convert the EitherOr view by applying a function on the View,
    /// and the function is fed different input types based on the current varient of the EitherOr View.
    fn cond_convert<T, F>(
        self, wrapping_fn: F, first_value: impl Into<T>, second_value: impl Into<T>
    ) -> OutView
    where
        T: 'static,
        F: Fn(Self, T) -> OutView + 'static,
        Self: Sized;
}

impl<OutView, FirstChildView, SecondChildView, State, Action> CondConvert<OutView> 
    for EitherOr<FirstChildView, SecondChildView, State, Action>
where
    FirstChildView: WidgetView<State, Action>,
    SecondChildView: WidgetView<State, Action>,
    State: 'static,
    Action: 'static,
    OutView: View<State, Action, ViewCtx>,
{
    /// Conditinally convert the EitherOr view by applying a function on the View,
    /// and the function is fed different input types based on the current varient of the EitherOr View.
    fn cond_convert<T, F>(
        self, wrapping_fn: F, first_value: impl Into<T>, second_value: impl Into<T>
    ) -> OutView
    where
        T: 'static,
        F: Fn(Self, T) -> OutView + 'static,
        Self: Sized,
    {
        match self.is_first {
            true => wrapping_fn(self, Into::<T>::into(first_value)),
            false => wrapping_fn(self, Into::<T>::into(second_value)),
        }
    }
}

/// A view that builds a child widget out of two choices for the child,
/// depending on a condition.
pub struct EitherOr<FirstChildView, SecondChildView, State, Action> {
    is_first: bool,
    first_child: FirstChildView,
    second_child: SecondChildView,
    phantom: PhantomData<(State, Action)>,
}

pub enum EitherOrViewState<FirstChildViewState, SecondChildViewState> {
    First(FirstChildViewState),
    Second(SecondChildViewState),
}

// --- MARK: VIEWSTATE ---
impl<FirstChildViewState, SecondChildViewState> EitherOrViewState<FirstChildViewState, SecondChildViewState> {
    /// Gets the ViewState of the child if it is First child.
    /// 
    /// Panics if the child is not First.
    pub fn first_child_state<'t>(&mut self) -> &mut FirstChildViewState {
        match self {
            EitherOrViewState::First(child_state) => child_state,
            EitherOrViewState::Second(_) => {
                panic!("Attempted to get the ViewState of First child, but the child is Second")
            }
        }
    }

    /// Gets the ViewState of the child if it is Second child.
    /// 
    /// Panics if the child is not Second.
    pub fn second_child_state<'t>(&mut self) -> &mut SecondChildViewState {
        match self {
            EitherOrViewState::First(_) => {
                panic!("Attempted to get the ViewState of Second child, but the child is First")
            }
            EitherOrViewState::Second(child_state) => child_state,
        }
    }

    /// Switches the child of the EitherOrViewState into First.
    /// 
    /// Panics if the child is not First.
    pub fn switch_into_first_child<'t>(&mut self, first: FirstChildViewState) -> SecondChildViewState {
        if let EitherOrViewState::First(_) = self {
            panic!("Attempted to switch into First child state, but the child is the Second");
        }
        let old = std::mem::replace(
            self, 
            EitherOrViewState::First(first)
        );
        let EitherOrViewState::Second(second) = old else { unreachable!() };
        second
    }

    /// Switches the child of the EitherOrViewState into Second.
    /// 
    /// Panics if the child is not Second.
    pub fn switch_into_second_child<'t>(&mut self, second: SecondChildViewState) -> FirstChildViewState {
        if let EitherOrViewState::Second(_) = self {
            panic!("Attempted to switch into Second child state, but the child is the First");
        }
        let old = std::mem::replace(
            self, 
            EitherOrViewState::Second(second)
        );
        let EitherOrViewState::First(first) = old else { unreachable!() };
        first
    }
}

impl<FirstChildView, SecondChildView, State, Action> ViewMarker 
    for EitherOr<FirstChildView, SecondChildView, State, Action> {}

// --- MARK: VIEW IMPL ---
impl<FirstChildView, SecondChildView, State, Action> View<State, Action, ViewCtx>
    for EitherOr<FirstChildView, SecondChildView, State, Action>
where
    FirstChildView: WidgetView<State, Action>,
    SecondChildView: WidgetView<State, Action>,
    State: 'static,
    Action: 'static,
{
    type Element = Pod<widget::EitherOr<FirstChildView::Widget, SecondChildView::Widget>>;

    type ViewState = EitherOrViewState<FirstChildView::ViewState, SecondChildView::ViewState>;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {

        if self.is_first {
            let (first_child, first_child_state) = self.first_child.build(ctx);
            let element = widget::EitherOr::new_first_pod(
                first_child.into_widget_pod()
            );
            let view_state = EitherOrViewState::First(first_child_state);
            let pod = ctx.new_pod(element);
            (pod, view_state)
        } else {
            let (second_child, second_child_state) = self.second_child.build(ctx);
            let element = widget::EitherOr::new_second_pod(
                second_child.into_widget_pod()
            );
            let view_state = EitherOrViewState::Second(second_child_state);
            let pod = ctx.new_pod(element);
            (pod, view_state)
        }
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        if prev.is_first != self.is_first {
            if self.is_first {

                // teardown Second child view
                let second_child_state = view_state.second_child_state();
                prev.second_child.teardown(second_child_state, ctx, widget::EitherOr::second_child_mut(&mut element));
                
                // build First child view

                let (first_child, first_child_state) = self.first_child.build(ctx);
                widget::EitherOr::switch_into_first_pod(&mut element, first_child.into_widget_pod());
                EitherOrViewState::switch_into_first_child(view_state, first_child_state);

            } else {

                // teardown First child view
                
                let first_child_state = view_state.first_child_state();
                prev.first_child.teardown(first_child_state, ctx, widget::EitherOr::first_child_mut(&mut element));
                
                // build Second child view

                let (second_child, second_child_state) = self.second_child.build(ctx);
                widget::EitherOr::switch_into_second_pod(&mut element, second_child.into_widget_pod());
                EitherOrViewState::switch_into_second_child(view_state, second_child_state);
            }
        } else {
            if self.is_first {
                let first_child = widget::EitherOr::first_child_mut(&mut element);
                let first_child_state = view_state
                    .first_child_state();
                self.first_child.rebuild(&prev.first_child, first_child_state, ctx, first_child);
            } else {
                let second_child = widget::EitherOr::second_child_mut(&mut element);
                let second_child_state = view_state
                    .second_child_state();
                self.second_child.rebuild(&prev.second_child, second_child_state, ctx, second_child);
            }
        }
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        if self.is_first {
            let first_child = widget::EitherOr::first_child_mut(&mut element);
            let first_child_state = view_state
                .first_child_state();
            self.first_child.teardown(first_child_state, ctx, first_child);
        } else {
            let second_child = widget::EitherOr::second_child_mut(&mut element);
            let second_child_state = view_state
                .second_child_state();
            self.second_child.teardown(second_child_state, ctx, second_child);
        }
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State) -> MessageResult<Action>
    {
        if self.is_first {
            let first_child_state = view_state
                .first_child_state();
            self.first_child.message(first_child_state, id_path, message, app_state)
        } else {
            let second_child_state = view_state
                .second_child_state();
            self.second_child.message(second_child_state, id_path, message, app_state)
        }
    }
}