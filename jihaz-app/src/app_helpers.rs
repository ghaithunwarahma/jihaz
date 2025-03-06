use std::{any::Any, sync::Arc};
use masonry::core::{Widget, WidgetMut, WidgetPod};
use xilem::Pod;
use xilem_core::MessageResult;
use crate::app_message::AppMessageResult;

// pub trait ChildMut<'a, Parent: View<State, Action, Context>, Child: ViewElement> {
//     fn child_mut(&mut self) -> Mut<'a, Child> {
        
//     }
// }

// impl<'a, Parent: View, Child: ViewElement> ChildMut<'a, Parent, Child> for Mut<'a, Parent> {
//     fn child_mut(&mut self, f: FnMut(&mut Self) -> Child) -> Mut<'a, Child> {
//         let mut child_element = self.widget.tablet.inner;
//         self.ctx.get_mut(f(self))
//     }
// }

// pub trait WithChildElement<MapChildElem> {
//     fn with_elem_child(f: MapChildElem);
// }

// impl<'a> WithChildElement<MapChildElem> for WidgetCtx<'a>
// where
//     MapChildElem: FnMut(WidgetMut<>) -> 
// {
//     fn with_elem_child(
//         f: Map,
//         child: &'a mut WidgetPod<Child>,
//     ) -> WidgetMut<'a, Child> {

//     }
// }

pub trait ChildMut<'a, 'b: 'a, Parent: Widget, Child: Widget> {
    fn child_mut(&'b mut self, f: impl FnOnce(&'b mut Parent) -> &'b mut WidgetPod<Child>) -> WidgetMut<'a, Child>;
}

impl<'a, 'b: 'a, Parent: Widget, Child: Widget> ChildMut<'a, 'b, Parent, Child> for WidgetMut<'a, Parent> {
    fn child_mut(&'b mut self, f: impl FnOnce(&'b mut Parent) -> &'b mut WidgetPod<Child>) -> WidgetMut<'a, Child> {
        self.ctx.get_mut(f(&mut self.widget))
    }
}

#[derive(Clone)]
pub struct ActionData {
    pub data: Arc<dyn Any + Sync + Send>,
}

impl ActionData {
    pub fn new<Data>(data: Data) -> ActionData
    where
        Data: Any + Sync + Send
    {
        Self { data: Arc::new(data) }
    }
}

pub type DynWiPod = Pod<Box<dyn Widget>>;

// pub trait InWiPod<T: Widget> {
//     fn in_pod(self) -> Pod<T>;
//     fn in_box_pod(self) -> DynWiPod;
// }

// impl<T: Widget> InWiPod<T> for T {
//     fn in_pod(self) -> Pod<T> {
//         ctx.new_pod(self)
//     }
//     fn in_box_pod(self) -> DynWiPod {
//         ctx.new_pod(Box::new(self))
//     }
// }

pub trait FullMessageResult {
    fn full(self) -> MessageResult<AppMessageResult>;
}

pub trait EmptyMessageResult {
    fn empty(self) -> MessageResult<()>;
}

impl FullMessageResult for MessageResult<()> {
    fn full(self) -> MessageResult<AppMessageResult> {
        match self {
            MessageResult::Action(_) => MessageResult::Action(AppMessageResult::Nop),
            MessageResult::RequestRebuild => MessageResult::RequestRebuild,
            MessageResult::Nop => MessageResult::Nop,
            MessageResult::Stale(msg) => MessageResult::Stale(msg),
        }
    }
}

impl EmptyMessageResult for MessageResult<AppMessageResult> {
    fn empty(self) -> MessageResult<()> {
        match self {
            MessageResult::Action(_action) => MessageResult::Action(()),
            MessageResult::RequestRebuild => MessageResult::RequestRebuild,
            MessageResult::Nop => MessageResult::Nop,
            MessageResult::Stale(msg) => MessageResult::Stale(msg),
        }
    }
}

pub fn is_stale(msg: MessageResult<AppMessageResult>) -> bool {
    if let MessageResult::Stale(_) = msg {
        true
    } else {
        false
    }
}