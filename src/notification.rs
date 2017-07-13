use Urgency;
use ffi;
use gdk_pixbuf;
use glib::translate::*;
use glib;
use glib_ffi;
use gobject_ffi;
use std;


glib_wrapper! {
    /// `Notification` represents a passive pop-up notification. It can contain
    /// summary text, body text, and an icon, as well as hints specifying how
    /// the notification should be presented. The notification is rendered by
    /// a notification daemon, and may present the notification in any number
    /// of ways. As such, there is a clear separation of content and
    /// presentation, and this API enforces that.
    pub struct Notification(Object<ffi::NotifyNotification>);

    match fn {
        get_type => || ffi::notify_notification_get_type(),
    }
}



impl Notification {
    /// Creates a new `Notification`. The summary text is required, but
    /// all other parameters are optional.
    /// ## `summary`
    /// The required summary text.
    /// ## `body`
    /// The optional body text.
    /// ## `icon`
    /// The optional icon theme icon name or filename.
    ///
    /// # Returns
    ///
    /// The new `Notification`.
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(
        summary: &str,
        body: P,
        icon: Q,
    ) -> Notification {
        assert_initialized_libnotify!();
        let body = body.into();
        let body = body.to_glib_none();
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            from_glib_full(ffi::notify_notification_new(
                summary.to_glib_none().0,
                body.0,
                icon.0,
            ))
        }
    }

    /// Synchronously tells the notification server to hide the notification on the screen.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(err)` on error
    pub fn close(&self) -> Result<(), glib::error::Error> {
        assert_initialized_libnotify!();
        unsafe {
            let mut err: *mut glib_ffi::GError = std::ptr::null_mut();
            ffi::notify_notification_close(self.to_glib_none().0, &mut err);

            if !err.is_null() {
                return Err(glib::error::Error::wrap(err));
            } else {
                return Ok(());
            }
        }
    }

    /// Tells the notification server to display the notification on the screen.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(err)` on error
    // TODO: test if Error leaks memory
    pub fn show(&self) -> Result<(), glib::error::Error> {
        assert_initialized_libnotify!();
        unsafe {
            let mut err: *mut glib_ffi::GError = std::ptr::null_mut();
            ffi::notify_notification_show(self.to_glib_none().0, &mut err);

            if !err.is_null() {
                return Err(glib::error::Error::wrap(err));
            } else {
                return Ok(());
            }
        }
    }

    /// Sets a hint for `key` with value `value`. If `value` is `None`,
    /// a previously set hint for `key` is unset.
    ///
    /// If `value` is floating, it is consumed.
    /// ## `key`
    /// the hint key
    /// ## `value`
    pub fn set_hint(&self, key: &str, value: Option<glib::variant::Variant>) {
        assert_initialized_libnotify!();

        let gvalue: *mut glib_ffi::GVariant = {
            match value {
                Some(ref value) => value.to_glib_none().0,
                None => std::ptr::null_mut(),
            }
        };

        unsafe {
            ffi::notify_notification_set_hint(
                self.to_glib_none().0,
                key.to_glib_none().0,
                gvalue,
            )
        }
    }

    /// Sets the image in the notification from a `gdk_pixbuf::Pixbuf`.
    /// ## `pixbuf`
    /// The image.
    pub fn set_image_from_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        assert_initialized_libnotify!();

        unsafe {
            ffi::notify_notification_set_image_from_pixbuf(
                self.to_glib_none().0,
                pixbuf.to_glib_none().0,
            );
        }
    }

    /// Clears all hints from the notification.
    pub fn clear_hints(&self) {
        unsafe {
            ffi::notify_notification_clear_hints(self.to_glib_none().0);
        }
    }

    /// Sets the application name for the notification. If this function is
    /// not called or if `app_name` is `None`, the application name will be
    /// set from the value used in `notify_init` or overridden with
    /// `notify_set_app_name`.
    /// ## `app_name`
    /// the localised application name
    pub fn set_app_name<'a, P: Into<Option<&'a str>>>(&self, app_name: P) {
        let app_name = app_name.into();
        let app_name = app_name.to_glib_none();
        unsafe {
            ffi::notify_notification_set_app_name(
                self.to_glib_none().0,
                app_name.0,
            );
        }
    }

    /// Sets the category of this notification. This can be used by the
    /// notification server to filter or display the data in a certain way.
    /// ## `category`
    /// The category.
    pub fn set_category(&self, category: &str) {
        unsafe {
            ffi::notify_notification_set_category(
                self.to_glib_none().0,
                category.to_glib_none().0,
            );
        }
    }

    /// Sets the timeout of the notification. To set the default time, pass
    /// `NOTIFY_EXPIRES_DEFAULT` as `timeout`. To set the notification to never
    /// expire, pass `NOTIFY_EXPIRES_NEVER`.
    ///
    /// Note that the timeout may be ignored by the server.
    /// ## `timeout`
    /// The timeout in milliseconds.
    pub fn set_timeout(&self, timeout: i32) {
        unsafe {
            ffi::notify_notification_set_timeout(
                self.to_glib_none().0,
                timeout,
            );
        }
    }

    /// Sets the urgency level of this notification.
    ///
    /// See: `Urgency`
    /// ## `urgency`
    /// The urgency level.
    pub fn set_urgency(&self, urgency: Urgency) {
        unsafe {
            ffi::notify_notification_set_urgency(
                self.to_glib_none().0,
                urgency.to_glib(),
            );
        }
    }

    /// Updates the notification text and icon. This won't send the update out
    /// and display it on the screen. For that, you will need to call
    /// `Notification::show`.
    /// ## `summary`
    /// The new required summary text.
    /// ## `body`
    /// The optional body text.
    /// ## `icon`
    /// The optional icon theme icon name or filename.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(str)` if an invalid parameter was passed
    // TODO: switch back to BoolError when it hits stable glib
    pub fn update<
        'a,
        'b,
        P: Into<Option<&'a str>>,
        Q: Into<Option<&'b str>>,
    >(
        &self,
        summary: &str,
        body: P,
        icon: Q,
    ) -> Result<(), String> {
        let body = body.into();
        let body = body.to_glib_none();
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            let b = ffi::notify_notification_update(
                self.to_glib_none().0,
                summary.to_glib_none().0,
                body.0,
                icon.0,
            );
            match b {
                glib_ffi::GFALSE => Err(
                    String::from("Invalid parameter passed"),
                ),
                _ => Ok(()),
            }
        }
    }
}
