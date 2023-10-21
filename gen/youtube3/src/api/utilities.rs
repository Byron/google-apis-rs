use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Manage your YouTube account
    Full,

    /// See a list of your current active channel members, their current level, and when they became a member
    ChannelMembershipCreator,

    /// See, edit, and permanently delete your YouTube videos, ratings, comments and captions
    ForceSsl,

    /// View your YouTube account
    Readonly,

    /// Manage your YouTube videos
    Upload,

    /// View and manage your assets and associated content on YouTube
    Partner,

    /// View private information of your YouTube channel relevant during the audit process with a YouTube partner
    PartnerChannelAudit,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/youtube",
            Scope::ChannelMembershipCreator => "https://www.googleapis.com/auth/youtube.channel-memberships.creator",
            Scope::ForceSsl => "https://www.googleapis.com/auth/youtube.force-ssl",
            Scope::Readonly => "https://www.googleapis.com/auth/youtube.readonly",
            Scope::Upload => "https://www.googleapis.com/auth/youtube.upload",
            Scope::Partner => "https://www.googleapis.com/auth/youtubepartner",
            Scope::PartnerChannelAudit => "https://www.googleapis.com/auth/youtubepartner-channel-audit",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}

