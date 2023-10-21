use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View and write monitoring data for all of your Google and third-party Cloud and API projects
    Monitoring,

    /// Publish metric data to your Google Cloud projects
    MonitoringWrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::Monitoring => "https://www.googleapis.com/auth/monitoring",
            Scope::MonitoringWrite => "https://www.googleapis.com/auth/monitoring.write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Monitoring
    }
}

