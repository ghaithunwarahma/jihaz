use accesskit::{Node, Role};
use jihaz::collection::SmallKeyArray;
use smallvec::{smallvec, SmallVec};
use masonry::{
    core::{
        AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, PaintCtx, PointerEvent, QueryCtx, TextEvent, Widget, WidgetId, WidgetMut, WidgetPod
    },
    vello::{kurbo::{Insets, Point, Size}, Scene}, 
    widgets::{Button, Label, Textbox}
};
use tracing::{trace_span, Span};

// use super::textbox::Textbox;


/// Represents the viewed spread of leaves
pub struct Tablet {
    pub labels: SmallKeyArray<TabletChild, WidgetPod<Label>, 5>,
    pub textboxes: SmallKeyArray<TabletChild, WidgetPod<Textbox>, 4>,
    pub buttons: SmallKeyArray<TabletChild, WidgetPod<Button>, 1>,
}

#[derive(PartialEq)]
pub enum TabletChild {
    AppName,
    ExecutablePath,
    /// Generate app packages
    GenerateAppPckgs,
    OriginalIconPath,
    ProgressMessage,
    /// Target packages directory path
    TargetPckgsDirPath,
}

impl Tablet {
    pub fn new(
        labels: SmallKeyArray<TabletChild, WidgetPod<Label>, 5>,
        textboxes: SmallKeyArray<TabletChild, WidgetPod<Textbox>, 4>,
        buttons: SmallKeyArray<TabletChild, WidgetPod<Button>, 1>,
    ) -> Tablet {
        Self {
            labels,
            textboxes,
            buttons,
        }
    }
}

// --- MARK: WIDGETMUT ---
impl Tablet {
    
    pub fn label_mut<'t>(
        key: &TabletChild,
        this: &'t mut WidgetMut<'_, Self>,
    ) -> WidgetMut<'t, Label> {
        this.ctx.get_mut(this.widget.labels.get_mut(key))
    }

    pub fn textbox_mut<'t>(
        key: &TabletChild,
        this: &'t mut WidgetMut<'_, Self>,
    ) -> WidgetMut<'t, Textbox> {
        this.ctx.get_mut(this.widget.textboxes.get_mut(key))
    }

    pub fn button_mut<'t>(
        key: &TabletChild,
        this: &'t mut WidgetMut<'_, Self>,
    ) -> WidgetMut<'t, Button> {
        this.ctx.get_mut(this.widget.buttons.get_mut(key))
    }
}

impl Widget for Tablet {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {}
    
    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {}
    
    fn register_children(&mut self, ctx: &mut masonry::core::RegisterCtx) {
        use TabletChild as TC;
        ctx.register_child(self.labels.get_mut(&TC::ExecutablePath));
        ctx.register_child(self.textboxes.get_mut(&TC::ExecutablePath));
        ctx.register_child(self.labels.get_mut(&TC::OriginalIconPath));
        ctx.register_child(self.textboxes.get_mut(&TC::OriginalIconPath));
        ctx.register_child(self.labels.get_mut(&TC::TargetPckgsDirPath));
        ctx.register_child(self.textboxes.get_mut(&TC::TargetPckgsDirPath));
        ctx.register_child(self.labels.get_mut(&TC::AppName));
        ctx.register_child(self.textboxes.get_mut(&TC::AppName));
        ctx.register_child(self.buttons.get_mut(&TC::GenerateAppPckgs));
        ctx.register_child(self.labels.get_mut(&TC::ProgressMessage));
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        use TabletChild as TC;
        let padding = Insets::uniform(10.0);
        let new_bc = bc.shrink((padding.x_value(), padding.y_value()));
        let mut total_size;
        let mut size;
        let mut origin = Point::new(padding.x0, padding.y0);
        let in_between_padding = 10.0;

        size = ctx.run_layout(self.labels.get_mut(&TC::ExecutablePath), &new_bc);
        ctx.place_child(self.labels.get_mut(&TC::ExecutablePath), origin);
        total_size = size;
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (executable_path_label)", size, origin);

        size = ctx.run_layout(self.textboxes.get_mut(&TC::ExecutablePath), &new_bc);
        ctx.place_child(self.textboxes.get_mut(&TC::ExecutablePath), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (executable_path_textbox)", size, origin);

        size = ctx.run_layout(self.labels.get_mut(&TC::OriginalIconPath), &new_bc);
        ctx.place_child(self.labels.get_mut(&TC::OriginalIconPath), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (original_icon_path_label)", size, origin);

        size = ctx.run_layout(self.textboxes.get_mut(&TC::OriginalIconPath), &new_bc);
        ctx.place_child(self.textboxes.get_mut(&TC::OriginalIconPath), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (original_icon_path_textbox)", size, origin);
        
        size = ctx.run_layout(self.labels.get_mut(&TC::TargetPckgsDirPath), &new_bc);
        ctx.place_child(self.labels.get_mut(&TC::TargetPckgsDirPath), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (target_packages_directory_path_label)", size, origin);

        size = ctx.run_layout(self.textboxes.get_mut(&TC::TargetPckgsDirPath), &new_bc);
        ctx.place_child(self.textboxes.get_mut(&TC::TargetPckgsDirPath), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (target_packages_directory_path_textbox)", size, origin);

        size = ctx.run_layout(self.labels.get_mut(&TC::AppName), &new_bc);
        ctx.place_child(self.labels.get_mut(&TC::AppName), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (app_name_label)", size, origin);

        size = ctx.run_layout(self.textboxes.get_mut(&TC::AppName), &new_bc);
        ctx.place_child(self.textboxes.get_mut(&TC::AppName), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (app_name_textbox)", size, origin);

        size = ctx.run_layout(self.buttons.get_mut(&TC::GenerateAppPckgs), &new_bc);
        ctx.place_child(self.buttons.get_mut(&TC::GenerateAppPckgs), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (generate_app_packages_button)", size, origin);

        size = ctx.run_layout(self.labels.get_mut(&TC::ProgressMessage), &new_bc);
        ctx.place_child(self.labels.get_mut(&TC::ProgressMessage), origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (progress_message_label)", size, origin);

        total_size.height = origin.y + size.height + padding.y1;

        // size * 2.0
        // Size::new(total_size.width, bc.max().height)
        total_size
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _scene: &mut Scene) {}

    fn accessibility_role(&self) -> Role {
        Role::Tab
    }

    fn accessibility(&mut self, _ctx: &mut AccessCtx, _node: &mut Node) {}

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        use TabletChild as TC;
        smallvec![
            self.labels.get(&TC::ExecutablePath).id(),
            self.textboxes.get(&TC::ExecutablePath).id(),
            self.labels.get(&TC::OriginalIconPath).id(),
            self.textboxes.get(&TC::OriginalIconPath).id(),
            self.labels.get(&TC::TargetPckgsDirPath).id(),
            self.textboxes.get(&TC::TargetPckgsDirPath).id(),
            self.labels.get(&TC::AppName).id(),
            self.textboxes.get(&TC::AppName).id(),
            self.buttons.get(&TC::GenerateAppPckgs).id(),
            self.labels.get(&TC::ProgressMessage).id(),
        ]
    }

    fn make_trace_span(&self, ctx: &QueryCtx<'_>) -> Span {
        trace_span!("Tablet", id = ctx.widget_id().trace())
    }
}