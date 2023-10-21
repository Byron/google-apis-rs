use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View your location
    GlasLocation,

    /// View and manage your Glass timeline
    GlasTimeline,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::GlasLocation => "https://www.googleapis.com/auth/glass.location",
            Scope::GlasTimeline => "https://www.googleapis.com/auth/glass.timeline",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::GlasLocation
    }
}

