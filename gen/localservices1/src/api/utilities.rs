use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, create, and delete your Google Ads accounts and data.
    Adword,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Adword => "https://www.googleapis.com/auth/adwords",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Adword
    }
}

