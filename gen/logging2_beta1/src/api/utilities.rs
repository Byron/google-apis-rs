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

    /// Administrate log data for your projects
    Admin,

    /// View log data for your projects
    Read,

    /// Submit log data for your projects
    Write,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::Admin => "https://www.googleapis.com/auth/logging.admin",
            Scope::Read => "https://www.googleapis.com/auth/logging.read",
            Scope::Write => "https://www.googleapis.com/auth/logging.write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Read
    }
}

