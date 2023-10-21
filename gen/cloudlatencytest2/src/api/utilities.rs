use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View monitoring data for all of your Google Cloud and API projects
    MonitoringReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::MonitoringReadonly => "https://www.googleapis.com/auth/monitoring.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::MonitoringReadonly
    }
}

