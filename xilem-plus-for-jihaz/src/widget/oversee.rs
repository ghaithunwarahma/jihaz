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

pub struct Oversee<W: Widget + ?Sized> {
    child: WidgetPod<W>,
}

// --- MARK: BUILDERS ---
impl<W: Widget> Oversee<W> {
    /// Create the Oversee widget
    pub fn new(child: W) -> Self {
        Self::new_pod(WidgetPod::new(child))
    }
}

impl<W: Widget + ?Sized> Oversee<W> {
    /// Create the Oversee widget from a WidgetPod
    pub fn new_pod(child: WidgetPod<W>) -> Self {
        Self { child }
    }
}

// --- MARK: WIDGETMUT ---
impl<W: Widget + FromDynWidget + ?Sized> Oversee<W> {
    /// Gets the WidgetMut for the child.
    /// 
    /// Panics if the child is not First.
    pub fn child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, W> {
        this.ctx.get_mut(&mut this.widget.child)
    }
}

// --- MARK: IMPL WIDGET ---
impl<W: Widget + FromDynWidget + ?Sized> Widget for Oversee<W> {
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
        ctx.register_child(&mut self.child);   
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        _props: &mut PropertiesMut<'_>,
        bc: &BoxConstraints,
    ) -> Size {
        let child_size = ctx.run_layout(&mut self.child, bc);
        ctx.place_child(&mut self.child, Point::ORIGIN);
        child_size
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
        smallvec::smallvec![self.child.id()]
    }

    fn make_trace_span(&self, ctx: &QueryCtx) -> Span {
        trace_span!("Oversee", id = ctx.widget_id().trace())
    }
}