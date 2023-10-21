use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your Ad Exchange data
    AdexchangeSeller,

    /// View your Ad Exchange data
    AdexchangeSellerReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AdexchangeSeller => "https://www.googleapis.com/auth/adexchange.seller",
            Scope::AdexchangeSellerReadonly => "https://www.googleapis.com/auth/adexchange.seller.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AdexchangeSellerReadonly
    }
}

