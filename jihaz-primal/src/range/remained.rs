//! When an object range undergoes a series of changes,
//! some of its elements remain intact and not removed.
//! A remained range represents these intact elements in the original object range.
//! 
//! An original object range is the range of the elements before a series of change operations
//! [SeriesOfChangeRequests] are applied.
//! 
//! After a change operation, the indexes and ranges in the list object
//! can be defined based to the time point at which there is exists a 
//! certain version of the list object.
//! 
//! 1 - Relative to the original list object (i.e. before any of the given change requsts take place).
//! 
//! 2 - Relative to the list object before the last change request takes place.
//! 
//! 3 - Relative to the list object after the last change request takes place.

use std::cmp::Ordering::*;
use super::{Range2, Set, SetSimilar, ToRange2};

/// The shift of an index in the list object, from time point A to time point B.
/// 
/// Used to translate between the state of the list object across time(i.e. across change operations).
pub enum IndexTranslation {
    /// Meaning that the index decreases when we go from the given list object at time A,
    /// to the target list object at time B.
    Negative(usize),
    
    /// Meaning that the index stays the same when we go from the given list object at time A,
    /// to the target list object at time B.
    Zero,
    
    /// Meaning that the index increases when we go from the given list object at time A,
    /// to the target list object at time B.
    Positive(usize),
}

impl IndexTranslation {
    /// Gets the index shift from given index to target index
    pub fn get(given_index: usize, target_index: usize) -> Self {
        match target_index.cmp(&given_index) {
            Less => IndexTranslation::Negative(given_index - target_index),
            Equal => IndexTranslation::Zero,
            Greater => IndexTranslation::Positive(target_index - given_index),
        }
    }

    /// Applies the shift of this index.
    /// 
    /// You should ensure that the shift actually belongs to this index. For example,
    /// if this shift is not meant for the index, you can have a case where the index is 3 and the shift
    /// is a negative shift, of value 5, and so that will result in an error as the 
    /// new index will be a negative value (3 - 5 = -2).
    pub fn apply_index_translation(&self, index: &mut usize) {
        match self {
            IndexTranslation::Negative(shift) => *index -= shift,
            IndexTranslation::Zero => (),
            IndexTranslation::Positive(shift) => *index += shift,
        }
    }

    /// Applies the shift of this index and returns the result.
    /// 
    /// You should ensure that the shift actually belongs to this index. For example,
    /// if this shift is not meant for the index, you can have a case where the index is 3 and the shift
    /// is a negative shift, of value 5, and so that will result in an error as the 
    /// new index will be a negative value (3 - 5 = -2).
    pub fn with_apply_index_translation(&self, mut index: usize) -> usize {
        self.apply_index_translation(&mut index);
        index
    }
}

// /// After a change operation, the indexes and ranges in the list object
// /// can be defined based to the time point at which there is exists a 
// /// certain version of the list object.
// /// 
// /// For example, for a range of length two, if it exists in all versions of the list object,
// /// but is positioned at different positions, these ranegs can be illustrated as follows:
// /// 
// /// Relative to the original list object before the requested series of changes:
// /// 
// /// _ _ _ _ _ * * * _ _
// /// 
// /// Relative to the list object right before the latest change:
// /// 
// /// _ _ _ * * * _ _
// /// 
// /// Relative to the list object right after the latest change:
// /// 
// /// _ _ _ _ * * * _ _ _ _ _
// /// 
// pub enum Relativeness {
//     /// Relative to the original list object (i.e. before any of the given change requsts take place).
//     Original,

//     /// Relative to the list object before the last change request takes place.
//     BeforeLatestChange,

//     /// Relative to the list object after the last change request takes place.
//     AfterLatestChange,
// }

// /// After a change operation, the indexes and ranges in the list object
// /// can be defined based to the time point at which there is exists a 
// /// certain version of the list object.
// /// 
// /// For example, for a range of length two, if it exists in all versions of the list object,
// /// but is positioned at different positions, these ranegs can be illustrated as follows:
// /// 
// /// Relative to the original list object before the requested series of changes:
// /// 
// /// _ _ _ _ _ * * * _ _
// /// 
// /// Relative to the list object right before the latest change:
// /// 
// /// _ _ _ * * * _ _
// /// 
// /// Relative to the list object right after the latest change:
// /// 
// /// _ _ _ _ * * * _ _ _ _ _
// /// 
// /// These structures are merely to increase code comprehensibility.
// pub mod relative_index {

//     /// Relative to the original list object before the requested series of changes.
//     pub struct Original(pub usize);

//     /// Relative to the list object right before the latest change.
//     pub struct BeforeLatestChange(pub usize);
    
//     /// Relative to the list object right after the latest change.
//     pub struct AfterLatestChange(pub usize);
// }

// /// After a change operation, the indexes and ranges in the list object
// /// can be defined based on the time state of the list object it relates to.
// /// 
// /// For example, for a range of length two, if it exists in all versions of the list object,
// /// but is positioned at different positions, these ranegs can be illustrated as follows:
// /// 
// /// Relative to the original list object before the requested series of changes:
// /// 
// /// _ _ _ _ _ * * * _ _
// /// 
// /// Relative to the list object right before the latest change:
// /// 
// /// _ _ _ * * * _ _
// /// 
// /// Relative to the list object right after the latest change:
// /// 
// /// _ _ _ _ * * * _ _ _ _ _
// /// 
// /// These structures are merely to increase code comprehensibility.
// pub mod relative_range {
//     use crate::range::Range2;

//     /// Relative to the original list object before the requested series of changes.
//     pub struct Original(pub Range2);

//     /// Relative to the list object right before the latest change.
//     pub struct BeforeLatestChange(pub Range2);
    
//     /// Relative to the list object right after the latest change.
//     pub struct AfterLatestChange(pub Range2);
// }

// /// An index of a certain state of the list object.
// pub struct RelativeIndex {
//     pub index: usize,
//     pub relativeness: Relativeness,
// }

/// Smaller, remained ranges of items, which represents items that were not affected by the changes applied.
/// 
/// Useful when we want to reuse unchanged items.
/// 
/// Note that the indexes are always based on the original range,
/// i.e. before applying the requested changes.
#[derive(Clone, Debug)]
pub struct RemainedRanges {
    pub remained: Vec<ResolvedRemained>,
    /// Indicates a previous apply_new_remained method call had
    /// no new remained ranges.
    /// 
    /// This meant that all of the elements in this range have
    /// been modified, and so this means any following change will not change
    /// the fact that the list object has had no elements remaining intact 
    /// and unmodified.
    pub no_unchanged_ranges: bool,
}

impl RemainedRanges {
    pub fn new() -> Self {
        Self { remained: Vec::new(), no_unchanged_ranges: false }
    }

    /// Apply new remained ranges. If the 
    /// Resolve a remained range following a new change. It resolves this remained range with the existing remained changes
    /// from the previous changes. The updated remained ranges will be smaller or at most equal to the new remained ranges.
    pub fn apply_new_remained(
        &mut self,
        mut new_remained: Vec<NewRemained>,
    ) {

        if self.no_unchanged_ranges {
            return;

        } else if new_remained.len() == 0 {
            // this means the new change has no remained ranges, 
            // so we set no unchaged ranges to true
            self.no_unchanged_ranges = true;
            return;
        }

        let mut total_up_to_date_resolved = Vec::new();

        // for new_remained in new_remained.iter() {
        //     eprintln!("Applying new remained [before {:?}] [after {:?}], everything changed {}",
        //     &new_remained.remained_before(), &new_remained.remained_after(), self.no_unchanged_ranges);
        // }

        if self.remained.len() == 0 {
            // prev remained will be zero and no_unchanged_range will be false only if
            // this is the first change with new remained ranges
            
            total_up_to_date_resolved = new_remained
                .drain(..)
                .map(|nr| ResolvedRemained {
                    length: nr.length,
                    index_original: nr.index_before,
                    index_current: nr.index_after,
                })
                .collect();

        } else {

            // Resolve by keeping only the ranges that remain across both previous changes and new change
            
            for prev_remained in &mut self.remained {
                for new_remained in new_remained.iter() {
    
                    if let Some(up_to_date_resolved) = prev_remained.resolve(new_remained) {
                        total_up_to_date_resolved.push(up_to_date_resolved);
                    }
                }
            }

            if total_up_to_date_resolved.len() == 0 {
                self.no_unchanged_ranges = true;
            }
        }

        let mut res = Vec::with_capacity(total_up_to_date_resolved.len());

        // Connect the intersecting new resolved remained ranges if any,
        // I don't know if this is logically plausible/possible to happen
        
        while let Some(mut up_to_date_resolved) = total_up_to_date_resolved.pop() {
            let mut next = total_up_to_date_resolved.last();
            while next.map(|n| up_to_date_resolved.combine(n)).unwrap_or(false) {
                next = total_up_to_date_resolved.last();
            }
            res.push(up_to_date_resolved);
        }

        let _ = std::mem::replace(&mut self.remained, res);
    }
}

/// Information about the new remained range resulting from the latest change operation.
#[derive(Debug)]
pub struct NewRemained {
    pub length: usize,

    /// The new remained range index relative to the list object before the last change operations.
    /// 
    /// Index before is the same as the index current of the ResolvedRemained range.
    pub index_before: usize,
    
    /// The new remained range index relative to the list object after the last change operations.
    pub index_after: usize,
}

impl NewRemained {

    /// Create a NewRemained range, following a change, from the raw fields.
    pub fn new_raw(index_before: usize, length: usize, index_after: usize) -> Self {
        Self { length, index_before, index_after }
    }

    /// Create a NewRemained range, following a change. From the new remained range before the change,
    /// and the shift of index from before to after the change.
    pub fn new(range_before_change: impl Into<Range2>, before_to_after_shift_for_index: IndexTranslation) -> Self {
        let range_before_change = range_before_change.into();
        Self {
            length: range_before_change.len(),
            index_before: range_before_change.index(),
            index_after: before_to_after_shift_for_index.with_apply_index_translation(range_before_change.index()),
        }
    }

    /// The new remained range relative to the list object before the last change operations.
    pub fn remained_before(&self) -> std::ops::Range<usize> {
        self.index_before..(self.index_before + self.length)
    }

    /// The new remained range relative to the list object after the last change operations.
    pub fn remained_after(&self) -> std::ops::Range<usize> {
        self.index_after..(self.index_after + self.length)
    }

    /// Get the shift in index for the new remained range, from the index in the list object after the change 
    /// to the index in the list object before the change.
    pub fn index_shift_after_to_before(&self) -> IndexTranslation {
        IndexTranslation::get(self.index_after, self.index_before)
    }

    /// Get the shift in index for the new remained range, from the index in the list object before the change 
    /// to the index in the list object after the change.
    pub fn index_shift_before_to_after(&self) -> IndexTranslation {
        IndexTranslation::get(self.index_before, self.index_after)
    }
}

/// A range that represents a span that has remained continuously intact despite change operations.
#[derive(Copy, Clone, Debug)]
pub struct ResolvedRemained {
    pub length: usize,

    /// The resolved remained range index relative to the original list object before undergoing change operations.
    pub index_original: usize,

    /// The resolved remained range index relative to the current list object that has undergone previous change operations.
    pub index_current: usize,
}

impl ResolvedRemained {

    /// The remained range relative to the original list object before undergoing change operations.
    pub fn remained_original(&self) -> std::ops::Range<usize> {
        let end_index = self.index_original + self.length;
        self.index_original..end_index
    }

    /// The remained range relative to the current list object that has undergone previous change operations.
    pub fn remained_current(&self) -> std::ops::Range<usize> {
        let end_index = self.index_current + self.length;
        self.index_current..end_index
    }

    /// Get the shift in index for the resolved remained range, from the index in the current list object (before the new change) 
    /// to the index in the original list object (before these series of changes have taken place).
    pub fn index_shift_current_to_original(&self) -> IndexTranslation {
        IndexTranslation::get(self.index_current, self.index_original)
    }

    /// Combine the two consecutive resolved remained ranges if they are connected.
    /// 
    /// Returns true if they are connected and combined.
    pub fn combine(&mut self, next: &Self) -> bool {
        match self.remained_current()
            .to_range2()
            .combine(&next.remained_current().to_range2())
        {
            Some(combined_current) => {
                self.index_original = self.index_original.min(next.index_original);
                self.index_current = combined_current.index();
                self.length = combined_current.len();
                true
            }
            None => false,
        }
    }

    // /// Produce the intersection range relative to the list object post the change.
    // /// 
    // /// Before and after change resolution entails shifting the intersection range post the change 
    // /// based on the shift that occured for the new remained range index during the change.
    // pub fn intersection_post_change(
    //     index_before: usize,
    //     index_after: usize,
    //     intersection_before_change: Range2
    // ) -> Range2 {

    //     // For example, say we have:
    //     //
    //     // Current remained is
    //     // . . + + + + + . . . . .

    //     // New remained relative to object range before the change is
    //     // . . . . - - - - - . . .
        
    //     // The intersection relative to object range before the change therefore is
    //     // . . . . ± ± ± . . . . .
        
    //     // But the new remained relative to object range after the change is moved as such
    //     // . . . . . . - - - - - .
        
    //     // so the intersection relative to object range after the change is therefore moved similarly, to become relative to the list object relative to object range after the change:
    //     // . . . . . . ± ± ± . . .

    //     // Remember, pre and post change remained ranges will always have the same length following a change operation,
    //     // they just may differ in the index (position) as a result of the change.
    //     //
    //     // For example, there may possibly be an addition before the new remained range that shifted it forward.
        
    //     match index_before.cmp(&index_after) {

    //         // Say the intersection relative to object range before the change is
    //         // . . . . . ± ± ± . . . .

    //         // And the new remained relative to object range before the change is
    //         // . . . . - - - - - . . .
            
    //         // And the new remained relative to object range after the change is
    //         // . . . . . . - - - - - .

    //         // so the new remained is pushed forward by 2,

    //         // so the intersection relative to object range after the change should also be pushed forward by 2:
    //         // . . . . . . . ± ± ± . .

    //         Less => {
    //             let pre_to_post_change_shift = index_after - index_before;

    //             let intersection_index_post_change = intersection_before_change
    //                 .index() + pre_to_post_change_shift;

    //             let intersection_end_index_post_change = intersection_index_post_change + intersection_before_change.len();

    //             (intersection_index_post_change..intersection_end_index_post_change).into()
    //         }
            
    //         // Say the intersection relative to object range before the change is
    //         // . . . . . ± ± ± . . . .

    //         // And the new remained relative to object range before the change is
    //         // . . . . - - - - - . . .
            
    //         // And the new remained relative to object range after the change is
    //         // . . . . - - - - - . . .

    //         // so the new remained remained in position

    //         // so the intersection relative to object range after the change also remains in position
    //         // . . . . . ± ± ± . . . .

    //         // so there is no change

    //         Equal => intersection_before_change,
            
    //         // Say the intersection relative to object range before the change is
    //         // . . . . . ± ± ± . . . . .

    //         // And the new remained relative to object range before the change is
    //         // . . . . - - - - - . . .
            
    //         // And the new remained relative to object range after the change is
    //         // . . - - - - - . . . . .

    //         // so the new remained is pushed backward by 2,

    //         // so the intersection relative to object range after the change should also be pushed backward by 2:
    //         // . . . ± ± ± . . . . . . .

    //         Greater => {
    //             let pre_to_post_change_shift = index_before - index_after;

    //             let intersection_index_post_change = intersection_before_change
    //                 .index() - pre_to_post_change_shift;

    //             let intersection_end_index_post_change = intersection_index_post_change + intersection_before_change.len();

    //             (intersection_index_post_change..intersection_end_index_post_change).into()
    //         }
    //     }
    // }

    // /// Determine the intersection index relative to the original list object.
    // /// 
    // /// Before and after change resolution entails shifting the intersection range post the change 
    // /// based on the shift that occured for the new remained range index during the change.
    // pub fn intersection_index_relative_to_original(
    //     &self,
    //     intersection_index_: usize,
    //     intersection_before_change: Range2
    // ) -> usize {

    // }

    /// Synthesise an updated resolved remained range, by comparing 
    /// the new remained range which resulted from the last change operation,
    /// to the resolved remained range.
    /// 
    /// The new range represents the range that remains consecutively intact across all changes including the last one.
    pub fn resolve(&self, new_remained: &NewRemained) -> Option<Self> {

        // We create a new resolved remained range, and to create it we do:
        //
        // 1 - we add the calculated intersection between the existing remained range
        // and the new remained range, if any. And if there is:
        //
        // 2 - we calculate a new index of the remained range; that
        // relative to the updated current list object that 
        // has just undergone a new change operation.
        //
        // 3 - we calculate a new index of the remained range; that
        // relative to the original list object.
        
        let new_remained_before = new_remained.remained_before().to_range2();

        let resolved_remained_before = self.remained_current().to_range2();

        let Some(intersection) = resolved_remained_before.intersection(&new_remained_before) else {
            return None
        };
        assert!(intersection.len() != 0);

        // we compare the previous remained spans with the new remained ones,
        // we want to keep only the remained spans that have not been changed in all of the series of change operations.
        // and these will be the intersection between the previous resolved remained spans and the new remained spans.

        // first we calculate the shifts in index from the later to the older time points.

        // Then we get the intersection between the new remained spans and the previous remained spans, relative
        // to the list object before this last change.

        // Then we create a new resolved remained span from the intersection, put shift the current index

        // Second, 

        let shift_current_to_original = self.index_shift_current_to_original();

        let shift_before_to_after = new_remained.index_shift_before_to_after();
        
        // If the intersection has the same index as the previous resolved remained range

        let intersection_at_start = resolved_remained_before.index() == intersection.index();

        // Provide the new resolved remained range

        Some(ResolvedRemained {

            // This will be the intersection's length

            length: intersection.len(),

            // This will be the intersection range index in the original list object

            index_original: {

                // we get this by applying the shift in the index, from the index in the list object
                // before the change to the index in the ranged original list object before this series of changes
                
                let mut index_original = intersection.index();
                shift_current_to_original.apply_index_translation(&mut index_original);

                index_original
            },

            // This will be the intersection range index in the list object after the new change

            index_current: {

                // we get this by applying the shift in the index, from the index in the list object
                // before the change to the index in the list object after the change

                let mut index_current = intersection.index();
                shift_before_to_after.apply_index_translation(&mut index_current);

                index_current
            },
        })
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::range::Range2;
//     use super::*;

//     const RANGED_OBJECT: [u64; 8] = [1, 13, 500, 843, 1200, 2000, 7777, 99999];

//     const RANGED_OBJECT_RANGE: Range2 = Range2::new(2, 5);

    
//     #[test]
//     fn resolve() {

//         assert_eq!(&RANGED_OBJECT[OBJECT_RANGE.range()], &[500, 843, 1200, 2000, 7777]);
        
//         let mut object_range = OBJECT_RANGE;

//         let mut resolved_remained = ResolvedRemained {
//             length: todo!(),
//             index_original: todo!(),
//             index_current: todo!(),
//         };

//     }
// }