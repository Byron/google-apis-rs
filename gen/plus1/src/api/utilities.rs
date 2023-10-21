use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View your basic profile info, including your age range and language
    Login,

    /// Associate you with your personal info on Google
    Me,

    /// View your email address
    UserinfoEmail,

    /// See your personal info, including any personal info you've made publicly available
    UserinfoProfile,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Login => "https://www.googleapis.com/auth/plus.login",
            Scope::Me => "https://www.googleapis.com/auth/plus.me",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Me
    }
}

