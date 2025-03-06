// Copyright 2018 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! A button widget.

use std::any::Any;
use std::sync::Arc;

use accesskit::{Node, Role};
use smallvec::{SmallVec, smallvec};
use tracing::{Span, trace, trace_span};

use masonry::core::{
    AccessCtx, AccessEvent, Action, ArcStr, BoxConstraints, EventCtx, LayoutCtx, PaintCtx, PointerButton, PointerEvent, QueryCtx, TextEvent, Update, UpdateCtx, Widget, WidgetId, WidgetMut, WidgetPod
};
use masonry::kurbo::{Insets, Size};
use masonry::theme;
use masonry::util::{UnitPoint, fill_lin_gradient, stroke};
use masonry::vello::Scene;
use masonry::widgets::Label;

use crate::app_helpers::ActionData;

// the minimum padding added to a button.
// NOTE: these values are chosen to match the existing look of Textbox; these
// should be reevaluated at some point.
const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

/// A button with a text label.
///
/// Emits [`Action::ButtonPressed`] when pressed.
pub struct SpecialButton {
    // Need this public as Receiver Widget is not yet stabilized,
    // which means can't do WidgetMut<'_, Button> implementations on SpecialButton yet.
    pub label: WidgetPod<Label>,
    action_data: ActionData,
}

// --- MARK: BUILDERS ---
impl SpecialButton {
    /// Create a new button with a text label and a special action.
    ///
    /// # Examples
    ///
    /// ```
    /// use jihaz::widget::SpecialButton;
    ///
    /// let button = SpecialButton::new("Increment", 1);
    /// ```
    pub fn new<ActionValue>(text: impl Into<ArcStr>, action: ActionValue) -> SpecialButton
    where
        ActionValue: Any + Send + Sync
    {
        SpecialButton::from_label(Label::new(text), action)
    }

    /// Create a new button with a text label and a special action.
    ///
    /// # Examples
    ///
    /// ```
    /// use jihaz::ActionData;
    /// use jihaz::widget::SpecialButton;
    ///
    /// let button = SpecialButton::new_from_action_data("Increment", ActionData::new(1));
    /// ```
    pub fn new_from_action_data(text: impl Into<ArcStr>, action_data: ActionData) -> SpecialButton {
        SpecialButton::from_label_and_action_data(Label::new(text), action_data)
    }

    /// Create a new button with the provided [`Label`], and with a special action.
    ///
    /// # Examples
    ///
    /// ```
    /// use masonry::Color;
    /// use masonry::widget::Label;
    /// use jihaz::widget::SpecialButton;
    ///
    /// let label = Label::new("Increment").with_text_brush(Color::rgb(0.5, 0.5, 0.5));
    /// let button = SpecialButton::from_label(label, 1);
    /// ```
    pub fn from_label<ActionValue>(label: Label, action: ActionValue) -> SpecialButton
    where
        ActionValue: Any + Send + Sync
    {
        SpecialButton {
            label: WidgetPod::new(label),
            action_data: ActionData {
                data: Arc::new(action)
            }
        }
    }

    /// Create a new button with the provided [`Label`], and with a special action.
    ///
    /// # Examples
    ///
    /// ```
    /// use masonry::Color;
    /// use masonry::widget::Label;
    /// use jihaz::widget::SpecialButton;
    /// use jihaz::ActionData;
    ///
    /// let label = Label::new("Increment").with_text_brush(Color::rgb(0.5, 0.5, 0.5));
    /// let button = SpecialButton::from_label_and_action_data(label, ActionData::new(1));
    /// ```
    pub fn from_label_and_action_data(label: Label, action_data: ActionData) -> SpecialButton {
        SpecialButton {
            label: WidgetPod::new(label),
            action_data,
        }
    }
}

// --- MARK: WIDGETMUT ---
impl SpecialButton {
    /// Set the text.
    pub fn set_text(this: &mut WidgetMut<'_, Self>, new_text: impl Into<ArcStr>) {
        Label::set_text(&mut Self::label_mut(this), new_text);
    }

    pub fn label_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, Label> {
        this.ctx.get_mut(&mut this.widget.label)
    }
}

// --- MARK: IMPL WIDGET ---
impl Widget for SpecialButton {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
         match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.capture_pointer();
                    // Changes in pointer capture impact appearance, but not accessibility node
                    ctx.request_paint_only();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(_button, _) => {
                if ctx.is_pointer_capture_target() && ctx.is_hovered() && !ctx.is_disabled() {
                    // ctx.submit_action(Action::ButtonPressed(*button));
                    ctx.submit_action(Action::Other(Box::new(self.action_data.data.clone())));
                    trace!("Button {:?} released", ctx.widget_id());
                }
                // Changes in pointer capture impact appearance, but not accessibility node
                ctx.request_paint_only();
                // This may no longer be needed in current Masonry (I think this was from a previous version of Masontry, and not actually my idea)
                ctx.release_pointer();
            }
            // This may no longer be needed in current Masonry (I think this was from a previous version of Masontry, and not actually my idea)
            PointerEvent::PointerLeave(_) => {
                // If the screen was locked whilst holding down the mouse button, we don't get a `PointerUp`
                // event, but should no longer be active
                ctx.release_pointer();
            }
            _ => (),
        }
        // self.label.on_pointer_event(ctx, event);
    }

    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        if ctx.target() == ctx.widget_id() {
            match event.action {
                accesskit::Action::Click => {
                    ctx.submit_action(Action::ButtonPressed(PointerButton::Primary));
                }
                _ => {}
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, event: &Update) {
        match event {
            Update::HoveredChanged(_) | Update::FocusChanged(_) | Update::DisabledChanged(_) => {
                ctx.request_paint_only();
            }
            _ => {}
        }
    }

    fn register_children(&mut self, ctx: &mut masonry::core::RegisterCtx) {
        ctx.register_child(&mut self.label);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();

        let label_size = ctx.run_layout(&mut self.label, &label_bc);

        let baseline = ctx.child_baseline_offset(&self.label);
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = theme::BORDERED_WIDGET_HEIGHT;

        let button_size = bc.constrain(Size::new(
            label_size.width + padding.width,
            (label_size.height + padding.height).max(min_height),
        ));

        let label_offset = (button_size.to_vec2() - label_size.to_vec2()) / 2.0;
        ctx.place_child(&mut self.label, label_offset.to_point());

        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let is_active = ctx.is_pointer_capture_target() && !ctx.is_disabled();
        let is_hovered = ctx.is_hovered();
        let size = ctx.size();
        let stroke_width = theme::BUTTON_BORDER_WIDTH;

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(theme::BUTTON_BORDER_RADIUS);

        let bg_gradient = if ctx.is_disabled() {
            [theme::DISABLED_BUTTON_LIGHT, theme::DISABLED_BUTTON_DARK]
        } else if is_active {
            [theme::BUTTON_DARK, theme::BUTTON_LIGHT]
        } else {
            [theme::BUTTON_LIGHT, theme::BUTTON_DARK]
        };

        let border_color = if is_hovered && !ctx.is_disabled() {
            theme::BORDER_LIGHT
        } else {
            theme::BORDER_DARK
        };

        stroke(scene, &rounded_rect, border_color, stroke_width);
        fill_lin_gradient(
            scene,
            &rounded_rect,
            bg_gradient,
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
        );
    }

    fn accessibility_role(&self) -> Role {
        Role::Button
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx, node: &mut Node) {
        // IMPORTANT: We don't want to merge this code in practice, because
        // the child label already has a 'name' property.
        // This is more of a proof of concept of `get_raw_ref()`.
        if false {
            let label = ctx.get_raw_ref(&self.label);
            let name = label.widget().text().as_ref().to_string();
            node.set_value(name);
        }
        node.add_action(accesskit::Action::Click);
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.label.id()]
    }

    fn make_trace_span(&self, ctx: &QueryCtx<'_>) -> Span {
        trace_span!("SpecialButton", id = ctx.widget_id().trace())
    }
}

// --- MARK: TESTS ---
#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;
    use masonry::assert_render_snapshot;
    use masonry::core::StyleProperty;
    use masonry::testing::{TestHarness, TestWidgetExt, widget_ids};
    use masonry::theme::PRIMARY_LIGHT;

    #[test]
    fn simple_button() {
        let [button_id] = widget_ids();
        let widget = SpecialButton::new("Hello", 0).with_id(button_id);

        let mut harness = TestHarness::create(widget);

        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "hello");

        assert_eq!(harness.pop_action(), None);

        harness.mouse_click_on(button_id);
        assert_eq!(
            harness.pop_action(),
            Some((Action::ButtonPressed(PointerButton::Primary), button_id))
        );
    }

    #[test]
    fn edit_button() {
        let image_1 = {
            let label = Label::new("The quick brown fox jumps over the lazy dog")
                .with_brush(PRIMARY_LIGHT)
                .with_style(StyleProperty::FontSize(20.0));
            let button = SpecialButton::from_label(label,0);

            let mut harness = TestHarness::create_with_size(button, Size::new(50.0, 50.0));

            harness.render()
        };

        let image_2 = {
            let button = SpecialButton::new("Hello world", 0);

            let mut harness = TestHarness::create_with_size(button, Size::new(50.0, 50.0));

            harness.edit_root_widget(|mut button| {
                let mut button = button.downcast::<SpecialButton>();
                SpecialButton::set_text(&mut button, "The quick brown fox jumps over the lazy dog");

                let mut label = SpecialButton::label_mut(&mut button);
                Label::set_brush(&mut label, PRIMARY_LIGHT);
                Label::insert_style(&mut label, StyleProperty::FontSize(20.0));
            });

            harness.render()
        };

        // We don't use assert_eq because we don't want rich assert
        assert!(image_1 == image_2);
    }
}
