use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View the activity history of your Google apps
    Activity,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Activity => "https://www.googleapis.com/auth/activity",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Activity
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Appsactivity related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appsactivity1 as appsactivity1;
/// use appsactivity1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use appsactivity1::{Appsactivity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Appsactivity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list()
///              .user_id("Lorem")
///              .source("gubergren")
///              .page_token("eos")
///              .page_size(-4)
///              .grouping_strategy("ea")
///              .drive_file_id("ipsum")
///              .drive_ancestor_id("invidunt")
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
pub struct Appsactivity<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Appsactivity<S> {}

impl<'a, S> Appsactivity<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Appsactivity<S> {
        Appsactivity {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2-beta-1".to_string(),
            _base_url: "https://www.googleapis.com/appsactivity/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, S> {
        ActivityMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2-beta-1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/appsactivity/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// An Activity resource is a combined view of multiple events. An activity has a list of individual events and a combined view of the common fields among all events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// The fields common to all of the singleEvents that make up the Activity.
    #[serde(rename="combinedEvent")]
    
    pub combined_event: Option<Event>,
    /// A list of all the Events that make up the Activity.
    #[serde(rename="singleEvents")]
    
    pub single_events: Option<Vec<Event>>,
}

impl client::Part for Activity {}


/// Represents the changes associated with an action taken by a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Additional event types. Some events may have multiple types when multiple actions are part of a single event. For example, creating a document, renaming it, and sharing it may be part of a single file-creation event.
    #[serde(rename="additionalEventTypes")]
    
    pub additional_event_types: Option<Vec<String>>,
    /// The time at which the event occurred formatted as Unix time in milliseconds.
    #[serde(rename="eventTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub event_time_millis: Option<u64>,
    /// Whether this event is caused by a user being deleted.
    #[serde(rename="fromUserDeletion")]
    
    pub from_user_deletion: Option<bool>,
    /// Extra information for move type events, such as changes in an object's parents.
    #[serde(rename="move")]
    
    pub move_: Option<Move>,
    /// Extra information for permissionChange type events, such as the user or group the new permission applies to.
    #[serde(rename="permissionChanges")]
    
    pub permission_changes: Option<Vec<PermissionChange>>,
    /// The main type of event that occurred.
    #[serde(rename="primaryEventType")]
    
    pub primary_event_type: Option<String>,
    /// Extra information for rename type events, such as the old and new names.
    
    pub rename: Option<Rename>,
    /// Information specific to the Target object modified by the event.
    
    pub target: Option<Target>,
    /// Represents the user responsible for the event.
    
    pub user: Option<User>,
}

impl client::Part for Event {}


/// The response from the list request. Contains a list of activities and a token to retrieve the next page of results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](ActivityListCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListActivitiesResponse {
    /// List of activities.
    
    pub activities: Option<Vec<Activity>>,
    /// Token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListActivitiesResponse {}


/// Contains information about changes in an object's parents as a result of a move type event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    /// The added parent(s).
    #[serde(rename="addedParents")]
    
    pub added_parents: Option<Vec<Parent>>,
    /// The removed parent(s).
    #[serde(rename="removedParents")]
    
    pub removed_parents: Option<Vec<Parent>>,
}

impl client::Part for Move {}


/// Contains information about a parent object. For example, a folder in Drive is a parent for all files within it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parent {
    /// The parent's ID.
    
    pub id: Option<String>,
    /// Whether this is the root folder.
    #[serde(rename="isRoot")]
    
    pub is_root: Option<bool>,
    /// The parent's title.
    
    pub title: Option<String>,
}

impl client::Part for Parent {}


/// Contains information about the permissions and type of access allowed with regards to a Google Drive object. This is a subset of the fields contained in a corresponding Drive Permissions object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// The name of the user or group the permission applies to.
    
    pub name: Option<String>,
    /// The ID for this permission. Corresponds to the Drive API's permission ID returned as part of the Drive Permissions resource.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// Indicates the Google Drive permissions role. The role determines a user's ability to read, write, or comment on the file.
    
    pub role: Option<String>,
    /// Indicates how widely permissions are granted.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The user's information if the type is USER.
    
    pub user: Option<User>,
    /// Whether the permission requires a link to the file.
    #[serde(rename="withLink")]
    
    pub with_link: Option<bool>,
}

impl client::Part for Permission {}


/// Contains information about a Drive object's permissions that changed as a result of a permissionChange type event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionChange {
    /// Lists all Permission objects added.
    #[serde(rename="addedPermissions")]
    
    pub added_permissions: Option<Vec<Permission>>,
    /// Lists all Permission objects removed.
    #[serde(rename="removedPermissions")]
    
    pub removed_permissions: Option<Vec<Permission>>,
}

impl client::Part for PermissionChange {}


/// Photo information for a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// The URL of the photo.
    
    pub url: Option<String>,
}

impl client::Part for Photo {}


/// Contains information about a renametype event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    /// The new title.
    #[serde(rename="newTitle")]
    
    pub new_title: Option<String>,
    /// The old title.
    #[serde(rename="oldTitle")]
    
    pub old_title: Option<String>,
}

impl client::Part for Rename {}


/// Information about the object modified by the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// The ID of the target. For example, in Google Drive, this is the file or folder ID.
    
    pub id: Option<String>,
    /// The MIME type of the target.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The name of the target. For example, in Google Drive, this is the title of the file.
    
    pub name: Option<String>,
}

impl client::Part for Target {}


/// A representation of a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A boolean which indicates whether the specified User was deleted. If true, name, photo and permission_id will be omitted.
    #[serde(rename="isDeleted")]
    
    pub is_deleted: Option<bool>,
    /// Whether the user is the authenticated user.
    #[serde(rename="isMe")]
    
    pub is_me: Option<bool>,
    /// The displayable name of the user.
    
    pub name: Option<String>,
    /// The permission ID associated with this user. Equivalent to the Drive API's permission ID for this user, returned as part of the Drive Permissions resource.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// The profile photo of the user. Not present if the user has no profile photo.
    
    pub photo: Option<Photo>,
}

impl client::Part for User {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the [`Appsactivity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appsactivity1 as appsactivity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use appsactivity1::{Appsactivity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Appsactivity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, S>
    where S: 'a {

    hub: &'a Appsactivity<S>,
}

impl<'a, S> client::MethodsBuilder for ActivityMethods<'a, S> {}

impl<'a, S> ActivityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of activities visible to the current logged in user. Visible activities are determined by the visibility settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.
    pub fn list(&self) -> ActivityListCall<'a, S> {
        ActivityListCall {
            hub: self.hub,
            _user_id: Default::default(),
            _source: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _grouping_strategy: Default::default(),
            _drive_file_id: Default::default(),
            _drive_ancestor_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns a list of activities visible to the current logged in user. Visible activities are determined by the visibility settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a [`ActivityMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_appsactivity1 as appsactivity1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use appsactivity1::{Appsactivity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Appsactivity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list()
///              .user_id("amet")
///              .source("duo")
///              .page_token("ipsum")
///              .page_size(-93)
///              .grouping_strategy("ut")
///              .drive_file_id("gubergren")
///              .drive_ancestor_id("rebum.")
///              .doit().await;
/// # }
/// ```
pub struct ActivityListCall<'a, S>
    where S: 'a {

    hub: &'a Appsactivity<S>,
    _user_id: Option<String>,
    _source: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _grouping_strategy: Option<String>,
    _drive_file_id: Option<String>,
    _drive_ancestor_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ActivityListCall<'a, S> {}

impl<'a, S> ActivityListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListActivitiesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "appsactivity.activities.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "userId", "source", "pageToken", "pageSize", "groupingStrategy", "drive.fileId", "drive.ancestorId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._user_id.as_ref() {
            params.push("userId", value);
        }
        if let Some(value) = self._source.as_ref() {
            params.push("source", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._grouping_strategy.as_ref() {
            params.push("groupingStrategy", value);
        }
        if let Some(value) = self._drive_file_id.as_ref() {
            params.push("drive.fileId", value);
        }
        if let Some(value) = self._drive_ancestor_id.as_ref() {
            params.push("drive.ancestorId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "activities";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Activity.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID used for ACL checks (does not filter the resulting event list by the assigned value). Use the special value me to indicate the currently authenticated user.
    ///
    /// Sets the *user id* query property to the given value.
    pub fn user_id(mut self, new_value: &str) -> ActivityListCall<'a, S> {
        self._user_id = Some(new_value.to_string());
        self
    }
    /// The Google service from which to return activities. Possible values of source are: 
    /// - drive.google.com
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> ActivityListCall<'a, S> {
        self._source = Some(new_value.to_string());
        self
    }
    /// A token to retrieve a specific page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivityListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of events to return on a page. The response includes a continuation token if there are more events.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ActivityListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Indicates the strategy to use when grouping singleEvents items in the associated combinedEvent object.
    ///
    /// Sets the *grouping strategy* query property to the given value.
    pub fn grouping_strategy(mut self, new_value: &str) -> ActivityListCall<'a, S> {
        self._grouping_strategy = Some(new_value.to_string());
        self
    }
    /// Identifies the Drive item to return activities for.
    ///
    /// Sets the *drive.file id* query property to the given value.
    pub fn drive_file_id(mut self, new_value: &str) -> ActivityListCall<'a, S> {
        self._drive_file_id = Some(new_value.to_string());
        self
    }
    /// Identifies the Drive folder containing the items for which to return activities.
    ///
    /// Sets the *drive.ancestor id* query property to the given value.
    pub fn drive_ancestor_id(mut self, new_value: &str) -> ActivityListCall<'a, S> {
        self._drive_ancestor_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ActivityListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Activity`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ActivityListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ActivityListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ActivityListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


