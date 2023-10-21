use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, share, and permanently delete all the calendars you can access using Google Calendar
    Full,

    /// View and edit events on all your calendars
    Event,

    /// View events on all your calendars
    EventReadonly,

    /// See and download any calendar you can access using your Google Calendar
    Readonly,

    /// View your Calendar settings
    SettingReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/calendar",
            Scope::Event => "https://www.googleapis.com/auth/calendar.events",
            Scope::EventReadonly => "https://www.googleapis.com/auth/calendar.events.readonly",
            Scope::Readonly => "https://www.googleapis.com/auth/calendar.readonly",
            Scope::SettingReadonly => "https://www.googleapis.com/auth/calendar.settings.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::EventReadonly
    }
}

