use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See and/or control the devices that you selected
    SdmService,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::SdmService => "https://www.googleapis.com/auth/sdm.service",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::SdmService
    }
}

