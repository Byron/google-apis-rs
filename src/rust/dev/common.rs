/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
pub enum Scope {
    /// Manage your YouTube account
    Account,
    /// View your YouTube account
    AccountReadOnly,
    /// Manage your YouTube videos, which includes uploads and meta-data changes
    Video,
    /// View and manage your assets and associated content on YouTube
    Partner,
    /// View private information of your YouTube channel relevant during the
    /// audit process with a YouTube partner.
    Audit,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::Account => "https://www.googleapis.com/auth/youtube",
            Scope::AccountReadOnly => "https://www.googleapis.com/auth/youtube.readonly",
            Scope::Video => "https://www.googleapis.com/auth/youtube.upload",
            Scope::Partner => "https://www.googleapis.com/auth/youtubepartner",
            Scope::Audit => "https://www.googleapis.com/auth/youtubepartner-channel-audit",
        }
    }
}