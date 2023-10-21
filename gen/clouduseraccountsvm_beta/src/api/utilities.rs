use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,

    /// Manage your Google Cloud User Accounts
    CloudUseraccount,

    /// View your Google Cloud User Accounts
    CloudUseraccountReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::CloudUseraccount => "https://www.googleapis.com/auth/cloud.useraccounts",
            Scope::CloudUseraccountReadonly => "https://www.googleapis.com/auth/cloud.useraccounts.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudUseraccountReadonly
    }
}

