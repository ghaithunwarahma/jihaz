//! Traits that provide multiple conversions of the values inside containers and collections

extern crate alloc;

use alloc::sync::Arc;
use std::{collections::HashMap, hash::Hash};

use super::{TakeOrCloneInner, TakeOrCloneInnerBlanket};

/// Drains the collection, maps the content by applying into to the values of T1
pub trait DrainMapInto<T>: Sized {
    fn drain_map_into(self) -> T;
}

impl<Key1, Key2, Value1, Value2>
    DrainMapInto<HashMap<Key2, Value2>>
for HashMap<Key1, Value1> 
where
    Key1: Eq + Hash,
    Key2: Eq + Hash + From<Key1>,
    Value2: From<Value1>
{
    fn drain_map_into(mut self) -> HashMap<Key2, Value2> {
        // into is reflexive so when the k type is the same, it just returns self
        self.drain().map(|(k, v)| (k.into(), v.into())).collect()
    }
}

impl<Value1, Value2> DrainMapInto<Vec<Value2>> for Vec<Value1> 
where
    Value2: From<Value1>
{
    fn drain_map_into(mut self) -> Vec<Value2> {
        self.drain(..).map(|v| v.into()).collect()
    }
}


pub trait TakeOrCloneInnerDrainMapInto<T>: Sized {
    fn take_or_clone_inner_drain_map_into(self) -> T;
}

impl<Key1, Key2, Value1, Value2>
    TakeOrCloneInnerDrainMapInto<HashMap<Key2, Value2>>
for 
    Arc<HashMap<Key1, Value1>> 
where
    Key1: Eq + Hash + Clone,
    Value1: Clone,
    Key2: Eq + Hash + From<Key1>,
    Value2: From<Value1>
{
    fn take_or_clone_inner_drain_map_into(self) -> HashMap<Key2, Value2> {
        // into is reflexive so when the k type is the same, it just returns self
        self.take_or_clone_inner().drain().map(|(k, v)| (k.into(), v.into())).collect()
    }
}

impl<Value1, Value2>
    TakeOrCloneInnerDrainMapInto<Vec<Value2>>
for 
    Arc<Vec<Value1>> 
where
    Value1: Clone,
    Value2: From<Value1>
{
    fn take_or_clone_inner_drain_map_into(self) -> Vec<Value2> {
        self.take_or_clone_inner().drain(..).map(|v| v.into()).collect()
    }
}


pub trait DrainMapIntoInArc<T>: Sized {
    fn drain_map_into_in_arc(self) -> T;
}

impl<Key1, Key2, Value1, Value2>
    DrainMapIntoInArc<HashMap<Key2, Arc<Value2>>>
for 
    HashMap<Key1, Value1> 
where
    Key1: Eq + Hash + Into<Key2>,
    Key2: Eq + Hash,
    Value2: From<Value1>
{
    fn drain_map_into_in_arc(mut self) -> HashMap<Key2, Arc<Value2>> {
        // into is reflexive so when the k type is the same, it just returns self
        self.drain().map(|(k, v)| (k.into(), Arc::new(v.into()))).collect()
    }
}

impl<Value1, Value2>
    DrainMapIntoInArc<Vec<Arc<Value2>>>
for 
    Vec<Value1> 
where
    Value2: From<Value1>
{
    fn drain_map_into_in_arc(mut self) -> Vec<Arc<Value2>> {
        self.drain(..).map(|v| Arc::new(v.into())).collect()
    }
}


pub trait DrainMapTakeOrCloneInner<T>: Sized {
    fn drain_map_take_or_clone_inner(self) -> T;
}

impl<Key1, Key2, Value1, Value2>
    DrainMapTakeOrCloneInner<HashMap<Key2, Value2>>
for 
    HashMap<Key1, Value1> 
where
    Key1: Eq + Hash + TakeOrCloneInner<Key2>,
    Key2: Eq + Hash,
    Value1: TakeOrCloneInner<Value2>
{
    fn drain_map_take_or_clone_inner(mut self) -> HashMap<Key2, Value2> {
        // take_or_clone_inner is reflexive so when the k type is the same, it just returns self
        self.drain().map(|(k, v)| (k.take_or_clone_inner(), v.take_or_clone_inner())).collect()
    }
}

impl<Value1, Value2>
    DrainMapTakeOrCloneInner<Vec<Value2>>
for 
    Vec<Value1> 
where
    Value1: TakeOrCloneInner<Value2>
{
    fn drain_map_take_or_clone_inner(mut self) -> Vec<Value2> {
        self.drain(..).map(|v| v.take_or_clone_inner()).collect()
    }
}


pub trait DrainMapDrainMapInto<T, DrainMapIntoT>: Sized {
    fn drain_map_drain_map_into(self) -> T;
}

impl<Key1, Key2, Value, DrainMapIntoT>
    DrainMapDrainMapInto<HashMap<Key2, DrainMapIntoT>, DrainMapIntoT>
for
    HashMap<Key1, Value>
where
    Key1: Eq + Hash + Into<Key2>,
    Key2: Eq + Hash,
    Value: DrainMapInto<DrainMapIntoT>,
{
    fn drain_map_drain_map_into(mut self) -> HashMap<Key2, DrainMapIntoT> {
        // into is reflexive so when the k type is the same, it just returns self
        self.drain().map(|(k, v)| (k.into(), v.drain_map_into())).collect()
    }
}

impl<Value, DrainMapIntoT>
    DrainMapDrainMapInto<Vec<DrainMapIntoT>, DrainMapIntoT>
for
    Vec<Value>
where
    Value: DrainMapInto<DrainMapIntoT>,
{
    fn drain_map_drain_map_into(mut self) -> Vec<DrainMapIntoT> {
        self.drain(..).map(|v| v.drain_map_into()).collect()
    }
}


pub trait DrainMapDrainMapIntoInArc<T, DrainMapIntoT>: Sized {
    fn drain_map_drain_map_into_in_arc(self) -> T;
}

impl<Key1, Key2, Value, DrainMapIntoT>
    DrainMapDrainMapIntoInArc<HashMap<Key2, Arc<DrainMapIntoT>>, DrainMapIntoT>
for
    HashMap<Key1, Value>
where
    Key1: Eq + Hash + Into<Key2>,
    Key2: Eq + Hash,
    Value: DrainMapInto<DrainMapIntoT>,
{
    fn drain_map_drain_map_into_in_arc(mut self) -> HashMap<Key2, Arc<DrainMapIntoT>> {
        self.drain().map(|(k, v)| (k.into(), Arc::new(v.drain_map_into()))).collect()
    }
}

impl<Value, DrainMapIntoT>
    DrainMapDrainMapIntoInArc<Vec<Arc<DrainMapIntoT>>, DrainMapIntoT>
for
    Vec<Value>
where
    Value: DrainMapInto<DrainMapIntoT>,
{
    fn drain_map_drain_map_into_in_arc(mut self) -> Vec<Arc<DrainMapIntoT>> {
        self.drain(..).map(|v| Arc::new(v.drain_map_into())).collect()
    }
}

pub trait DrainMapTakeOrCloneInnerDrainMapInto<T, TakeOrCloneInnerDrainMapIntoT>: Sized {
    fn drain_map_take_or_clone_inner_drain_map_into(self) -> T;
}

impl<Key1, Key2, Value, TakeOrCloneInnerDrainMapIntoT>
    DrainMapTakeOrCloneInnerDrainMapInto<HashMap<Key2, TakeOrCloneInnerDrainMapIntoT>, TakeOrCloneInnerDrainMapIntoT>
for
    HashMap<Key1, Value>
where
    Key1: Eq + Hash + TakeOrCloneInnerBlanket<Key2>,
    Key2: Eq + Hash,
    Value: TakeOrCloneInnerDrainMapInto<TakeOrCloneInnerDrainMapIntoT>,
{
    fn drain_map_take_or_clone_inner_drain_map_into(mut self) -> HashMap<Key2, TakeOrCloneInnerDrainMapIntoT> {
        // take_or_clone_inner is reflexive so when the k type is the same, it just returns self
        self.drain().map(|(k, v)| (k.take_or_clone_inner_blanket(), v.take_or_clone_inner_drain_map_into())).collect()
    }
}

impl<Value, TakeOrCloneInnerDrainMapIntoT>
DrainMapTakeOrCloneInnerDrainMapInto<Vec<TakeOrCloneInnerDrainMapIntoT>, TakeOrCloneInnerDrainMapIntoT>
for 
    Vec<Value>
where
    Value: TakeOrCloneInnerDrainMapInto<TakeOrCloneInnerDrainMapIntoT>,
{
    fn drain_map_take_or_clone_inner_drain_map_into(mut self) -> Vec<TakeOrCloneInnerDrainMapIntoT> {
        self.drain(..).map(|v| v.take_or_clone_inner_drain_map_into()).collect()
    }
}

pub trait DrainMapDrainMapTakeOrCloneInner<T, DrainMapTakeOrCloneInnerT>: Sized {
    fn drain_map_drain_map_take_or_clone_inner(self) -> T;
}

impl<Key1, Key2, Value, DrainMapTakeOrCloneInnerT>
    DrainMapDrainMapTakeOrCloneInner<HashMap<Key2, DrainMapTakeOrCloneInnerT>, DrainMapTakeOrCloneInnerT>
for 
    HashMap<Key1, Value>
where
    Key1: Eq + Hash + TakeOrCloneInner<Key2>,
    Key2: Eq + Hash,
    Value: DrainMapTakeOrCloneInner<DrainMapTakeOrCloneInnerT>,
{
    fn drain_map_drain_map_take_or_clone_inner(mut self) -> HashMap<Key2, DrainMapTakeOrCloneInnerT> {
        // take_or_clone_inner is reflexive so when the k type is the same, it just returns self
        self.drain().map(|(k, v)| (k.take_or_clone_inner(), v.drain_map_take_or_clone_inner())).collect()
    }
}

impl<Value, DrainMapTakeOrCloneInnerT> 
    DrainMapDrainMapTakeOrCloneInner<Vec<DrainMapTakeOrCloneInnerT>, DrainMapTakeOrCloneInnerT> 
for 
    Vec<Value>
where
    Value: DrainMapTakeOrCloneInner<DrainMapTakeOrCloneInnerT>,
{
    fn drain_map_drain_map_take_or_clone_inner(mut self) -> Vec<DrainMapTakeOrCloneInnerT> {
        self.drain(..).map(|v| v.drain_map_take_or_clone_inner()).collect()
    }
}


pub trait DrainMapDrainMapTakeOrCloneInnerInto<T, DrainMapTakeOrCloneInnerT, IntoT>: Sized {
    fn drain_map_drain_map_take_or_clone_inner(self) -> T;
}

impl<Key1, Key2, Value, DrainMapTakeOrCloneInnerT, IntoT>
    DrainMapDrainMapTakeOrCloneInnerInto<HashMap<Key2, IntoT>, DrainMapTakeOrCloneInnerT, IntoT> 
for 
    HashMap<Key1, Value>
where
    Key1: Eq + Hash + TakeOrCloneInner<Key2>,
    Key2: Eq + Hash,
    Value: DrainMapTakeOrCloneInner<DrainMapTakeOrCloneInnerT>,
    DrainMapTakeOrCloneInnerT: Into<IntoT>,
{
    fn drain_map_drain_map_take_or_clone_inner(mut self) -> HashMap<Key2, IntoT> {
        // take_or_clone_inner and into is reflexive so when the k type is the same, it just returns self
        // can Arc::into converts Arc into T?
        self.drain().map(|(k, v)| (k.take_or_clone_inner(), v.drain_map_take_or_clone_inner().into())).collect()
    }
}

impl<Value, DrainMapTakeOrCloneInnerT, IntoT>
    DrainMapDrainMapTakeOrCloneInnerInto<Vec<IntoT>, DrainMapTakeOrCloneInnerT, IntoT> 
for 
    Vec<Value>
where
    Value: DrainMapTakeOrCloneInner<DrainMapTakeOrCloneInnerT>,
    DrainMapTakeOrCloneInnerT: Into<IntoT>,
{
    fn drain_map_drain_map_take_or_clone_inner(mut self) -> Vec<IntoT> {
        self.drain(..).map(|v| v.drain_map_take_or_clone_inner().into()).collect()
    }
}



/// ToCrossSerde

/// Drains the collection, maps the content by applying into to the values of T1
pub trait MapInto<T1: Into<U>, T2, U>: Sized {
    type Out;
    fn dmapi(self) -> Self::Out;
    // requires T2 for HashMap to always be copy.
    // fn imapi(&self) -> Self::Out;
}

impl<T1: Into<U>, T2: Eq + Hash, U>
    MapInto<T1, T2, U>
for
    HashMap<T2, T1>
{
    type Out = HashMap<T2, U>;
    fn dmapi(mut self) -> Self::Out {
        self.drain().map(|(k, v)| (k, v.into())).collect()
    }
    // fn imapi(&self) -> Self::Out {
    //     self.iter().map(|(k, v)| (*k, (*v).into())).collect()
    // }
}

impl<T1: Into<U>, U>
    MapInto<T1, (), U> 
for 
    Vec<T1> 
{
    type Out = Vec<U>;
    fn dmapi(mut self) -> Self::Out {
        self.drain(..).map(|v| v.into()).collect()
    }
    // fn imapi(&self) -> Self::Out {
    //     self.iter().map(|v| (*v).into()).collect()
    // }
}

/// Drains the collection, maps the content by applying into to the values of T1
/// Generally used for version management
pub trait MapInto2<T1: Into<U1>, T2: Into<U2>, U1, U2>: Sized {
    type Out;
    fn dmapi2(self) -> Self::Out;
}

impl<
    T1: Into<U1>, 
    T2: Into<U2> + Eq + Hash, 
    U1, 
    U2: Eq + Hash
>
    MapInto2<T1, T2, U1, U2> 
for 
    HashMap<T2, T1> 
{
    type Out = HashMap<U2, U1>;
    fn dmapi2(mut self) -> Self::Out {
        self.drain().map(|(k, v)| (k.into(), v.into())).collect()
    }
}

impl<T1: Into<U1>, T2: Into<U2>, U1, U2>
    MapInto2<T1, T2, U1, U2> 
for 
    Vec<(T2, T1)> 
{
    type Out = Vec<(U2, U1)>;
    fn dmapi2(mut self) -> Self::Out {
        self.drain(..).map(|(k, v)| (k.into(), v.into())).collect()
    }
}

/// Drains the collection, maps the content by acrtaking the values of T1
pub trait MapTake<T1, T2, U: TakeOrCloneInner<T1>>: Sized {
    type Out;
    fn dmapt(self) -> Self::Out;
}


impl<T1: TakeOrCloneInner<Arc<T1>> + Clone, T2: Eq + Hash>
    MapTake<Arc<T1>, T2, T1> 
for 
    HashMap<T2, Arc<T1>> 
{
    type Out = HashMap<T2, T1>;
    fn dmapt(mut self) -> Self::Out {
        self.drain().map(|(k, v)| (k, v.take_or_clone_inner())).collect()
    }
}

impl<T1: TakeOrCloneInner<Arc<T1>> + Clone> 
    MapTake<Arc<T1>, (), T1> 
for 
    Vec<Arc<T1>> 
{
    type Out = Vec<T1>;
    fn dmapt(mut self) -> Self::Out {
        self.drain(..).map(|v| v.take_or_clone_inner()).collect()
    }
}




// Revisit:

// /// Drains the collection, maps the content by applying into to the values of T1
// pub trait DrainMapInto<Key, Value1, Value2>:
// where
//     Self: Sized,
//     Value2: From<Value1>
// {
//     type Out;
//     fn drain_map_into(self) -> Self::Out;
// }

// impl<Key, Value1, Value2> DrainMapInto<Key, Value1, Value2> for HashMap<Key, Value1> 
// where
//     Key: Eq + Hash,
//     Value2: From<Value1>
// {
//     type Out = HashMap<Key, Value2>;
//     fn drain_map_into(mut self) -> Self::Out {
//         self.drain().map(|(k, v)| (k, v.into())).collect()
//     }
// }

// impl<Key, Value1, Value2> DrainMapInto<Key, Value1, Value2> for Vec<Value1> 
// where
//     Value2: From<Value1>
// {
//     type Out = Vec<Value2>;
//     fn drain_map_into(mut self) -> Self::Out {
//         self.drain(..).map(|v| v.into()).collect()
//     }
// }

// /// Drains the collection, maps the content by applying into to the values of T1
// pub trait DrainMapDrainMapInto<Key, Value1, Value2>
// where
//     Self: Sized,
//     Value1: DrainMapInto<Key, Value1, Value2>,
//     Value2: From<Value1>,
// {
//     type Out;
//     fn drain_map_drain_map_into(self) -> Self::Out;
// }

// impl<Key, Value1, Value2> DrainMapDrainMapInto<Key, Value1, Value2> for HashMap<Key, Value1>
// where
//     Key: Eq + Hash,
//     Value1: DrainMapInto<Key, Value1, Value2>,
//     Value2: From<Value1>,
// {
//     type Out = HashMap<Key, Value2>;
//     fn drain_map_drain_map_into(mut self) -> Self::Out {
//         self.drain().map(|(k, mut v)| (k, v.drain_map_into())).collect()
//     }
// }

// impl<Key, Value1, Value2> DrainMapDrainMapInto<Key, Value1, Value2> for Vec<Value1>
// where
//     Key: Eq + Hash,
//     Value1: DrainMapInto<Key, Value1, Value2>,
//     Value2: From<Value1>,
// {
//     type Out = Vec<Value2>;
//     fn drain_map_drain_map_into(mut self) -> Self::Out {
//         self.drain(..).map(|mut v| v.drain_map_into()).collect()
//     }
// }

// pub trait DrainMapIntoInArc<Key, Value1, Value2>:
// where
//     Self: Sized,
//     Value1: Into<Value2>
// {
//     type Out;
//     fn drain_map_into_in_arc(self) -> Self::Out;
// }

// impl<Key, Value1, Value2> DrainMapIntoInArc<Key, Value1, Value2> for HashMap<Key, Value1> 
// where
//     Key: Eq + Hash,
//     Value1: Into<Value2>,
// {
//     type Out = HashMap<Key, Arc<Value2>>;
//     fn drain_map_into_in_arc(mut self) -> Self::Out {
//         self.drain().map(|(k, v)| (k, Arc::new(v.into()))).collect()
//     }
// }

// impl<Key, Value1, Value2> DrainMapIntoInArc<Key, Value1, Value2> for Vec<Value1> 
// where
//     Value1: Into<Value2>,
// {
//     type Out = Vec<Arc<Value2>>;
//     fn drain_map_into_in_arc(mut self) -> Self::Out {
//         self.drain(..).map(|v| Arc::new(v.into())).collect()
//     }
// }