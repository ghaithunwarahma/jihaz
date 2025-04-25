// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

pub use masonry::core::PointerButton;
use xilem_core::{MessageResult, ViewId, ViewPathTracker};

use xilem::core::{DynMessage, Mut, View, ViewMarker};
use xilem::{Pod, ViewCtx};

use crate::widget;

use super::Label;

/// A button which returns an action message result when the primary mouse button (normally left) is pressed.
pub fn action_button<Action>(
    label: impl Into<Label>,
    action: Action,
) -> ActionButton<Action> {
    ActionButton {
        label: label.into(),
        action,
        any_pointer_pressed: false
    }
}

/// A button which returns an action message result when pressed.
pub fn action_button_any_pointer<Action>(
    label: impl Into<Label>,
    action: Action,
) -> ActionButton<Action> {
    ActionButton {
        label: label.into(),
        action,
        any_pointer_pressed: true,
    }
}

/// The [`View`] created by [`button`] from a `label` and a callback.
///
/// See `button` documentation for more context.
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct ActionButton<Action> {
    // N.B. This widget is *implemented* to handle any kind of view with an element
    // type of `Label` even though it currently does not do so.
    label: Label,
    action: Action,
    any_pointer_pressed: bool,
}

const LABEL_VIEW_ID: ViewId = ViewId::new(0);

impl<Action> ViewMarker for ActionButton<Action> {}

impl<State, Action> View<State, Action, ViewCtx> for ActionButton<Action>
where
    Action: Clone + 'static
{
    type Element = Pod<widget::Button>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (child, ()) = ctx.with_id(LABEL_VIEW_ID, |ctx| {
            View::<State, Action, _>::build(&self.label, ctx)
        });
        ctx.with_leaf_action_widget(|ctx| {
            ctx.new_pod(widget::Button::from_label_pod(child.into_widget_pod()))
        })
    }

    fn rebuild(
        &self,
        prev: &Self,
        state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        ctx.with_id(LABEL_VIEW_ID, |ctx| {
            View::<State, Action, _>::rebuild(
                &self.label,
                &prev.label,
                state,
                ctx,
                widget::Button::label_mut(&mut element),
            );
        });
    }

    fn teardown(
        &self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        ctx.with_id(LABEL_VIEW_ID, |ctx| {
            View::<State, Action, _>::teardown(
                &self.label,
                &mut (),
                ctx,
                widget::Button::label_mut(&mut element),
            );
        });
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        _: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut State,
    ) -> MessageResult<Action> {
        match id_path.split_first() {
            Some((&LABEL_VIEW_ID, rest)) => self.label.message(&mut (), rest, message, app_state),
            None => match message.downcast::<masonry::core::Action>() {
                Ok(action) => {
                    if let masonry::core::Action::ButtonPressed(button) = *action {
                        match self.any_pointer_pressed {
                            true => MessageResult::Action(self.action.clone()),
                            false => match button {
                                PointerButton::Primary => {
                                    MessageResult::Action(self.action.clone())
                                }
                                _ => MessageResult::Nop,
                            }
                        }
                    } else {
                        tracing::error!("Wrong action type in ActionButton::message: {action:?}");
                        MessageResult::Stale(action)
                    }
                }
                Err(message) => {
                    tracing::error!("Wrong message type in ActionButton::message: {message:?}");
                    MessageResult::Stale(message)
                }
            },
            _ => {
                tracing::warn!("Got unexpected id path in ActionButton::message");
                MessageResult::Stale(message)
            }
        }
    }
}
