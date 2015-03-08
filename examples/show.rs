extern crate libnotify;

fn main() {
    let notify = libnotify::Context::new("hello").unwrap();
    let n = notify.new_notification("This is the summary.",
                                    Some("This is the optional body text."),
                                    None).unwrap();
    n.show().unwrap();
}
