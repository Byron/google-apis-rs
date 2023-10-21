use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Edit Google Analytics management entities
    AnalyticEdit,

    /// Manage Google Analytics Account users by email address
    AnalyticManageUser,

    /// View Google Analytics user permissions
    AnalyticManageUserReadonly,

    /// See and download your Google Analytics data
    AnalyticReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AnalyticEdit => "https://www.googleapis.com/auth/analytics.edit",
            Scope::AnalyticManageUser => "https://www.googleapis.com/auth/analytics.manage.users",
            Scope::AnalyticManageUserReadonly => "https://www.googleapis.com/auth/analytics.manage.users.readonly",
            Scope::AnalyticReadonly => "https://www.googleapis.com/auth/analytics.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AnalyticManageUserReadonly
    }
}

