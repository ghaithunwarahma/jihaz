use accesskit::{Node, Role};
use kurbo::{Point, Size};
use masonry::{
    core::{
        AccessCtx, AccessEvent, BoxConstraints, EventCtx, FromDynWidget, LayoutCtx, PaintCtx, PointerEvent, PropertiesMut, PropertiesRef, QueryCtx, RegisterCtx, TextEvent, Widget, WidgetId, WidgetMut, WidgetPod
    }, 
    vello::Scene
};
use smallvec::SmallVec;
use tracing::{trace_span, Span};

pub enum EitherOr<FirstChild: Widget + ?Sized, SecondChild: Widget + ?Sized> {
    First(WidgetPod<FirstChild>),
    Second(WidgetPod<SecondChild>),
}

impl<FirstChild: Widget + ?Sized, SecondChild: Widget + ?Sized> std::fmt::Debug for EitherOr<FirstChild, SecondChild> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First(_) => write!(f, "First(...)"),
            Self::Second(_) => write!(f, "Second(...)"),
        }
    }
}

// --- MARK: BUILDERS ---
impl<FirstChild: Widget, SecondChild: Widget> EitherOr<FirstChild, SecondChild> {
    /// Create the EitherOr widget with the First child
    pub fn new_first(child: FirstChild) -> Self {
        Self::First(WidgetPod::new(child))
    }

    /// Create the EitherOr widget with the Second child
    pub fn new_second(child: SecondChild) -> Self {
        Self::Second(WidgetPod::new(child))
    }
}

impl<FirstChild: Widget + ?Sized, SecondChild: Widget + ?Sized> EitherOr<FirstChild, SecondChild> {
    /// Create the EitherOr widget with the First child
    pub fn new_first_pod(child: WidgetPod<FirstChild>) -> Self {
        Self::First(child)
    }

    /// Create the EitherOr widget with the Second child
    pub fn new_second_pod(child: WidgetPod<SecondChild>) -> Self {
        Self::Second(child)
    }
}

// --- MARK: WIDGETMUT ---
impl<FirstChild, SecondChild> EitherOr<FirstChild, SecondChild>
where
    FirstChild: Widget + FromDynWidget + ?Sized, 
    SecondChild: Widget + FromDynWidget + ?Sized
{
    /// Gets the WidgetMut for the First child.
    /// 
    /// Panics if the child is not First.
    pub fn first_child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, FirstChild> {
        match &mut this.widget {
            EitherOr::First(child_pod) => this.ctx.get_mut(child_pod),
            EitherOr::Second(_) => {
                panic!("Attempted to get a WidgetMut for the First child, but the widget child is the Second");
            }
        }
    }

    /// Gets the WidgetMut for the Second child.
    /// 
    /// Panics if the child is not Second.
    pub fn second_child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, SecondChild> {
        match &mut this.widget {
            EitherOr::First(_) => {
                panic!("Attempted to get a WidgetMut for the Second child, but the widget child is the First");
            }
            EitherOr::Second(child_pod) => this.ctx.get_mut(child_pod),
        }
    }

    /// Switches the child of the EitherOr widget into First.
    /// 
    /// Panics if the child is not First.
    pub fn switch_into_first_pod<'t>(
        this: &'t mut WidgetMut<'_, Self>,
        first: WidgetPod<FirstChild>
    ) {
        if let EitherOr::First(_) = this.widget {
            panic!("Attempted to switch into First child, but the widget child is Second");
        }
        let old = std::mem::replace(
            this.widget, 
            EitherOr::new_first_pod(first)
        );
        let EitherOr::Second(second) = old else { unreachable!() };
        this.ctx.remove_child(second);
        this.ctx.children_changed();
        this.ctx.request_layout();
    }

    /// Switches the child of the EitherOr widget into Second.
    /// 
    /// Panics if the child is not Second.
    pub fn switch_into_second_pod<'t>(
        this: &'t mut WidgetMut<'_, Self>,
        second: WidgetPod<SecondChild>
    ) {
        if let EitherOr::Second(_) = this.widget {
            panic!("Attempted to switch into Second child, but the widget child is the First");
        }
        let old = std::mem::replace(
            this.widget, 
            EitherOr::new_second_pod(second)
        );
        let EitherOr::First(first) = old else { unreachable!() };
        this.ctx.remove_child(first);
        this.ctx.children_changed();
        this.ctx.request_layout();
    }
}

// --- MARK: IMPL WIDGET ---
impl<FirstChild, SecondChild> Widget for EitherOr<FirstChild, SecondChild>
where
    FirstChild: Widget + FromDynWidget + ?Sized, 
    SecondChild: Widget + FromDynWidget + ?Sized
{
    fn on_pointer_event(
        &mut self,
        _ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        _event: &PointerEvent,
    ) {
    }

    fn on_text_event(
        &mut self,
        _ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        _event: &TextEvent,
    ) {
    }

    fn on_access_event(
        &mut self,
        _ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        _event: &AccessEvent,
    ) {
    }

    fn register_children(&mut self, ctx: &mut RegisterCtx) {
        match self {
            EitherOr::First(widget_pod) => {
                ctx.register_child(widget_pod);
            }
            EitherOr::Second(widget_pod) => {
                ctx.register_child(widget_pod);
            }
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        _props: &mut PropertiesMut<'_>,
        bc: &BoxConstraints,
    ) -> Size {

        match self {
            EitherOr::First(widget_pod) => {
                let child_size = ctx.run_layout(widget_pod, bc);
                ctx.place_child(widget_pod, Point::ORIGIN);
                child_size
            }
            EitherOr::Second(widget_pod) => {
                let child_size = ctx.run_layout(widget_pod, bc);
                ctx.place_child(widget_pod, Point::ORIGIN);
                child_size
            }
        }
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _props: &PropertiesRef<'_>, _scene: &mut Scene) {}

    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }

    fn accessibility(
        &mut self,
        _ctx: &mut AccessCtx,
        _props: &PropertiesRef<'_>,
        _node: &mut Node,
    ) {
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        match self {
            EitherOr::First(widget_pod) => {
                smallvec::smallvec![widget_pod.id()]

            }
            EitherOr::Second(widget_pod) => {
                smallvec::smallvec![widget_pod.id()]
            }
        }
    }

    fn make_trace_span(&self, ctx: &QueryCtx) -> Span {
        trace_span!("EitherOr", id = ctx.widget_id().trace())
    }
}