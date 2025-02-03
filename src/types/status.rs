/// A simple struct to represent the status
pub struct Status {
    pub is_ok: bool,
    pub message: String,
}

impl Status {
    /// Create a new okay status with is_ok = true and an empty message
    pub fn ok() -> Status {
        Status {
            is_ok: true,
            message: String::from(""),
        }
    }

    /// Create a new error status with is_ok = false and a message
    pub fn error(message: String) -> Status {
        Status {
            is_ok: false,
            message,
        }
    }
}
