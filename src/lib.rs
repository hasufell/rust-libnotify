#![feature(std_misc)]

extern crate "libnotify-sys" as sys;
extern crate "glib-2_0-sys" as glib;

use std::ffi::CString;
use std::marker::PhantomData;

use glib::types::{
    TRUE,
    FALSE
};

/// Error that can happen on context creation
#[derive(Debug)]
pub enum ContextCreationError {
    /// Context already exists
    AlreadyExists,
    InitFailure,
    NulError
}

#[derive(Debug)]
pub enum NotificationCreationError {
    NulError,
    Unknown
}

pub struct Context;

impl Context {
    pub fn new(app_name: &str) -> Result<Context, ContextCreationError> {
        unsafe {
            if sys::notify_is_initted() == TRUE {
                return Err(ContextCreationError::AlreadyExists);
            }
            let app_name = match CString::new(app_name) {
                Ok(name) => name,
                Err(_) => return Err(ContextCreationError::NulError)
            };
            if sys::notify_init(app_name.as_ptr()) == FALSE {
                return Err(ContextCreationError::InitFailure);
            }
        }
        Ok(Context)
    }
    pub fn new_notification(&self, summary: &str, body: &str)
        -> Result<Notification, NotificationCreationError> {
        let summary = match CString::new(summary) {
            Ok(cstr) => cstr,
            Err(_) => return Err(NotificationCreationError::NulError)
        };
        let body = match CString::new(body) {
            Ok(cstr) => cstr,
            Err(_) => return Err(NotificationCreationError::NulError)
        };
        unsafe {
            let n = sys::notify_notification_new(summary.as_ptr(),
                                                 body.as_ptr(),
                                                 std::ptr::null());
            if n.is_null() {
                return Err(NotificationCreationError::Unknown);
            }

            Ok(Notification{handle: n, _phantom: PhantomData})
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sys::notify_uninit();
        }
    }
}

pub struct Notification<'a> {
    handle: *mut sys::NotifyNotification,
    _phantom: PhantomData<&'a Context>
}

impl<'a> Notification<'a> {
    pub fn show(&'a self) -> Result<(), ()> {
        unsafe {
            let mut err: *mut glib::GError = std::ptr::null_mut();
            sys::notify_notification_show(self.handle, &mut err);
            if !err.is_null() {
                glib::g_error_free(err);
                return Err(())
            }
            return Ok(())
        }
    }
}
