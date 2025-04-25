//! Events or changes that occur to a ranged span of a larger list of items.
//! 
//! Range2 represets a range of span of items of a larger list of items, 
//! we can call it object range.
//! 
//! The change requests are applied on the object range, 
//! and the final resulting range will be new object range.
//! 
//! The change sync

mod change;
pub use change::*;
mod categorical;
pub use categorical::*;
mod remained;
pub use remained::*;
mod tests;

/// A range of items. Usually of a larger list or array or items.
#[derive(Copy, Clone, Debug)]
pub struct Range2 {
    pub index: usize,
    pub len: usize,
}

impl Range2 {
    pub const NONE: Self = Self { index: 0, len: 0 };

    /// The length is 0, meaning that this does not represent a spanning range
    pub const fn is_none(&self) -> bool {
        self.len == 0
    }

    /// The length is more than 0, meaning that this represents a spaning range
    pub const fn is_some(&self) -> bool {
        self.len > 0
    }

    /// Represents a caret range, as the length is 0
    pub const fn is_caret(&self) -> bool {
        self.is_none()
    }

    /// Represents a spaning range, as the length is more than 0
    pub const fn is_span(&self) -> bool {
        self.is_some()
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    /// Used when doing comarisons with the end index of the range
    pub const fn index_plus_one(&self) -> usize {
        self.index + 1
    }

    pub const fn len(&self) -> usize {
        self.len
    }
    
    /// This end index is exclusive. In other words,
    /// it is equal to last item index + 1.
    pub const fn end_index(&self) -> usize {
        self.index + self.len
    }

    /// Gets the last index in the range. In other words,
    /// it is equivalent to the inclusive end of the range.
    /// 
    /// Warning: panics when len == 0.
    pub const fn last_index(&self) -> usize {
        self.end_index() - 1
    }

    /// Sets the values
    pub fn set(&mut self, index: usize, len: usize) {
        *self = Self::new(index, len);
    }

    /// Sets the start index
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// Sets the length of the range
    pub fn set_len(&mut self, length: usize) {
        self.len = length;
    }

    /// Sets a caret range
    pub fn set_caret(&mut self, index: usize) {
        *self = Self::new(index, 0);
    }

    /// Sets the values, using two indexes
    pub fn set_indexes(&mut self, start_index: usize, end_index: usize) {
        *self = Self::from_indexes(start_index, end_index);
    }

    /// Sets the values, using the end index. From the reserve side.
    pub fn set_rev(&mut self, len: usize, end_index: usize) {
        *self = Self::new_from_end(len, end_index);
    }

    /// Creates a new instance
    pub const fn new(index: usize, len: usize) -> Self {
        Self { index, len }
    }

    /// Creates a new instance, using two indexes
    pub const fn from_indexes(start_index: usize, end_index: usize) -> Self {
        Self { index: start_index, len: end_index - start_index }
    }

    /// Creates a new instance, relative to the given end index
    pub const fn new_from_end(len: usize, end_index: usize) -> Self {
        Self { index: end_index - len, len }
    }

    /// Gets the std::ops::Range<usize> corresponding to self
    pub const fn range(&self) -> std::ops::Range<usize> {
        self.index..self.end_index()
    }

    /// Returns Some of self if self is non-empty
    pub fn then(&self) -> Option<Self> {
        self.is_some().then(|| *self)
    }

    /// Index is within self
    pub const fn contains(&self, index: usize) -> bool {
        self.index <= index && index < self.end_index()
    }

    /// The start of Other is within self
    pub const fn contains_start(&self, other: &Self) -> bool {
        self.contains(other.index)
    }

    /// The last index of Other is within self
    pub const fn contains_last(&self, other: &Self) -> bool {
        if other.end_index() > 0 {
            self.index <= other.last_index() && other.last_index() < self.end_index()
        } else {
            // special considerations
            self.index < other.end_index() && other.end_index() <= self.end_index()
        }
    }

    /// Other is within self
    pub const fn contains_range(&self, other: &Self) -> bool {
        if other.is_caret() {
            // in the case of text editing, if the position is caret and is at the end index,
            // this means it doesn't belong in the range
            self.index <= other.index && other.end_index() < self.end_index()
        } else {
            // notice the double equalities,
            self.index <= other.index && other.end_index() <= self.end_index()
        }
    }

    /// Self is within Other
    pub const fn contained_in(&self, other: &Self) -> bool {
        if self.is_caret() {
            // in the case of text editing, if the position is caret and is at the end index,
            // this means it doesn't belong in the range
            other.index <= self.index && self.end_index() < other.end_index()
        } else {
            // notice the double equalities,
            other.index <= self.index && self.end_index() <= other.end_index()
        }
    }

    /// Returns true if the index follows self
    pub const fn follows_index(&self, index: usize) -> bool {
        index < self.index
    }

    /// Returns true if the index preceeds self
    pub const fn preceeds_index(&self, index: usize) -> bool {
        self.end_index() <= index
    }

    /// Returns true if other follows self
    pub const fn follows(&self, other: &Self) -> bool {
        other.end_index() <= self.index
    }

    /// Returns true if other preceeds self
    pub const fn preceeds(&self, other: &Self) -> bool {
        self.end_index() <= other.index
    }

    /// Gets the range following self, starting from the end of self and spanning a certain length
    pub const fn get_following(&self, following_len: usize) -> Self {
        Self::new(self.end_index(), following_len)
    }

    /// Gets the range preceeding self, starting from zero and ending at the beginning of self
    pub const fn get_preceeding(&self) -> Self {
        Self::new(0, self.index)
    }
}

impl From<std::ops::Range<usize>> for Range2 {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self { index: value.start, len: value.len() }
    }
}

impl From<&std::ops::Range<usize>> for Range2 {
    fn from(value: &std::ops::Range<usize>) -> Self {
        Self { index: value.start, len: value.len() }
    }
}

trait ToRange2 {
    fn to_range2(&self) -> Range2;
}

impl ToRange2 for std::ops::Range<usize> {
    fn to_range2(&self) -> Range2 {
        self.into()
    }
}