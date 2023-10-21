use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Upload messages to any Google group in your domain
    AppGroupMigration,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppGroupMigration => "https://www.googleapis.com/auth/apps.groups.migration",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppGroupMigration
    }
}

