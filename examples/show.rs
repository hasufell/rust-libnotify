extern crate libnotify;

fn main() {
    let notify = libnotify::Context::new("hello").unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let body_text = Some("This is the optional body text.");
    let n = notify.new_notification("This is the summary.", body_text, None)
                  .unwrap_or_else(|e| panic!("{}", e));
    n.show().unwrap_or_else(|e| panic!("{}", e));
}
