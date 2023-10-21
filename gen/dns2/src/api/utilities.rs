use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,

    /// View your DNS records hosted by Google Cloud DNS
    NdevClouddnReadonly,

    /// View and manage your DNS records hosted by Google Cloud DNS
    NdevClouddnReadwrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::NdevClouddnReadonly => "https://www.googleapis.com/auth/ndev.clouddns.readonly",
            Scope::NdevClouddnReadwrite => "https://www.googleapis.com/auth/ndev.clouddns.readwrite",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::NdevClouddnReadonly
    }
}

