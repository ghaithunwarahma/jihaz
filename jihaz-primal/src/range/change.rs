//! Events or changes that occur to a ranged span of a larger list of items.
//! 
//! Range2 represets a range of span of items of a larger list of items, 
//! we can call it object range.
//! 
//! The change requests are applied on the object range, 
//! and the final resulting range will represent the new object range.
//! 
//! A series of change operations can be applied to an object range. This object
//! range is called the original object range.
//! 
//! ChangeSessionSummary can be used by the list objects to synchronize their items following the occurence of changes
//! 
//! The original list object is the object that has yet to undergo the current series of change operations).
//! 
use std::cmp::Ordering::*;
use super::{IndexTranslation, NewRemained, Range2, RemainedRanges, ResolvedRemained};

/// A change on the list object, or on an object range within the object.
#[derive(Copy, Clone, Debug)]
pub enum Change {
    /// A shift change. If the range was a smaller span of a larger list of items, then
    /// a shift will move the range to the direction of the end or the start; which means the range will
    /// now cover different items of the greater list.
    /// 
    /// The shift will be either an increase (forward direction), or a decrease (backward direction) 
    /// by the length of the shift.
    /// 
    /// For a shift change, the length of the shift can be more the length of this range.
    /// 
    /// Warning: user should ensure that the length of the shift should not
    /// push the range at a point where index is less than zero,
    /// or end_index is larger than the end_index of the list object.
    Shift {
        len: usize,
        direction: ChangeDirection, 
    },
    Add {
        index: usize,
        len: usize,
    },
    Remove {
        index: usize,
        len: usize,
    },
}

impl Change {
    pub const fn shift(len: usize, direction: ChangeDirection) -> Self {
        Self::Shift { len, direction }
    }

    pub const fn add(index: usize, len: usize) -> Self {
        Self::Add { index, len }
    }

    pub const fn remove(index: usize, len: usize) -> Self {
        Self::Remove { index, len }
    }
}

/// The kind of the change
#[derive(Copy, Clone)]
pub enum ChangeKind {
    /// A shift change. If the range was a smaller span of a larger list of items, then
    /// a shift will move the range to the direction of the end or the start; which means the range will
    /// now cover different items of the greater list.
    /// 
    /// The shift will be either an increase (forward direction), or a decrease (backward direction) 
    /// by the length of the shift.
    /// 
    /// For a shift change, the length of the shift can be more the length of this range.
    Shift,
    Add,
    Remove,
}

// The direction of the change that happened to the range of elements.
#[derive(Copy, Clone, Debug)]
pub enum ChangeDirection {
    /// To the end. Start to end forward direction.
    /// 
    /// In case of a shift in the forward direction:
    /// 
    /// The additions will be at the end of the list items, and the removals will be from the start
    End,
    /// To the start. End to start reverse/backward direction.
    /// 
    /// In case of a shift in the reverse direction:
    /// 
    /// The additions will be at the start of the list items, and the removals will be from the end
    Start,
}

// /// A change to the items of the range
// #[derive(Copy, Clone)]
// pub struct ChangeRequest {
//     /// The length of this change.
//     pub len: usize,
//     pub direction: ChangeDirection,
//     pub kind: ChangeKind,
// }

// impl ChangeRequest {
//     pub fn new(len: usize, direction: ChangeDirection, kind: ChangeKind) -> Self {
//     // pub fn new(direction: ChangeDirection, kind: ChangeKind) -> Self {
//         Self { len, direction, kind }
//         // Self { direction, kind }
//     }
// }

/// A series of change requests to be applied consecutively on a list object.
/// 
/// The list object that is yet to have these changes applied to is called the original list object.
#[derive(Clone, Debug)]
pub struct ChangeRequests {
    pub requests: Vec<Change>,
}

impl ChangeRequests {
    pub fn apply_changes(&self, range: &mut Range2, summary: &mut ChangeSessionSummary) {
        for change in self.requests.iter() {
            range.change_cont(change, summary);
            eprintln!("Change and resulting range {:#?} {:#?}", change, range);
        }
    }
}

impl From<Change> for ChangeRequests {
    fn from(request: Change) -> Self {
        Self { requests: vec![request] }
    }
}

impl From<&[Change]> for ChangeRequests {
    fn from(requests: &[Change]) -> Self {
        Self { requests: requests.into() }
    }
}

impl From<Vec<Change>> for ChangeRequests {
    fn from(requests: Vec<Change>) -> Self {
        Self { requests }
    }
}

/// Information about the change event that occured on the object range.
#[derive(Copy, Clone, Debug)]
pub enum ChangeInfo {
    /// An add event.
    /// 
    /// The index represents the position of this addition,
    /// and the len represents the length of the added items.
    Add(Range2),
    /// A remove event.
    /// 
    /// The index represents the start of the items that were removed
    /// in this object range, and the length is the length of the items removed.
    Remove(Range2),
}

/// Summary of the change(s) that occured in a change session. Containing the sorted change events that occured in this object range,
/// as well as the remained spans of elements that were not affected by the changes.
/// 
/// These are used to mainain synchronisation.
#[derive(Clone, Debug)]
pub struct ChangeSessionSummary {

    /// Smaller, remained ranges of items, which represents items that were not affected by the changes applied.
    /// 
    /// Allows for reuse of unchanged items.
    /// 
    /// Note that the indexes are always based on the original range,
    /// i.e. before applying the requested changes.
    pub remained: RemainedRanges,

    /// The sorted list of events that occured on the object range.
    pub events: Vec<ChangeInfo>,
}

impl ChangeSessionSummary {
    /// Create a new empty ChangeSessionSummary
    pub fn new() -> Self {
        Self { remained: RemainedRanges::new(), events: Vec::new()  }
    }

    pub fn remained(&self) -> &Vec<ResolvedRemained> {
        &self.remained.remained
    }

    pub fn events(&self) -> &Vec<ChangeInfo> {
        &self.events
    }
}

/// A trait to handle change that occurs in a list object, were Self is an object range
/// in this object.
/// 
/// The summary type can record information about the change results, mostly to
/// help manange the synchronisation of the list object before and after 
/// the change request is applied.
pub trait RangeChange: RangeChangeCont {
    
    /// An addition change
    /// 
    /// Transforms the length and index of this range based on the addition operation applied
    fn add(&mut self, addition_range: &Self) -> Self::Summary;
    
    /// A removal change
    /// 
    /// Transforms the length and index of this range based on the removal operation applied.
    /// 
    /// Returns the range(s) that did not undergo removal, in the original indexes prior to removal.
    /// 
    /// If there are no remaining range(s), the range(s) returned will be empty.
    fn remove(&mut self, removal_range: &Self) -> Self::Summary;
    
    /// A change that occurs at the edge
    fn edge_change(
        &mut self,
        kind: ChangeKind,
        side_direction: ChangeDirection,
        change_len: usize,
    ) -> Self::Summary;
    
    /// A general change
    fn change(&mut self, change: &Change) -> Self::Summary;
}

/// A trait to handle a continuous series of changes that occur in a list object, were Self is a range
/// in this object.
/// 
/// 
/// The summary type can record information about the change results, mostly to
/// help manange the synchronisation of the list object before and after 
/// the change requests are applied.
pub trait RangeChangeCont {
    
    /// A summary type
    type Summary;
    
    /// An addition change, as part of a continuous series of changes
    /// 
    /// Transforms the length and index of this range based on the addition operation applied
    fn add_cont(&mut self, addition_range: &Self, summary: &mut Self::Summary);
    
    /// A removal change, as part of a continuous series of changes
    /// 
    /// Transforms the length and index of this range based on the removal operation applied.
    /// 
    /// Returns the range(s) that did not undergo removal, in the original indexes prior to removal.
    /// 
    /// If there are no remaining range(s), the range(s) returned will be empty.
    fn remove_cont(&mut self, removal_range: &Self, summary: &mut Self::Summary);
    
    /// A change that occurs at the edge, as part of a continuous series of changes
    fn edge_change_cont(
        &mut self,
        kind: ChangeKind,
        side_direction: ChangeDirection,
        change_len: usize,
        summary: &mut Self::Summary,
    );
    
    /// A general change, as part of a continuous series of changes
    fn change_cont(&mut self, change: &Change, summary: &mut Self::Summary);
}

impl RangeChange for Range2 {
    fn add(&mut self, addition_range: &Self) -> Self::Summary {
        let mut sum = Self::Summary::new();
        self.add_cont(addition_range, &mut sum);
        sum
    }

    fn remove(&mut self, removal_range: &Self) -> Self::Summary {
        let mut sum = Self::Summary::new();
        self.remove_cont(removal_range, &mut sum);
        sum
    }

    fn edge_change(
        &mut self,
        kind: ChangeKind,
        side_direction: ChangeDirection,
        change_len: usize,
    ) -> Self::Summary {
        let mut sum = Self::Summary::new();
        self.edge_change_cont(kind, side_direction, change_len, &mut sum);
        sum
    }
    
    fn change(&mut self, change: &Change) -> Self::Summary {
        let mut sum = Self::Summary::new();
        self.change_cont(change, &mut sum);
        sum
    }
}

impl RangeChangeCont for Range2 {
    type Summary = ChangeSessionSummary;

    fn add_cont(&mut self, addition_range: &Self, summary: &mut Self::Summary) {

        let mut new_remained_ranges = Vec::new();
        match self.index().cmp(&addition_range.index()) {

            // Note that X is the addition index
            //
            // | _ _ _ + + + + + _ _ _
            // |           X
            // or
            // | _ _ _ + + + + + _ _ _
            // |               X
            // or
            // | _ _ _ + + + + + _ _ _
            // |                 X

            Less => match addition_range.index().cmp(&self.end_index()) {

                // | _ _ _ + + + + + _ _ _
                // |           X

                Less => {
                    
                    // the remaining from the start
                    let remained_range_before_change = self.index()..addition_range.index();
                    
                    new_remained_ranges.push(NewRemained::new(
                        remained_range_before_change,
                        IndexTranslation::Zero,
                    ));
                    
                    // the remaining from the end
                    let remained_range_before_change = addition_range.index()..self.end_index();
                    
                    new_remained_ranges.push(NewRemained::new(
                        remained_range_before_change,
                        // The index of the remained range after the change is shifted to the 
                        // end direction by the length of the added elements
                        IndexTranslation::Positive(addition_range.len())
                    ));
                    
                    // new self
                    let new_len = self.len() + addition_range.len();
                    *self = (self.index()..(self.index() + new_len)).into();
                }

                // | _ _ _ + + + + + _ _ _
                // |               X

                Equal => (),

                // | _ _ _ + + + + + _ _ _
                // |                 X

                Greater => (),
            }

            // | _ _ _ + + + + + _ _ _
            // |       X

            Equal => {

                // everything remained
                let remained_range_before_change = self.index()..self.end_index();
                
                new_remained_ranges.push(NewRemained::new(
                    remained_range_before_change,
                    // The index of the remained range after the change is shifted to the 
                    // end direction by the length of the added elements
                    IndexTranslation::Positive(addition_range.len())
                ));
                
                // new self
                let new_len = self.len() + addition_range.len();
                *self = (self.index()..(self.index() + new_len)).into();
            }

            // | _ _ _ + + + + + _ _ _
            // |     X

            Greater => {

                // everything remained
                let remained_range_before_change = self.index()..self.end_index();
                
                new_remained_ranges.push(NewRemained::new(
                    remained_range_before_change,
                    // The index of the remained range after the change is shifted to the 
                    // end direction by the length of the added elements
                    IndexTranslation::Positive(addition_range.len())
                ));

                // new self
                let new_index = self.index() + addition_range.len();
                *self = (new_index..self.len()).into();
            }
        }
        summary.remained.apply_new_remained(new_remained_ranges);
    }

    fn remove_cont(&mut self, removal_range: &Self, summary: &mut Self::Summary) {
        
        let mut new_remained_ranges = Vec::new();
        
        // the remaining calculations take into the account the
        // initial indexes, before applying the removal

        match self.index().cmp(&removal_range.index()) {

            // | _ _ _ + + + + + _ _ _
            // |           X + +
            // or
            // | _ _ _ + + + + + _ _ _
            // |               X + +
            // or
            // | _ _ _ + + + + + _ _ _
            // |                 X + +

            Less => match removal_range.index().cmp(&self.end_index()) {

                // | _ _ _ + + + + + _ _ _
                // |           X + + +
                // or
                // | _ _ _ + + + + + _ _ _
                // |           X + +
                // or 
                // | _ _ _ + + + + + _ _ _
                // |           X +

                Less => match self.end_index().cmp(&removal_range.end_index()) {

                    // | _ _ _ + + + + + _ _ _
                    // |           X + + +

                    Less => {

                        // the remaining from the start
                        let remained_range_before_change = self.index()..removal_range.index();
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            IndexTranslation::Zero,
                        ));
                        
                        // new self
                        *self = (self.index()..removal_range.index()).into();
                    }

                    // | _ _ _ + + + + + _ _ _
                    // |           X + +

                    Equal => {

                        // the remaining from the start
                        let remained_range_before_change = self.index()..removal_range.index();
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            IndexTranslation::Zero,
                        ));
                        
                        // new self
                        *self = (self.index()..removal_range.index()).into();
                    }

                    // | _ _ _ + + + + + _ _ _
                    // |           X +

                    Greater => {

                        // the remaining from the start
                        let remained_range_before_change = self.index()..removal_range.index();
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            IndexTranslation::Zero,
                        ));
                        
                        // the remaining from the end
                        let remained_range_before_change = removal_range.end_index()..self.end_index();
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            // The index of the remained range after the change is shifted to the 
                            // start direction by the length of the removed elements
                            IndexTranslation::Negative(removal_range.len())
                        ));
                        
                        // new self
                        let new_len = self.len() - removal_range.len();
                        *self = (self.index()..(self.index() + new_len)).into();
                    }
                }

                // | _ _ _ + + + + + _ _ _
                // |               X + +

                Equal => (),

                // | _ _ _ + + + + + _ _ _
                // |                 X + +

                Greater => (),
            }

            // | _ _ _ + + + + + _ _ _
            // |       X + +
            // or
            // | _ _ _ + + + + + _ _ _
            // |       X + + + +
            // or
            // | _ _ _ + + + + + _ _ _
            // |       X + + + + + +

            Equal => match self.end_index().cmp(&removal_range.end_index()) {

                // | _ _ _ + + + + + _ _ _
                // |       X + + + + + +
                
                Less => {
                    // new self
                    self.set_caret(self.index());
                }

                // | _ _ _ + + + + + _ _ _
                // |       X + + + +
                
                Equal => {
                    // new self
                    self.set_caret(self.index());
                }

                // | _ _ _ + + + + + _ _ _
                // |       X + +
                
                Greater => {
                    
                    // the remaining from the end
                    let remained_range_before_change = removal_range.end_index()..self.end_index();
                    
                    new_remained_ranges.push(NewRemained::new(
                        remained_range_before_change,
                        // The index of the remained range after the change is shifted to the 
                        // start direction by the length of the removed elements
                        IndexTranslation::Negative(removal_range.len())
                    ));

                    // new self
                    let new_len = self.len() - removal_range.len();
                    *self = (self.index()..(self.index() + new_len)).into();
                }
            }

            // | _ _ _ + + + + + _ _ _
            // |     X + +
            // or
            // | _ _ _ + + + + + _ _ _
            // |     X + + + +
            // or
            // | _ _ _ + + + + + _ _ _
            // |     X + + + + + +

            Greater => match self.end_index().cmp(&removal_range.end_index()) {

                // | _ _ _ + + + + + _ _ _
                // |     X + + + + + +

                Less => {
                    // new self
                    self.set_caret(removal_range.index());
                }

                // | _ _ _ + + + + + _ _ _
                // |     X + + + + +

                Equal => {
                    // new self
                    self.set_caret(removal_range.index());
                }

                // | _ _ _ + + + + + _ _ _
                // |     X + +
                
                Greater => match self.index().cmp(&removal_range.end_index()) {

                    // | _ _ _ _ + + + + + _ _ _
                    // |       X + +.

                    Less => {
                        
                        // the remaining from the end
                        // notice this start index means that the range does not include any removed element
                        let remained_range_before_change = removal_range.end_index()..self.end_index();
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            // The index of the remained range after the change is shifted to the 
                            // start direction by the length of the removed elements
                            IndexTranslation::Negative(removal_range.len())
                        ));

                        // new self
                    
                        // this is tricky: the index here reverts to the outer removal index,
                        // but the length will only be reduced by the length of elements removed in self only.

                        // the new index for the range will be the start index of the removal range
                        let new_index = removal_range.index();
                        
                        // the new length for the range, will be the items that were not removed,
                        // so it will be
                        let new_len = self.end_index() - removal_range.end_index();

                        *self = (new_index..new_index + new_len).into();
                    }

                    // | _ _ _ _ + + + + + _ _ _
                    // |     X + +.

                    Equal => {

                        // everything remained
                        // notice this start index means that the range does not include any removed element
                        let remained_range_before_change = removal_range.end_index()..self.end_index();
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            // The index of the remained range after the change is shifted to the 
                            // start direction by the length of the removed elements
                            IndexTranslation::Negative(removal_range.len())
                        ));

                        // new self
                    
                        // the index here reverts to the outer removal index,
                        // but the length will not be reduced as the removal did not cover elements in self.

                        self.set_index(self.index() - removal_range.len());
                    }

                    // | _ _ _ _ + + + + + _ _ _
                    // |   X + +.

                    Greater => {
                        
                        // everything remained
                        let remained_range_before_change = self.index()..self.end_index();

                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            // The index of the remained range after the change is shifted to the 
                            // start direction by the length of the removed elements
                            IndexTranslation::Negative(removal_range.len())
                        ));

                        // new self
                    
                        // the index here reverts to the outer removal index,
                        // but the length will not be reduced as the removal did not cover elements in self.
                
                        self.set_index(self.index() - removal_range.len());
                    }
                }
            }
        }
        summary.remained.apply_new_remained(new_remained_ranges);
    }

    fn edge_change_cont(
        &mut self,
        kind: ChangeKind,
        side_direction: ChangeDirection,
        change_len: usize,
        summary: &mut Self::Summary,
    ) {
        let index = match side_direction {
            ChangeDirection::End => self.end_index() - change_len,
            ChangeDirection::Start => 0,
        };
        let change = match kind {
            ChangeKind::Shift => Change::shift(change_len, side_direction),
            ChangeKind::Add => Change::add(index, change_len),
            ChangeKind::Remove => Change::remove(index, change_len),
        };
        self.change_cont(&change, summary)
    }

    fn change_cont(&mut self, change: &Change, summary: &mut Self::Summary) {

        match change {

            Change::Shift { len, direction } => match direction {

                // . . . * * * * . . . . .
                // . . . . . * * * * . . .

                ChangeDirection::End => {
                    
                    let mut new_remained_ranges = Vec::new();
                    
                    if self.len() > *len {
                        
                        // the remaining from the end
                        let remained_len = self.len() - *len;
                        let remained_range_before_change = Range2::new(self.index() + *len, remained_len);
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            // The index of the remained range after the change is shifted to the 
                            // start direction by the length of the removed elements
                            IndexTranslation::Zero
                        ));
                    }

                    // we shift the range towards the end (causing an increase in the indexes)
                    // so the removal happens at the start of the range

                    // the shifted self
                    self.set_index(self.index() + *len);
                    
                    summary.remained.apply_new_remained(new_remained_ranges);
                }

                // . . . . . * * * * . . .
                // . . . * * * * . . . . .

                ChangeDirection::Start => {
                    
                    let mut new_remained_ranges = Vec::new();
                    
                    if self.len() > *len {
                        
                        // the remaining from the end
                        let remained_len = self.len() - *len;
                        let remained_range_before_change = Range2::new(self.index(), remained_len);
                        
                        new_remained_ranges.push(NewRemained::new(
                            remained_range_before_change,
                            // The index of the remained range after the change is shifted to the 
                            // start direction by the length of the removed elements
                            IndexTranslation::Zero
                        ));
                    }

                    // we shift the range towards the start (causing a decrease in the indexes), 
                    // so the removal happens at the end of the range
        
                    // the shifted self
                    self.set_index(self.index() - *len);
                    
                    summary.remained.apply_new_remained(new_remained_ranges);
                }
            }

            Change::Add { index, len } => {
                let addition_range = Range2::new(*index, *len);
                self.add_cont(&addition_range, summary);
            }

            Change::Remove { index, len } => {
                let removal_range = Range2::new(*index, *len);
                self.remove_cont(&removal_range, summary);
            }

        }
    }
}