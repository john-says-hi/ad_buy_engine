//! UIkit related create_element_helpers

use std::fmt;

/// The UIkit service
pub struct UIkitService;

/// Possible status for notifications
pub enum NotificationStatus {
    Warning,
    Danger,
    Success,
    Primary,
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationStatus::Warning => write!(f, "warning"),
            NotificationStatus::Danger => write!(f, "danger"),
            NotificationStatus::Success => write!(f, "success"),
            NotificationStatus::Primary => write!(f, "primary"),
        }
    }
}

impl UIkitService {
    /// Create a new UIkitService instance
    pub fn new() -> Self {
        Self {}
    }

    /// Create a new notification
    pub fn notify(&self, message: &str, status: NotificationStatus) {
        let message = format!(
            "<div style=\"border:2px solid black;height:30px;\"><div class=uk-margin-left uk-margin-top>{}</div></div>",
            message
        );
        ()
        // js! {
        //     UIkit.notification({
        //         message: @{message},
        //         status: @{status.to_string()},
        //         pos: "top-right",
        //         timeout: 3000,
        //     });
        // };
    }
}
