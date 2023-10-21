use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, create or delete policies applied to Chrome OS and Chrome Browsers managed within your organization
    ChromeManagementPolicy,

    /// See policies applied to Chrome OS and Chrome Browsers managed within your organization
    ChromeManagementPolicyReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ChromeManagementPolicy => "https://www.googleapis.com/auth/chrome.management.policy",
            Scope::ChromeManagementPolicyReadonly => "https://www.googleapis.com/auth/chrome.management.policy.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ChromeManagementPolicyReadonly
    }
}

