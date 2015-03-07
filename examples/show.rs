extern crate libnotify;

fn main() {
    let n = {
        let notify = libnotify::Context::new("hello").unwrap();
        notify.new_notification("Hello, ", "World!").unwrap()
    };
    n.show().unwrap();
}
