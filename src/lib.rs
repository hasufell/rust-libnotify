//! Rustic bindings to [libnotify](https://developer.gnome.org/libnotify/)
//!
//! ```rust
//! extern crate libnotify;
//!
//! fn main() {
//!     // Init libnotify
//!     libnotify::init("myapp").unwrap();
//!     // Create a new notification (doesn't show it yet)
//!     let n = libnotify::Notification::new("Summary",
//!                                          Some("Optional Body"),
//!                                          None);
//!     // Show the notification
//!     n.show().unwrap();
//!     // Update the existent notification
//!     n.update("I am another notification", None, None).unwrap();
//!     // Show the updated notification
//!     n.show().unwrap();
//!     // We are done, deinit
//!     libnotify::uninit();
//! }
//!
//! ```

#![warn(missing_docs)]

extern crate gdk_pixbuf;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate libnotify_sys as ffi;


pub use enums::*;
pub use functions::*;
pub use notification::*;


macro_rules! assert_initialized_libnotify {
    () => {
        use functions::*;
        if !is_initted() {
            panic!("Notify system not initialized, invalid call of function");
        }
    }
}


mod enums;
mod functions;
mod notification;
