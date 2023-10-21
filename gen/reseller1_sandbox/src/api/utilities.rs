use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Manage users on your domain
    AppOrder,

    /// Manage users on your domain
    AppOrderReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppOrder => "https://www.googleapis.com/auth/apps.order",
            Scope::AppOrderReadonly => "https://www.googleapis.com/auth/apps.order.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppOrderReadonly
    }
}

