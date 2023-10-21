use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View and manage your Google Compute Engine resources
    Full,

    /// View your Google Compute Engine resources
    Readonly,

    /// Manage your data and permissions in Cloud Storage and see the email address for your Google Account
    DevstorageFullControl,

    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Cloud Storage and see the email address of your Google Account
    DevstorageReadWrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::Full => "https://www.googleapis.com/auth/compute",
            Scope::Readonly => "https://www.googleapis.com/auth/compute.readonly",
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}

