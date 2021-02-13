//!
//! This module defines utility functions for translating C types to rust types.
//!

use crate::cffi::*;
pub use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::NonNull;

//
// Convert interrupt callback
//

/// Utility to cast the void pointer to the interrupt checker function a valid Rust type.
pub unsafe fn interrupt_callback(
    checker: *mut c_void,
    check_interrupt: Option<extern "C" fn(*const c_void) -> bool>,
) -> impl Fn() -> bool + Clone {
    let interrupt_ref = &*checker; // conversion needed since *mut c_void is not Send
    move || match check_interrupt {
        Some(cb) => cb(interrupt_ref as *const c_void),
        None => true,
    }
}

//
// Convert result type
//

/// The Rust version of the cook result enum.
pub enum CookResult {
    Success(String),
    Warning(String),
    Error(String),
}

impl From<CookResult> for HR_CookResult {
    fn from(res: CookResult) -> HR_CookResult {
        match res {
            CookResult::Success(msg) => HR_CookResult {
                message: CString::new(msg.as_str()).unwrap().into_raw(),
                tag: HRCookResultTag::HR_SUCCESS,
            },
            CookResult::Warning(msg) => HR_CookResult {
                message: CString::new(msg.as_str()).unwrap().into_raw(),
                tag: HRCookResultTag::HR_WARNING,
            },
            CookResult::Error(msg) => HR_CookResult {
                message: CString::new(msg.as_str()).unwrap().into_raw(),
                tag: HRCookResultTag::HR_ERROR,
            },
        }
    }
}

//
// Convert pointers
//

macro_rules! impl_mesh_wrapper_convert {
    ($mesh_wrapper:ident, $mesh:ty) => {
        //
        // Into wrapper conversions
        //

        impl From<$mesh> for $mesh_wrapper {
            fn from(mesh: $mesh) -> $mesh_wrapper {
                $mesh_wrapper { mesh }
            }
        }

        //
        // Unwrap conversions
        //

        impl Into<$mesh> for $mesh_wrapper {
            fn into(self) -> $mesh {
                self.mesh
            }
        }

        // Rust reference conversions
        impl<'a> Into<&'a $mesh> for &'a $mesh_wrapper {
            fn into(self) -> &'a $mesh {
                &self.mesh
            }
        }
        impl<'a> Into<&'a mut $mesh> for &'a mut $mesh_wrapper {
            fn into(self) -> &'a mut $mesh {
                &mut self.mesh
            }
        }

        // Owned pointer conversion
        impl<'a> Into<Box<$mesh>> for Box<$mesh_wrapper> {
            fn into(self) -> Box<$mesh> {
                Box::new(self.mesh)
            }
        }
    };
}

impl_mesh_wrapper_convert!(HR_TetMesh, gut::mesh::TetMesh<f64>);
impl_mesh_wrapper_convert!(HR_PolyMesh, gut::mesh::PolyMesh<f64>);
impl_mesh_wrapper_convert!(HR_PointCloud, gut::mesh::PointCloud<f64>);

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
