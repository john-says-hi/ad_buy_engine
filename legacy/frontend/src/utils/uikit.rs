//! UIkit related create_element_helpers

use std::fmt;

/// The UIkit service
pub struct UIkitService;

/// Possible status for notifications
pub enum NotificationStatus {
    Primary,
    Success,
    Warning,
    Danger,
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationStatus::Primary => write!(f, "primary"),
            NotificationStatus::Success => write!(f, "success"),
            NotificationStatus::Warning => write!(f, "warning"),
            NotificationStatus::Danger => write!(f, "danger"),
        }
    }
}

impl UIkitService {
    /// Create a new UIkitService instance
    pub fn new() -> Self {
        Self {}
    }

    /// Create a new notification
    pub fn notify(&self, message: &str, status: &NotificationStatus) {
        // javascript! {
        //     UIkit.notification({
        //         message: @{message},
        //         status: @{status.to_string()},
        //         pos: "top-right",
        //         timeout: 5000,
        //     });
        // };
        ()
    }
}
