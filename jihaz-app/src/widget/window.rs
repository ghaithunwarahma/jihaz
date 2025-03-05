use accesskit::Role;
use smallvec::{smallvec, SmallVec};
use vello::kurbo::{Insets, Point, Size};
use masonry::{
    vello::Scene, AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, PointerEvent, StatusChange, TextEvent, Widget, WidgetId
};
use xilem::Pod;
use xilem_core::ViewElement;
use super::tablet::TabletWi;


/// Represents the viewed spread of leaves or a leaf of the Quran
pub struct WindowWi {
    pub tablet: Pod<TabletWi>
}

impl WindowWi {
    pub fn new(tablet: Pod<TabletWi>) -> WindowWi {
        Self {
            tablet,
        }
    }
}

impl ViewElement for WindowWi {
    type Mut<'a> = &'a mut WindowWi;
}

impl Widget for WindowWi {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {
        // self.tablet.inner.on_pointer_event(ctx, event);
    }
    
    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {
        // self.tablet.inner.on_text_event(ctx, event);
    }

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {
        // self.tablet.inner.on_access_event(ctx, event);
    }

    #[allow(missing_docs)]
    fn on_status_change(&mut self, _ctx: &mut LifeCycleCtx, _event: &StatusChange) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        // if let LifeCycle::BuildFocusChain = event {
        //     eprintln!("build focus chain");
        //     ctx.register_for_focus();
        //     eprintln!("has focus {}", ctx.is_focused());
        // }
        self.tablet.inner.lifecycle(ctx, event);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let padding = Insets::uniform(0.0);
        let new_bc = bc.loosen().shrink((padding.x_value(), padding.y_value()));
        
        let tablet_size = self.tablet.inner.layout(ctx, &new_bc);
        let mut origin = Point::new(padding.x0, padding.y0);
        origin.x += 0.5 * (bc.max().width - tablet_size.width);
        origin.y += 0.5 * (bc.max().height - tablet_size.height);
        eprintln!("Tablet BcConstraints: {:?} | Origin: {:?}, Tablet Size {:?}", new_bc, origin, tablet_size);

        ctx.place_child(&mut self.tablet.inner, origin);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        self.tablet.inner.paint(ctx, scene);
    }

    fn accessibility_role(&self) -> Role {
        Role::Window
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        self.tablet.inner.accessibility(ctx);
    }

    // fn children(&self) -> SmallVec<[WidgetRef<'_, dyn Widget>; 16]> {
    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.tablet.inner.id()]
    }
}