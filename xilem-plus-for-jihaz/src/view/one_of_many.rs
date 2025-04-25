use std::marker::PhantomData;
use xilem::{view::{flex_item, FlexItem, FlexParams}, AnyWidgetView, Pod, ViewCtx, WidgetView};
use xilem_core::{
    DynMessage, MessageResult, Mut, View, ViewId, ViewMarker
};

use crate::widget;

/// A view that builds a child widget out of two choices for the child,
/// depending on a condition.
pub fn one_of_many<K, State, Action, Seq: OneOfManySequence<State, Action>>(
    active: K,
    sequence: Seq
) -> OneOfMany<K, Seq, State, Action> 
where
    Seq: WidgetView<State, Action>,
    State: 'static,
    Action: 'static,
{
    OneOfMany {
        active,
        sequence,
        phantom: PhantomData,
    }
}

/// A view that builds a child widget out of two choices for the child,
/// depending on a condition.
pub struct OneOfMany<K, Seq, State, Action> {
    active: K,
    sequence: Seq,
    phantom: PhantomData<(State, Action)>,
}

pub enum OneOfManyViewState<FirstChildViewState, SecondChildViewState> {
    First(FirstChildViewState),
    Second(SecondChildViewState),
}

// --- MARK: VIEWSTATE ---
impl<FirstChildViewState, SecondChildViewState> OneOfManyViewState<FirstChildViewState, SecondChildViewState> {
    /// Gets the ViewState of the child if it is First child.
    /// 
    /// Panics if the child is not First.
    pub fn first_child_state<'t>(&mut self) -> &mut FirstChildViewState {
        match self {
            OneOfManyViewState::First(child_state) => child_state,
            OneOfManyViewState::Second(_) => {
                panic!("Attempted to get the ViewState of First child, but the child is Second")
            }
        }
    }

    /// Gets the ViewState of the child if it is Second child.
    /// 
    /// Panics if the child is not Second.
    pub fn second_child_state<'t>(&mut self) -> &mut SecondChildViewState {
        match self {
            OneOfManyViewState::First(_) => {
                panic!("Attempted to get the ViewState of Second child, but the child is First")
            }
            OneOfManyViewState::Second(child_state) => child_state,
        }
    }

    /// Switches the child of the OneOfManyViewState into First.
    /// 
    /// Panics if the child is not First.
    pub fn switch_into_first_child<'t>(&mut self, first: FirstChildViewState) -> SecondChildViewState {
        if let OneOfManyViewState::First(_) = self {
            panic!("Attempted to switch into First child state, but the child is the Second");
        }
        let old = std::mem::replace(
            self, 
            OneOfManyViewState::First(first)
        );
        let OneOfManyViewState::Second(second) = old else { unreachable!() };
        second
    }

    /// Switches the child of the OneOfManyViewState into Second.
    /// 
    /// Panics if the child is not Second.
    pub fn switch_into_second_child<'t>(&mut self, second: SecondChildViewState) -> FirstChildViewState {
        if let OneOfManyViewState::Second(_) = self {
            panic!("Attempted to switch into Second child state, but the child is the First");
        }
        let old = std::mem::replace(
            self, 
            OneOfManyViewState::Second(second)
        );
        let OneOfManyViewState::First(first) = old else { unreachable!() };
        first
    }
}

impl<Seq, State, Action> ViewMarker 
    for OneOfMany<Seq, State, Action> {}

// --- MARK: VIEW IMPL ---
impl<Seq, State, Action> View<State, Action, ViewCtx>
    for OneOfMany<Seq, State, Action>
where
    FirstChildView: WidgetView<State, Action>,
    SecondChildView: WidgetView<State, Action>,
    State: 'static,
    Action: 'static,
{
    type Element = Pod<widget::OneOfMany<FirstChildView::Widget, SecondChildView::Widget>>;

    type ViewState = OneOfManyViewState<FirstChildView::ViewState, SecondChildView::ViewState>;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {

        if self.is_first {
            let (first_child, first_child_state) = self.first_child.build(ctx);
            let element = widget::OneOfMany::new_first_pod(
                first_child.into_widget_pod()
            );
            let view_state = OneOfManyViewState::First(first_child_state);
            let pod = ctx.new_pod(element);
            (pod, view_state)
        } else {
            let (second_child, second_child_state) = self.second_child.build(ctx);
            let element = widget::OneOfMany::new_second_pod(
                second_child.into_widget_pod()
            );
            let view_state = OneOfManyViewState::Second(second_child_state);
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
                prev.second_child.teardown(second_child_state, ctx, widget::OneOfMany::second_child_mut(&mut element));
                
                // build First child view

                let (first_child, first_child_state) = self.first_child.build(ctx);
                widget::OneOfMany::switch_into_first_pod(&mut element, first_child.into_widget_pod());
                OneOfManyViewState::switch_into_first_child(view_state, first_child_state);

            } else {

                // teardown First child view
                
                let first_child_state = view_state.first_child_state();
                prev.first_child.teardown(first_child_state, ctx, widget::OneOfMany::first_child_mut(&mut element));
                
                // build Second child view

                let (second_child, second_child_state) = self.second_child.build(ctx);
                widget::OneOfMany::switch_into_second_pod(&mut element, second_child.into_widget_pod());
                OneOfManyViewState::switch_into_second_child(view_state, second_child_state);
            }
        } else {
            if self.is_first {
                let first_child = widget::OneOfMany::first_child_mut(&mut element);
                let first_child_state = view_state
                    .first_child_state();
                self.first_child.rebuild(&prev.first_child, first_child_state, ctx, first_child);
            } else {
                let second_child = widget::OneOfMany::second_child_mut(&mut element);
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
            let first_child = widget::OneOfMany::first_child_mut(&mut element);
            let first_child_state = view_state
                .first_child_state();
            self.first_child.teardown(first_child_state, ctx, first_child);
        } else {
            let second_child = widget::OneOfMany::second_child_mut(&mut element);
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

pub enum FlexElement {
    Child(Pod<dyn Widget>, FlexParams),
    FixedSpacer(f64),
    FlexSpacer(f64),
}

pub struct FlexElementMut<'w> {
    parent: WidgetMut<'w, widgets::Flex>,
    idx: usize,
}

struct FlexSplice<'w> {
    idx: usize,
    element: WidgetMut<'w, widgets::Flex>,
    scratch: AppendVec<FlexElement>,
}

impl<'w> FlexSplice<'w> {
    fn new(element: WidgetMut<'w, widgets::Flex>) -> Self {
        Self {
            idx: 0,
            element,
            scratch: AppendVec::default(),
        }
    }
}

impl ViewElement for FlexElement {
    type Mut<'w> = FlexElementMut<'w>;
}

impl SuperElement<Self, ViewCtx> for FlexElement {
    fn upcast(_ctx: &mut ViewCtx, child: Self) -> Self {
        child
    }

    fn with_downcast_val<R>(
        mut this: Mut<Self>,
        f: impl FnOnce(Mut<Self>) -> R,
    ) -> (Self::Mut<'_>, R) {
        let r = {
            let parent = this.parent.reborrow_mut();
            let reborrow = FlexElementMut {
                idx: this.idx,
                parent,
            };
            f(reborrow)
        };
        (this, r)
    }
}

impl<W: Widget + FromDynWidget + ?Sized> SuperElement<Pod<W>, ViewCtx> for FlexElement {
    fn upcast(_: &mut ViewCtx, child: Pod<W>) -> Self {
        Self::Child(child.erased(), FlexParams::default())
    }

    fn with_downcast_val<R>(
        mut this: Mut<Self>,
        f: impl FnOnce(Mut<Pod<W>>) -> R,
    ) -> (Mut<Self>, R) {
        let ret = {
            let mut child = widgets::Flex::child_mut(&mut this.parent, this.idx)
                .expect("This is supposed to be a widget");
            let downcast = child.downcast();
            f(downcast)
        };

        (this, ret)
    }
}

impl ElementSplice<FlexElement> for FlexSplice<'_> {
    fn insert(&mut self, element: FlexElement) {
        match element {
            FlexElement::Child(child, params) => {
                widgets::Flex::insert_flex_child_pod(
                    &mut self.element,
                    self.idx,
                    child.erased_widget_pod(),
                    params,
                );
            }
            FlexElement::FixedSpacer(len) => {
                widgets::Flex::insert_spacer(&mut self.element, self.idx, len);
            }
            FlexElement::FlexSpacer(len) => {
                widgets::Flex::insert_flex_spacer(&mut self.element, self.idx, len);
            }
        };
        self.idx += 1;
    }

    fn with_scratch<R>(&mut self, f: impl FnOnce(&mut AppendVec<FlexElement>) -> R) -> R {
        let ret = f(&mut self.scratch);
        for element in self.scratch.drain() {
            match element {
                FlexElement::Child(child, params) => {
                    widgets::Flex::insert_flex_child_pod(
                        &mut self.element,
                        self.idx,
                        child.erased_widget_pod(),
                        params,
                    );
                }
                FlexElement::FixedSpacer(len) => {
                    widgets::Flex::insert_spacer(&mut self.element, self.idx, len);
                }
                FlexElement::FlexSpacer(len) => {
                    widgets::Flex::insert_flex_spacer(&mut self.element, self.idx, len);
                }
            };
            self.idx += 1;
        }
        ret
    }

    fn mutate<R>(&mut self, f: impl FnOnce(Mut<FlexElement>) -> R) -> R {
        let child = FlexElementMut {
            parent: self.element.reborrow_mut(),
            idx: self.idx,
        };
        let ret = f(child);
        self.idx += 1;
        ret
    }

    fn delete<R>(&mut self, f: impl FnOnce(Mut<FlexElement>) -> R) -> R {
        let ret = {
            let child = FlexElementMut {
                parent: self.element.reborrow_mut(),
                idx: self.idx,
            };
            f(child)
        };
        widgets::Flex::remove_child(&mut self.element, self.idx);
        ret
    }

    fn skip(&mut self, n: usize) {
        self.idx += n;
    }
}

/// An ordered sequence of views for a [`Flex`] view.
/// See [`ViewSequence`] for more technical details.
///
/// # Examples
///
/// ```
/// use xilem::view::{label, FlexSequence, FlexExt as _};
///
/// fn label_sequence<State: 'static>(
///     labels: impl Iterator<Item = &'static str>,
///     flex: f64,
/// ) -> impl FlexSequence<State> {
///     labels.map(|l| label(l).flex(flex)).collect::<Vec<_>>()
/// }
/// ```
pub trait FlexSequence<State, Action = ()>:
    ViewSequence<State, Action, ViewCtx, FlexElement>
{
}

impl<Seq, State, Action> FlexSequence<State, Action> for Seq where
    Seq: ViewSequence<State, Action, ViewCtx, FlexElement>
{
}

/// A trait which extends a [`WidgetView`] with methods to provide parameters for a flex item, or being able to use it interchangeably with a spacer.
pub trait FlexExt<State, Action>: WidgetView<State, Action> {
    /// Applies [`impl Into<FlexParams>`](`FlexParams`) to this view, can be used as child of a [`Flex`] [`View`]
    ///
    /// # Examples
    /// ```
    /// use xilem::{view::{button, label, flex, CrossAxisAlignment, FlexSpacer, FlexExt}};
    /// # use xilem::{WidgetView};
    ///
    /// # fn view<State: 'static>() -> impl WidgetView<State> {
    /// flex((
    ///     button("click me", |_| ()).flex(2.0),
    ///     FlexSpacer::Fixed(2.0),
    ///     label("a label").flex(CrossAxisAlignment::Fill),
    ///     FlexSpacer::Fixed(2.0),
    /// ))
    /// # }
    ///
    /// ```
    fn flex(self, params: impl Into<FlexParams>) -> FlexItem<Self, State, Action>
    where
        State: 'static,
        Action: 'static,
        Self: Sized,
    {
        flex_item(self, params)
    }

    /// Turns this [`WidgetView`] into an [`AnyFlexChild`],
    /// which can be used interchangeably with an `FlexSpacer`, as child of a [`Flex`] [`View`]
    ///
    /// # Examples
    /// ```
    /// use xilem::{view::{flex, label, FlexSpacer, FlexExt, AnyFlexChild}};
    /// # use xilem::{WidgetView};
    ///
    /// # fn view<State: 'static>() -> impl WidgetView<State> {
    /// flex([label("a label").into_any_flex(), AnyFlexChild::Spacer(FlexSpacer::Fixed(1.0))])
    /// # }
    ///
    /// ```
    fn into_any_flex(self) -> AnyFlexChild<State, Action>
    where
        State: 'static,
        Action: 'static,
        Self: Sized,
    {
        AnyFlexChild::Item(flex_item(self.boxed(), FlexParams::default()))
    }
}

impl<State, Action, V: WidgetView<State, Action>> FlexExt<State, Action> for V {}

/// A `WidgetView` that can be used within a [`OneOfMany`] [`View`].
pub struct OneOfManyChild<K, V, State, Action> {
    key: K,
    view: V,
    phantom: PhantomData<fn() -> (State, Action)>,
}

/// Applies [`impl Into<FlexParams>`](`FlexParams`) to the [`View`] `V`, can be used as child of a [`Flex`] View.
///
/// # Examples
/// ```
/// use xilem::view::{button, label, flex_item, flex, CrossAxisAlignment, FlexSpacer};
/// # use xilem::{WidgetView};
///
/// # fn view<State: 'static>() -> impl WidgetView<State> {
/// flex((
///     flex_item(button("click me", |_| ()), 2.0),
///     FlexSpacer::Fixed(2.0),
///     flex_item(label("a label"), CrossAxisAlignment::Fill),
///     FlexSpacer::Fixed(2.0),
/// ))
/// # }
///
/// ```
pub fn flex_item<V, State, Action>(
    view: V,
    params: impl Into<FlexParams>,
) -> FlexItem<V, State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action>,
{
    FlexItem {
        params: params.into(),
        view,
        phantom: PhantomData,
    }
}

impl<State, Action, V> From<FlexItem<V, State, Action>> for AnyFlexChild<State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action, ViewState: 'static>,
{
    fn from(value: FlexItem<V, State, Action>) -> Self {
        Self::Item(flex_item(value.view.boxed(), value.params))
    }
}

impl<V, State, Action> ViewMarker for FlexItem<V, State, Action> {}
impl<State, Action, V> View<State, Action, ViewCtx> for FlexItem<V, State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action>,
{
    type Element = FlexElement;

    type ViewState = V::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (pod, state) = self.view.build(ctx);
        (FlexElement::Child(pod.erased(), self.params), state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        {
            if self.params != prev.params {
                widgets::Flex::update_child_flex_params(
                    &mut element.parent,
                    element.idx,
                    self.params,
                );
            }
            let mut child = widgets::Flex::child_mut(&mut element.parent, element.idx)
                .expect("FlexWrapper always has a widget child");
            self.view
                .rebuild(&prev.view, view_state, ctx, child.downcast());
        }
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        let mut child = widgets::Flex::child_mut(&mut element.parent, element.idx)
            .expect("FlexWrapper always has a widget child");
        self.view.teardown(view_state, ctx, child.downcast());
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State,
    ) -> MessageResult<Action> {
        self.view.message(view_state, id_path, message, app_state)
    }
}

/// A widget-type-erased flex child [`View`], can be used within a [`OneOfMany`] [`View`]
pub struct AnyOneOfManyChild<K, State, Action = ()>(
    OneOfManyChild<K, Box<AnyWidgetView<State, Action>>, State, Action>
);