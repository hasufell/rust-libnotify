//! Rustic bindings to [libnotify](https://developer.gnome.org/libnotify/)
//!
//! ```rust
//! extern crate libnotify;
//!
//! fn main() {
//!     // Init libnotify
//!     libnotify::init("myapp").unwrap();
//!     // Create a new notification and show it
//!     let n = libnotify::Notification::new("Summary",
//!                                          Some("Optional Body"),
//!                                          None).unwrap();
//!     // Show the notification
//!     n.show().unwrap();
//!     // You can also use the .show() convenience method on the context
//!     n.update("I am another notification", None, None).unwrap();
//!     // Show the update notification
//!     n.show().unwrap();
//!     // We are done, deinit
//!     libnotify::uninit();
//! }
//!
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate gdk_pixbuf;
extern crate gdk_pixbuf_sys;
extern crate glib;
extern crate glib_sys;
extern crate gtypes;
extern crate libnotify_sys as sys;

pub mod errors;

use errors::*;
use gdk_pixbuf_sys::GdkPixbuf;
use glib::translate::ToGlibPtr;
use gtypes::{TRUE, FALSE};
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::os::raw::c_char;


macro_rules! init_panic {
    () => {
        if !is_initted() {
            panic!("Notify system not initialized, invalid call of function");
        }
    }
}


/// The urgency level of the notification.
pub enum Urgency {
    /// Low urgency. Used for unimportant notifications.
    Low,
    /// Normal urgency. Used for most standard notifications.
    Normal,
    /// Critical urgency. Used for very important notifications.
    Critical,
}

impl From<sys::NotifyUrgency> for Urgency {
    fn from(urgency: sys::NotifyUrgency) -> Urgency {
        match urgency {
            sys::NotifyUrgency::NotifyUrgencyLow => Urgency::Low,
            sys::NotifyUrgency::NotifyUrgencyNormal => Urgency::Normal,
            sys::NotifyUrgency::NotifyUrgencyCritical => Urgency::Critical,
        }
    }
}

impl From<Urgency> for sys::NotifyUrgency {
    fn from(urgency: Urgency) -> sys::NotifyUrgency {
        match urgency {
            Urgency::Low => sys::NotifyUrgency::NotifyUrgencyLow,
            Urgency::Normal => sys::NotifyUrgency::NotifyUrgencyNormal,
            Urgency::Critical => sys::NotifyUrgency::NotifyUrgencyCritical,
        }
    }
}




/// A passive pop-up notification
pub struct Notification {
    handle: *mut sys::NotifyNotification,
}

impl Notification {
    /// Creates a new Notification.
    ///
    /// Arguments:
    ///
    /// - summary: Required summary text
    /// - body: Optional body text
    /// - icon: Optional icon theme icon name or filename
    pub fn new(summary: &str,
               body: Option<&str>,
               icon: Option<&str>)
               -> Result<Notification> {
        init_panic!();
        let summary = CString::new(summary)?;
        let body = match body {
            Some(body) => Some(CString::new(body)?),
            None => None,
        };
        let body_ptr = match body {
            Some(ref body) => body.as_ptr(),
            None => std::ptr::null(),
        };
        let icon = match icon {
            Some(icon) => Some(CString::new(icon)?),
            None => None,
        };
        let icon_ptr = match icon {
            Some(ref icon) => icon.as_ptr(),
            None => std::ptr::null(),
        };

        unsafe {
            let n = sys::notify_notification_new(summary.as_ptr(),
                                                 body_ptr,
                                                 icon_ptr);
            if n.is_null() {
                bail!(ErrorKind::UnknownError);
            }

            Ok(Notification { handle: n })
        }
    }

    /// Tells the notification server to display the notification
    /// on the screen.
    pub fn show(&self) -> Result<()> {
        init_panic!();
        unsafe {
            let mut err: *mut glib_sys::GError = std::ptr::null_mut();
            sys::notify_notification_show(self.handle, &mut err);
            if !err.is_null() {
                let msg = CStr::from_ptr((*err).message)
                    .to_string_lossy()
                    .into_owned();
                glib_sys::g_error_free(err);
                bail!(ErrorKind::NotificationShowError(msg));
            }
            Ok(())
        }
    }

    /// Set the notification timeout. Note that the server might ignore
    /// the timeout.
    pub fn set_notification_timeout(&self, timeout: i32) {
        init_panic!();
        let _timeout: c_int = From::from(timeout);

        unsafe { sys::notify_notification_set_timeout(self.handle, _timeout) }
    }

    /// Updates the notification text and icon. This won't send the update
    /// out and display it on the screen. For that, you will need to
    /// call `.show()`.
    pub fn update(&self,
                  summary: &str,
                  body: Option<&str>,
                  icon: Option<&str>)
                  -> Result<()> {
        init_panic!();
        let summary = CString::new(summary)?;
        let body = match body {
            Some(body) => Some(CString::new(body)?),
            None => None,
        };
        let body_ptr = match body {
            Some(ref body) => body.as_ptr(),
            None => std::ptr::null(),
        };
        let icon = match icon {
            Some(icon) => Some(CString::new(icon)?),
            None => None,
        };
        let icon_ptr = match icon {
            Some(ref icon) => icon.as_ptr(),
            None => std::ptr::null(),
        };

        unsafe {
            let b = sys::notify_notification_update(self.handle,
                                                    summary.as_ptr(),
                                                    body_ptr,
                                                    icon_ptr);
            if b == FALSE {
                bail!(ErrorKind::InvalidParameter);
            }
        }

        return Ok(());
    }

    /// Sets a hint for `key` with value `value`. If value is `None`,
    /// then key is unset.
    pub fn set_hint(&self,
                    key: &str,
                    value: Option<glib::variant::Variant>)
                    -> Result<()> {
        init_panic!();
        let key = CString::new(key)?;

        let gvalue: *mut glib_sys::GVariant = {
            match value {
                Some(ref value) => value.to_glib_none().0,
                None => std::ptr::null_mut(),
            }
        };

        unsafe {
            sys::notify_notification_set_hint(self.handle, key.as_ptr(), gvalue)
        }

        return Ok(());
    }

    /// Sets the category of this notification. This can be used by the
    /// notification server to filter or display the data in a certain way.
    pub fn set_category(&self, category: &str) -> Result<()> {
        init_panic!();
        let category = CString::new(category)?;
        unsafe {
            sys::notify_notification_set_category(self.handle,
                                                  category.as_ptr());
        }

        return Ok(());
    }

    /// Sets the urgency level of this notification.
    pub fn set_urgency(&self, urgency: Urgency) {
        init_panic!();
        let urgency: sys::NotifyUrgency = From::from(urgency);

        unsafe {
            sys::notify_notification_set_urgency(self.handle, urgency);
        }
    }

    /// Sets the image in the notification from a Pixbuf.
    pub fn set_image_from_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        init_panic!();
        let pixbuf: *mut GdkPixbuf = pixbuf.to_glib_none().0;

        unsafe {
            sys::notify_notification_set_image_from_pixbuf(self.handle, pixbuf);
        }
    }

    /// Clears all hints from the notification.
    pub fn clear_hints(&self) {
        init_panic!();
        unsafe {
            sys::notify_notification_clear_hints(self.handle);
        }
    }

    /// Synchronously tells the notification server to hide the
    /// notification on the screen.
    pub fn close(&self) -> Result<()> {
        init_panic!();
        unsafe {
            let mut err: *mut glib_sys::GError = std::ptr::null_mut();
            sys::notify_notification_close(self.handle, &mut err);

            if !err.is_null() {
                let msg = CStr::from_ptr((*err).message)
                    .to_string_lossy()
                    .into_owned();
                glib_sys::g_error_free(err);
                bail!(ErrorKind::NotificationShowError(msg));
            }
        }
        return Ok(());
    }
}


/// Initialized libnotify. This must be called before any other functions.
pub fn init(app_name: &str) -> Result<()> {
    let app_name = CString::new(app_name)?;

    unsafe {
        if sys::notify_is_initted() == TRUE {
            bail!(ErrorKind::NotifyAlreadyInitialized);
        }
        let app_name = CString::new(app_name)?;
        if sys::notify_init(app_name.as_ptr()) == FALSE {
            bail!(ErrorKind::NotifyInitError);
        }
    }

    return Ok(());
}


/// Uninitialized libnotify.
/// This should be called when the program no longer needs libnotify for
/// the rest of its lifecycle, typically just before exitting.
pub fn uninit() {
    init_panic!();
    unsafe {
        sys::notify_uninit();
    }
}


/// Gets whether or not libnotify is initialized.
pub fn is_initted() -> bool {
    unsafe {
        if sys::notify_is_initted() == TRUE {
            return true;
        } else {
            return false;
        }
    }

}


/// Sets the application name.
pub fn set_app_name(app_name: &str) -> Result<()> {
    init_panic!();
    let app_name = CString::new(app_name)?;

    unsafe {
        sys::notify_set_app_name(app_name.as_ptr());
    }

    return Ok(());
}


/// Gets the application name registered.
pub fn get_app_name() -> Result<String> {
    init_panic!();
    unsafe {
        let c_name: *const c_char = sys::notify_get_app_name();
        let c_str = CStr::from_ptr(c_name);
        let string = c_str.to_str()?;
        return Ok(String::from(string));
    }
}
