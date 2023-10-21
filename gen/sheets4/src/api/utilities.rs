use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    DriveFile,

    /// See and download all your Google Drive files
    DriveReadonly,

    /// See, edit, create, and delete all your Google Sheets spreadsheets
    Spreadsheet,

    /// See all your Google Sheets spreadsheets
    SpreadsheetReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::Spreadsheet => "https://www.googleapis.com/auth/spreadsheets",
            Scope::SpreadsheetReadonly => "https://www.googleapis.com/auth/spreadsheets.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DriveReadonly
    }
}

