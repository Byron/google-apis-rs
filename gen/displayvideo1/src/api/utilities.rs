use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Create, see, edit, and permanently delete your Display & Video 360 entities and reports
    DisplayVideo,

    /// Create, see, and edit Display & Video 360 Campaign entities and see billing invoices
    DisplayVideoMediaplanning,

    /// Private Service: https://www.googleapis.com/auth/display-video-user-management
    DisplayVideoUserManagement,

    /// View and manage your reports in DoubleClick Bid Manager
    Doubleclickbidmanager,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DisplayVideo => "https://www.googleapis.com/auth/display-video",
            Scope::DisplayVideoMediaplanning => "https://www.googleapis.com/auth/display-video-mediaplanning",
            Scope::DisplayVideoUserManagement => "https://www.googleapis.com/auth/display-video-user-management",
            Scope::Doubleclickbidmanager => "https://www.googleapis.com/auth/doubleclickbidmanager",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DisplayVideo
    }
}

