// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use xilem::{core::View, Pod, ViewCtx};
use masonry::ArcStr;
use xilem_core::{MessageResult, Mut, ViewId, ViewMarker};
use crate::{widget, app_helpers::ActionData};

/// A button that only sends Action::ButtonPressed and lets parent view handle the action.
pub fn special_button<Data>(label: impl Into<ArcStr>, action_data: Data) -> SpecialButton
where
    Data: Any + Send + Sync
{
    SpecialButton {
        label: label.into(),
        action_data: ActionData::new(action_data),
    }
}
    
/// A button that only sends Action::ButtonPressed and lets parent view handle the action.
pub struct SpecialButton {
    label: ArcStr,
    action_data: ActionData,
}

impl ViewMarker for SpecialButton {}
impl<State, Action> View<State, Action, ViewCtx> for SpecialButton {
    type Element = Pod<widget::SpecialButton>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let element = ctx.with_leaf_action_widget(|ctx| {
            ctx.new_pod(widget::SpecialButton::new_from_action_data(
                self.label.clone(), 
                self.action_data.clone()
            ))
        });
        element
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        if prev.label != self.label {
            let mut child_element = element.ctx.get_mut(&mut element.widget.label);
            child_element.set_text(self.label.clone());
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
        _view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: xilem_core::DynMessage,
        _app_state: &mut State,
    ) -> MessageResult<Action> {
        debug_assert!(
            id_path.is_empty(),
            "id path should be empty in Button::message"
        );
        match message.downcast::<masonry::Action>() {
            Ok(action) => {
                tracing::error!("Wrong action type in Button::message: {action:?}. SpecialButton does not handle its actions.");
                MessageResult::Stale(action)
            }
            Err(message) => {
                tracing::error!("Wrong message type in Button::message: {message:?}. SpecialButton does not handle its actions.");
                MessageResult::Stale(message)
            }
        }
    }
}
