// Copyright 2018 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! A button widget.

use std::any::Any;
use std::sync::Arc;

use accesskit::{DefaultActionVerb, Role};
use masonry::text::TextStorage;
use smallvec::{smallvec, SmallVec};
use tracing::{trace, trace_span, Span};

use masonry::{Action, PointerButton, WidgetId};
use masonry::paint_scene_helpers::{fill_lin_gradient, stroke, UnitPoint};
use masonry::widget::{Label, WidgetPod};
use masonry::{
    theme, AccessCtx, AccessEvent, ArcStr, BoxConstraints, EventCtx, Insets, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, PointerEvent, Size, StatusChange, TextEvent, Widget,
    vello::Scene,
};

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

// can't do this yet as Receiver trait is not yet stabilized
// // --- MARK: WIDGETMUT ---
// impl WidgetMut<'_, Button> {
//     /// Set the text.
//     pub fn set_text(&mut self, new_text: impl Into<ArcStr>) {
//         self.label_mut().set_text(new_text);
//     }

//     pub fn label_mut(&mut self) -> WidgetMut<'_, Label> {
//         self.ctx.get_mut(&mut self.widget.label)
//     }
// }

// --- MARK: IMPL WIDGET ---
impl Widget for SpecialButton {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(_, _) => {
                if ctx.is_active() && ctx.is_hot() && !ctx.is_disabled() {
                    println!("submitted action data");
                    ctx.submit_action(Action::Other(Box::new(self.action_data.data.clone())));
                    trace!("Button {:?} released", ctx.widget_id());
                }
                ctx.request_paint();
                ctx.set_active(false);
            }
            PointerEvent::PointerLeave(_) => {
                // If the screen was locked whilst holding down the mouse button, we don't get a `PointerUp`
                // event, but should no longer be active
                ctx.set_active(false);
            }
            _ => (),
        }
        // self.label.on_pointer_event(ctx, event);
    }

    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {
        // self.label.on_text_event(ctx, event);
    }

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        if event.target == ctx.widget_id() {
            match event.action {
                accesskit::Action::Default => {
                    ctx.submit_action(Action::ButtonPressed(PointerButton::Primary));
                    ctx.request_paint();
                }
                _ => {}
            }
        }
        // self.label.on_access_event(ctx, event);
    }

    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, _event: &StatusChange) {
        ctx.request_paint();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.label.lifecycle(ctx, event);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();

        let label_size = self.label.layout(ctx, &label_bc);

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

        trace!("Computed button size: {}", button_size);
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
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

        let border_color = if is_hot && !ctx.is_disabled() {
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

        self.label.paint(ctx, scene);
    }

    fn accessibility_role(&self) -> Role {
        Role::Button
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        // IMPORTANT: We don't want to merge this code in practice, because
        // the child label already has a 'name' property.
        // This is more of a proof of concept of `get_raw_ref()`.
        if false {
            let label = ctx.get_raw_ref(&self.label);
            let name = label.widget().text().as_str().to_string();
            ctx.current_node().set_name(name);
        }
        ctx.current_node()
            .set_default_action_verb(DefaultActionVerb::Click);

        self.label.accessibility(ctx);
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.label.id()]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("Button")
    }

    // FIXME
    #[cfg(FALSE)]
    fn get_debug_text(&self) -> Option<String> {
        Some(self.label.as_ref().text().as_str().to_string())
    }
}

// --- MARK: TESTS ---
#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;
    use masonry::{assert_render_snapshot, PointerButton};
    use masonry::testing::{widget_ids, TestHarness, TestWidgetExt};
    use masonry::theme::PRIMARY_LIGHT;

    #[test]
    fn simple_button() {
        let [button_id] = widget_ids();
        let widget = SpecialButton::new("Hello", Action::ButtonPressed).with_id(button_id);

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
                .with_text_brush(PRIMARY_LIGHT)
                .with_text_size(20.0);
            let button = SpecialButton::from_label(label, Action::ButtonPressed);

            let mut harness = TestHarness::create_with_size(button, Size::new(50.0, 50.0));

            harness.render()
        };

        let image_2 = {
            let button = SpecialButton::new("Hello world", Action::ButtonPressed);

            let mut harness = TestHarness::create_with_size(button, Size::new(50.0, 50.0));

            harness.edit_root_widget(|mut button| {
                let mut button = button.downcast::<SpecialButton>();
                // button.set_text("The quick brown fox jumps over the lazy dog");

                // let mut label = button.label_mut();
                let mut label = button.ctx.get_mut(&mut button.widget.label);
                label.set_text("The quick brown fox jumps over the lazy dog");
                label.set_text_properties(|props| {
                    props.set_brush(PRIMARY_LIGHT);
                    props.set_text_size(20.0);
                });
            });

            harness.render()
        };

        // We don't use assert_eq because we don't want rich assert
        assert!(image_1 == image_2);
    }
}
