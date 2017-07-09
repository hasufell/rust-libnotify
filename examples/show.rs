extern crate libnotify;

fn main() {
    // Create a libnotify context
    let notify = libnotify::Context::new("myapp").unwrap();
    // Create a new notification and show it
    let n = notify.new_notification("Summary", Some("Optional Body"), None)
        .unwrap();
    n.show().unwrap();
    // You can also use the .show() convenience method on the context
    notify.show("I am another notification", None, None).unwrap();
}
