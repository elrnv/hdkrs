//!
//! This module defines utility functions for translating C types to rust types.
//!

use crate::CookResult as HdkCookResult;
use crate::CookResultTag;
pub use std::ffi::c_void;
use std::ptr::NonNull;

//
// Convert result type
//

/// The pure Rust version of the cook result enum.
pub enum CookResult {
    Success(String),
    Warning(String),
    Error(String),
}

impl From<CookResult> for HdkCookResult {
    fn from(res: CookResult) -> HdkCookResult {
        match res {
            CookResult::Success(msg) => HdkCookResult {
                message: msg,
                tag: CookResultTag::SUCCESS,
            },
            CookResult::Warning(msg) => HdkCookResult {
                message: msg,
                tag: CookResultTag::WARNING,
            },
            CookResult::Error(msg) => HdkCookResult {
                message: msg,
                tag: CookResultTag::ERROR,
            },
        }
    }
}

//
// Convert pointers
//

/// A convenience utility to convert a mutable pointer to an optional mutable reference.
pub unsafe fn as_mut<'a, U: 'a, T: 'a>(ptr: *mut T) -> Option<&'a mut U>
where
    &'a mut T: Into<&'a mut U>,
{
    NonNull::new(ptr).map(|x| Into::<&mut U>::into(&mut *x.as_ptr()))
}

/// A convenience utility to convert a mutable pointer to an optional owning box.
pub unsafe fn into_box<'a, U: 'a, T: 'a>(ptr: *mut T) -> Option<Box<U>>
where
    Box<T>: Into<Box<U>>,
{
    NonNull::new(ptr).map(|x| Into::<Box<U>>::into(Box::from_raw(x.as_ptr())))
}
