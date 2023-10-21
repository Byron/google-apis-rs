use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Private Service: https://www.googleapis.com/auth/wallet_object.issuer
    WalletObjectIssuer,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::WalletObjectIssuer => "https://www.googleapis.com/auth/wallet_object.issuer",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::WalletObjectIssuer
    }
}

