extern crate libnotify;

fn main() {
    // Init libnotify
    libnotify::init("myapp");
    // Create a new notification (doesn't show it yet)
    let n =
        libnotify::Notification::new("Summary", Some("Optional Body"), None);
    // Show the notification
    n.show().unwrap();
    // Update the existent notification
    n.update("I am another notification", None, None).unwrap();
    // Show the updated notification
    n.show().unwrap();
    // We are done, deinit
    libnotify::uninit();
}
