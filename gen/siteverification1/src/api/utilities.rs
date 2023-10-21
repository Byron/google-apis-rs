use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Manage the list of sites and domains you control
    Full,

    /// Manage your new site verifications with Google
    VerifyOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/siteverification",
            Scope::VerifyOnly => "https://www.googleapis.com/auth/siteverification.verify_only",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}

