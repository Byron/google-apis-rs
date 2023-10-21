use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Private Service: https://www.googleapis.com/auth/cloud-identity.devices
    CloudIdentityDevice,

    /// See your device details
    CloudIdentityDeviceLookup,

    /// Private Service: https://www.googleapis.com/auth/cloud-identity.devices.readonly
    CloudIdentityDeviceReadonly,

    /// See, change, create, and delete any of the Cloud Identity Groups that you can access, including the members of each group
    CloudIdentityGroup,

    /// See any Cloud Identity Groups that you can access, including group members and their emails
    CloudIdentityGroupReadonly,

    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudIdentityDevice => "https://www.googleapis.com/auth/cloud-identity.devices",
            Scope::CloudIdentityDeviceLookup => "https://www.googleapis.com/auth/cloud-identity.devices.lookup",
            Scope::CloudIdentityDeviceReadonly => "https://www.googleapis.com/auth/cloud-identity.devices.readonly",
            Scope::CloudIdentityGroup => "https://www.googleapis.com/auth/cloud-identity.groups",
            Scope::CloudIdentityGroupReadonly => "https://www.googleapis.com/auth/cloud-identity.groups.readonly",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudIdentityDeviceReadonly
    }
}

