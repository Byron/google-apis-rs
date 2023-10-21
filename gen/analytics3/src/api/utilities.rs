use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your Google Analytics data
    Full,

    /// Edit Google Analytics management entities
    Edit,

    /// Manage Google Analytics Account users by email address
    ManageUser,

    /// View Google Analytics user permissions
    ManageUserReadonly,

    /// Create a new Google Analytics account along with its default property and view
    Provision,

    /// View your Google Analytics data
    Readonly,

    /// Manage Google Analytics user deletion requests
    UserDeletion,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/analytics",
            Scope::Edit => "https://www.googleapis.com/auth/analytics.edit",
            Scope::ManageUser => "https://www.googleapis.com/auth/analytics.manage.users",
            Scope::ManageUserReadonly => "https://www.googleapis.com/auth/analytics.manage.users.readonly",
            Scope::Provision => "https://www.googleapis.com/auth/analytics.provision",
            Scope::Readonly => "https://www.googleapis.com/auth/analytics.readonly",
            Scope::UserDeletion => "https://www.googleapis.com/auth/analytics.user.deletion",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ManageUserReadonly
    }
}

