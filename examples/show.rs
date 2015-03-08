extern crate libnotify;

fn main() {
    let notify = libnotify::Context::new("hello").unwrap();
    let n = notify.new_notification("Hello, ", Some("World!"), None).unwrap();
    n.show().unwrap();
}
