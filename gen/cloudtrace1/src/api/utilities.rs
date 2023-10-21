use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// Write Trace data for a project or application
    TraceAppend,

    /// Read Trace data for a project or application
    TraceReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::TraceAppend => "https://www.googleapis.com/auth/trace.append",
            Scope::TraceReadonly => "https://www.googleapis.com/auth/trace.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::TraceReadonly
    }
}

