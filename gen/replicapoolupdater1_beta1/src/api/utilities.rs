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

    /// View and manage replica pools
    Replicapool,

    /// View replica pools
    ReplicapoolReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::Replicapool => "https://www.googleapis.com/auth/replicapool",
            Scope::ReplicapoolReadonly => "https://www.googleapis.com/auth/replicapool.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ReplicapoolReadonly
    }
}

