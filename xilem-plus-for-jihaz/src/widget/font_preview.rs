use accesskit::{Node, Role};
use kurbo::{Point, Size};
use masonry::{
    core::{
        AccessCtx, AccessEvent, Action, ArcStr, BoxConstraints, BrushIndex, EventCtx, LayoutCtx, PaintCtx, PointerEvent, PropertiesMut, PropertiesRef, QueryCtx, RegisterCtx, TextEvent, Update, UpdateCtx, Widget, WidgetId, WidgetMut, WidgetPod
    }, 
    vello::Scene
};
use smallvec::SmallVec;
use tracing::{trace, trace_span, Span};
use xilem::view::PointerButton;
use crate::text::{CowFontFamily, CowFontStack, CowStyleProperty, GeneralTextStyles};

use super::Label;

pub struct FontPreview {
    // #[cfg(debug_assertions)]
    // font_name: std::sync::Arc<str>,
    name: WidgetPod<Label>,
    preview: WidgetPod<Label>,
}


// --- MARK: BUILDERS ---
impl FontPreview  {
    /// Create a new font preview with a preview text.
    /// 
    /// To showcase the font in the preview label,
    /// this view automatically creates a font stack style property
    /// from the given font name and inserts it as a style for the preview label.
    /// 
    /// # Examples
    ///
    /// ```
    /// use xilem_plus_for_jihaz::text_related::{CowFontFamily, CowStyleProperty, GeneralTextStyles};
    /// use xilem_plus_for_jihaz::widget::FontPreview;
    ///
    /// let name = "Calibri";
    /// let preview = "A text sentence for previewing the font";
    /// let name_styles = GeneralTextStyles::default_styles();
    /// let mut preview_styles = name_styles.clone();
    /// preview_styles.insert(CowStyleProperty::FontSize(14.0));
    /// 
    /// let font_preview = FontPreview::new(name, preview, name_styles, preview_styles);
    /// 
    /// ```
    pub fn new(
        name: impl Into<ArcStr>, 
        preview: impl Into<ArcStr>,
        name_styles: GeneralTextStyles<BrushIndex>,
        preview_styles: GeneralTextStyles<BrushIndex>,
    ) -> Self {
        // setting the font property of the preview label styles as
        // a named font family property that has its font name taken from the font name label.
        let name = name.into();
        // Create styles for safe rendering with system UI font
        let mut preview_styles = preview_styles;
        // Use font stack with fallback to system UI
        let preview_fonts = CowFontStack::List(vec![
            CowFontFamily::Named(name.clone()),
            CowFontFamily::Generic(parley::GenericFamily::SystemUi)
        ].into());
        preview_styles.insert(CowStyleProperty::FontStack(preview_fonts));
        // Add explicit font size for consistent rendering
        preview_styles.insert(CowStyleProperty::FontSize(14.0));
        // Add explicit line height for stable layout
        preview_styles.insert(CowStyleProperty::LineHeight(1.5));

        Self::from_labels(
            Label::new(name, name_styles), 
            Label::new(preview, preview_styles),
        )
    }

    /// Create a new font preview with the provided name and preview [`Labels`](Label).
    ///
    /// To showcase the font in the preview label,
    /// this view automatically inherits the font name from the name label,
    /// and creates from it a font stack style property, and then inserts it
    /// as a style for the preview label.
    /// 
    /// # Examples
    ///
    /// ```
    /// use xilem_plus_for_jihaz::text_related::{CowFontFamily, CowStyleProperty, GeneralTextStyles};
    /// use xilem_plus_for_jihaz::widget::{FontPreview, Label};
    ///
    /// let name = "Calibri";
    /// let name_styles = GeneralTextStyles::default_styles();
    /// let mut preview_styles = name_styles.clone();
    /// preview_styles.insert(CowStyleProperty::FontSize(14.0));
    /// 
    /// let name = Label::new(name, name_styles);
    /// let preview = Label::new("A text sentence for previewing the font", preview_styles);
    /// 
    /// let font_preview = FontPreview::from_labels(name, preview);
    /// ```
    pub fn from_labels(name: Label, preview: Label) -> Self {
        // setting the font property of the preview label styles as
        // a named font family property that has its font name taken from the font name label.
        // Use system UI font for preview to avoid shaping issues
        // Get the font name from the name label
        let font_name = name.text().clone();
        let preview_fonts = CowFontStack::List(vec![
            CowFontFamily::Named(font_name.clone()),
            CowFontFamily::Generic(parley::GenericFamily::SystemUi)
        ].into());
        let preview = preview
            .with_style(CowStyleProperty::FontStack(preview_fonts))
            .with_style(CowStyleProperty::FontSize(14.0))
            .with_style(CowStyleProperty::LineHeight(1.5));

        Self {
            // #[cfg(debug_assertions)]
            // font_name,
            name: WidgetPod::new(name),
            preview: WidgetPod::new(preview),
        }
    }

    /// Create a new font preview with the provided name and preview [`Labels`](Label) with predetermined ids.
    ///
    /// This constructor is useful for toolkits which use Masonry (such as Xilem).
    /// 
    /// Warning: this assumes a font stack property of the font name has already been
    /// inserted in the general styles of the preview label.
    pub fn from_label_pods(
        _font_name: std::sync::Arc<str>,
        name: WidgetPod<Label>, 
        preview: WidgetPod<Label>
    ) -> Self {
        Self {
            // #[cfg(debug_assertions)]
            // _font_name,
            name,
            preview
        }
    }
}

// --- MARK: WIDGETMUT ---
impl FontPreview {
    /// Gets the WidgetMut for the font name label.
    pub fn name_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, Label> {
        this.ctx.get_mut(&mut this.widget.name)
    }

    /// Gets the WidgetMut for the font preview label.
    pub fn preview_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, Label> {
        this.ctx.get_mut(&mut this.widget.preview)
    }
}

// --- MARK: IMPL WIDGET ---
impl Widget for FontPreview {
    fn on_pointer_event(
        &mut self,
        ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        event: &PointerEvent,
    ) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.capture_pointer();
                    // Changes in pointer capture impact appearance, but not accessibility node
                    ctx.request_paint_only();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(button, _) => {
                if ctx.is_pointer_capture_target() && ctx.is_hovered() && !ctx.is_disabled() {
                    ctx.submit_action(Action::ButtonPressed(*button));
                    trace!("Button {:?} released", ctx.widget_id());
                }
                // Changes in pointer capture impact appearance, but not accessibility node
                ctx.request_paint_only();
            }
            _ => (),
        }
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
        ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        event: &AccessEvent,
    ) {
        if ctx.target() == ctx.widget_id() {
            match event.action {
                accesskit::Action::Click => {
                    ctx.submit_action(Action::ButtonPressed(PointerButton::Primary));
                }
                _ => {}
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _props: &mut PropertiesMut<'_>, event: &masonry::core::Update) {
        match event {
            Update::HoveredChanged(_) | Update::FocusChanged(_) | Update::DisabledChanged(_) => {
                ctx.request_paint_only();
            }
            _ => {}
        }
    }

    fn register_children(&mut self, ctx: &mut RegisterCtx) {
        ctx.register_child(&mut self.name);
        ctx.register_child(&mut self.preview);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        _props: &mut PropertiesMut<'_>,
        bc: &BoxConstraints,
    ) -> Size {

        let inner_spacing = 3.0;

        // #[cfg(debug_assertions)]
        // tracing::debug!("FontPreview layout for font: {}", &self.font_name);
        let mut size = ctx.run_layout(&mut self.name, bc);
        ctx.place_child(&mut self.name, Point::ORIGIN);

        let preview_size = ctx.run_layout(&mut self.preview, bc);
        ctx.place_child(&mut self.preview, Point::new(0.0, size.height + inner_spacing));

        size.height += preview_size.height + inner_spacing;
        size
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
        smallvec::smallvec![self.name.id(), self.preview.id()]
    }

    fn make_trace_span(&self, ctx: &QueryCtx) -> Span {
        trace_span!("FontPreview", id = ctx.widget_id().trace())
    }
}