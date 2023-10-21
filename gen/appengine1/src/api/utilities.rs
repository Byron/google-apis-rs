use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your applications deployed on Google App Engine
    Admin,

    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Admin => "https://www.googleapis.com/auth/appengine.admin",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}

