use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See detailed information about apps installed on Chrome browsers and devices managed by your organization
    ChromeManagementAppdetailReadonly,

    /// See reports about devices and Chrome browsers managed within your organization
    ChromeManagementReportReadonly,

    /// See basic device and telemetry information collected from Chrome OS devices or users managed within your organization
    ChromeManagementTelemetryReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ChromeManagementAppdetailReadonly => "https://www.googleapis.com/auth/chrome.management.appdetails.readonly",
            Scope::ChromeManagementReportReadonly => "https://www.googleapis.com/auth/chrome.management.reports.readonly",
            Scope::ChromeManagementTelemetryReadonly => "https://www.googleapis.com/auth/chrome.management.telemetry.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ChromeManagementAppdetailReadonly
    }
}

