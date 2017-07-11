use ffi;
use glib::translate::*;
use glib_ffi;
use std::ptr;



/// Gets whether or not libnotify is initialized.
///
/// # Returns
///
/// `true` if libnotify is initialized, or `false` otherwise.
pub fn is_initted() -> bool {
    unsafe { from_glib(ffi::notify_is_initted()) }
}

/// Initialized libnotify. This must be called before any other functions.
///
/// # Returns
///
/// `Ok(())` if successful, `Err(str)` on error.
// TODO: switch back to BoolError when it hits stable glib
pub fn init(app_name: &str) -> Result<(), String> {
    unsafe {
        let b = ffi::notify_init(app_name.to_glib_none().0);

        match b {
            glib_ffi::GFALSE => Err(
                String::from("Failed to initialize libnotify"),
            ),
            _ => Ok(()),
        }
    }
}

/// Gets the application name registered.
///
/// # Returns
///
/// The registered application name, passed to `init()`.
pub fn get_app_name() -> Option<String> {
    assert_initialized_libnotify!();
    unsafe { from_glib_none(ffi::notify_get_app_name()) }
}

/// Synchronously queries the server for its capabilities and returns them as
/// a Vector.
///
/// # Returns
///
/// A Vector of server capability Strings.
pub fn get_server_caps() -> Vec<String> {
    assert_initialized_libnotify!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::notify_get_server_caps())
    }
}

/// Synchronously queries the server for its information, specifically,
/// the name, vendor, server version, and the version of the notifications
/// specification that it is compliant with.
///
/// # Returns
///
/// `Some(ret_name, ret_vendor, ret_version, ret_spec_version)` on
/// success, otherwise `None` on error.
pub fn get_server_info() -> Option<(String, String, String, String)> {
    assert_initialized_libnotify!();
    unsafe {
        let mut ret_name = ptr::null_mut();
        let mut ret_vendor = ptr::null_mut();
        let mut ret_version = ptr::null_mut();
        let mut ret_spec_version = ptr::null_mut();
        let ret = from_glib(ffi::notify_get_server_info(
            &mut ret_name,
            &mut ret_vendor,
            &mut ret_version,
            &mut ret_spec_version,
        ));
        if ret {
            Some((
                from_glib_full(ret_name),
                from_glib_full(ret_vendor),
                from_glib_full(ret_version),
                from_glib_full(ret_spec_version),
            ))
        } else {
            None
        }
    }
}

/// Sets the application name.
/// ## `app_name`
/// The name of the application.
pub fn set_app_name(app_name: &str) {
    assert_initialized_libnotify!();
    unsafe {
        ffi::notify_set_app_name(app_name.to_glib_none().0);
    }
}

/// Uninitialized libnotify.
///
/// This should be called when the program no longer needs libnotify for
/// the rest of its lifecycle, typically just before exitting.
pub fn uninit() {
    assert_initialized_libnotify!();
    unsafe {
        ffi::notify_uninit();
    }
}
