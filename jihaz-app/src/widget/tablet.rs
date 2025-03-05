use accesskit::Role;
use smallvec::{smallvec, SmallVec};
use vello::kurbo::{Insets, Point, Size};
use masonry::{
    vello::Scene, widget::{Button, Label}, AccessCtx, AccessEvent, 
    BoxConstraints, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, 
    PaintCtx, PointerEvent, StatusChange, TextEvent, Widget, WidgetId
};
use xilem::Pod;
use xilem_core::ViewElement;

use super::textbox::Textbox;


/// Represents the viewed spread of leaves or a leaf of the Quran
pub struct TabletWi {
    pub executable_path_label: Pod<Label>,
    pub executable_path_textbox: Pod<Textbox>,
    pub original_icon_path_label: Pod<Label>,
    pub original_icon_path_textbox: Pod<Textbox>,
    pub target_packages_directory_path_label: Pod<Label>,
    pub target_packages_directory_path_textbox: Pod<Textbox>,
    pub app_name_label: Pod<Label>,
    pub app_name_textbox: Pod<Textbox>,
    pub generate_app_packages_button: Pod<Button>,
    pub progress_message_label: Pod<Label>,
}

impl TabletWi {
    pub fn new(
        executable_path_label: Pod<Label>,
        executable_path_textbox: Pod<Textbox>,
        original_icon_path_label: Pod<Label>,
        original_icon_path_textbox: Pod<Textbox>,
        target_packages_directory_path_label: Pod<Label>,
        target_packages_directory_path_textbox: Pod<Textbox>,
        app_name_label: Pod<Label>,
        app_name_textbox: Pod<Textbox>,
        generate_app_packages_button: Pod<Button>,
        progress_message_label: Pod<Label>,
    ) -> TabletWi {
        Self {
            executable_path_label,
            executable_path_textbox,
            original_icon_path_label,
            original_icon_path_textbox,
            target_packages_directory_path_label,
            target_packages_directory_path_textbox,
            app_name_label,
            app_name_textbox,
            generate_app_packages_button,
            progress_message_label,
        }
    }
}

impl ViewElement for TabletWi {
    type Mut<'a> = &'a mut TabletWi;
}

impl Widget for TabletWi {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {
        // self.executable_path_label.inner.on_pointer_event(ctx, event);
        // self.executable_path_textbox.inner.on_pointer_event(ctx, event);
        // self.original_icon_path_label.inner.on_pointer_event(ctx, event);
        // self.original_icon_path_textbox.inner.on_pointer_event(ctx, event);
        // self.target_packages_directory_path_label.inner.on_pointer_event(ctx, event);
        // self.target_packages_directory_path_textbox.inner.on_pointer_event(ctx, event);
        // self.app_name_label.inner.on_pointer_event(ctx, event);
        // self.app_name_textbox.inner.on_pointer_event(ctx, event);
        // self.generate_app_packages_button.inner.on_pointer_event(ctx, event);
        // self.progress_message_label.inner.lifecycle(ctx, event);
        // match event {
        //     PointerEvent::PointerDown(_btn, _pointer) => {
        //         ctx.set_active(true);
        //         // did this to refresh page following rebuilding tablet leafs
        //         ctx.request_layout();
        //         ctx.request_paint();
        //     }
        //     PointerEvent::PointerUp(_, _) => {
        //         ctx.set_active(false);
        //         // did this to refresh page following rebuilding tablet leafs
        //         ctx.request_layout();
        //         ctx.request_paint();
        //     }
        //     _ => (),
        // }
    }
    
    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {
        // self.executable_path_label.inner.on_text_event(ctx, event);
        // self.executable_path_textbox.inner.on_text_event(ctx, event);
        // self.original_icon_path_label.inner.on_text_event(ctx, event);
        // self.original_icon_path_textbox.inner.on_text_event(ctx, event);
        // self.target_packages_directory_path_label.inner.on_text_event(ctx, event);
        // self.target_packages_directory_path_textbox.inner.on_text_event(ctx, event);
        // self.app_name_label.inner.on_text_event(ctx, event);
        // self.app_name_textbox.inner.on_text_event(ctx, event);
        // self.generate_app_packages_button.inner.on_text_event(ctx, event);
        // self.progress_message_label.inner.lifecycle(ctx, event);
    }

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {
        // self.executable_path_label.inner.on_access_event(ctx, event);
        // self.executable_path_textbox.inner.on_access_event(ctx, event);
        // self.original_icon_path_label.inner.on_access_event(ctx, event);
        // self.original_icon_path_textbox.inner.on_access_event(ctx, event);
        // self.target_packages_directory_path_label.inner.on_access_event(ctx, event);
        // self.target_packages_directory_path_textbox.inner.on_access_event(ctx, event);
        // self.app_name_label.inner.on_access_event(ctx, event);
        // self.app_name_textbox.inner.on_access_event(ctx, event);
        // self.generate_app_packages_button.inner.on_access_event(ctx, event);
        // self.progress_message_label.inner.lifecycle(ctx, event);
    }

    #[allow(missing_docs)]
    fn on_status_change(&mut self, _ctx: &mut LifeCycleCtx, _event: &StatusChange) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.executable_path_label.inner.lifecycle(ctx, event);
        self.executable_path_textbox.inner.lifecycle(ctx, event);
        self.original_icon_path_label.inner.lifecycle(ctx, event);
        self.original_icon_path_textbox.inner.lifecycle(ctx, event);
        self.target_packages_directory_path_label.inner.lifecycle(ctx, event);
        self.target_packages_directory_path_textbox.inner.lifecycle(ctx, event);
        self.app_name_label.inner.lifecycle(ctx, event);
        self.app_name_textbox.inner.lifecycle(ctx, event);
        self.generate_app_packages_button.inner.lifecycle(ctx, event);
        self.progress_message_label.inner.lifecycle(ctx, event);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let padding = Insets::uniform(10.0);
        let new_bc = bc.shrink((padding.x_value(), padding.y_value()));
        let mut total_size;
        let mut size;
        let mut origin = Point::new(padding.x0, padding.y0);
        let in_between_padding = 10.0;

        size = self.executable_path_label.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.executable_path_label.inner, origin);
        total_size = size;
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (executable_path_label)", size, origin);

        size = self.executable_path_textbox.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.executable_path_textbox.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (executable_path_textbox)", size, origin);

        size = self.original_icon_path_label.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.original_icon_path_label.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (original_icon_path_label)", size, origin);

        size = self.original_icon_path_textbox.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.original_icon_path_textbox.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (original_icon_path_textbox)", size, origin);
        
        size = self.target_packages_directory_path_label.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.target_packages_directory_path_label.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (target_packages_directory_path_label)", size, origin);

        size = self.target_packages_directory_path_textbox.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.target_packages_directory_path_textbox.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (target_packages_directory_path_textbox)", size, origin);

        size = self.app_name_label.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.app_name_label.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (app_name_label)", size, origin);

        size = self.app_name_textbox.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.app_name_textbox.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (app_name_textbox)", size, origin);

        size = self.generate_app_packages_button.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.generate_app_packages_button.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (generate_app_packages_button)", size, origin);

        size = self.progress_message_label.inner.layout(ctx, &new_bc);
        ctx.place_child(&mut self.progress_message_label.inner, origin);
        total_size.width = size.width.max(total_size.width);
        origin.y += size.height + in_between_padding;
        // eprintln!("Size: {:?} | Origin: {:?} (progress_message_label)", size, origin);

        total_size.height = origin.y + size.height + padding.y1;

        // size * 2.0
        // Size::new(total_size.width, bc.max().height)
        total_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        self.executable_path_label.inner.paint(ctx, scene);
        self.executable_path_textbox.inner.paint(ctx, scene);
        self.original_icon_path_label.inner.paint(ctx, scene);
        self.original_icon_path_textbox.inner.paint(ctx, scene);
        self.target_packages_directory_path_label.inner.paint(ctx, scene);
        self.target_packages_directory_path_textbox.inner.paint(ctx, scene);
        self.app_name_label.inner.paint(ctx, scene);
        self.app_name_textbox.inner.paint(ctx, scene);
        self.generate_app_packages_button.inner.paint(ctx, scene);
        self.progress_message_label.inner.paint(ctx, scene);
    }

    fn accessibility_role(&self) -> Role {
        Role::Tab
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        self.executable_path_label.inner.accessibility(ctx);
        self.executable_path_textbox.inner.accessibility(ctx);
        self.original_icon_path_label.inner.accessibility(ctx);
        self.original_icon_path_textbox.inner.accessibility(ctx);
        self.target_packages_directory_path_label.inner.accessibility(ctx);
        self.target_packages_directory_path_textbox.inner.accessibility(ctx);
        self.app_name_label.inner.accessibility(ctx);
        self.app_name_textbox.inner.accessibility(ctx);
        self.generate_app_packages_button.inner.accessibility(ctx);
        self.progress_message_label.inner.accessibility(ctx);
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![
            self.executable_path_label.inner.id(),
            self.executable_path_textbox.inner.id(),
            self.original_icon_path_label.inner.id(),
            self.original_icon_path_textbox.inner.id(),
            self.target_packages_directory_path_label.inner.id(),
            self.target_packages_directory_path_textbox.inner.id(),
            self.app_name_label.inner.id(),
            self.app_name_textbox.inner.id(),
            self.generate_app_packages_button.inner.id(),
            self.progress_message_label.inner.id(),
        ]
    }
}