use super::*;
/// Request message for ExchangeOauthToken
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [oauthtoken](MethodOauthtokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityStsV1ExchangeOauthTokenRequest {
    /// Optional. The client identifier for the OAuth 2.0 client that requested the provided token. It is REQUIRED when the [client] (https://www.rfc-editor.org/rfc/rfc6749#section-1.1) is not authenticating with the authorization server, i.e. when authentication method is [client authentication] (https://www.rfc-editor.org/rfc/rfc6749#section-3.2.1).
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Optional. The authorization code that was previously from workforce identity federation's `authorize` endpoint. Required if the flow is authorization code flow, i.e. if grant_type is 'authorization_code'
    
    pub code: Option<String>,
    /// Optional. The code verifier for the PKCE request, Google Cloud CLI originally generates it before the authorization request. PKCE is used to protect authorization code from interception attacks. See https://www.rfc-editor.org/rfc/rfc7636#section-1.1 and https://www.rfc-editor.org/rfc/rfc7636#section-3. It is required when the flow is authorization code flow, i.e. if grant_type is 'authorization_code'
    #[serde(rename="codeVerifier")]
    
    pub code_verifier: Option<String>,
    /// Required. The grant types are as follows: - 'authorization_code' : an authorization code flow, i.e. exchange of authorization code for the Oauth access token - 'refresh_token' : a refresh token flow, i.e. obtain a new access token by providing the refresh token. See https://www.rfc-editor.org/rfc/rfc6749#section-6
    #[serde(rename="grantType")]
    
    pub grant_type: Option<String>,
    /// Optional. redirect_url is required when the flow is authorization code flow i.e. if grant_type is `authorization_code` See https://www.rfc-editor.org/rfc/rfc6749#section-4.1.3
    #[serde(rename="redirectUri")]
    
    pub redirect_uri: Option<String>,
    /// Optional. The Refresh token is the credential that is used to obtain a new access token when the current access token becomes invalid or expires. Required when using refresh token flow, i.e. if `grant_type` is 'refresh_token' See https://www.rfc-editor.org/rfc/rfc6749#section-1.5 and https://www.rfc-editor.org/rfc/rfc6749#section-6
    #[serde(rename="refreshToken")]
    
    pub refresh_token: Option<String>,
    /// Optional. An optional list of scopes that are requested for the token to be returned. See https://www.rfc-editor.org/rfc/rfc6749#section-3.3 Must be a list of space-delimited, case-sensitive strings. Note: Currently, the scopes in the request are not supported
    
    pub scope: Option<String>,
}

impl client::RequestValue for GoogleIdentityStsV1ExchangeOauthTokenRequest {}


/// Response message for ExchangeOauthToken. see https://www.rfc-editor.org/rfc/rfc6749#section-5.1
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [oauthtoken](MethodOauthtokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityStsV1ExchangeOauthTokenResponse {
    /// An OAuth 2.0 security token, issued by Google, in response to the Oauth token exchange request for the authorization code and refresh token flows. The returned [access token](https://www.rfc-editor.org/rfc/rfc6749#section-4.1.4). Tokens can vary in size, depending, in part, on the size of mapped claims, up to a maximum of 12288 bytes (12 KB). Google reserves the right to change the token size and the maximum length at any time.
    
    pub access_token: Option<String>,
    /// The amount of time, in seconds, between the time when the access token was issued and the time when the access token will expires.
    
    pub expires_in: Option<i32>,
    /// A refresh token, issued by Google, in response to the OAuth token exchange request for refresh token flow
    
    pub refresh_token: Option<String>,
    /// A list of scopes associated with the returned token.
    
    pub scope: Option<String>,
    /// The type of token. Field reserved for RFC compliance. See https://www.rfc-editor.org/rfc/rfc6749#section-5.1 Note: No token_type is returned for current implementation
    
    pub token_type: Option<String>,
}

impl client::ResponseResult for GoogleIdentityStsV1ExchangeOauthTokenResponse {}


/// Request message for ExchangeToken.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [token](MethodTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityStsV1ExchangeTokenRequest {
    /// The full resource name of the identity provider; for example: `//iam.googleapis.com/projects//locations/global/workloadIdentityPools//providers/` for workload identity pool providers, or `//iam.googleapis.com/locations/global/workforcePools//providers/` for workforce pool providers. Required when exchanging an external credential for a Google access token.
    
    pub audience: Option<String>,
    /// Required. The grant type. Must be `urn:ietf:params:oauth:grant-type:token-exchange`, which indicates a token exchange.
    #[serde(rename="grantType")]
    
    pub grant_type: Option<String>,
    /// A set of features that Security Token Service supports, in addition to the standard OAuth 2.0 token exchange, formatted as a serialized JSON object of Options. The size of the parameter value must not exceed 4096 characters.
    
    pub options: Option<String>,
    /// Required. An identifier for the type of requested security token. Must be `urn:ietf:params:oauth:token-type:access_token`.
    #[serde(rename="requestedTokenType")]
    
    pub requested_token_type: Option<String>,
    /// The OAuth 2.0 scopes to include on the resulting access token, formatted as a list of space-delimited, case-sensitive strings. Required when exchanging an external credential for a Google access token.
    
    pub scope: Option<String>,
    /// Required. The input token. This token is either an external credential issued by a workload identity pool provider, or a short-lived access token issued by Google. If the token is an OIDC JWT, it must use the JWT format defined in [RFC 7523](https://tools.ietf.org/html/rfc7523), and the `subject_token_type` must be either `urn:ietf:params:oauth:token-type:jwt` or `urn:ietf:params:oauth:token-type:id_token`. The following headers are required: - `kid`: The identifier of the signing key securing the JWT. - `alg`: The cryptographic algorithm securing the JWT. Must be `RS256` or `ES256`. The following payload fields are required. For more information, see [RFC 7523, Section 3](https://tools.ietf.org/html/rfc7523#section-3): - `iss`: The issuer of the token. The issuer must provide a discovery document at the URL `/.well-known/openid-configuration`, where `` is the value of this field. The document must be formatted according to section 4.2 of the [OIDC 1.0 Discovery specification](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfigurationResponse). - `iat`: The issue time, in seconds, since the Unix epoch. Must be in the past. - `exp`: The expiration time, in seconds, since the Unix epoch. Must be less than 48 hours after `iat`. Shorter expiration times are more secure. If possible, we recommend setting an expiration time less than 6 hours. - `sub`: The identity asserted in the JWT. - `aud`: For workload identity pools, this must be a value specified in the allowed audiences for the workload identity pool provider, or one of the audiences allowed by default if no audiences were specified. See https://cloud.google.com/iam/docs/reference/rest/v1/projects.locations.workloadIdentityPools.providers#oidc. For workforce pools, this must match the client ID specified in the provider configuration. See https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers#oidc. Example header: ``` { "alg": "RS256", "kid": "us-east-11" } ``` Example payload: ``` { "iss": "https://accounts.google.com", "iat": 1517963104, "exp": 1517966704, "aud": "//iam.googleapis.com/projects/1234567890123/locations/global/workloadIdentityPools/my-pool/providers/my-provider", "sub": "113475438248934895348", "my_claims": { "additional_claim": "value" } } ``` If `subject_token` is for AWS, it must be a serialized `GetCallerIdentity` token. This token contains the same information as a request to the AWS [`GetCallerIdentity()`](https://docs.aws.amazon.com/STS/latest/APIReference/API_GetCallerIdentity) method, as well as the AWS [signature](https://docs.aws.amazon.com/general/latest/gr/signing_aws_api_requests.html) for the request information. Use Signature Version 4. Format the request as URL-encoded JSON, and set the `subject_token_type` parameter to `urn:ietf:params:aws:token-type:aws4_request`. The following parameters are required: - `url`: The URL of the AWS STS endpoint for `GetCallerIdentity()`, such as `https://sts.amazonaws.com?Action=GetCallerIdentity&Version=2011-06-15`. Regional endpoints are also supported. - `method`: The HTTP request method: `POST`. - `headers`: The HTTP request headers, which must include: - `Authorization`: The request signature. - `x-amz-date`: The time you will send the request, formatted as an [ISO8601 Basic](https://docs.aws.amazon.com/general/latest/gr/sigv4_elements.html#sigv4_elements_date) string. This value is typically set to the current time and is used to help prevent replay attacks. - `host`: The hostname of the `url` field; for example, `sts.amazonaws.com`. - `x-goog-cloud-target-resource`: The full, canonical resource name of the workload identity pool provider, with or without an `https:` prefix. To help ensure data integrity, we recommend including this header in the `SignedHeaders` field of the signed request. For example: //iam.googleapis.com/projects//locations/global/workloadIdentityPools//providers/ https://iam.googleapis.com/projects//locations/global/workloadIdentityPools//providers/ If you are using temporary security credentials provided by AWS, you must also include the header `x-amz-security-token`, with the value set to the session token. The following example shows a `GetCallerIdentity` token: ``` { "headers": [ {"key": "x-amz-date", "value": "20200815T015049Z"}, {"key": "Authorization", "value": "AWS4-HMAC-SHA256+Credential=$credential,+SignedHeaders=host;x-amz-date;x-goog-cloud-target-resource,+Signature=$signature"}, {"key": "x-goog-cloud-target-resource", "value": "//iam.googleapis.com/projects//locations/global/workloadIdentityPools//providers/"}, {"key": "host", "value": "sts.amazonaws.com"} . ], "method": "POST", "url": "https://sts.amazonaws.com?Action=GetCallerIdentity&Version=2011-06-15" } ``` If the token is a SAML 2.0 assertion, it must use the format defined in [the SAML 2.0 spec](https://www.oasis-open.org/committees/download.php/56776/sstc-saml-core-errata-2.0-wd-07.pdf), and the `subject_token_type` must be `urn:ietf:params:oauth:token-type:saml2`. See [Verification of external credentials](https://cloud.google.com/iam/docs/using-workload-identity-federation#verification_of_external_credentials) for details on how SAML 2.0 assertions are validated during token exchanges. You can also use a Google-issued OAuth 2.0 access token with this field to obtain an access token with new security attributes applied, such as a Credential Access Boundary. In this case, set `subject_token_type` to `urn:ietf:params:oauth:token-type:access_token`. If an access token already contains security attributes, you cannot apply additional security attributes.
    #[serde(rename="subjectToken")]
    
    pub subject_token: Option<String>,
    /// Required. An identifier that indicates the type of the security token in the `subject_token` parameter. Supported values are `urn:ietf:params:oauth:token-type:jwt`, `urn:ietf:params:oauth:token-type:id_token`, `urn:ietf:params:aws:token-type:aws4_request`, `urn:ietf:params:oauth:token-type:access_token`, and `urn:ietf:params:oauth:token-type:saml2`.
    #[serde(rename="subjectTokenType")]
    
    pub subject_token_type: Option<String>,
}

impl client::RequestValue for GoogleIdentityStsV1ExchangeTokenRequest {}


/// Response message for ExchangeToken.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [token](MethodTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityStsV1ExchangeTokenResponse {
    /// An OAuth 2.0 security token, issued by Google, in response to the token exchange request. Tokens can vary in size, depending in part on the size of mapped claims, up to a maximum of 12288 bytes (12 KB). Google reserves the right to change the token size and the maximum length at any time.
    
    pub access_token: Option<String>,
    /// The amount of time, in seconds, between the time when the access token was issued and the time when the access token will expire. This field is absent when the `subject_token` in the request is a Google-issued, short-lived access token. In this case, the access token has the same expiration time as the `subject_token`.
    
    pub expires_in: Option<i32>,
    /// The token type. Always matches the value of `requested_token_type` from the request.
    
    pub issued_token_type: Option<String>,
    /// The type of access token. Always has the value `Bearer`.
    
    pub token_type: Option<String>,
}

impl client::ResponseResult for GoogleIdentityStsV1ExchangeTokenResponse {}


/// Request message for IntrospectToken.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [introspect](MethodIntrospectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityStsV1IntrospectTokenRequest {
    /// Required. The OAuth 2.0 security token issued by the Security Token Service API.
    
    pub token: Option<String>,
    /// Optional. The type of the given token. Supported values are `urn:ietf:params:oauth:token-type:access_token` and `access_token`.
    #[serde(rename="tokenTypeHint")]
    
    pub token_type_hint: Option<String>,
}

impl client::RequestValue for GoogleIdentityStsV1IntrospectTokenRequest {}


/// Response message for IntrospectToken.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [introspect](MethodIntrospectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIdentityStsV1IntrospectTokenResponse {
    /// A boolean value that indicates whether the provided access token is currently active.
    
    pub active: Option<bool>,
    /// The client identifier for the OAuth 2.0 client that requested the provided token.
    
    pub client_id: Option<String>,
    /// The expiration timestamp, measured in the number of seconds since January 1 1970 UTC, indicating when this token will expire.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub exp: Option<i64>,
    /// The issued timestamp, measured in the number of seconds since January 1 1970 UTC, indicating when this token was originally issued.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub iat: Option<i64>,
    /// The issuer of the provided token.
    
    pub iss: Option<String>,
    /// A list of scopes associated with the provided token.
    
    pub scope: Option<String>,
    /// The unique user ID associated with the provided token. For Google Accounts, this value is based on the Google Account's user ID. For federated identities, this value is based on the identity pool ID and the value of the mapped `google.subject` attribute.
    
    pub sub: Option<String>,
    /// The human-readable identifier for the token principal subject. For example, if the provided token is associated with a workload identity pool, this field contains a value in the following format: `principal://iam.googleapis.com/projects//locations/global/workloadIdentityPools//subject/`. If the provided token is associated with a workforce pool, this field contains a value in the following format: `principal://iam.googleapis.com/locations/global/workforcePools//subject/`.
    
    pub username: Option<String>,
}

impl client::ResponseResult for GoogleIdentityStsV1IntrospectTokenResponse {}


