
use masonry::core::{FromDynWidget, Widget};
use masonry::kurbo::Affine;

// TODO - Document extension trait workaround.
// See https://xi.zulipchat.com/#narrow/stream/317477-masonry/topic/Thoughts.20on.20simplifying.20WidgetMut/near/436478885
/// A rich mutable reference to a [`Widget`].
///
/// In Masonry, widgets can't be mutated directly. All mutations go through a `WidgetMut`
/// wrapper. So, to change a label's text, you might call `WidgetMut<Label>::set_text()`.
/// This helps Masonry make sure that internal metadata is propagated after every widget
/// change.
///
/// You can create a `WidgetMut` from [`TestHarness`](crate::testing::TestHarness),
/// [`EventCtx`](crate::core::EventCtx), [`UpdateCtx`](crate::core::UpdateCtx) or from a parent
/// `WidgetMut` with [`MutateCtx`].
///
/// `WidgetMut` implements [`Deref`](std::ops::Deref) with `W::Mut` as target.
///
/// ## `WidgetMut` as a Receiver
///
/// Once the Receiver trait is stabilized, `WidgetMut` will implement it so that custom
/// widgets in downstream crates can use `WidgetMut` as the receiver for inherent methods.
pub struct WidgetMut<'a, W: Widget + ?Sized> {
    pub masonry_mut: masonry::core::WidgetMut<'a, W>
}

/// Convert masonry's WidgetMut into jihaz's.
pub trait WrapWidgetMut<'a, W: Widget> {
    /// Convert masonry's WidgetMut into jihaz's.
    fn wrap(self) -> WidgetMut<'a, W>;
}

impl<'a, W: Widget> WrapWidgetMut<'a, W> for masonry::core::WidgetMut<'a, W> {
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

impl<'a, W: Widget> WidgetMut<'a, W> {
    pub fn unwrap(self) -> masonry::core::WidgetMut<'a, W> {
        self.masonry_mut
    }
}

impl<W: Widget + ?Sized> WidgetMut<'_, W> {

    /// Get a `WidgetMut` for the same underlying widget with a shorter lifetime.
    pub fn reborrow_mut(&mut self) -> WidgetMut<'_, W> {
        WidgetMut {
            masonry_mut: self.masonry_mut.reborrow_mut(),
        }
    }

    /// Set the local transform of this widget.
    ///
    /// It behaves similarly as CSS transforms.
    pub fn set_transform(&mut self, transform: Affine) {
        self.masonry_mut.ctx.set_transform(transform);
    }

    /// Attempt to downcast to `WidgetMut` of concrete Widget type.
    pub fn try_downcast<W2: Widget + FromDynWidget + ?Sized>(
        &mut self,
    ) -> Option<WidgetMut<'_, W2>> {
        self.masonry_mut
            .try_downcast()
            .map(|masonry_mut| {
                WidgetMut { masonry_mut }
            })
    }

    /// Downcasts to `WidgetMut` of concrete Widget type.
    ///
    /// ## Panics
    ///
    /// Panics if the downcast fails, with an error message that shows the
    /// discrepancy between the expected and actual types.
    pub fn downcast<W2: Widget + FromDynWidget + ?Sized>(&mut self) -> WidgetMut<'_, W2> {
        WidgetMut {
            masonry_mut: self.masonry_mut.downcast() 
        }
    }
}

// TODO - unit tests
