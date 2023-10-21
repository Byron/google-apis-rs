use super::*;
/// Associates `members` with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__Binding {
    /// The condition that is associated with this binding.
    /// NOTE: An unsatisfied condition will not allow user access via current
    /// binding. Different bindings, including their conditions, are examined
    /// independently.
    
    pub condition: Option<GoogleType__Expr>,
    /// Specifies the identities requesting access for a Cloud Platform resource.
    /// `members` can have the following values:
    /// 
    /// * `allUsers`: A special identifier that represents anyone who is
    ///    on the internet; with or without a Google account.
    /// 
    /// * `allAuthenticatedUsers`: A special identifier that represents anyone
    ///    who is authenticated with a Google account or a service account.
    /// 
    /// * `user:{emailid}`: An email address that represents a specific Google
    ///    account. For example, `alice@gmail.com` .
    /// 
    /// 
    /// * `serviceAccount:{emailid}`: An email address that represents a service
    ///    account. For example, `my-other-app@appspot.gserviceaccount.com`.
    /// 
    /// * `group:{emailid}`: An email address that represents a Google group.
    ///    For example, `admins@example.com`.
    /// 
    /// 
    /// * `domain:{domain}`: The G Suite domain (primary) that represents all the
    ///    users of that domain. For example, `google.com` or `example.com`.
    /// 
    /// 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to `members`.
    /// For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1__Binding {}


/// Defines an Identity and Access Management (IAM) policy. It is used to
/// specify access control policies for Cloud Platform resources.
/// 
/// A `Policy` consists of a list of `bindings`. A `binding` binds a list of
/// `members` to a `role`, where the members can be user accounts, Google groups,
/// Google domains, and service accounts. A `role` is a named list of permissions
/// defined by IAM.
/// 
/// **JSON Example**
/// 
/// ````text
/// {
///   "bindings": [
///     {
///       "role": "roles/owner",
///       "members": [
///         "user:mike@example.com",
///         "group:admins@example.com",
///         "domain:google.com",
///         "serviceAccount:my-other-app@appspot.gserviceaccount.com"
///       ]
///     },
///     {
///       "role": "roles/viewer",
///       "members": ["user:sean@example.com"]
///     }
///   ]
/// }
/// ````
/// 
/// **YAML Example**
/// 
/// ````text
/// bindings:
/// - members:
///   - user:mike@example.com
///   - group:admins@example.com
///   - domain:google.com
///   - serviceAccount:my-other-app@appspot.gserviceaccount.com
///   role: roles/owner
/// - members:
///   - user:sean@example.com
///   role: roles/viewer
/// ````
/// 
/// For a description of IAM and its features, see the
/// [IAM developer’s guide](https://cloud.google.com/iam/docs).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy](MethodGetIamPolicyCall) (response)
/// * [set iam policy](MethodSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__Policy {
    /// Associates a list of `members` to a `role`.
    /// `bindings` with no members will result in an error.
    
    pub bindings: Option<Vec<GoogleIamV1__Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a policy from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform policy updates in order to avoid race
    /// conditions: An `etag` is returned in the response to `getIamPolicy`, and
    /// systems are expected to put that etag in the request to `setIamPolicy` to
    /// ensure that their change will be applied to the same version of the policy.
    /// 
    /// If no `etag` is provided in the call to `setIamPolicy`, then the existing
    /// policy is overwritten blindly.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Deprecated.
    
    pub version: Option<i32>,
}

impl client::ResponseResult for GoogleIamV1__Policy {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy](MethodSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of
    /// the policy is limited to a few 10s of KB. An empty policy is a
    /// valid policy but certain Cloud Platform services (such as Projects)
    /// might reject them.
    
    pub policy: Option<GoogleIamV1__Policy>,
}

impl client::RequestValue for GoogleIamV1__SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions](MethodTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with
    /// wildcards (such as '*' or 'storage.*') are not allowed. For more
    /// information see
    /// [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleIamV1__TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions](MethodTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is
    /// allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleIamV1__TestIamPermissionsResponse {}


/// Represents an expression text. Example:
/// 
/// ````text
/// title: "User account presence"
/// description: "Determines whether the request has a user account"
/// expression: "size(request.user) > 0"
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleType__Expr {
    /// An optional description of the expression. This is a longer text which
    /// describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in
    /// Common Expression Language syntax.
    /// 
    /// The application context of the containing message determines which
    /// well-known feature set of CEL is supported.
    
    pub expression: Option<String>,
    /// An optional string indicating the location of the expression for error
    /// reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// An optional title for the expression, i.e. a short string describing
    /// its purpose. This can be used e.g. in UIs which allow to enter the
    /// expression.
    
    pub title: Option<String>,
}

impl client::Part for GoogleType__Expr {}


