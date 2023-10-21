use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View the digital assets you publish on Google Play Movies and TV
    PlaymovyPartnerReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PlaymovyPartnerReadonly => "https://www.googleapis.com/auth/playmovies_partner.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PlaymovyPartnerReadonly
    }
}

