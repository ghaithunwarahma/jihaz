//! Convenience conversions for Arc and Rc

extern crate alloc;

/// A more convenient call of the associated function [`make_mut`]
pub trait MakeMutable {
    type Inner: Clone;
    /// Makes a mutable reference.
    /// 
    /// A more convenient call of the associated function [`make_mut`] for Arc's and Rc's
    fn make_mutable(&mut self) -> &mut Self::Inner;
}

impl<T: Clone> MakeMutable for alloc::sync::Arc<T> {
    type Inner = T;
    fn make_mutable(&mut self) -> &mut T {
        alloc::sync::Arc::make_mut(self)
    }
}

impl<T: Clone> MakeMutable for alloc::rc::Rc<T> {
    type Inner = T;
    fn make_mutable(&mut self) -> &mut T {
        alloc::rc::Rc::make_mut(self)
    }
}

pub trait InnerMutable {
    type Inner: Clone;
    /// Gets the inner value as a mutable reference
    fn inner_mutable(&mut self) -> &mut Self::Inner;
}

impl<T: Clone> InnerMutable for alloc::sync::Arc<T> {
    type Inner = T;
    fn inner_mutable(&mut self) -> &mut T {
        alloc::sync::Arc::make_mut(self)
    }
}

impl<T: Clone> InnerMutable for alloc::rc::Rc<T> {
    type Inner = T;
    fn inner_mutable(&mut self) -> &mut T {
        alloc::rc::Rc::make_mut(self)
    }
}

/// Returns the inner value, if container has exactly one strong reference, 
/// If not, clones the inner value.
pub trait TakeOrCloneInner<T> {
    /// Returns the inner value, if container has exactly one strong reference, 
    /// If not, clones the inner value.
    fn take_or_clone_inner(self) -> T;
}

impl<T: Clone> TakeOrCloneInner<T> for alloc::sync::Arc<T> {
    fn take_or_clone_inner(self: alloc::sync::Arc<T>) -> T {
        match alloc::sync::Arc::try_unwrap(self) {
            Ok(inner) => inner,
            Err(data) => data.as_ref().clone(),
        }
    }
}

impl<T: Clone> TakeOrCloneInner<T> for alloc::rc::Rc<T> {
    fn take_or_clone_inner(self: alloc::rc::Rc<T>) -> T {
        match alloc::rc::Rc::try_unwrap(self) {
            Ok(inner) => inner,
            Err(data) => data.as_ref().clone(),
        }
    }
}

/// This method is reflexive, it does nothing for general types due to blanket implementation,
/// but when trait is set as a bound in other traits, it will require qualifying the type.
/// 
/// Basically this trait is implemented for all types, and only applies TakeOrCloneInner for types
/// that implement that trait.
pub trait TakeOrCloneInnerBlanket<T> {
    /// This method is reflexive, it does nothing for general types due to blanket implementation,
    /// but when trait is set as a bound in other traits, it will require qualifying the type.
    /// 
    /// Basically this trait is implemented for all types, and only applies TakeOrCloneInner for types
    /// that implement that trait.
    fn take_or_clone_inner_blanket(self) -> T;
}

impl<T> TakeOrCloneInnerBlanket<T> for T {
    /// Returns the argument unchanged.
    fn take_or_clone_inner_blanket(self: T) -> T {
        self
    }
}

impl<T: Clone> TakeOrCloneInnerBlanket<T> for alloc::sync::Arc<T> {
    fn take_or_clone_inner_blanket(self: alloc::sync::Arc<T>) -> T {
        self.take_or_clone_inner()
    }
}

impl<T: Clone> TakeOrCloneInnerBlanket<T> for alloc::rc::Rc<T> {
    fn take_or_clone_inner_blanket(self: alloc::rc::Rc<T>) -> T {
        self.take_or_clone_inner()
    }
}