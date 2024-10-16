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
    /// View and manage your data across Google Cloud Platform services
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

/// Central instance to access all CloudVideoIntelligence related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_videointelligence1_beta1 as videointelligence1_beta1;
/// use videointelligence1_beta1::api::GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest;
/// use videointelligence1_beta1::{Result, Error};
/// # async fn dox() {
/// use videointelligence1_beta1::{CloudVideoIntelligence, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = CloudVideoIntelligence::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().annotate(req)
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
pub struct CloudVideoIntelligence<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for CloudVideoIntelligence<C> {}

impl<'a, C> CloudVideoIntelligence<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> CloudVideoIntelligence<C> {
        CloudVideoIntelligence {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://videointelligence.googleapis.com/".to_string(),
            _root_url: "https://videointelligence.googleapis.com/".to_string(),
        }
    }

    pub fn videos(&'a self) -> VideoMethods<'a, C> {
        VideoMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://videointelligence.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://videointelligence.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// This resource represents a long-running operation that is the result of a
/// network API call.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [annotate videos](VideoAnnotateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleLongrunning_Operation {
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: Option<bool>,
    /// The normal response of the operation in case of success.  If the original
    /// method returns no data on success, such as `Delete`, the response is
    /// `google.protobuf.Empty`.  If the original method is standard
    /// `Get`/`Create`/`Update`, the response should be the resource.  For other
    /// methods, the response should have the type `XxxResponse`, where `Xxx`
    /// is the original method name.  For example, if the original method name
    /// is `TakeSnapshot()`, the inferred response type is
    /// `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, serde_json::Value>>,
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should have the format of `operations/some/unique/name`.
    pub name: Option<String>,
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<GoogleRpc_Status>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl common::ResponseResult for GoogleLongrunning_Operation {}

/// Video annotation request.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [annotate videos](VideoAnnotateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest {
    /// Optional location where the output (in JSON format) should be stored.
    /// Currently, only [Google Cloud Storage](https://cloud.google.com/storage/)
    /// URIs are supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// google.rpc.Code.INVALID_ARGUMENT). For more information, see
    /// [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    #[serde(rename = "outputUri")]
    pub output_uri: Option<String>,
    /// Requested video annotation features.
    pub features: Option<Vec<String>>,
    /// Additional video context and/or feature-specific parameters.
    #[serde(rename = "videoContext")]
    pub video_context: Option<GoogleCloudVideointelligenceV1beta1_VideoContext>,
    /// Optional cloud region where annotation should take place. Supported cloud
    /// regions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region
    /// is specified, a region will be determined based on video file location.
    #[serde(rename = "locationId")]
    pub location_id: Option<String>,
    /// Input video location. Currently, only
    /// [Google Cloud Storage](https://cloud.google.com/storage/) URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// google.rpc.Code.INVALID_ARGUMENT). For more information, see
    /// [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    /// A video URI may include wildcards in `object-id`, and thus identify
    /// multiple videos. Supported wildcards: ‘\*’ to match 0 or more characters;
    /// ‘?’ to match 1 character. If unset, the input video should be embedded
    /// in the request as `input_content`. If set, `input_content` should be unset.
    #[serde(rename = "inputUri")]
    pub input_uri: Option<String>,
    /// The video data bytes. Encoding: base64. If unset, the input video(s)
    /// should be specified via `input_uri`. If set, `input_uri` should be unset.
    #[serde(rename = "inputContent")]
    pub input_content: Option<String>,
}

impl common::RequestValue for GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest {}

/// Video segment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleCloudVideointelligenceV1beta1_VideoSegment {
    /// End offset in microseconds (inclusive). Unset means 0.
    #[serde(rename = "endTimeOffset")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub end_time_offset: Option<i64>,
    /// Start offset in microseconds (inclusive). Unset means 0.
    #[serde(rename = "startTimeOffset")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub start_time_offset: Option<i64>,
}

impl common::Part for GoogleCloudVideointelligenceV1beta1_VideoSegment {}

/// Video context and/or feature-specific parameters.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleCloudVideointelligenceV1beta1_VideoContext {
    /// Model to use for safe search detection.
    /// Supported values: "latest" and "stable" (the default).
    #[serde(rename = "safeSearchDetectionModel")]
    pub safe_search_detection_model: Option<String>,
    /// Video segments to annotate. The segments may overlap and are not required
    /// to be contiguous or span the whole video. If unspecified, each video
    /// is treated as a single segment.
    pub segments: Option<Vec<GoogleCloudVideointelligenceV1beta1_VideoSegment>>,
    /// Model to use for label detection.
    /// Supported values: "latest" and "stable" (the default).
    #[serde(rename = "labelDetectionModel")]
    pub label_detection_model: Option<String>,
    /// Model to use for shot change detection.
    /// Supported values: "latest" and "stable" (the default).
    #[serde(rename = "shotChangeDetectionModel")]
    pub shot_change_detection_model: Option<String>,
    /// If label detection has been requested, what labels should be detected
    /// in addition to video-level labels or segment-level labels. If unspecified,
    /// defaults to `SHOT_MODE`.
    #[serde(rename = "labelDetectionMode")]
    pub label_detection_mode: Option<String>,
    /// Whether the video has been shot from a stationary (i.e. non-moving) camera.
    /// When set to true, might improve detection accuracy for moving objects.
    #[serde(rename = "stationaryCamera")]
    pub stationary_camera: Option<bool>,
}

impl common::Part for GoogleCloudVideointelligenceV1beta1_VideoContext {}

/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
///
/// * Simple to use and understand for most users
/// * Flexible enough to meet unexpected needs
///
/// # Overview
///
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
///
/// # Language mapping
///
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
///
/// # Other uses
///
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
///
/// Example uses of this error model include:
///
/// * Partial errors. If a service needs to return partial errors to the client,
///   it may embed the `Status` in the normal response to indicate the partial
///   errors.
///
/// * Workflow errors. A typical workflow has multiple steps. Each step may
///   have a `Status` message for error reporting.
///
/// * Batch operations. If a client uses batch request and batch response, the
///   `Status` message should be used directly inside batch response, one for
///   each error sub-response.
///
/// * Asynchronous operations. If an API call embeds asynchronous operation
///   results in its response, the status of those operations should be
///   represented directly using the `Status` message.
///
/// * Logging. If some API errors are stored in logs, the message `Status` could
///   be used directly after any stripping needed for security/privacy reasons.
///
/// This type is not used in any activity, and only used as *part* of another schema.
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleRpc_Status {
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
}

impl common::Part for GoogleRpc_Status {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *video* resources.
/// It is not used directly, but through the [`CloudVideoIntelligence`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_videointelligence1_beta1 as videointelligence1_beta1;
///
/// # async fn dox() {
/// use videointelligence1_beta1::{CloudVideoIntelligence, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = CloudVideoIntelligence::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotate(...)`
/// // to build up your call.
/// let rb = hub.videos();
/// # }
/// ```
pub struct VideoMethods<'a, C>
where
    C: 'a,
{
    hub: &'a CloudVideoIntelligence<C>,
}

impl<'a, C> common::MethodsBuilder for VideoMethods<'a, C> {}

impl<'a, C> VideoMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Performs asynchronous video annotation. Progress and results can be
    /// retrieved through the `google.longrunning.Operations` interface.
    /// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
    /// `Operation.response` contains `AnnotateVideoResponse` (results).
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotate(
        &self,
        request: GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest,
    ) -> VideoAnnotateCall<'a, C> {
        VideoAnnotateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Performs asynchronous video annotation. Progress and results can be
/// retrieved through the `google.longrunning.Operations` interface.
/// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
/// `Operation.response` contains `AnnotateVideoResponse` (results).
///
/// A builder for the *annotate* method supported by a *video* resource.
/// It is not used directly, but through a [`VideoMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_videointelligence1_beta1 as videointelligence1_beta1;
/// use videointelligence1_beta1::api::GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest;
/// # async fn dox() {
/// # use videointelligence1_beta1::{CloudVideoIntelligence, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = CloudVideoIntelligence::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().annotate(req)
///              .doit().await;
/// # }
/// ```
pub struct VideoAnnotateCall<'a, C>
where
    C: 'a,
{
    hub: &'a CloudVideoIntelligence<C>,
    _request: GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for VideoAnnotateCall<'a, C> {}

impl<'a, C> VideoAnnotateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GoogleLongrunning_Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "videointelligence.videos.annotate",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/videos:annotate";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
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
        new_value: GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest,
    ) -> VideoAnnotateCall<'a, C> {
        self._request = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> VideoAnnotateCall<'a, C> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> VideoAnnotateCall<'a, C>
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
    pub fn add_scope<St>(mut self, scope: St) -> VideoAnnotateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> VideoAnnotateCall<'a, C>
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
    pub fn clear_scopes(mut self) -> VideoAnnotateCall<'a, C> {
        self._scopes.clear();
        self
    }
}
