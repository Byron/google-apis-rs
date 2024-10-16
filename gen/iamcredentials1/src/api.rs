#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all IAMCredentials related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::api::GenerateAccessTokenRequest;
/// use iamcredentials1::{Result, Error};
/// # async fn dox() {
/// use iamcredentials1::{IAMCredentials, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = IAMCredentials::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateAccessTokenRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_access_token(req, "name")
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct IAMCredentials<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for IAMCredentials<C> {}

impl<'a, C> IAMCredentials<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> IAMCredentials<C> {
        IAMCredentials {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://iamcredentials.googleapis.com/".to_string(),
            _root_url: "https://iamcredentials.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C> {
        ProjectMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://iamcredentials.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://iamcredentials.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts generate access token projects](ProjectServiceAccountGenerateAccessTokenCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenerateAccessTokenRequest {
    /// The sequence of service accounts in a delegation chain. This field is required for [delegated requests](https://cloud.google.com/iam/help/credentials/delegated-request). For [direct requests](https://cloud.google.com/iam/help/credentials/direct-request), which are more common, do not specify this field. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub delegates: Option<Vec<String>>,
    /// The desired lifetime duration of the access token in seconds. By default, the maximum allowed value is 1 hour. To set a lifetime of up to 12 hours, you can add the service account as an allowed value in an Organization Policy that enforces the `constraints/iam.allowServiceAccountCredentialLifetimeExtension` constraint. See detailed instructions at https://cloud.google.com/iam/help/credentials/lifetime If a value is not specified, the token's lifetime will be set to a default value of 1 hour.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub lifetime: Option<chrono::Duration>,
    /// Required. Code to identify the scopes to be included in the OAuth 2.0 access token. See https://developers.google.com/identity/protocols/googlescopes for more information. At least one value required.
    pub scope: Option<Vec<String>>,
}

impl common::RequestValue for GenerateAccessTokenRequest {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts generate access token projects](ProjectServiceAccountGenerateAccessTokenCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenerateAccessTokenResponse {
    /// The OAuth 2.0 access token.
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    /// Token expiration time. The expiration time is always set.
    #[serde(rename = "expireTime")]
    pub expire_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::ResponseResult for GenerateAccessTokenResponse {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts generate id token projects](ProjectServiceAccountGenerateIdTokenCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenerateIdTokenRequest {
    /// Required. The audience for the token, such as the API or account that this token grants access to.
    pub audience: Option<String>,
    /// The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub delegates: Option<Vec<String>>,
    /// Include the service account email in the token. If set to `true`, the token will contain `email` and `email_verified` claims.
    #[serde(rename = "includeEmail")]
    pub include_email: Option<bool>,
}

impl common::RequestValue for GenerateIdTokenRequest {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts generate id token projects](ProjectServiceAccountGenerateIdTokenCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenerateIdTokenResponse {
    /// The OpenId Connect ID token.
    pub token: Option<String>,
}

impl common::ResponseResult for GenerateIdTokenResponse {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts sign blob projects](ProjectServiceAccountSignBlobCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SignBlobRequest {
    /// The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub delegates: Option<Vec<String>>,
    /// Required. The bytes to sign.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub payload: Option<Vec<u8>>,
}

impl common::RequestValue for SignBlobRequest {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts sign blob projects](ProjectServiceAccountSignBlobCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SignBlobResponse {
    /// The ID of the key used to sign the blob. The key used for signing will remain valid for at least 12 hours after the blob is signed. To verify the signature, you can retrieve the public key in several formats from the following endpoints: - RSA public key wrapped in an X.509 v3 certificate: `https://www.googleapis.com/service_accounts/v1/metadata/x509/{ACCOUNT_EMAIL}` - Raw key in JSON format: `https://www.googleapis.com/service_accounts/v1/metadata/raw/{ACCOUNT_EMAIL}` - JSON Web Key (JWK): `https://www.googleapis.com/service_accounts/v1/metadata/jwk/{ACCOUNT_EMAIL}`
    #[serde(rename = "keyId")]
    pub key_id: Option<String>,
    /// The signature for the blob. Does not include the original blob. After the key pair referenced by the `key_id` response field expires, Google no longer exposes the public key that can be used to verify the blob. As a result, the receiver can no longer verify the signature.
    #[serde(rename = "signedBlob")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub signed_blob: Option<Vec<u8>>,
}

impl common::ResponseResult for SignBlobResponse {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts sign jwt projects](ProjectServiceAccountSignJwtCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SignJwtRequest {
    /// The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub delegates: Option<Vec<String>>,
    /// Required. The JWT payload to sign. Must be a serialized JSON object that contains a JWT Claims Set. For example: `{"sub": "user@example.com", "iat": 313435}` If the JWT Claims Set contains an expiration time (`exp`) claim, it must be an integer timestamp that is not in the past and no more than 12 hours in the future.
    pub payload: Option<String>,
}

impl common::RequestValue for SignJwtRequest {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [service accounts sign jwt projects](ProjectServiceAccountSignJwtCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SignJwtResponse {
    /// The ID of the key used to sign the JWT. The key used for signing will remain valid for at least 12 hours after the JWT is signed. To verify the signature, you can retrieve the public key in several formats from the following endpoints: - RSA public key wrapped in an X.509 v3 certificate: `https://www.googleapis.com/service_accounts/v1/metadata/x509/{ACCOUNT_EMAIL}` - Raw key in JSON format: `https://www.googleapis.com/service_accounts/v1/metadata/raw/{ACCOUNT_EMAIL}` - JSON Web Key (JWK): `https://www.googleapis.com/service_accounts/v1/metadata/jwk/{ACCOUNT_EMAIL}`
    #[serde(rename = "keyId")]
    pub key_id: Option<String>,
    /// The signed JWT. Contains the automatically generated header; the client-supplied payload; and the signature, which is generated using the key referenced by the `kid` field in the header. After the key pair referenced by the `key_id` response field expires, Google no longer exposes the public key that can be used to verify the JWT. As a result, the receiver can no longer verify the signature.
    #[serde(rename = "signedJwt")]
    pub signed_jwt: Option<String>,
}

impl common::ResponseResult for SignJwtResponse {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`IAMCredentials`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_iamcredentials1 as iamcredentials1;
///
/// # async fn dox() {
/// use iamcredentials1::{IAMCredentials, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = IAMCredentials::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `service_accounts_generate_access_token(...)`, `service_accounts_generate_id_token(...)`, `service_accounts_sign_blob(...)` and `service_accounts_sign_jwt(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C>
where
    C: 'a,
{
    hub: &'a IAMCredentials<C>,
}

impl<'a, C> common::MethodsBuilder for ProjectMethods<'a, C> {}

impl<'a, C> ProjectMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Generates an OAuth 2.0 access token for a service account.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub fn service_accounts_generate_access_token(
        &self,
        request: GenerateAccessTokenRequest,
        name: &str,
    ) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C> {
        ProjectServiceAccountGenerateAccessTokenCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Generates an OpenID Connect ID token for a service account.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub fn service_accounts_generate_id_token(
        &self,
        request: GenerateIdTokenRequest,
        name: &str,
    ) -> ProjectServiceAccountGenerateIdTokenCall<'a, C> {
        ProjectServiceAccountGenerateIdTokenCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Signs a blob using a service account's system-managed private key.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub fn service_accounts_sign_blob(
        &self,
        request: SignBlobRequest,
        name: &str,
    ) -> ProjectServiceAccountSignBlobCall<'a, C> {
        ProjectServiceAccountSignBlobCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Signs a JWT using a service account's system-managed private key.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    pub fn service_accounts_sign_jwt(
        &self,
        request: SignJwtRequest,
        name: &str,
    ) -> ProjectServiceAccountSignJwtCall<'a, C> {
        ProjectServiceAccountSignJwtCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Generates an OAuth 2.0 access token for a service account.
///
/// A builder for the *serviceAccounts.generateAccessToken* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::api::GenerateAccessTokenRequest;
/// # async fn dox() {
/// # use iamcredentials1::{IAMCredentials, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = IAMCredentials::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateAccessTokenRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_access_token(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectServiceAccountGenerateAccessTokenCall<'a, C>
where
    C: 'a,
{
    hub: &'a IAMCredentials<C>,
    _request: GenerateAccessTokenRequest,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectServiceAccountGenerateAccessTokenCall<'a, C> {}

impl<'a, C> ProjectServiceAccountGenerateAccessTokenCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GenerateAccessTokenResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "iamcredentials.projects.serviceAccounts.generateAccessToken",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}:generateAccessToken";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GenerateAccessTokenRequest,
    ) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(
        mut self,
        name: T,
        value: T,
    ) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(
        mut self,
        scopes: I,
    ) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Generates an OpenID Connect ID token for a service account.
///
/// A builder for the *serviceAccounts.generateIdToken* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::api::GenerateIdTokenRequest;
/// # async fn dox() {
/// # use iamcredentials1::{IAMCredentials, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = IAMCredentials::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateIdTokenRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_id_token(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectServiceAccountGenerateIdTokenCall<'a, C>
where
    C: 'a,
{
    hub: &'a IAMCredentials<C>,
    _request: GenerateIdTokenRequest,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectServiceAccountGenerateIdTokenCall<'a, C> {}

impl<'a, C> ProjectServiceAccountGenerateIdTokenCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GenerateIdTokenResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "iamcredentials.projects.serviceAccounts.generateIdToken",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}:generateIdToken";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GenerateIdTokenRequest,
    ) -> ProjectServiceAccountGenerateIdTokenCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountGenerateIdTokenCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectServiceAccountGenerateIdTokenCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountGenerateIdTokenCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectServiceAccountGenerateIdTokenCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectServiceAccountGenerateIdTokenCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectServiceAccountGenerateIdTokenCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Signs a blob using a service account's system-managed private key.
///
/// A builder for the *serviceAccounts.signBlob* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::api::SignBlobRequest;
/// # async fn dox() {
/// # use iamcredentials1::{IAMCredentials, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = IAMCredentials::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SignBlobRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_sign_blob(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectServiceAccountSignBlobCall<'a, C>
where
    C: 'a,
{
    hub: &'a IAMCredentials<C>,
    _request: SignBlobRequest,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectServiceAccountSignBlobCall<'a, C> {}

impl<'a, C> ProjectServiceAccountSignBlobCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, SignBlobResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "iamcredentials.projects.serviceAccounts.signBlob",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}:signBlob";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: SignBlobRequest,
    ) -> ProjectServiceAccountSignBlobCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountSignBlobCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectServiceAccountSignBlobCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountSignBlobCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectServiceAccountSignBlobCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectServiceAccountSignBlobCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectServiceAccountSignBlobCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Signs a JWT using a service account's system-managed private key.
///
/// A builder for the *serviceAccounts.signJwt* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::api::SignJwtRequest;
/// # async fn dox() {
/// # use iamcredentials1::{IAMCredentials, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = IAMCredentials::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SignJwtRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_sign_jwt(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectServiceAccountSignJwtCall<'a, C>
where
    C: 'a,
{
    hub: &'a IAMCredentials<C>,
    _request: SignJwtRequest,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectServiceAccountSignJwtCall<'a, C> {}

impl<'a, C> ProjectServiceAccountSignJwtCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, SignJwtResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "iamcredentials.projects.serviceAccounts.signJwt",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}:signJwt";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: SignJwtRequest) -> ProjectServiceAccountSignJwtCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountSignJwtCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectServiceAccountSignJwtCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountSignJwtCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectServiceAccountSignJwtCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectServiceAccountSignJwtCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectServiceAccountSignJwtCall<'a, C> {
        self._scopes.clear();
        self
    }
}
