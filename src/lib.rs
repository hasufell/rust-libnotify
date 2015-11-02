//! Rustic bindings to [libnotify](https://developer.gnome.org/libnotify/)
//!
//! ```rust
//!extern crate libnotify;
//!
//!fn main() {
//!    let notify = libnotify::Context::new("hello").unwrap_or_else(|e| {
//!        panic!("{}", e);
//!    });
//!    let body_text = Some("This is the optional body text.");
//!    let n = notify.new_notification("This is the summary.", body_text, None)
//!                  .unwrap_or_else(|e| panic!("{}", e));
//!    n.show().unwrap_or_else(|e| panic!("{}", e));
//!}
//!
//! ```

#![warn(missing_docs)]

extern crate libnotify_sys as sys;
extern crate glib_2_0_sys as glib;
extern crate gtypes;

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::fmt;
use std::error::Error;

use gtypes::{TRUE, FALSE};

/// Error that can happen on context creation
#[derive(Debug)]
pub enum ContextCreationError {
    /// Context already exists.
    AlreadyExists,
    /// Failed to initialize libnotify.
    InitError,
    NulError,
}

impl fmt::Display for ContextCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ContextCreationError::*;
        match *self {
            AlreadyExists => write!(f, "A Libnotify context already exists."),
            InitError => write!(f, "Failed to initialize libnotify."),
            NulError => write!(f, "Argument contains a nul character."),
        }
    }
}

#[derive(Debug)]
pub enum NotificationCreationError {
    NulError,
    Unknown,
}

impl fmt::Display for NotificationCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NotificationCreationError::*;
        match *self {
            NulError => write!(f, "Argument contains a nul character."),
            Unknown => write!(f, "Unknown error"),
        }
    }
}

/// The context which within libnotify operates.
///
/// Only one context can exist at a time.
pub struct Context;

impl Context {
    /// Create a new context
    ///
    /// Arguments:
    ///
    /// - app_name: The name of the application using the context
    pub fn new(app_name: &str) -> Result<Context, ContextCreationError> {
        unsafe {
            if sys::notify_is_initted() == TRUE {
                return Err(ContextCreationError::AlreadyExists);
            }
            let app_name = match CString::new(app_name) {
                Ok(name) => name,
                Err(_) => return Err(ContextCreationError::NulError),
            };
            if sys::notify_init(app_name.as_ptr()) == FALSE {
                return Err(ContextCreationError::InitError);
            }
        }
        Ok(Context)
    }
    /// Creates a new Notification.
    ///
    /// Arguments:
    ///
    /// - summary: Required summary text
    /// - body: Optional body text
    /// - icon: Optional icon theme icon name or filename
    pub fn new_notification(&self,
                            summary: &str,
                            body: Option<&str>,
                            icon: Option<&str>)
                            -> Result<Notification, NotificationCreationError> {
        let summary = match CString::new(summary) {
            Ok(cstr) => cstr,
            Err(_) => return Err(NotificationCreationError::NulError),
        };
        let body = match body {
            Some(body) => match CString::new(body) {
                Ok(cstr) => Some(cstr),
                Err(_) => return Err(NotificationCreationError::NulError),
            },
            None => None,
        };
        let body_ptr = match body {
            Some(body) => body.as_ptr(),
            None => std::ptr::null(),
        };
        let icon = match icon {
            Some(icon) => match CString::new(icon) {
                Ok(cstr) => Some(cstr),
                Err(_) => return Err(NotificationCreationError::NulError),
            },
            None => None,
        };
        let icon_ptr = match icon {
            Some(icon) => icon.as_ptr(),
            None => std::ptr::null(),
        };
        unsafe {
            let n = sys::notify_notification_new(summary.as_ptr(), body_ptr, icon_ptr);
            if n.is_null() {
                return Err(NotificationCreationError::Unknown);
            }

            Ok(Notification {
                handle: n,
                _phantom: PhantomData,
            })
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

/// A passive pop-up notification
pub struct Notification<'a> {
    handle: *mut sys::NotifyNotification,
    _phantom: PhantomData<&'a Context>,
}

impl<'a> Notification<'a> {
    /// Tells the notification server to display the notification
    /// on the screen.
    pub fn show(&'a self) -> Result<(), NotificationShowError> {
        unsafe {
            let mut err: *mut glib::GError = std::ptr::null_mut();
            sys::notify_notification_show(self.handle, &mut err);
            if !err.is_null() {
                let result = Err(NotificationShowError {
                    message: CStr::from_ptr((*err).message).to_string_lossy().into_owned(),
                });
                glib::g_error_free(err);
                return result;
            }
            return Ok(());
        }
    }
}

#[derive(Debug)]
pub struct NotificationShowError {
    message: String,
}

impl fmt::Display for NotificationShowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error showing notification: {}", self.message)
    }
}

impl Error for NotificationShowError {
    fn description(&self) -> &str {
        "Notification show error"
    }
}
