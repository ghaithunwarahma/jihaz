// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::{core::View, ViewCtx, Pod};
use masonry::{widget, ArcStr};
use xilem_core::{Mut, ViewMarker};

pub use masonry::PointerButton;

use xilem_core::{MessageResult , ViewId};

type Callback<State, Action> = Box<dyn Fn(&mut State) -> Action + Send + Sync + 'static>;

/// A button which calls `callback` when the primary mouse button (normally left) is pressed.
pub fn button<F, State, Action>(
    label: impl Into<ArcStr>,
    callback: F
) -> ButtonOfAction<State, Action>
where
    F: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    ButtonOfAction {
        label: label.into(),
        callback: Box::new(callback),
        any_pointer_pressed: false,
    }
}

/// A button which calls `callback` when pressed.
pub fn button_any_pointer<F, State, Action>(
    label: impl Into<ArcStr>,
    callback: F,
) -> ButtonOfAction<State, Action>
where
    F: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    ButtonOfAction {
        label: label.into(),
        callback: Box::new(callback),
        any_pointer_pressed: true,
    }
}

pub struct ButtonOfAction<State, Action> {
    label: ArcStr,
    callback: Callback<State, Action>,
    any_pointer_pressed: bool,
}

impl<State, Action> ViewMarker for ButtonOfAction<State, Action> {}
impl<State: 'static, Action: 'static> View<State, Action, ViewCtx> for ButtonOfAction<State, Action> {
    type Element = Pod<widget::Button>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        ctx.with_leaf_action_widget(|ctx| ctx.new_pod(widget::Button::new(self.label.clone())))
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        if prev.label != self.label {
            element.set_text(self.label.clone());
            ctx.mark_changed();
        }
        element
    }

    fn teardown(
        &self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<'_, Self::Element>,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        _: &mut Self::ViewState,
        id_path: &[ViewId],
        message: xilem_core::DynMessage,
        app_state: &mut State,
    ) -> MessageResult<Action> {
        debug_assert!(
            id_path.is_empty(),
            "id path should be empty in ButtonOfAction::message"
        );
        match message.downcast::<masonry::Action>() {
            Ok(action) => {
                if let masonry::Action::ButtonPressed(button) = *action {
                    if self.any_pointer_pressed {
                        MessageResult::Action((self.callback)(app_state))
                    } else if let PointerButton::Primary = button {
                        MessageResult::Action((self.callback)(app_state))
                    } else {
                        MessageResult::Stale(action)
                    }
                } else {
                    tracing::error!("Wrong action type in ButtonOfAction::message: {action:?}");
                    MessageResult::Stale(action)
                }
            }
            Err(message) => {
                tracing::error!("Wrong message type in ButtonOfAction::message: {message:?}");
                MessageResult::Stale(message)
            }
        }
    }
}
