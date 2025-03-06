use accesskit::Role;
use smallvec::{smallvec, SmallVec};
use masonry::{
    core::{
        AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, PaintCtx, PointerEvent, TextEvent, Widget, WidgetId, WidgetMut, WidgetPod
    },
    vello::{kurbo::{Insets, Point, Size}, Scene}, 
};
use super::tablet::Tablet;

pub struct Window {
    pub tablet: WidgetPod<Tablet>
}

impl Window {
    pub fn new(tablet: WidgetPod<Tablet>) -> Window {
        Self {
            tablet,
        }
    }
}

// --- MARK: WIDGETMUT ---
impl Window {
    pub fn child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, Tablet> {
        this.ctx.get_mut(&mut this.widget.tablet)
    }
}

impl Widget for Window {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {}
    
    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {}

    fn register_children(&mut self, ctx: &mut masonry::core::RegisterCtx) {
        ctx.register_child(&mut self.tablet);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let padding = Insets::uniform(0.0);
        let new_bc = bc.loosen().shrink((padding.x_value(), padding.y_value()));
        
        let tablet_size = ctx.run_layout(&mut self.tablet, &new_bc);
        let mut origin = Point::new(padding.x0, padding.y0);
        origin.x += 0.5 * (bc.max().width - tablet_size.width);
        origin.y += 0.5 * (bc.max().height - tablet_size.height);
        eprintln!("Tablet BcConstraints: {:?} | Origin: {:?}, Tablet Size {:?}", new_bc, origin, tablet_size);

        ctx.place_child(&mut self.tablet, origin);
        bc.max()
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _scene: &mut Scene) {}

    fn accessibility_role(&self) -> Role {
        Role::Window
    }

    fn accessibility(&mut self, _ctx: &mut AccessCtx, _node: &mut accesskit::Node) {}

    // fn children(&self) -> SmallVec<[WidgetRef<'_, dyn Widget>; 16]> {
    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.tablet.id()]
    }
}