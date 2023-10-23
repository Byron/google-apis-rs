use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects delete users](UserProjectDeleteCall) (response)
/// * [ssh public keys delete users](UserSshPublicKeyDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A response message for importing an SSH public key.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [import ssh public key users](UserImportSshPublicKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportSshPublicKeyResponse {
    /// Detailed information about import results.
    
    pub details: Option<String>,
    /// The login profile information for the user.
    #[serde(rename="loginProfile")]
    
    pub login_profile: Option<LoginProfile>,
}

impl client::ResponseResult for ImportSshPublicKeyResponse {}


/// The user profile information used for logging in to a virtual machine on Google Compute Engine.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get login profile users](UserGetLoginProfileCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoginProfile {
    /// Required. A unique user ID.
    
    pub name: Option<String>,
    /// The list of POSIX accounts associated with the user.
    #[serde(rename="posixAccounts")]
    
    pub posix_accounts: Option<Vec<PosixAccount>>,
    /// A map from SSH public key fingerprint to the associated key object.
    #[serde(rename="sshPublicKeys")]
    
    pub ssh_public_keys: Option<HashMap<String, SshPublicKey>>,
}

impl client::ResponseResult for LoginProfile {}


/// The POSIX account information associated with a Google account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosixAccount {
    /// Output only. A POSIX account identifier.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The GECOS (user information) entry for this account.
    
    pub gecos: Option<String>,
    /// The default group ID.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub gid: Option<i64>,
    /// The path to the home directory for this account.
    #[serde(rename="homeDirectory")]
    
    pub home_directory: Option<String>,
    /// Output only. The canonical resource name.
    
    pub name: Option<String>,
    /// The operating system type where this account applies.
    #[serde(rename="operatingSystemType")]
    
    pub operating_system_type: Option<PosixAccountOperatingSystemTypeEnum>,
    /// Only one POSIX account can be marked as primary.
    
    pub primary: Option<bool>,
    /// The path to the logic shell for this account.
    
    pub shell: Option<String>,
    /// System identifier for which account the username or uid applies to. By default, the empty value is used.
    #[serde(rename="systemId")]
    
    pub system_id: Option<String>,
    /// The user ID.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub uid: Option<i64>,
    /// The username of the POSIX account.
    
    pub username: Option<String>,
}

impl client::Part for PosixAccount {}


/// The SSH public key information associated with a Google account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ssh public keys create users](UserSshPublicKeyCreateCall) (request|response)
/// * [ssh public keys get users](UserSshPublicKeyGetCall) (response)
/// * [ssh public keys patch users](UserSshPublicKeyPatchCall) (request|response)
/// * [import ssh public key users](UserImportSshPublicKeyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SshPublicKey {
    /// An expiration time in microseconds since epoch.
    #[serde(rename="expirationTimeUsec")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration_time_usec: Option<i64>,
    /// Output only. The SHA-256 fingerprint of the SSH public key.
    
    pub fingerprint: Option<String>,
    /// Public key text in SSH format, defined by RFC4253 section 6.6.
    
    pub key: Option<String>,
    /// Output only. The canonical resource name.
    
    pub name: Option<String>,
}

impl client::RequestValue for SshPublicKey {}
impl client::ResponseResult for SshPublicKey {}


