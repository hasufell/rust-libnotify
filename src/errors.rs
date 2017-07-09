use std;

/// Error variants for Libnotify.
error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
        Nul(std::ffi::NulError);
    }

    errors {
        UnknownError {
            description("Unknown Error")
            display("Unknown Error")
        }
        InvalidParameter {
            description("Invalid parameter")
            display("Invalid parameter")
        }
        NotificationShowError(t: String) {
            description("Failed to show notification")
            display("Failed to show notification: {}", t)
        }
        NotifyAlreadyInitialized {
            description("Notify system already initialized")
            display("Notify system already initialized")
        }
        NotifyInitError {
            description("Notify system initialization error")
            display("Notify system initialization error")
        }
    }

}
