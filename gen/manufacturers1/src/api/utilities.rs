use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Manage your product listings for Google Manufacturer Center
    Manufacturercenter,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Manufacturercenter => "https://www.googleapis.com/auth/manufacturercenter",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Manufacturercenter
    }
}

