use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Manage DoubleClick Digital Marketing conversions
    Ddmconversion,

    /// View and manage DoubleClick for Advertisers reports
    Full,

    /// View and manage your DoubleClick Campaign Manager's (DCM) display ad campaigns
    Dfatrafficking,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Ddmconversion => "https://www.googleapis.com/auth/ddmconversions",
            Scope::Full => "https://www.googleapis.com/auth/dfareporting",
            Scope::Dfatrafficking => "https://www.googleapis.com/auth/dfatrafficking",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}

