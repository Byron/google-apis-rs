use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Administer your Cloud Bigtable tables and clusters
    BigtableAdmin,

    /// Administer your Cloud Bigtable clusters
    BigtableAdminCluster,

    /// Administer your Cloud Bigtable clusters
    BigtableAdminInstance,

    /// Administer your Cloud Bigtable tables
    BigtableAdminTable,

    /// Administer your Cloud Bigtable tables and clusters
    CloudBigtableAdmin,

    /// Administer your Cloud Bigtable clusters
    CloudBigtableAdminCluster,

    /// Administer your Cloud Bigtable tables
    CloudBigtableAdminTable,

    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::BigtableAdmin => "https://www.googleapis.com/auth/bigtable.admin",
            Scope::BigtableAdminCluster => "https://www.googleapis.com/auth/bigtable.admin.cluster",
            Scope::BigtableAdminInstance => "https://www.googleapis.com/auth/bigtable.admin.instance",
            Scope::BigtableAdminTable => "https://www.googleapis.com/auth/bigtable.admin.table",
            Scope::CloudBigtableAdmin => "https://www.googleapis.com/auth/cloud-bigtable.admin",
            Scope::CloudBigtableAdminCluster => "https://www.googleapis.com/auth/cloud-bigtable.admin.cluster",
            Scope::CloudBigtableAdminTable => "https://www.googleapis.com/auth/cloud-bigtable.admin.table",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::BigtableAdmin
    }
}

