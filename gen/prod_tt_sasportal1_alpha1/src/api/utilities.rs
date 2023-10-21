use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Read, create, update, and delete your SAS Portal data.
    Sasportal,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Sasportal => "https://www.googleapis.com/auth/sasportal",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Sasportal
    }
}

