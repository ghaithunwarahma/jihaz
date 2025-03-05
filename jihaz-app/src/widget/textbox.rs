// Copyright 2018 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

use accesskit::Role;
use jihaz::sys_task::handle::SystemTaskHandle;
use jihaz_macros::debug_panic;
use masonry::parley::{
    layout::Alignment,
    style::{FontFamily, FontStack},
};
use smallvec::SmallVec;
use tracing::{trace, trace_span, Span};
use masonry::vello::{
    kurbo::{Affine, Point, Size, Stroke},
    peniko::{BlendMode, Color},
    Scene,
};

use masonry::widget::LineBreaking;
use masonry::{
    dpi::{LogicalPosition, LogicalSize},
    text::{TextBrush, TextEditor, TextStorage, TextWithSelection},
    AccessCtx, AccessEvent, BoxConstraints, CursorIcon, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, PointerEvent, StatusChange, TextEvent, Widget, WidgetId,
};
use winit::{event::{ElementState, Ime}, keyboard::{Key, SmolStr}};

use super::local_widget_mut::WidgetMut;

const TEXTBOX_PADDING: f64 = 3.0;
/// HACK: A "margin" which is placed around the outside of all textboxes, ensuring that
/// they do not fill the entire width of the window.
///
/// This is added by making the width of the textbox be (twice) this amount less than
/// the space available, which is absolutely horrible.
///
/// In theory, this should be proper margin/padding in the parent widget, but that hasn't been
/// designed.
const TEXTBOX_MARGIN: f64 = 8.0;
/// The fallback minimum width for a textbox with infinite provided maximum width.
const INFINITE_TEXTBOX_WIDTH: f64 = 400.0;

/// The textbox widget is a widget which shows text which can be edited by the user
///
/// For immutable text [`Prose`](super::Prose) should be preferred
// TODO: RichTextBox 👀
pub struct Textbox {
    // We hardcode the underlying storage type as `String`.
    // We might change this to a rope based structure at some point.
    // If you need a text box which uses a different text type, you should
    // create a custom widget
    editor: TextEditor<String>,
    line_break_mode: LineBreaking,
    show_disabled: bool,
    brush: TextBrush,
}

// --- MARK: BUILDERS ---
impl Textbox {
    pub fn new(initial_text: impl Into<String>) -> Self {
        Textbox {
            editor: TextEditor::new(initial_text.into(), masonry::theme::TEXT_SIZE_NORMAL as f32),
            line_break_mode: LineBreaking::WordWrap,
            show_disabled: true,
            brush: masonry::theme::TEXT_COLOR.into(),
        }
    }

    // TODO: Can we reduce code duplication with `Label` widget somehow?
    pub fn text(&self) -> &str {
        self.editor.text()
    }

    #[doc(alias = "with_text_color")]
    pub fn with_text_brush(mut self, brush: impl Into<TextBrush>) -> Self {
        self.brush = brush.into();
        self.editor.set_brush(self.brush.clone());
        self
    }

    pub fn with_text_size(mut self, size: f32) -> Self {
        self.editor.set_text_size(size);
        self
    }

    pub fn with_text_alignment(mut self, alignment: Alignment) -> Self {
        self.editor.set_text_alignment(alignment);
        self
    }

    pub fn with_font(mut self, font: FontStack<'static>) -> Self {
        self.editor.set_font(font);
        self
    }
    pub fn with_font_family(self, font: FontFamily<'static>) -> Self {
        self.with_font(FontStack::Single(font))
    }

    pub fn with_line_break_mode(mut self, line_break_mode: LineBreaking) -> Self {
        self.line_break_mode = line_break_mode;
        self
    }
}

// --- MARK: WIDGETMUT ---
impl WidgetMut<'_, Textbox> {
    pub fn text(&self) -> &str {
        self.masonry_mut.widget.editor.text()
    }

    pub fn set_text_properties<R>(
        &mut self,
        f: impl FnOnce(&mut TextWithSelection<String>) -> R,
    ) -> R {
        let ret = f(&mut self.masonry_mut.widget.editor);
        if self.masonry_mut.widget.editor.needs_rebuild() {
            self.masonry_mut.ctx.request_layout();
        }
        ret
    }

    /// Reset the contents of the text box.
    ///
    /// This is likely to be disruptive if the user is focused on this widget,
    /// and so should be avoided if possible.
    // FIXME - it's not clear whether this is the right behaviour, or if there even
    // is one.
    // TODO: Create a method which sets the text and the cursor selection to be used if focused?
    pub fn reset_text(&mut self, new_text: String) {
        if self.masonry_mut.ctx.is_focused() {
            tracing::warn!(
                "Called reset_text on a focused `Textbox`. This will lose the user's current selection and cursor"
            );
        }
        self.masonry_mut.widget.editor.reset_preedit();
        self.set_text_properties(|layout| layout.set_text(new_text));
    }

    #[doc(alias = "set_text_color")]
    pub fn set_text_brush(&mut self, brush: impl Into<TextBrush>) {
        let brush = brush.into();
        self.masonry_mut.widget.brush = brush;
        if !self.masonry_mut.ctx.is_disabled() {
            let brush = self.masonry_mut.widget.brush.clone();
            self.set_text_properties(|layout| layout.set_brush(brush));
        }
    }
    pub fn set_text_size(&mut self, size: f32) {
        self.set_text_properties(|layout| layout.set_text_size(size));
    }
    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.set_text_properties(|layout| layout.set_text_alignment(alignment));
    }
    pub fn set_font(&mut self, font_stack: FontStack<'static>) {
        self.set_text_properties(|layout| layout.set_font(font_stack));
    }
    pub fn set_font_family(&mut self, family: FontFamily<'static>) {
        self.set_font(FontStack::Single(family));
    }
    pub fn set_line_break_mode(&mut self, line_break_mode: LineBreaking) {
        self.masonry_mut.widget.line_break_mode = line_break_mode;
        self.masonry_mut.ctx.request_paint();
    }
}

// --- MARK: IMPL WIDGET ---
impl Widget for Textbox {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        let window_origin = ctx.window_origin();
        let inner_origin = Point::new(
            window_origin.x + TEXTBOX_PADDING,
            window_origin.y + TEXTBOX_PADDING,
        );
        match event {
            PointerEvent::PointerDown(button, state) => {
                if !ctx.is_disabled() {
                    // TODO: Start tracking currently pressed link?
                    let made_change = self.editor.pointer_down(inner_origin, state, *button);
                    if made_change {
                        ctx.request_layout();
                        ctx.request_paint();
                        ctx.request_focus();
                        ctx.set_active(true);
                    }
                }
            }
            PointerEvent::PointerMove(state) => {
                if !ctx.is_disabled()
                    && ctx.is_active()
                    && self.editor.pointer_move(inner_origin, state)
                {
                    // We might have changed text colours, so we need to re-request a layout
                    ctx.request_layout();
                    ctx.request_paint();
                }
            }
            PointerEvent::PointerUp(button, state) => {
                // TODO: Follow link (if not now dragging ?)
                if !ctx.is_disabled() && ctx.is_active() {
                    self.editor.pointer_up(inner_origin, state, *button);
                }
                ctx.set_active(false);
            }
            PointerEvent::PointerLeave(_state) => {
                ctx.set_active(false);
            }
            _ => {}
        }
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) { 
        let mut new_event = None;
        match event {
            TextEvent::KeyboardKey(ke, mods) if ke.state.is_pressed() => {
                // We don't input actual text when these keys are pressed
                if mods.control_key() || mods.super_key()
                // TODO: do things differently on mac, rather than capturing both super and control.
                {
                    match &ke.logical_key {
                        // Paste operation
                        Key::Character(str) 
                        if ( (*str == SmolStr::new_static("c") || *str == SmolStr::new_static("ذ")) 
                            && ke.state == ElementState::Pressed ) =>
                        {
                            let mut clipboard = SystemTaskHandle::global().clipboard();
                            let text = self.editor.text().to_owned();
                            clipboard.put_string(text);

                            // did this to refresh page following rebuilding tablet leafs
                            ctx.request_layout();
                            ctx.request_paint();
                            // eprintln!("clicked m, {:?}", ke);
                            // println!("Copy operation");
                        }
                        // Cut operation
                        Key::Character(str) 
                        if ( (*str == SmolStr::new_static("x") || *str == SmolStr::new_static("ط")) 
                            && ke.state == ElementState::Pressed ) =>
                        {
                            let mut clipboard = SystemTaskHandle::global().clipboard();
                            let text = self.editor.text().to_owned();
                            clipboard.put_string(text);
                            
                            // reset the selection
                            new_event = Some(TextEvent::Ime(Ime::Commit(String::new())));

                            // did this to refresh page following rebuilding tablet leafs
                            ctx.request_layout();
                            ctx.request_paint();
                            // eprintln!("clicked m, {:?}", ke);
                            // println!("Cut operation");
                        }
                        // Copy operation
                        Key::Character(str) 
                        if ( (*str == SmolStr::new_static("v") || *str == SmolStr::new_static("د")) 
                            && ke.state == ElementState::Pressed ) =>
                        {
                            let clipboard = SystemTaskHandle::global().clipboard();
                            if let Some(paste_string) = clipboard.get_string() {
                                new_event = Some(TextEvent::Ime(Ime::Commit(paste_string)));
                            }

                            // did this to refresh page following rebuilding tablet leafs
                            ctx.request_layout();
                            ctx.request_paint();
                            // eprintln!("clicked m, {:?}", ke);
                            // println!("Paste operation");
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        

        let result = match new_event {
            Some(new_event) => self.editor.text_event(ctx, &new_event),
            None => self.editor.text_event(ctx, event),
        };
        // If focused on a link and enter pressed, follow it?
        if result.is_handled() {
            ctx.set_handled();
            // TODO: only some handlers need this repaint
            ctx.request_layout();
            ctx.request_paint();
        }
    }

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {
        // TODO - Handle accesskit::Action::SetTextSelection
        // TODO - Handle accesskit::Action::ReplaceSelectedText
        // TODO - Handle accesskit::Action::SetValue
    }

    #[allow(missing_docs)]
    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, event: &StatusChange) {
        match event {
            StatusChange::FocusChanged(false) => {
                self.editor.focus_lost();
                ctx.request_layout();
                // TODO: Stop focusing on any links
            }
            StatusChange::FocusChanged(true) => {
                // TODO: Focus on first link
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        match event {
            LifeCycle::DisabledChanged(disabled) => {
                if self.show_disabled {
                    if *disabled {
                        self.editor.set_brush(masonry::theme::DISABLED_TEXT_COLOR);
                    } else {
                        self.editor.set_brush(self.brush.clone());
                    }
                }
                // TODO: Parley seems to require a relayout when colours change
                ctx.request_layout();
            }
            LifeCycle::BuildFocusChain => {
                // TODO: This will always be empty
                if !self.editor.text().links().is_empty() {
                    tracing::warn!("Links present in text, but not yet integrated");
                }
            }
            _ => {}
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        // Compute max_advance from box constraints
        let max_advance = if self.line_break_mode != LineBreaking::WordWrap {
            None
        } else if bc.max().width.is_finite() {
            Some((bc.max().width - 2. * TEXTBOX_PADDING - 2. * TEXTBOX_MARGIN) as f32)
        } else if bc.min().width.is_sign_negative() {
            Some(0.0)
        } else {
            None
        };
        self.editor.set_max_advance(max_advance);
        if self.editor.needs_rebuild() {
            let (font_ctx, layout_ctx) = ctx.text_contexts();
            self.editor.rebuild(font_ctx, layout_ctx);
        }
        let text_size = self.editor.size();
        let width = if bc.max().width.is_finite() {
            // If we have a finite width, chop off the margin
            bc.max().width - 2. * TEXTBOX_MARGIN
        } else {
            // If we're drawing based on the width of the text instead, request proper padding
            text_size.width.max(INFINITE_TEXTBOX_WIDTH) + 2. * TEXTBOX_PADDING
        };
        let label_size = Size {
            height: text_size.height + 2. * TEXTBOX_PADDING,
            // TODO: Better heuristic here?
            width,
        };
        let size = bc.constrain(label_size);
        trace!(
            "Computed layout: max={:?}. w={}, h={}",
            max_advance,
            size.width,
            size.height,
        );
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        if self.editor.needs_rebuild() {
            debug_panic!("Called Label paint before layout");
        }
        if self.line_break_mode == LineBreaking::Clip {
            let clip_rect = ctx.size().to_rect();
            scene.push_layer(BlendMode::default(), 1., Affine::IDENTITY, &clip_rect);
        }

        self.editor
            .draw(scene, Point::new(TEXTBOX_PADDING, TEXTBOX_PADDING));

        if self.line_break_mode == LineBreaking::Clip {
            scene.pop_layer();
        }
        let size = ctx.size();
        let outline_rect = size.to_rect().inset(1.0);
        scene.stroke(
            &Stroke::new(1.0),
            Affine::IDENTITY,
            Color::WHITE,
            None,
            &outline_rect,
        );
        let origin = ctx.window_origin();
        if ctx.has_focus() {
            ctx.signal(masonry::render_root::RenderRootSignal::ImeMoved(
                LogicalPosition {
                    x: origin.x,
                    y: origin.y + size.height,
                },
                LogicalSize {
                    width: size.width,
                    height: size.height,
                },
            ));
        }
    }

    fn get_cursor(&self) -> CursorIcon {
        CursorIcon::Text
    }

    fn accessibility_role(&self) -> Role {
        Role::TextInput
    }

    fn accessibility(&mut self, _ctx: &mut AccessCtx) {
        // TODO
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        SmallVec::new()
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("Textbox")
    }

    fn get_debug_text(&self) -> Option<String> {
        Some(self.editor.text().as_str().chars().take(100).collect())
    }
}
