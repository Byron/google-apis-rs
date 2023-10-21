use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Manage your eDiscovery data
    Ediscovery,

    /// View your eDiscovery data
    EdiscoveryReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Ediscovery => "https://www.googleapis.com/auth/ediscovery",
            Scope::EdiscoveryReadonly => "https://www.googleapis.com/auth/ediscovery.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::EdiscoveryReadonly
    }
}

