use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View monetary and non-monetary YouTube Analytics reports for your YouTube content
    YtAnalyticMonetaryReadonly,

    /// View YouTube Analytics reports for your YouTube content
    YtAnalyticReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::YtAnalyticMonetaryReadonly => "https://www.googleapis.com/auth/yt-analytics-monetary.readonly",
            Scope::YtAnalyticReadonly => "https://www.googleapis.com/auth/yt-analytics.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::YtAnalyticMonetaryReadonly
    }
}

