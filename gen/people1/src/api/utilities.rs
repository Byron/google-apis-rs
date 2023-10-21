use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, download, and permanently delete your contacts
    Contact,

    /// See and download contact info automatically saved in your "Other contacts"
    ContactOtherReadonly,

    /// See and download your contacts
    ContactReadonly,

    /// See and download your organization's GSuite directory
    DirectoryReadonly,

    /// View your street addresses
    UserAddressRead,

    /// See and download your exact date of birth
    UserBirthdayRead,

    /// See and download all of your Google Account email addresses
    UserEmailRead,

    /// See your gender
    UserGenderRead,

    /// See your education, work history and org info
    UserOrganizationRead,

    /// See and download your personal phone numbers
    UserPhonenumberRead,

    /// See your primary Google Account email address
    UserinfoEmail,

    /// See your personal info, including any personal info you've made publicly available
    UserinfoProfile,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Contact => "https://www.googleapis.com/auth/contacts",
            Scope::ContactOtherReadonly => "https://www.googleapis.com/auth/contacts.other.readonly",
            Scope::ContactReadonly => "https://www.googleapis.com/auth/contacts.readonly",
            Scope::DirectoryReadonly => "https://www.googleapis.com/auth/directory.readonly",
            Scope::UserAddressRead => "https://www.googleapis.com/auth/user.addresses.read",
            Scope::UserBirthdayRead => "https://www.googleapis.com/auth/user.birthday.read",
            Scope::UserEmailRead => "https://www.googleapis.com/auth/user.emails.read",
            Scope::UserGenderRead => "https://www.googleapis.com/auth/user.gender.read",
            Scope::UserOrganizationRead => "https://www.googleapis.com/auth/user.organization.read",
            Scope::UserPhonenumberRead => "https://www.googleapis.com/auth/user.phonenumbers.read",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ContactOtherReadonly
    }
}

