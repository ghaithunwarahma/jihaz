// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Utilities, macOS specific.
//! 
//! These are copied from Druid's implementation.

use std::ffi::c_void;

use cocoa::base::{id, nil, BOOL, YES};
use cocoa::foundation::{NSAutoreleasePool, NSString, NSUInteger};
use objc::{class, msg_send, sel, sel_impl};

/// Panic if not on the main thread.
///
/// Many Cocoa operations are only valid on the main thread, and (I think)
/// undefined behavior is possible if invoked from other threads. If so,
/// failing on non main thread is necessary for safety.
pub(crate) fn assert_main_thread() {
    unsafe {
        let is_main_thread: BOOL = msg_send!(class!(NSThread), isMainThread);
        assert_eq!(is_main_thread, YES);
    }
}

/// Create a new NSString from a &str.
pub(crate) fn make_nsstring(s: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(s).autorelease() }
}

pub(crate) fn from_nsstring(s: id) -> String {
    unsafe {
        let slice = std::slice::from_raw_parts(s.UTF8String() as *const _, s.len());
        let result = std::str::from_utf8_unchecked(slice);
        result.into()
    }
}

pub(crate) fn make_nsdata(bytes: &[u8]) -> id {
    let dlen = bytes.len() as NSUInteger;
    unsafe {
        msg_send![class!(NSData), dataWithBytes: bytes.as_ptr() as *const c_void length: dlen]
    }
}

pub(crate) fn from_nsdata(data: id) -> Vec<u8> {
    unsafe {
        let len: NSUInteger = msg_send![data, length];
        let bytes: *const c_void = msg_send![data, bytes];
        let mut out: Vec<u8> = Vec::with_capacity(len as usize);
        std::ptr::copy_nonoverlapping(bytes as *const u8, out.as_mut_ptr(), len as usize);
        out.set_len(len as usize);
        out
    }
}
