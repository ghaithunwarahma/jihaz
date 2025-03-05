// Copyright 2018 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

use masonry::{Widget, WidgetState};

/// This is a wrapper to allow for implementing WidgetMut on this foreign crate.
/// You need the WrapWidgetMut trait to do the conversion within the View::rebuild method.
/// 
// TODO - Document extension trait workaround.
// See https://xi.zulipchat.com/#narrow/stream/317477-masonry/topic/Thoughts.20on.20simplifying.20WidgetMut/near/436478885
/// A mutable reference to a [`Widget`].
///
/// In Masonry, widgets can't be mutated directly. All mutations go through a `WidgetMut`
/// wrapper. So, to change a label's text, you might call `WidgetMut<Label>::set_text()`.
/// This helps Masonry make sure that internal metadata is propagated after every widget
/// change.
///
/// You can create a `WidgetMut` from [`TestHarness`](crate::testing::TestHarness),
/// [`EventCtx`](crate::EventCtx), [`LifeCycleCtx`](crate::LifeCycleCtx) or from a parent
/// `WidgetMut` with [`MutateCtx`].
///
/// `WidgetMut` implements [`Deref`](std::ops::Deref) with `W::Mut` as target.
///
/// ## `WidgetMut` as a Receiver
///
/// Once the Receiver trait is stabilized, `WidgetMut` will implement it so that custom
/// widgets in downstream crates can use `WidgetMut` as the receiver for inherent methods.
pub struct WidgetMut<'a, W: Widget> {
    pub masonry_mut: masonry::widget::WidgetMut<'a, W>
}

/// Convert masonry's WidgetMut into jihaz's.
pub trait WrapWidgetMut<'a, W: Widget> {
    /// Convert masonry's WidgetMut into jihaz's.
    fn wrap(self) -> WidgetMut<'a, W>;
}

impl<'a, W: Widget> WrapWidgetMut<'a, W> for masonry::widget::WidgetMut<'a, W> {
    fn wrap(self) -> WidgetMut<'a, W> {
        WidgetMut { masonry_mut: self }
    }
}

// I don't have a drop impl in order to have the unwrap method for WidgetMut
// impl<W: Widget> Drop for WidgetMut<'_, W> {
//     fn drop(&mut self) {
//         // // If this `WidgetMut` is a reborrow, a parent non-reborrow `WidgetMut`
//         // // still exists which will do the merge-up in `Drop`.
//         // if let Some(parent_widget_state) = self.ctx.parent_widget_state.take() {
//         //     parent_widget_state.merge_up(self.ctx.widget_state);
//         // }
//         drop(&mut self.masonry_mut);
//     }
// }

impl<'w, W: Widget> WidgetMut<'w, W> {
    pub fn unwrap(self) -> masonry::widget::WidgetMut<'w, W> {
        self.masonry_mut
    }

    // TODO - Replace with individual methods from WidgetState
    /// Get the [`WidgetState`] of the current widget.
    pub fn state(&self) -> &WidgetState {
        self.masonry_mut.state()
    }

    /// Get a `WidgetMut` for the same underlying widget with a shorter lifetime.
    pub fn reborrow_mut(&mut self) -> WidgetMut<'_, W> {
        WidgetMut {
            masonry_mut: self.masonry_mut.reborrow_mut(),
        }
    }
}

impl<'a> WidgetMut<'a, Box<dyn Widget>> {
    /// Attempt to downcast to `WidgetMut` of concrete Widget type.
    pub fn try_downcast<W2: Widget>(&mut self) -> Option<WidgetMut<'_, W2>> {
        self.masonry_mut.try_downcast().map(|masonry_mut| {
            WidgetMut {
                masonry_mut
            }
        })
    }

    /// Downcasts to `WidgetMut` of concrete Widget type.
    ///
    /// ## Panics
    ///
    /// Panics if the downcast fails, with an error message that shows the
    /// discrepancy between the expected and actual types.
    pub fn downcast<W2: Widget>(&mut self) -> WidgetMut<'_, W2> {
        WidgetMut {
            masonry_mut: self.masonry_mut.downcast(),
        }
    }
}

// TODO - unit tests
