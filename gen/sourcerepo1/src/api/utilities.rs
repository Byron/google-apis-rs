use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// Manage your source code repositories
    SourceFullControl,

    /// View the contents of your source code repositories
    SourceReadOnly,

    /// Manage the contents of your source code repositories
    SourceReadWrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::SourceFullControl => "https://www.googleapis.com/auth/source.full_control",
            Scope::SourceReadOnly => "https://www.googleapis.com/auth/source.read_only",
            Scope::SourceReadWrite => "https://www.googleapis.com/auth/source.read_write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}

