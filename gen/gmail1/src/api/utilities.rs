use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Read, compose, send, and permanently delete all your email from Gmail
    Gmai,

    /// Manage drafts and send emails when you interact with the add-on
    AddonCurrentActionCompose,

    /// View your email messages when you interact with the add-on
    AddonCurrentMessageAction,

    /// View your email message metadata when the add-on is running
    AddonCurrentMessageMetadata,

    /// View your email messages when the add-on is running
    AddonCurrentMessageReadonly,

    /// Manage drafts and send emails
    Compose,

    /// Add emails into your Gmail mailbox
    Insert,

    /// See and edit your email labels
    Label,

    /// View your email message metadata such as labels and headers, but not the email body
    Metadata,

    /// Read, compose, and send emails from your Gmail account
    Modify,

    /// View your email messages and settings
    Readonly,

    /// Send email on your behalf
    Send,

    /// See, edit, create, or change your email settings and filters in Gmail
    SettingBasic,

    /// Manage your sensitive mail settings, including who can manage your mail
    SettingSharing,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Gmai => "https://mail.google.com/",
            Scope::AddonCurrentActionCompose => "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
            Scope::AddonCurrentMessageAction => "https://www.googleapis.com/auth/gmail.addons.current.message.action",
            Scope::AddonCurrentMessageMetadata => "https://www.googleapis.com/auth/gmail.addons.current.message.metadata",
            Scope::AddonCurrentMessageReadonly => "https://www.googleapis.com/auth/gmail.addons.current.message.readonly",
            Scope::Compose => "https://www.googleapis.com/auth/gmail.compose",
            Scope::Insert => "https://www.googleapis.com/auth/gmail.insert",
            Scope::Label => "https://www.googleapis.com/auth/gmail.labels",
            Scope::Metadata => "https://www.googleapis.com/auth/gmail.metadata",
            Scope::Modify => "https://www.googleapis.com/auth/gmail.modify",
            Scope::Readonly => "https://www.googleapis.com/auth/gmail.readonly",
            Scope::Send => "https://www.googleapis.com/auth/gmail.send",
            Scope::SettingBasic => "https://www.googleapis.com/auth/gmail.settings.basic",
            Scope::SettingSharing => "https://www.googleapis.com/auth/gmail.settings.sharing",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AddonCurrentMessageReadonly
    }
}

