use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// Manage your data and permissions in Google Cloud Storage
    DevstorageFullControl,

    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Google Cloud Storage
    DevstorageReadWrite,

    /// Manage your data in the Google Prediction API
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
            Scope::Full => "https://www.googleapis.com/auth/prediction",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}

