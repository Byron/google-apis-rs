use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, create, edit, and delete your Authorized Buyers Marketplace entities.
    AuthorizedBuyerMarketplace,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AuthorizedBuyerMarketplace => "https://www.googleapis.com/auth/authorized-buyers-marketplace",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AuthorizedBuyerMarketplace
    }
}

