//! Convencience builder functions for some commonly used views

use masonry::peniko::Brush;
use xilem::{view::{flex, portal, sized_box, Axis, Flex, FlexSequence, MainAxisAlignment, Portal, SizedBox}, WidgetView};

/// Builds a flex row with start main axis alignment
pub fn flex_row<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex(sequence).direction(Axis::Horizontal)
}

/// Builds a flex row with end main axis alignment
pub fn flex_row_end_aligned<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex_row(sequence).main_axis_alignment(MainAxisAlignment::End)
}

/// Builds a flex row with center main axis alignment
pub fn flex_row_center_aligned<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex_row(sequence).main_axis_alignment(MainAxisAlignment::Center)
}

/// Builds a flex row that fills major axis, with start main axis alignment
pub fn flex_row_consumed_fully<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex_row(sequence).must_fill_major_axis(true)
}

/// Builds a flex row that fills major axis, with end main axis alignment
//
// Does this make logical sense?
pub fn flex_row_consumed_fully_end_aligned<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex_row_end_aligned(sequence).must_fill_major_axis(true)
}

/// Builds a flex column with start main axis alignment
pub fn flex_col<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex(sequence)
}

/// Builds a flex column that fills major axis, with start main axis alignment
pub fn flex_col_consumed_fully<State, Action, Seq: FlexSequence<State, Action>>(
    sequence: Seq,
) -> Flex<Seq, State, Action> {
    flex_col(sequence).must_fill_major_axis(true)
}

/// A view that puts child into a scrollable region, with the child set to fully consume
/// the size of the parent.
///
/// If `false` (the default) there is no minimum constraint on the child's
/// size. If `true`, the child is passed the same minimum constraints as
/// the `Portal`.
pub fn portal_consumed_fully<Child, State, Action>(child: Child) -> Portal<Child, State, Action>
where
    Child: WidgetView<State, Action> + 'static,
{
    portal(child).content_must_fill(true)
}

/// Expand the child's size to occupy the parent's size.
/// 
/// Here the container is expanded to fit the parent.
///
/// Only call this method if you want your widget to occupy all available
/// space. If you only care about expanding in one of width or height, use
/// [`expand_width`] or [`expand_height`] instead.
///
/// Sizedbox widget forces its child to have a specific width and/or height (assuming values are permitted by
/// this widget's parent). If either the width or height is not set, this widget will size itself
/// to match the child's size in that dimension.
pub fn expand_to_parent_size<State, Action, V>(inner: V) -> SizedBox<V, State, Action>
where
    V: WidgetView<State, Action>,
{
    sized_box(inner).expand()
}

/// Expand the child's width to occupy the parent's width.
/// 
/// This will force the child to have maximum width.
///
/// Only call this method if you want your widget to occupy all available
/// space. If you only care about expanding in one of width or height, use
/// [`expand_width`] or [`expand_height`] instead.
///
/// Sizedbox widget forces its child to have a specific width and/or height (assuming values are permitted by
/// this widget's parent). If either the width or height is not set, this widget will size itself
/// to match the child's size in that dimension.
pub fn expand_to_parent_width<State, Action, V>(inner: V) -> SizedBox<V, State, Action>
where
    V: WidgetView<State, Action>,
{
    sized_box(inner).expand_width()
}

/// Expand the child's height to occupy the parent's height.
/// 
/// This will force the child to have maximum height.
///
/// Only call this method if you want your widget to occupy all available
/// space. If you only care about expanding in one of width or height, use
/// [`expand_width`] or [`expand_height`] instead.
///
/// Sizedbox widget forces its child to have a specific width and/or height (assuming values are permitted by
/// this widget's parent). If either the width or height is not set, this widget will size itself
/// to match the child's size in that dimension.
pub fn expand_to_parent_height<State, Action, V>(inner: V) -> SizedBox<V, State, Action>
where
    V: WidgetView<State, Action>,
{
    sized_box(inner).expand_height()
}

/// Background setting sized box view
pub fn background<State, Action, V>(inner: V, brush: impl Into<Brush>) -> SizedBox<V, State, Action>
where
    V: WidgetView<State, Action>,
{
    sized_box(inner).background(brush)
}

// pub trait MoreViewsExt<State, Action>: WidgetView<State, Action> {
    
//     /// Background setting sized box view
//     fn background(self, brush: impl Into<Brush>) -> SizedBox<Self, State, Action>
//     where
//         State: 'static,
//         Action: 'static,
//         Self: Sized,
//     {
//         sized_box(self).background(brush)
//     }
// }