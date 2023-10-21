use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Delete your Google Tag Manager containers
    DeleteContainer,

    /// Manage your Google Tag Manager container and its subcomponents, excluding versioning and publishing
    EditContainer,

    /// Manage your Google Tag Manager container versions
    EditContainerversion,

    /// View and manage your Google Tag Manager accounts
    ManageAccount,

    /// Manage user permissions of your Google Tag Manager account and container
    ManageUser,

    /// Publish your Google Tag Manager container versions
    Publish,

    /// View your Google Tag Manager container and its subcomponents
    Readonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DeleteContainer => "https://www.googleapis.com/auth/tagmanager.delete.containers",
            Scope::EditContainer => "https://www.googleapis.com/auth/tagmanager.edit.containers",
            Scope::EditContainerversion => "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
            Scope::ManageAccount => "https://www.googleapis.com/auth/tagmanager.manage.accounts",
            Scope::ManageUser => "https://www.googleapis.com/auth/tagmanager.manage.users",
            Scope::Publish => "https://www.googleapis.com/auth/tagmanager.publish",
            Scope::Readonly => "https://www.googleapis.com/auth/tagmanager.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}

