use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View your circles and the people and pages in them
    PluCircleRead,

    /// View your basic profile info, including your age range and language
    PluLogin,

    /// Associate you with your personal info on Google
    PluMe,

    /// Send your photos and videos to Google+
    PluMediaUpload,

    /// View your own Google+ profile and profiles visible to you
    PluProfileRead,

    /// View your Google+ posts, comments, and stream
    PluStreamRead,

    /// View your email address
    UserinfoEmail,

    /// See your personal info, including any personal info you've made publicly available
    UserinfoProfile,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PluCircleRead => "https://www.googleapis.com/auth/plus.circles.read",
            Scope::PluLogin => "https://www.googleapis.com/auth/plus.login",
            Scope::PluMe => "https://www.googleapis.com/auth/plus.me",
            Scope::PluMediaUpload => "https://www.googleapis.com/auth/plus.media.upload",
            Scope::PluProfileRead => "https://www.googleapis.com/auth/plus.profiles.read",
            Scope::PluStreamRead => "https://www.googleapis.com/auth/plus.stream.read",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PluMe
    }
}

