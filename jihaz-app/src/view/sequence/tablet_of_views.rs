use std::sync::atomic::{AtomicBool, Ordering};

use jihaz::range::Range2;
// using smallvec with const_generics feature
use smallvec::SmallVec;
use xilem_core::{
    AppendVec, ElementSplice, MessageResult, ViewElement, ViewId, ViewPathTracker, ViewSequence
};

/// A change to the views of the tablet of views
pub struct Change {
    // /// The length of the change. i.e. how many views in the tablet.
    // change_len: usize,
    direction: ChangeDirection,
    kind: ChangeKind,
}

impl Change {
    // pub fn new(change_len: usize, direction: ChangeDirection, kind: ChangeKind) -> Self {
    pub fn new(direction: ChangeDirection, kind: ChangeKind) -> Self {
        // Self { change_len, direction, kind }
        Self { direction, kind }
    }
}

/// The change that happened to the views of the tablet, from the prev Tablet to the new, if any.
pub enum ChangeKind {
    /// The change of indexes covered by this tablet.
    /// 
    /// Either increased (forward direction), or decreased (backward direction) 
    /// by the length of the shift.
    ///
    /// In other words, the tablet now covers views that are to the end, or to the start, of the overall possible views.
    /// 
    /// For a shift change, the length of the shift can be more the length of the tablet.
    Shift,
    Addition,
    Removal,
    /// In the case of a newly built tablet
    None,
}

// The direction of the change that happened to the views of the tablet
pub enum ChangeDirection {
    /// To the end. Start to end forward direction.
    /// 
    /// In case of a shift in the forward direction:
    /// 
    /// The additions will be at the end of the tablet elements vector, and the deletions will be from the start
    End,
    /// To the start. End to start reverse/backward direction.
    /// 
    /// In case of a shift in the reverse direction:
    /// 
    /// The additions will be at the start of the tablet elements vector, and the deletions will be from the end
    Start,
}

pub struct TabletOfViews<V, const N: usize> {
    views: SmallVec<[V; N]>,
    change: Change,
}

/// The remainder, removal, and addition instructions to bring
/// the elements from the previous view state to the new view state.
/// 
/// These indexes are not absolute, but are relative to the Tablet views.
pub struct RebuildInformation {
    remained: Range2,
    removed: Range2,
    added: Range2,
}

impl<V, const N: usize> TabletOfViews<V, N> {
    // /// The actual length of the tablet views.
    // /// 
    // /// For example, a tablet can act as a two page view of a book; in which case, it
    // /// can view a single page or two pages. 
    // /// 
    // /// 
    // /// We have this because the N length may not be fully used, as N is currently the maximum possible size.
    // fn effective_len(&self) -> usize {
    //     self.views.iter().filter(|v| v.is_some()).count()
    // }

    fn len(&self) -> usize {
        self.views.len()
    }

    fn rebuild_information(&self, prev: &TabletOfViews<V, N>) -> RebuildInformation {
        let mut current = Range2::new(0, prev.len());
        // we remove, then add, and then handle the remained.
        let mut removed = Range2::NONE;
        let mut added = Range2::NONE;
        let mut remained = Range2::NONE;

        let change = &self.change;
        let change_length = (self.views.len() as isize - prev.views.len() as isize).abs() as usize;

        match (&change.kind, &change.direction) {
            (ChangeKind::Shift, ChangeDirection::End) => {

                // we go towards the end, so the removal happens at the start side
                removed.set(0, change_length);

                // so the remained, if any, will be at the end side
                (_, remained) = current.apply_removal(&removed);

                added.set(current.len, change_length);
            }
            (ChangeKind::Shift, ChangeDirection::Start) => {

                // we go towards the start, so the removal happens at the end side
                removed.set_rev(change_length, prev.len() - change_length);

                // so the remained, if any, will be at the start side
                (remained, _) = current.apply_removal(&removed);

                added.set(0, change_length);
            }
            (ChangeKind::Addition, ChangeDirection::End) => {

                remained.set(0, prev.len());
                added.set(prev.len(), change_length);
            }
            (ChangeKind::Addition, ChangeDirection::Start) => {

                remained.set(0, prev.len());
                added.set(0, change_length);
            }
            (ChangeKind::Removal, ChangeDirection::End) => {

                // removal happening at the end side, 
                removed.set_rev(change_length, prev.len());
                
                // so the remained, if any, will be at the start side
                (remained, _) = current.apply_removal(&removed);
            }
            (ChangeKind::Removal, ChangeDirection::Start) => {

                // removal happening at the start side
                removed.set(0, change_length);

                // so the remained, if any, will be at the end side
                (_, remained) = current.apply_removal(&removed);
            }
            (ChangeKind::None, _) => unreachable!(),
        }
        RebuildInformation { added, remained, removed }
    }
}

/// The state used to implement `ViewSequence` for `TabletOfViews<impl ViewSequence, const N>`
///
/// We use a generation arena for vector types, with half of the `ViewId` dedicated
/// to the index, and the other half used for the generation.
///
// This is managed in [`create_vector_view_id`] and [`view_id_to_index_generation`]
#[doc(hidden)] // Implementation detail, public because of trait visibility rules
pub struct TabletOfViewsState<VS, const N: usize> {
    states_of_views: SmallVec<[VS; N]>,
    generations: SmallVec<[u32; N]>,
}

/// The implementation for an `Vec` of a `ViewSequence`.
///
/// Will mark messages which were sent to any index as stale if
/// that index has been unused in the meantime.
impl<State, Action, Context, Element, Seq, Message, const N: usize>
    ViewSequence<State, Action, Context, Element, Message> for TabletOfViews<Seq, N>
where
    Seq: ViewSequence<State, Action, Context, Element, Message>,
    Context: ViewPathTracker,
    Element: ViewElement,
{
    // We hide all the items in these implementation so that the top-level
    // comment is always shown. This lets us explain the caveats.
    #[doc(hidden)]
    type SeqState = TabletOfViewsState<Seq::SeqState, N>;

    #[doc(hidden)]
    fn seq_build(&self, ctx: &mut Context, elements: &mut AppendVec<Element>) -> Self::SeqState {

        let mut generations = SmallVec::new();
        generations.extend(vec![0; self.len()].into_iter());

        let states_of_views = self
            .views
            .iter()
            .enumerate()
            .zip(&generations)
            .map(|((index, seq), generation)| {
                let id = create_generational_view_id(index, *generation);
                ctx.with_id(id, |ctx| seq.seq_build(ctx, elements))
            })
            .collect();
        TabletOfViewsState {
            generations,
            states_of_views,
        }
    }

    #[doc(hidden)]
    fn seq_rebuild(
        &self,
        prev: &Self,
        seq_state: &mut Self::SeqState,
        ctx: &mut Context,
        elements: &mut impl ElementSplice<Element>,
    ) {
        let RebuildInformation { 
            remained, removed, added 
        } = self.rebuild_information(prev);

        if remained.is_some() {
            for (i, (((child, child_prev), child_state), child_generation)) in self
                .views[remained.range()]
                .iter()
                .zip(&prev.views[remained.range()])
                .zip(&mut seq_state.states_of_views[remained.range()])
                .zip(&seq_state.generations[remained.range()])
                .enumerate()
            {
                let i = i + remained.index;
    
                // Rebuild the items which are common to both vectors
                let id = create_generational_view_id(i, *child_generation);
                ctx.with_id(id, |ctx| {
                    child.seq_rebuild(child_prev, child_state, ctx, elements);
                });
            }
        }
        if removed.is_some() {
            let to_teardown = prev.views[removed.range()].iter();
            // Keep the generations
            let generations = seq_state.generations[removed.range()].iter_mut();
            // But remove the old states
            let states = seq_state.states_of_views.drain(removed.range());
            for (index, ((old_seq, generation), mut inner_state)) in
                to_teardown.zip(generations).zip(states).enumerate()
            {
                let index = index + removed.index;

                let id = create_generational_view_id(index, *generation);
                ctx.with_id(id, |ctx| {
                    old_seq.seq_teardown(&mut inner_state, ctx, elements);
                });
                // We increment the generation on the "falling edge" by convention
                *generation = generation.checked_add(1).unwrap_or_else(|| {
                    static SHOULD_WARN: AtomicBool = AtomicBool::new(true);
                    // We only want to warn about this once
                    // because e.g. if every item in a vector hits
                    // this at the same time, we don't want to repeat it too many times
                    if SHOULD_WARN.swap(false, Ordering::Relaxed) {
                        tracing::warn!(
                            inner_type = core::any::type_name::<Seq>(),
                            issue_url = "https://github.com/linebender/xilem/issues",
                            "Got overflowing generation in ViewSequence from `Vec<inner_type>`.\
                            This can possibly cause incorrect routing of async messages in extreme cases.\
                            Please open an issue if you see this. There are known solutions"
                        );
                    }
                    // The known solution mentioned in the above message is to use a different ViewId for the index and the generation
                    // We believe this to be superfluous for the default use case, as even with 1000 rebuilds a second, each adding
                    // to the same array, this would take 50 days of the application running continuously.
                    // See also https://github.com/bevyengine/bevy/pull/9907, where they warn in their equivalent case
                    // Note that we have a slightly different strategy to Bevy, where we use a global generation
                    // This theoretically allows some of the memory in `seq_state` to be reclaimed, at the cost of making overflow
                    // more likely here. Note that we don't actually reclaim this memory at the moment.

                    // We use 0 to wrap around. It would require extremely unfortunate timing to get an async event
                    // with the correct generation exactly u32::MAX generations late, so wrapping is the best option
                    0
                });
            }
        }
        if added.is_some() {
            // If needed, create new generations
            seq_state.generations.resize(added.len, 0);
            elements.with_scratch(|elements| {
                seq_state.states_of_views.extend(
                    self.views[added.range()]
                        .iter()
                        .zip(&seq_state.generations[added.range()])
                        .enumerate()
                        .map(|(index, (seq, generation))| {

                            let index = index + removed.index;

                            let id = create_generational_view_id(index, *generation);
                            ctx.with_id(id, |ctx| seq.seq_build(ctx, elements))
                        }),
                );
            });
        }
    }

    #[doc(hidden)]
    fn seq_teardown(
        &self,
        seq_state: &mut Self::SeqState,
        ctx: &mut Context,
        elements: &mut impl ElementSplice<Element>,
    ) {
        for (index, ((seq, state), generation)) in self
            .views
            .iter()
            .zip(&mut seq_state.states_of_views)
            .zip(&seq_state.generations)
            .enumerate()
        {
            let id = create_generational_view_id(index, *generation);
            ctx.with_id(id, |ctx| seq.seq_teardown(state, ctx, elements));
        }
    }

    #[doc(hidden)]
    fn seq_message(
        &self,
        seq_state: &mut Self::SeqState,
        id_path: &[ViewId],
        message: Message,
        app_state: &mut State,
    ) -> MessageResult<Action, Message> {
        let (start, rest) = id_path
            .split_first()
            .expect("Id path has elements for Vec<ViewSequence>");
        let (index, generation) = view_id_to_index_generation(*start);
        let stored_generation = &seq_state.generations[index];
        if *stored_generation != generation {
            // The value in the sequence i
            return MessageResult::Stale(message);
        }
        // Panics if index is out of bounds, but we know it isn't because this is the same generation
        let inner_state = &mut seq_state.states_of_views[index];
        self.views[index].seq_message(inner_state, rest, message, app_state)
    }
}

/// Turns an index and a generation into a packed id, suitable for use in
/// [`ViewId`]s
fn create_generational_view_id(index: usize, generation: u32) -> ViewId {
    let id_low: u32 = index
        .try_into()
        // If you're seeing this panic, you can use a nested `Vec<Vec<...>>`, where each individual `Vec`
        // has fewer than u32::MAX-1 elements.
        .expect("Views in a vector backed sequence must be indexable by u32");
    let id_low: u64 = id_low.into();
    let id_high: u64 = u64::from(generation) << 32;
    ViewId::new(id_high | id_low)
}

/// Undoes [`create_vector_view_id`]
fn view_id_to_index_generation(view_id: ViewId) -> (usize, u32) {
    #![allow(clippy::cast_possible_truncation)]
    let view_id = view_id.routing_id();
    let id_low_ix = view_id as u32;
    let id_high_gen = (view_id >> 32) as u32;
    (id_low_ix as usize, id_high_gen)
}