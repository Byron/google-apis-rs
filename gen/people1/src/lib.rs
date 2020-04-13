// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *People Service* crate version *1.0.13+20200407*, where *20200407* is the exact revision of the *people:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.
//! 
//! Everything else about the *People Service* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/people/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/people1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.PeopleService.html) ... 
//! 
//! * [contact groups](struct.ContactGroup.html)
//!  * [*batch get*](struct.ContactGroupBatchGetCall.html), [*create*](struct.ContactGroupCreateCall.html), [*delete*](struct.ContactGroupDeleteCall.html), [*get*](struct.ContactGroupGetCall.html), [*list*](struct.ContactGroupListCall.html), [*members modify*](struct.ContactGroupMemberModifyCall.html) and [*update*](struct.ContactGroupUpdateCall.html)
//! * people
//!  * [*connections list*](struct.PeopleConnectionListCall.html), [*create contact*](struct.PeopleCreateContactCall.html), [*delete contact*](struct.PeopleDeleteContactCall.html), [*delete contact photo*](struct.PeopleDeleteContactPhotoCall.html), [*get*](struct.PeopleGetCall.html), [*get batch get*](struct.PeopleGetBatchGetCall.html), [*update contact*](struct.PeopleUpdateContactCall.html) and [*update contact photo*](struct.PeopleUpdateContactPhotoCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.PeopleService.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.contact_groups().batch_get(...).doit()
//! let r = hub.contact_groups().list(...).doit()
//! let r = hub.contact_groups().create(...).doit()
//! let r = hub.contact_groups().update(...).doit()
//! let r = hub.contact_groups().delete(...).doit()
//! let r = hub.contact_groups().members_modify(...).doit()
//! let r = hub.contact_groups().get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-people1 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_people1 as people1;
//! use people1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use people1::PeopleService;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.contact_groups().list()
//!              .sync_token("sed")
//!              .page_token("et")
//!              .page_size(-18)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View your complete date of birth
    UserBirthdayRead,

    /// See and download your contacts
    ContactReadonly,

    /// See your education, work history and org info
    UserOrganizationRead,

    /// View your email addresses
    UserEmailRead,

    /// View your street addresses
    UserAddresseRead,

    /// See, edit, download, and permanently delete your contacts
    Contact,

    /// View your phone numbers
    UserPhonenumberRead,

    /// See your personal info, including any personal info you've made publicly available
    UserinfoProfile,

    /// View your email address
    UserinfoEmail,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::UserBirthdayRead => "https://www.googleapis.com/auth/user.birthday.read",
            Scope::ContactReadonly => "https://www.googleapis.com/auth/contacts.readonly",
            Scope::UserOrganizationRead => "https://www.googleapis.com/auth/user.organization.read",
            Scope::UserEmailRead => "https://www.googleapis.com/auth/user.emails.read",
            Scope::UserAddresseRead => "https://www.googleapis.com/auth/user.addresses.read",
            Scope::Contact => "https://www.googleapis.com/auth/contacts",
            Scope::UserPhonenumberRead => "https://www.googleapis.com/auth/user.phonenumbers.read",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ContactReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all PeopleService related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_people1 as people1;
/// use people1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use people1::PeopleService;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().list()
///              .sync_token("kasd")
///              .page_token("accusam")
///              .page_size(-8)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct PeopleService<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for PeopleService<C, A> {}

impl<'a, C, A> PeopleService<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> PeopleService<C, A> {
        PeopleService {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.13".to_string(),
            _base_url: "https://people.googleapis.com/".to_string(),
            _root_url: "https://people.googleapis.com/".to_string(),
        }
    }

    pub fn contact_groups(&'a self) -> ContactGroupMethods<'a, C, A> {
        ContactGroupMethods { hub: &self }
    }
    pub fn people(&'a self) -> PeopleMethods<'a, C, A> {
        PeopleMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.13`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://people.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://people.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The response to a list contact groups request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list contact groups](struct.ContactGroupListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListContactGroupsResponse {
    /// The token that can be used to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of contact groups. Members of the contact groups are not
    /// populated.
    #[serde(rename="contactGroups")]
    pub contact_groups: Option<Vec<ContactGroup>>,
    /// The token that can be used to retrieve changes since the last request.
    #[serde(rename="nextSyncToken")]
    pub next_sync_token: Option<String>,
    /// The total number of items in the list without pagination.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
}

impl ResponseResult for ListContactGroupsResponse {}


/// The response to a request for the authenticated user's connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [connections list people](struct.PeopleConnectionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectionsResponse {
    /// The list of people that the requestor is connected to.
    pub connections: Option<Vec<Person>>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A token, which can be sent as `sync_token` to retrieve changes since the
    /// last request. Request must set `request_sync_token` to return the sync
    /// token.
    #[serde(rename="nextSyncToken")]
    pub next_sync_token: Option<String>,
    /// The total number of items in the list without pagination.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
    /// **DEPRECATED** (Please use totalItems)
    /// The total number of people in the list without pagination.
    #[serde(rename="totalPeople")]
    pub total_people: Option<i32>,
}

impl ResponseResult for ListConnectionsResponse {}


/// **DEPRECATED**: No data will be returned
/// A person's relationship status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipStatus {
    /// Output only. The value of the relationship status translated and formatted in
    /// the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The relationship status. The value can be custom or one of these
    /// predefined values:
    /// 
    /// * `single`
    /// * `inARelationship`
    /// * `engaged`
    /// * `married`
    /// * `itsComplicated`
    /// * `openRelationship`
    /// * `widowed`
    /// * `inDomesticPartnership`
    /// * `inCivilUnion`
    pub value: Option<String>,
    /// Metadata about the relationship status.
    pub metadata: Option<FieldMetadata>,
}

impl Part for RelationshipStatus {}


/// The response for a single person
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonResponse {
    /// The original requested resource name. May be different than the resource
    /// name on the returned person.
    /// 
    /// The resource name can change when adding or removing fields that link a
    /// contact and profile such as a verified email, verified phone number, or a
    /// profile URL.
    #[serde(rename="requestedResourceName")]
    pub requested_resource_name: Option<String>,
    /// The person.
    pub person: Option<Person>,
    /// **DEPRECATED** (Please use status instead)
    /// 
    /// [HTTP 1.1 status code]
    /// (http://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html).
    #[serde(rename="httpStatusCode")]
    pub http_status_code: Option<i32>,
    /// The status of the response.
    pub status: Option<Status>,
}

impl Part for PersonResponse {}


/// Arbitrary user data that is populated by the end users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDefined {
    /// The end user specified key of the user defined data.
    pub key: Option<String>,
    /// The end user specified value of the user defined data.
    pub value: Option<String>,
    /// Metadata about the user defined data.
    pub metadata: Option<FieldMetadata>,
}

impl Part for UserDefined {}


/// **DEPRECATED**: No data will be returned
/// A brief one-line description of the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tagline {
    /// The tagline.
    pub value: Option<String>,
    /// Metadata about the tagline.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Tagline {}


/// A Google contact group membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroupMembership {
    /// Output only. The contact group ID for the contact group membership.
    #[serde(rename="contactGroupId")]
    pub contact_group_id: Option<String>,
    /// The resource name for the contact group, assigned by the server. An ASCII
    /// string, in the form of `contactGroups/{contact_group_id}`.
    /// Only contact_group_resource_name can be used for modifying memberships.
    /// Any contact group membership can be removed, but only user group or
    /// "myContacts" or "starred" system groups memberships can be added. A
    /// contact must always have at least one contact group membership.
    #[serde(rename="contactGroupResourceName")]
    pub contact_group_resource_name: Option<String>,
}

impl Part for ContactGroupMembership {}


/// The source of a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// Output only. **Only populated in `person.metadata.sources`.**
    /// 
    /// Last update timestamp of this source.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// **Only populated in `person.metadata.sources`.**
    /// 
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the
    /// source. Used for web cache validation.
    pub etag: Option<String>,
    /// The source type.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The unique identifier within the source type generated by the server.
    pub id: Option<String>,
    /// Output only. **Only populated in `person.metadata.sources`.**
    /// 
    /// Metadata about a source of type PROFILE.
    #[serde(rename="profileMetadata")]
    pub profile_metadata: Option<ProfileMetadata>,
}

impl Part for Source {}


/// One of the person's interests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Interest {
    /// The interest; for example, `stargazing`.
    pub value: Option<String>,
    /// Metadata about the interest.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Interest {}


/// A request to update an existing user contact group. All updated fields will
/// be replaced.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact groups](struct.ContactGroupUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateContactGroupRequest {
    /// Required. The contact group to update.
    #[serde(rename="contactGroup")]
    pub contact_group: Option<ContactGroup>,
}

impl RequestValue for UpdateContactGroupRequest {}


/// The response for updating a contact's photo.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact photo people](struct.PeopleUpdateContactPhotoCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateContactPhotoResponse {
    /// The updated person, if person_fields is set in the
    /// UpdateContactPhotoRequest; otherwise this will be unset.
    pub person: Option<Person>,
}

impl ResponseResult for UpdateContactPhotoResponse {}


/// A person's cover photo. A large image shown on the person's
/// profile page that represents who they are or what they care about.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CoverPhoto {
    /// True if the cover photo is the default cover photo;
    /// false if the cover photo is a user-provided cover photo.
    pub default: Option<bool>,
    /// The URL of the cover photo.
    pub url: Option<String>,
    /// Metadata about the cover photo.
    pub metadata: Option<FieldMetadata>,
}

impl Part for CoverPhoto {}


/// Metadata about a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldMetadata {
    /// The source of the field.
    pub source: Option<Source>,
    /// Output only. True if the field is verified; false if the field is unverified. A
    /// verified field is typically a name, email address, phone number, or
    /// website that has been confirmed to be owned by the person.
    pub verified: Option<bool>,
    /// True if the field is the primary field; false if the field is a secondary
    /// field.
    pub primary: Option<bool>,
}

impl Part for FieldMetadata {}


/// The response to a get request for a list of people by resource name.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get batch get people](struct.PeopleGetBatchGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPeopleResponse {
    /// The response for each requested resource name.
    pub responses: Option<Vec<PersonResponse>>,
}

impl ResponseResult for GetPeopleResponse {}


/// **DEPRECATED**: No data will be returned
/// A person's relationship interest .
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipInterest {
    /// Output only. The value of the relationship interest translated and formatted
    /// in the viewer's account locale or the locale specified in the
    /// Accept-Language HTTP header.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The kind of relationship the person is looking for. The value can be custom
    /// or one of these predefined values:
    /// 
    /// * `friend`
    /// * `date`
    /// * `relationship`
    /// * `networking`
    pub value: Option<String>,
    /// Metadata about the relationship interest.
    pub metadata: Option<FieldMetadata>,
}

impl Part for RelationshipInterest {}


/// **DEPRECATED**: No data will be returned
/// A person's bragging rights.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BraggingRights {
    /// The bragging rights; for example, `climbed mount everest`.
    pub value: Option<String>,
    /// Metadata about the bragging rights.
    pub metadata: Option<FieldMetadata>,
}

impl Part for BraggingRights {}


/// A contact group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get contact groups](struct.ContactGroupBatchGetCall.html) (none)
/// * [list contact groups](struct.ContactGroupListCall.html) (none)
/// * [create contact groups](struct.ContactGroupCreateCall.html) (response)
/// * [update contact groups](struct.ContactGroupUpdateCall.html) (response)
/// * [delete contact groups](struct.ContactGroupDeleteCall.html) (none)
/// * [members modify contact groups](struct.ContactGroupMemberModifyCall.html) (none)
/// * [get contact groups](struct.ContactGroupGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroup {
    /// Output only. The name translated and formatted in the viewer's account locale
    /// or the `Accept-Language` HTTP header locale for system groups names.
    /// Group names set by the owner are the same as name.
    #[serde(rename="formattedName")]
    pub formatted_name: Option<String>,
    /// Output only. The contact group type.
    #[serde(rename="groupType")]
    pub group_type: Option<String>,
    /// The contact group name set by the group owner or a system provided name
    /// for system groups.
    pub name: Option<String>,
    /// Output only. The list of contact person resource names that are members of the contact
    /// group. The field is not populated for LIST requests and can only be updated
    /// through the
    /// [ModifyContactGroupMembers](/people/api/rest/v1/contactgroups/members/modify).
    #[serde(rename="memberResourceNames")]
    pub member_resource_names: Option<Vec<String>>,
    /// Output only. The total number of contacts in the group irrespective of max members in
    /// specified in the request.
    #[serde(rename="memberCount")]
    pub member_count: Option<i32>,
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the
    /// resource. Used for web cache validation.
    pub etag: Option<String>,
    /// The resource name for the contact group, assigned by the server. An ASCII
    /// string, in the form of `contactGroups/{contact_group_id}`.
    #[serde(rename="resourceName")]
    pub resource_name: Option<String>,
    /// Output only. Metadata about the contact group.
    pub metadata: Option<ContactGroupMetadata>,
}

impl Resource for ContactGroup {}
impl ResponseResult for ContactGroup {}


/// A person's membership in a group. Only contact group memberships can be
/// modified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Membership {
    /// The contact group membership.
    #[serde(rename="contactGroupMembership")]
    pub contact_group_membership: Option<ContactGroupMembership>,
    /// Output only. The domain membership.
    #[serde(rename="domainMembership")]
    pub domain_membership: Option<DomainMembership>,
    /// Metadata about the membership.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Membership {}


/// A person's birthday. At least one of the `date` and `text` fields are
/// specified. The `date` and `text` fields typically represent the same
/// date, but are not guaranteed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Birthday {
    /// The date of the birthday.
    pub date: Option<Date>,
    /// A free-form string representing the user's birthday.
    pub text: Option<String>,
    /// Metadata about the birthday.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Birthday {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete contact groups](struct.ContactGroupDeleteCall.html) (response)
/// * [delete contact people](struct.PeopleDeleteContactCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// A person's physical address. May be a P.O. box or street address. All fields
/// are optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// The city of the address.
    pub city: Option<String>,
    /// The [ISO 3166-1 alpha-2](http://www.iso.org/iso/country_codes.htm) country
    /// code of the address.
    #[serde(rename="countryCode")]
    pub country_code: Option<String>,
    /// Output only. The type of the address translated and formatted in the viewer's
    /// account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The unstructured value of the address. If this is not set by the user it
    /// will be automatically constructed from structured values.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The region of the address; for example, the state or province.
    pub region: Option<String>,
    /// The P.O. box of the address.
    #[serde(rename="poBox")]
    pub po_box: Option<String>,
    /// The street address.
    #[serde(rename="streetAddress")]
    pub street_address: Option<String>,
    /// The country of the address.
    pub country: Option<String>,
    /// The postal code of the address.
    #[serde(rename="postalCode")]
    pub postal_code: Option<String>,
    /// The extended address of the address; for example, the apartment number.
    #[serde(rename="extendedAddress")]
    pub extended_address: Option<String>,
    /// The type of the address. The type can be custom or one of these predefined
    /// values:
    /// 
    /// * `home`
    /// * `work`
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Metadata about the address.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Address {}


/// Represents a whole or partial calendar date, e.g. a birthday. The time of day
/// and time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. This can represent:
/// 
/// * A full date, with non-zero year, month and day values
/// * A month and day value, with a zero year, e.g. an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, e.g. a credit card expiration date
/// 
/// Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    pub month: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    pub day: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    pub year: Option<i32>,
}

impl Part for Date {}


/// A person's email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    /// Output only. The type of the email address translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the email address. The type can be custom or one of these
    /// predefined values:
    /// 
    /// * `home`
    /// * `work`
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The display name of the email.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The email address.
    pub value: Option<String>,
    /// Metadata about the email address.
    pub metadata: Option<FieldMetadata>,
}

impl Part for EmailAddress {}


/// A person's SIP address. Session Initial Protocol addresses are used for VoIP
/// communications to make voice or video calls over the internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SipAddress {
    /// Output only. The type of the SIP address translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the SIP address. The type can be custom or or one of these
    /// predefined values:
    /// 
    /// * `home`
    /// * `work`
    /// * `mobile`
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The SIP address in the
    /// [RFC 3261 19.1](https://tools.ietf.org/html/rfc3261#section-19.1) SIP URI
    /// format.
    pub value: Option<String>,
    /// Metadata about the SIP address.
    pub metadata: Option<FieldMetadata>,
}

impl Part for SipAddress {}


/// A person's nickname.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Nickname {
    /// The type of the nickname.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The nickname.
    pub value: Option<String>,
    /// Metadata about the nickname.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Nickname {}


/// The response for a specific contact group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroupResponse {
    /// The original requested resource name.
    #[serde(rename="requestedResourceName")]
    pub requested_resource_name: Option<String>,
    /// The contact group.
    #[serde(rename="contactGroup")]
    pub contact_group: Option<ContactGroup>,
    /// The status of the response.
    pub status: Option<Status>,
}

impl Part for ContactGroupResponse {}


/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// 
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for Status {}


/// A person's instant messaging client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImClient {
    /// The user name used in the IM client.
    pub username: Option<String>,
    /// Output only. The type of the IM client translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The protocol of the IM client. The protocol can be custom or one of these
    /// predefined values:
    /// 
    /// * `aim`
    /// * `msn`
    /// * `yahoo`
    /// * `skype`
    /// * `qq`
    /// * `googleTalk`
    /// * `icq`
    /// * `jabber`
    /// * `netMeeting`
    pub protocol: Option<String>,
    /// Output only. The protocol of the IM client formatted in the viewer's account
    /// locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedProtocol")]
    pub formatted_protocol: Option<String>,
    /// The type of the IM client. The type can be custom or one of these
    /// predefined values:
    /// 
    /// * `home`
    /// * `work`
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Metadata about the IM client.
    pub metadata: Option<FieldMetadata>,
}

impl Part for ImClient {}


/// A person's age range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AgeRangeType {
    /// The age range.
    #[serde(rename="ageRange")]
    pub age_range: Option<String>,
    /// Metadata about the age range.
    pub metadata: Option<FieldMetadata>,
}

impl Part for AgeRangeType {}


/// A person's name. If the name is a mononym, the family name is empty.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    /// The middle name(s) spelled as they sound.
    #[serde(rename="phoneticMiddleName")]
    pub phonetic_middle_name: Option<String>,
    /// The given name spelled as it sounds.
    #[serde(rename="phoneticGivenName")]
    pub phonetic_given_name: Option<String>,
    /// The honorific prefixes, such as `Mrs.` or `Dr.`
    #[serde(rename="honorificPrefix")]
    pub honorific_prefix: Option<String>,
    /// Output only. The display name with the last name first formatted according to
    /// the locale specified by the viewer's account or the
    /// `Accept-Language` HTTP header.
    #[serde(rename="displayNameLastFirst")]
    pub display_name_last_first: Option<String>,
    /// Output only. The display name formatted according to the locale specified by
    /// the viewer's account or the `Accept-Language` HTTP header.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The middle name(s).
    #[serde(rename="middleName")]
    pub middle_name: Option<String>,
    /// The honorific prefixes spelled as they sound.
    #[serde(rename="phoneticHonorificPrefix")]
    pub phonetic_honorific_prefix: Option<String>,
    /// The family name.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
    /// The full name spelled as it sounds.
    #[serde(rename="phoneticFullName")]
    pub phonetic_full_name: Option<String>,
    /// The family name spelled as it sounds.
    #[serde(rename="phoneticFamilyName")]
    pub phonetic_family_name: Option<String>,
    /// The honorific suffixes spelled as they sound.
    #[serde(rename="phoneticHonorificSuffix")]
    pub phonetic_honorific_suffix: Option<String>,
    /// The given name.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// The honorific suffixes, such as `Jr.`
    #[serde(rename="honorificSuffix")]
    pub honorific_suffix: Option<String>,
    /// Metadata about the name.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Name {}


/// A person's associated URLs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    /// Output only. The type of the URL translated and formatted in the viewer's
    /// account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the URL. The type can be custom or one of these predefined
    /// values:
    /// 
    /// * `home`
    /// * `work`
    /// * `blog`
    /// * `profile`
    /// * `homePage`
    /// * `ftp`
    /// * `reservations`
    /// * `appInstallPage`: website for a Google+ application.
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The URL.
    pub value: Option<String>,
    /// Metadata about the URL.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Url {}


/// A person's past or current residence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Residence {
    /// True if the residence is the person's current residence;
    /// false if the residence is a past residence.
    pub current: Option<bool>,
    /// The address of the residence.
    pub value: Option<String>,
    /// Metadata about the residence.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Residence {}


/// A request to update an existing contact's photo.
/// All requests must have a valid photo format: JPEG or PNG.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact photo people](struct.PeopleUpdateContactPhotoCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateContactPhotoRequest {
    /// Optional. A field mask to restrict which fields on the person are returned. Multiple
    /// fields can be specified by separating them with commas. Defaults to empty
    /// if not set, which will skip the post mutate get. Valid values are:
    /// 
    /// * addresses
    /// * ageRanges
    /// * biographies
    /// * birthdays
    /// * coverPhotos
    /// * emailAddresses
    /// * events
    /// * genders
    /// * imClients
    /// * interests
    /// * locales
    /// * memberships
    /// * metadata
    /// * names
    /// * nicknames
    /// * occupations
    /// * organizations
    /// * phoneNumbers
    /// * photos
    /// * relations
    /// * residences
    /// * sipAddresses
    /// * skills
    /// * urls
    /// * userDefined
    #[serde(rename="personFields")]
    pub person_fields: Option<String>,
    /// Required. Raw photo bytes
    #[serde(rename="photoBytes")]
    pub photo_bytes: Option<String>,
}

impl RequestValue for UpdateContactPhotoRequest {}


/// A request to modify an existing contact group's members. Contacts can be
/// removed from any group but they can only be added to a user group or
/// "myContacts" or "starred" system groups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [members modify contact groups](struct.ContactGroupMemberModifyCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyContactGroupMembersRequest {
    /// Optional. The resource names of the contact people to remove in the form of
    /// `people/{person_id}`.
    #[serde(rename="resourceNamesToRemove")]
    pub resource_names_to_remove: Option<Vec<String>>,
    /// Optional. The resource names of the contact people to add in the form of
    /// `people/{person_id}`.
    #[serde(rename="resourceNamesToAdd")]
    pub resource_names_to_add: Option<Vec<String>>,
}

impl RequestValue for ModifyContactGroupMembersRequest {}


/// The response to a batch get contact groups request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get contact groups](struct.ContactGroupBatchGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetContactGroupsResponse {
    /// The list of responses for each requested contact group resource.
    pub responses: Option<Vec<ContactGroupResponse>>,
}

impl ResponseResult for BatchGetContactGroupsResponse {}


/// Information about a person merged from various data sources such as the
/// authenticated user's contacts and profile data.
/// 
/// Most fields can have multiple items. The items in a field have no guaranteed
/// order, but each non-empty field is guaranteed to have exactly one field with
/// `metadata.primary` set to true.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update contact people](struct.PeopleUpdateContactCall.html) (request|response)
/// * [create contact people](struct.PeopleCreateContactCall.html) (request|response)
/// * [get people](struct.PeopleGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    /// The person's interests.
    pub interests: Option<Vec<Interest>>,
    /// **DEPRECATED**: No data will be returned
    /// The person's bragging rights.
    #[serde(rename="braggingRights")]
    pub bragging_rights: Option<Vec<BraggingRights>>,
    /// The person's street addresses.
    pub addresses: Option<Vec<Address>>,
    /// The person's nicknames.
    pub nicknames: Option<Vec<Nickname>>,
    /// The person's occupations.
    pub occupations: Option<Vec<Occupation>>,
    /// The person's phone numbers.
    #[serde(rename="phoneNumbers")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    /// The person's names.
    pub names: Option<Vec<Name>>,
    /// The resource name for the person, assigned by the server. An ASCII string
    /// with a max length of 27 characters, in the form of
    /// `people/{person_id}`.
    #[serde(rename="resourceName")]
    pub resource_name: Option<String>,
    /// Output only. The person's age ranges.
    #[serde(rename="ageRanges")]
    pub age_ranges: Option<Vec<AgeRangeType>>,
    /// The person's birthdays.
    pub birthdays: Option<Vec<Birthday>>,
    /// The person's relations.
    pub relations: Option<Vec<Relation>>,
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the
    /// resource. Used for web cache validation.
    pub etag: Option<String>,
    /// Output only. **DEPRECATED**: No data will be returned
    /// The person's relationship statuses.
    #[serde(rename="relationshipStatuses")]
    pub relationship_statuses: Option<Vec<RelationshipStatus>>,
    /// The person's instant messaging clients.
    #[serde(rename="imClients")]
    pub im_clients: Option<Vec<ImClient>>,
    /// The person's events.
    pub events: Option<Vec<Event>>,
    /// Output only. Metadata about the person.
    pub metadata: Option<PersonMetadata>,
    /// The person's SIP addresses.
    #[serde(rename="sipAddresses")]
    pub sip_addresses: Option<Vec<SipAddress>>,
    /// Output only. The person's photos.
    pub photos: Option<Vec<Photo>>,
    /// The person's residences.
    pub residences: Option<Vec<Residence>>,
    /// Output only. **DEPRECATED**: No data will be returned
    /// The person's relationship interests.
    #[serde(rename="relationshipInterests")]
    pub relationship_interests: Option<Vec<RelationshipInterest>>,
    /// Output only. The person's cover photos.
    #[serde(rename="coverPhotos")]
    pub cover_photos: Option<Vec<CoverPhoto>>,
    /// The person's locale preferences.
    pub locales: Option<Vec<Locale>>,
    /// The person's past or current organizations.
    pub organizations: Option<Vec<Organization>>,
    /// The person's user defined data.
    #[serde(rename="userDefined")]
    pub user_defined: Option<Vec<UserDefined>>,
    /// The person's skills.
    pub skills: Option<Vec<Skill>>,
    /// The person's group memberships.
    pub memberships: Option<Vec<Membership>>,
    /// Output only. **DEPRECATED**: No data will be returned
    /// The person's taglines.
    pub taglines: Option<Vec<Tagline>>,
    /// The person's associated URLs.
    pub urls: Option<Vec<Url>>,
    /// Output only. **DEPRECATED** (Please use `person.ageRanges` instead)
    /// 
    /// The person's age range.
    #[serde(rename="ageRange")]
    pub age_range: Option<String>,
    /// The person's genders.
    pub genders: Option<Vec<Gender>>,
    /// The person's email addresses.
    #[serde(rename="emailAddresses")]
    pub email_addresses: Option<Vec<EmailAddress>>,
    /// The person's biographies.
    pub biographies: Option<Vec<Biography>>,
}

impl RequestValue for Person {}
impl ResponseResult for Person {}


/// A person's gender.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Gender {
    /// Output only. The value of the gender translated and formatted in the viewer's account
    /// locale or the `Accept-Language` HTTP header locale. Unspecified or custom
    /// value are not localized.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The type of pronouns that should be used to address the person. The value
    /// can be custom or one of these predefined values:
    /// 
    /// * `male`
    /// * `female`
    /// * `other`
    #[serde(rename="addressMeAs")]
    pub address_me_as: Option<String>,
    /// The gender for the person. The gender can be custom or one of these
    /// predefined values:
    /// 
    /// * `male`
    /// * `female`
    /// * `unspecified`
    pub value: Option<String>,
    /// Metadata about the gender.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Gender {}


/// A person's phone number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    /// Output only. The canonicalized [ITU-T
    /// E.164](https://law.resource.org/pub/us/cfr/ibr/004/itu-t.E.164.1.2008.pdf)
    /// form of the phone number.
    #[serde(rename="canonicalForm")]
    pub canonical_form: Option<String>,
    /// Output only. The type of the phone number translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the phone number. The type can be custom or one of these
    /// predefined values:
    /// 
    /// * `home`
    /// * `work`
    /// * `mobile`
    /// * `homeFax`
    /// * `workFax`
    /// * `otherFax`
    /// * `pager`
    /// * `workMobile`
    /// * `workPager`
    /// * `main`
    /// * `googleVoice`
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The phone number.
    pub value: Option<String>,
    /// Metadata about the phone number.
    pub metadata: Option<FieldMetadata>,
}

impl Part for PhoneNumber {}


/// A person's past or current organization. Overlapping date ranges are
/// permitted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// Output only. The type of the organization translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The domain name associated with the organization; for example,
    /// `google.com`.
    pub domain: Option<String>,
    /// The end date when the person left the organization.
    #[serde(rename="endDate")]
    pub end_date: Option<Date>,
    /// The name of the organization.
    pub name: Option<String>,
    /// The start date when the person joined the organization.
    #[serde(rename="startDate")]
    pub start_date: Option<Date>,
    /// The symbol associated with the organization; for example, a stock ticker
    /// symbol, abbreviation, or acronym.
    pub symbol: Option<String>,
    /// The person's job title at the organization.
    pub title: Option<String>,
    /// True if the organization is the person's current organization;
    /// false if the organization is a past organization.
    pub current: Option<bool>,
    /// The person's job description at the organization.
    #[serde(rename="jobDescription")]
    pub job_description: Option<String>,
    /// The location of the organization office the person works at.
    pub location: Option<String>,
    /// The person's department at the organization.
    pub department: Option<String>,
    /// The type of the organization. The type can be custom or  one of these
    /// predefined values:
    /// 
    /// * `work`
    /// * `school`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The phonetic name of the organization.
    #[serde(rename="phoneticName")]
    pub phonetic_name: Option<String>,
    /// Metadata about the organization.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Organization {}


/// A request to create a new contact group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create contact groups](struct.ContactGroupCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateContactGroupRequest {
    /// Required. The contact group to create.
    #[serde(rename="contactGroup")]
    pub contact_group: Option<ContactGroup>,
}

impl RequestValue for CreateContactGroupRequest {}


/// A person's locale preference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Locale {
    /// The well-formed [IETF BCP 47](https://tools.ietf.org/html/bcp47)
    /// language tag representing the locale.
    pub value: Option<String>,
    /// Metadata about the locale.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Locale {}


/// A person's photo. A picture shown next to the person's name to
/// help others recognize the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// True if the photo is a default photo;
    /// false if the photo is a user-provided photo.
    pub default: Option<bool>,
    /// The URL of the photo. You can change the desired size by appending a query
    /// parameter `sz={size}` at the end of the url, where {size} is the size in
    /// pixels. Example:
    /// https://lh3.googleusercontent.com/-T_wVWLlmg7w/AAAAAAAAAAI/AAAAAAAABa8/00gzXvDBYqw/s100/photo.jpg?sz=50
    pub url: Option<String>,
    /// Metadata about the photo.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Photo {}


/// The metadata about a contact group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactGroupMetadata {
    /// Output only. True if the contact group resource has been deleted. Populated only for
    /// [`ListContactGroups`](/people/api/rest/v1/contactgroups/list) requests
    /// that include a sync token.
    pub deleted: Option<bool>,
    /// Output only. The time the group was last updated.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
}

impl Part for ContactGroupMetadata {}


/// The metadata about a person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonMetadata {
    /// Output only. True if the person resource has been deleted. Populated only for
    /// [`connections.list`](/people/api/rest/v1/people.connections/list) requests
    /// that include a sync token.
    pub deleted: Option<bool>,
    /// Output only. Any former resource names this person has had. Populated only for
    /// [`connections.list`](/people/api/rest/v1/people.connections/list) requests
    /// that include a sync token.
    /// 
    /// The resource name may change when adding or removing fields that link a
    /// contact and profile such as a verified email, verified phone number, or
    /// profile URL.
    #[serde(rename="previousResourceNames")]
    pub previous_resource_names: Option<Vec<String>>,
    /// Output only. Resource names of people linked to this resource.
    #[serde(rename="linkedPeopleResourceNames")]
    pub linked_people_resource_names: Option<Vec<String>>,
    /// Output only. **DEPRECATED** (Please use
    /// `person.metadata.sources.profileMetadata.objectType` instead)
    /// 
    /// The type of the person object.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
    /// The sources of data for the person.
    pub sources: Option<Vec<Source>>,
}

impl Part for PersonMetadata {}


/// The response to a modify contact group members request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [members modify contact groups](struct.ContactGroupMemberModifyCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyContactGroupMembersResponse {
    /// The contact people resource names that cannot be removed from their
    /// last contact group.
    #[serde(rename="canNotRemoveLastContactGroupResourceNames")]
    pub can_not_remove_last_contact_group_resource_names: Option<Vec<String>>,
    /// The contact people resource names that were not found.
    #[serde(rename="notFoundResourceNames")]
    pub not_found_resource_names: Option<Vec<String>>,
}

impl ResponseResult for ModifyContactGroupMembersResponse {}


/// The response for deleteing a contact's photo.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete contact photo people](struct.PeopleDeleteContactPhotoCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteContactPhotoResponse {
    /// The updated person, if person_fields is set in the
    /// DeleteContactPhotoRequest; otherwise this will be unset.
    pub person: Option<Person>,
}

impl ResponseResult for DeleteContactPhotoResponse {}


/// A person's relation to another person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Relation {
    /// Output only. The type of the relation translated and formatted in the viewer's
    /// account locale or the locale specified in the Accept-Language HTTP header.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// Metadata about the relation.
    pub metadata: Option<FieldMetadata>,
    /// The person's relation to the other person. The type can be custom or one of
    /// these predefined values:
    /// 
    /// * `spouse`
    /// * `child`
    /// * `mother`
    /// * `father`
    /// * `parent`
    /// * `brother`
    /// * `sister`
    /// * `friend`
    /// * `relative`
    /// * `domesticPartner`
    /// * `manager`
    /// * `assistant`
    /// * `referredBy`
    /// * `partner`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The name of the other person this relation refers to.
    pub person: Option<String>,
}

impl Part for Relation {}


/// A skill that the person has.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    /// The skill; for example, `underwater basket weaving`.
    pub value: Option<String>,
    /// Metadata about the skill.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Skill {}


/// A person's occupation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Occupation {
    /// The occupation; for example, `carpenter`.
    pub value: Option<String>,
    /// Metadata about the occupation.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Occupation {}


/// A G Suite Domain membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMembership {
    /// True if the person is in the viewer's G Suite domain.
    #[serde(rename="inViewerDomain")]
    pub in_viewer_domain: Option<bool>,
}

impl Part for DomainMembership {}


/// An event related to the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// The date of the event.
    pub date: Option<Date>,
    /// Output only. The type of the event translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the event. The type can be custom or one of these predefined
    /// values:
    /// 
    /// * `anniversary`
    /// * `other`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Metadata about the event.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Event {}


/// A person's short biography.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Biography {
    /// The content type of the biography.
    #[serde(rename="contentType")]
    pub content_type: Option<String>,
    /// The short biography.
    pub value: Option<String>,
    /// Metadata about the biography.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Biography {}


/// The metadata about a profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Output only. The user types.
    #[serde(rename="userTypes")]
    pub user_types: Option<Vec<String>>,
    /// Output only. The profile object type.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
}

impl Part for ProfileMetadata {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *contactGroup* resources.
/// It is not used directly, but through the `PeopleService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_people1 as people1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use people1::PeopleService;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `members_modify(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.contact_groups();
/// # }
/// ```
pub struct ContactGroupMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
}

impl<'a, C, A> MethodsBuilder for ContactGroupMethods<'a, C, A> {}

impl<'a, C, A> ContactGroupMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a list of contact groups owned by the authenticated user by specifying
    /// a list of contact group resource names.
    pub fn batch_get(&self) -> ContactGroupBatchGetCall<'a, C, A> {
        ContactGroupBatchGetCall {
            hub: self.hub,
            _resource_names: Default::default(),
            _max_members: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all contact groups owned by the authenticated user. Members of the
    /// contact groups are not populated.
    pub fn list(&self) -> ContactGroupListCall<'a, C, A> {
        ContactGroupListCall {
            hub: self.hub,
            _sync_token: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new contact group owned by the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateContactGroupRequest) -> ContactGroupCreateCall<'a, C, A> {
        ContactGroupCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the name of an existing contact group owned by the authenticated
    /// user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - The resource name for the contact group, assigned by the server. An ASCII
    ///                    string, in the form of `contactGroups/{contact_group_id}`.
    pub fn update(&self, request: UpdateContactGroupRequest, resource_name: &str) -> ContactGroupUpdateCall<'a, C, A> {
        ContactGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an existing contact group owned by the authenticated user by
    /// specifying a contact group resource name.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact group to delete.
    pub fn delete(&self, resource_name: &str) -> ContactGroupDeleteCall<'a, C, A> {
        ContactGroupDeleteCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _delete_contacts: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modify the members of a contact group owned by the authenticated user.
    /// 
    /// The only system contact groups that can have members added are
    /// `contactGroups/myContacts` and `contactGroups/starred`. Other system
    /// contact groups are deprecated and can only have contacts removed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - Required. The resource name of the contact group to modify.
    pub fn members_modify(&self, request: ModifyContactGroupMembersRequest, resource_name: &str) -> ContactGroupMemberModifyCall<'a, C, A> {
        ContactGroupMemberModifyCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific contact group owned by the authenticated user by specifying
    /// a contact group resource name.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact group to get.
    pub fn get(&self, resource_name: &str) -> ContactGroupGetCall<'a, C, A> {
        ContactGroupGetCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _max_members: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *people* resources.
/// It is not used directly, but through the `PeopleService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_people1 as people1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use people1::PeopleService;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `connections_list(...)`, `create_contact(...)`, `delete_contact(...)`, `delete_contact_photo(...)`, `get(...)`, `get_batch_get(...)`, `update_contact(...)` and `update_contact_photo(...)`
/// // to build up your call.
/// let rb = hub.people();
/// # }
/// ```
pub struct PeopleMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
}

impl<'a, C, A> MethodsBuilder for PeopleMethods<'a, C, A> {}

impl<'a, C, A> PeopleMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a contact's photo.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact whose photo will be deleted.
    pub fn delete_contact_photo(&self, resource_name: &str) -> PeopleDeleteContactPhotoCall<'a, C, A> {
        PeopleDeleteContactPhotoCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of the authenticated user's contacts merged with any
    /// connected profiles.
    /// 
    /// The request throws a 400 error if 'personFields' is not specified.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name to return connections for. Only `people/me` is valid.
    pub fn connections_list(&self, resource_name: &str) -> PeopleConnectionListCall<'a, C, A> {
        PeopleConnectionListCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _sync_token: Default::default(),
            _sort_order: Default::default(),
            _request_sync_token: Default::default(),
            _request_mask_include_field: Default::default(),
            _person_fields: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new contact and return the person resource for that contact.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create_contact(&self, request: Person) -> PeopleCreateContactCall<'a, C, A> {
        PeopleCreateContactCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a contact person. Any non-contact data will not be deleted.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact to delete.
    pub fn delete_contact(&self, resource_name: &str) -> PeopleDeleteContactCall<'a, C, A> {
        PeopleDeleteContactCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides information about a person by specifying a resource name. Use
    /// `people/me` to indicate the authenticated user.
    /// 
    /// The request throws a 400 error if 'personFields' is not specified.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the person to provide information about.
    ///                    - To get information about the authenticated user, specify `people/me`.
    ///                    - To get information about a google account, specify
    ///                     `people/{account_id}`.
    ///                    - To get information about a contact, specify the resource name that
    ///                      identifies the contact as returned by
    ///                    [`people.connections.list`](/people/api/rest/v1/people.connections/list).
    pub fn get(&self, resource_name: &str) -> PeopleGetCall<'a, C, A> {
        PeopleGetCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _request_mask_include_field: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update contact data for an existing contact person. Any non-contact data
    /// will not be modified.
    /// 
    /// The request throws a 400 error if `updatePersonFields` is not specified.
    /// 
    /// The request throws a 400 error if `person.metadata.sources` is not
    /// specified for the contact to be updated.
    /// 
    /// The request throws a 400 error with an error with reason
    /// `"failedPrecondition"` if `person.metadata.sources.etag` is different than
    /// the contact's etag, which indicates the contact has changed since its data
    /// was read. Clients should get the latest person and re-apply their updates
    /// to the latest person.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - The resource name for the person, assigned by the server. An ASCII string
    ///                    with a max length of 27 characters, in the form of
    ///                    `people/{person_id}`.
    pub fn update_contact(&self, request: Person, resource_name: &str) -> PeopleUpdateContactCall<'a, C, A> {
        PeopleUpdateContactCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _update_person_fields: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a contact's photo.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - Required. Person resource name
    pub fn update_contact_photo(&self, request: UpdateContactPhotoRequest, resource_name: &str) -> PeopleUpdateContactPhotoCall<'a, C, A> {
        PeopleUpdateContactPhotoCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides information about a list of specific people by specifying a list
    /// of requested resource names. Use `people/me` to indicate the authenticated
    /// user.
    /// 
    /// The request throws a 400 error if 'personFields' is not specified.
    pub fn get_batch_get(&self) -> PeopleGetBatchGetCall<'a, C, A> {
        PeopleGetBatchGetCall {
            hub: self.hub,
            _resource_names: Default::default(),
            _request_mask_include_field: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get a list of contact groups owned by the authenticated user by specifying
/// a list of contact group resource names.
///
/// A builder for the *batchGet* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().batch_get()
///              .add_resource_names("justo")
///              .max_members(-1)
///              .doit();
/// # }
/// ```
pub struct ContactGroupBatchGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_names: Vec<String>,
    _max_members: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupBatchGetCall<'a, C, A> {}

impl<'a, C, A> ContactGroupBatchGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchGetContactGroupsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.batchGet",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if self._resource_names.len() > 0 {
            for f in self._resource_names.iter() {
                params.push(("resourceNames", f.to_string()));
            }
        }
        if let Some(value) = self._max_members {
            params.push(("maxMembers", value.to_string()));
        }
        for &field in ["alt", "resourceNames", "maxMembers"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/contactGroups:batchGet";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ContactReadonly.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource names of the contact groups to get.
    ///
    /// Append the given value to the *resource names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_resource_names(mut self, new_value: &str) -> ContactGroupBatchGetCall<'a, C, A> {
        self._resource_names.push(new_value.to_string());
        self
    }
    /// Optional. Specifies the maximum number of members to return for each group. Defaults
    /// to 0 if not set, which will return zero members.
    ///
    /// Sets the *max members* query property to the given value.
    pub fn max_members(mut self, new_value: i32) -> ContactGroupBatchGetCall<'a, C, A> {
        self._max_members = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupBatchGetCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupBatchGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ContactReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupBatchGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List all contact groups owned by the authenticated user. Members of the
/// contact groups are not populated.
///
/// A builder for the *list* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().list()
///              .sync_token("erat")
///              .page_token("labore")
///              .page_size(-9)
///              .doit();
/// # }
/// ```
pub struct ContactGroupListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _sync_token: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupListCall<'a, C, A> {}

impl<'a, C, A> ContactGroupListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListContactGroupsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._sync_token {
            params.push(("syncToken", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "syncToken", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/contactGroups";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ContactReadonly.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Optional. A sync token, returned by a previous call to `contactgroups.list`.
    /// Only resources changed since the sync token was created will be returned.
    ///
    /// Sets the *sync token* query property to the given value.
    pub fn sync_token(mut self, new_value: &str) -> ContactGroupListCall<'a, C, A> {
        self._sync_token = Some(new_value.to_string());
        self
    }
    /// Optional. The next_page_token value returned from a previous call to
    /// [ListContactGroups](/people/api/rest/v1/contactgroups/list).
    /// Requests the next page of resources.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ContactGroupListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of resources to return. Valid values are between 1 and
    /// 1000, inclusive. Defaults to 30 if not set or set to 0.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ContactGroupListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupListCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ContactReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Create a new contact group owned by the authenticated user.
///
/// A builder for the *create* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// use people1::CreateContactGroupRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateContactGroupRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().create(req)
///              .doit();
/// # }
/// ```
pub struct ContactGroupCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _request: CreateContactGroupRequest,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupCreateCall<'a, C, A> {}

impl<'a, C, A> ContactGroupCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContactGroup)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/contactGroups";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreateContactGroupRequest) -> ContactGroupCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupCreateCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Update the name of an existing contact group owned by the authenticated
/// user.
///
/// A builder for the *update* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// use people1::UpdateContactGroupRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UpdateContactGroupRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().update(req, "resourceName")
///              .doit();
/// # }
/// ```
pub struct ContactGroupUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _request: UpdateContactGroupRequest,
    _resource_name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupUpdateCall<'a, C, A> {}

impl<'a, C, A> ContactGroupUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContactGroup)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        for &field in ["alt", "resourceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: UpdateContactGroupRequest) -> ContactGroupUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name for the contact group, assigned by the server. An ASCII
    /// string, in the form of `contactGroups/{contact_group_id}`.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> ContactGroupUpdateCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupUpdateCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Delete an existing contact group owned by the authenticated user by
/// specifying a contact group resource name.
///
/// A builder for the *delete* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().delete("resourceName")
///              .delete_contacts(false)
///              .doit();
/// # }
/// ```
pub struct ContactGroupDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _delete_contacts: Option<bool>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupDeleteCall<'a, C, A> {}

impl<'a, C, A> ContactGroupDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._delete_contacts {
            params.push(("deleteContacts", value.to_string()));
        }
        for &field in ["alt", "resourceName", "deleteContacts"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name of the contact group to delete.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> ContactGroupDeleteCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Optional. Set to true to also delete the contacts in the specified group.
    ///
    /// Sets the *delete contacts* query property to the given value.
    pub fn delete_contacts(mut self, new_value: bool) -> ContactGroupDeleteCall<'a, C, A> {
        self._delete_contacts = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupDeleteCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Modify the members of a contact group owned by the authenticated user.
/// 
/// The only system contact groups that can have members added are
/// `contactGroups/myContacts` and `contactGroups/starred`. Other system
/// contact groups are deprecated and can only have contacts removed.
///
/// A builder for the *members.modify* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// use people1::ModifyContactGroupMembersRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ModifyContactGroupMembersRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().members_modify(req, "resourceName")
///              .doit();
/// # }
/// ```
pub struct ContactGroupMemberModifyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _request: ModifyContactGroupMembersRequest,
    _resource_name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupMemberModifyCall<'a, C, A> {}

impl<'a, C, A> ContactGroupMemberModifyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ModifyContactGroupMembersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.members.modify",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        for &field in ["alt", "resourceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}/members:modify";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ModifyContactGroupMembersRequest) -> ContactGroupMemberModifyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the contact group to modify.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> ContactGroupMemberModifyCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupMemberModifyCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupMemberModifyCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupMemberModifyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get a specific contact group owned by the authenticated user by specifying
/// a contact group resource name.
///
/// A builder for the *get* method supported by a *contactGroup* resource.
/// It is not used directly, but through a `ContactGroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.contact_groups().get("resourceName")
///              .max_members(-66)
///              .doit();
/// # }
/// ```
pub struct ContactGroupGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _max_members: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ContactGroupGetCall<'a, C, A> {}

impl<'a, C, A> ContactGroupGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContactGroup)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.contactGroups.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._max_members {
            params.push(("maxMembers", value.to_string()));
        }
        for &field in ["alt", "resourceName", "maxMembers"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ContactReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name of the contact group to get.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> ContactGroupGetCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Optional. Specifies the maximum number of members to return. Defaults to 0 if not
    /// set, which will return zero members.
    ///
    /// Sets the *max members* query property to the given value.
    pub fn max_members(mut self, new_value: i32) -> ContactGroupGetCall<'a, C, A> {
        self._max_members = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ContactGroupGetCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ContactGroupGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ContactReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ContactGroupGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Delete a contact's photo.
///
/// A builder for the *deleteContactPhoto* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().delete_contact_photo("resourceName")
///              .person_fields("justo")
///              .doit();
/// # }
/// ```
pub struct PeopleDeleteContactPhotoCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _person_fields: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleDeleteContactPhotoCall<'a, C, A> {}

impl<'a, C, A> PeopleDeleteContactPhotoCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DeleteContactPhotoResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.deleteContactPhoto",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._person_fields {
            params.push(("personFields", value.to_string()));
        }
        for &field in ["alt", "resourceName", "personFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}:deleteContactPhoto";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name of the contact whose photo will be deleted.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleDeleteContactPhotoCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Optional. A field mask to restrict which fields on the person are returned. Multiple
    /// fields can be specified by separating them with commas. Defaults to empty
    /// if not set, which will skip the post mutate get. Valid values are:
    /// 
    /// * addresses
    /// * ageRanges
    /// * biographies
    /// * birthdays
    /// * coverPhotos
    /// * emailAddresses
    /// * events
    /// * genders
    /// * imClients
    /// * interests
    /// * locales
    /// * memberships
    /// * metadata
    /// * names
    /// * nicknames
    /// * occupations
    /// * organizations
    /// * phoneNumbers
    /// * photos
    /// * relations
    /// * residences
    /// * sipAddresses
    /// * skills
    /// * urls
    /// * userDefined
    ///
    /// Sets the *person fields* query property to the given value.
    pub fn person_fields(mut self, new_value: &str) -> PeopleDeleteContactPhotoCall<'a, C, A> {
        self._person_fields = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleDeleteContactPhotoCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleDeleteContactPhotoCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleDeleteContactPhotoCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Provides a list of the authenticated user's contacts merged with any
/// connected profiles.
/// 
/// The request throws a 400 error if 'personFields' is not specified.
///
/// A builder for the *connections.list* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().connections_list("resourceName")
///              .sync_token("et")
///              .sort_order("et")
///              .request_sync_token(true)
///              .request_mask_include_field("ipsum")
///              .person_fields("Lorem")
///              .page_token("et")
///              .page_size(-70)
///              .doit();
/// # }
/// ```
pub struct PeopleConnectionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _sync_token: Option<String>,
    _sort_order: Option<String>,
    _request_sync_token: Option<bool>,
    _request_mask_include_field: Option<String>,
    _person_fields: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleConnectionListCall<'a, C, A> {}

impl<'a, C, A> PeopleConnectionListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListConnectionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.connections.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._sync_token {
            params.push(("syncToken", value.to_string()));
        }
        if let Some(value) = self._sort_order {
            params.push(("sortOrder", value.to_string()));
        }
        if let Some(value) = self._request_sync_token {
            params.push(("requestSyncToken", value.to_string()));
        }
        if let Some(value) = self._request_mask_include_field {
            params.push(("requestMask.includeField", value.to_string()));
        }
        if let Some(value) = self._person_fields {
            params.push(("personFields", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "resourceName", "syncToken", "sortOrder", "requestSyncToken", "requestMask.includeField", "personFields", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}/connections";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ContactReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name to return connections for. Only `people/me` is valid.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Optional. A sync token, received from a previous `ListConnections` call.
    /// Provide this to retrieve only the resources changed since the last request.
    /// Sync requests that specify `sync_token` have an additional rate limit.
    /// 
    /// When syncing, all other parameters provided to `ListConnections`
    /// must match the call that provided the sync token.
    ///
    /// Sets the *sync token* query property to the given value.
    pub fn sync_token(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._sync_token = Some(new_value.to_string());
        self
    }
    /// Optional. The order in which the connections should be sorted. Defaults to
    /// `LAST_MODIFIED_ASCENDING`.
    ///
    /// Sets the *sort order* query property to the given value.
    pub fn sort_order(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._sort_order = Some(new_value.to_string());
        self
    }
    /// Optional. Whether the response should include `next_sync_token`, which can be used to
    /// get all changes since the last request. For subsequent sync requests use
    /// the `sync_token` param instead. Initial sync requests that specify
    /// `request_sync_token` have an additional rate limit.
    ///
    /// Sets the *request sync token* query property to the given value.
    pub fn request_sync_token(mut self, new_value: bool) -> PeopleConnectionListCall<'a, C, A> {
        self._request_sync_token = Some(new_value);
        self
    }
    /// Required. Comma-separated list of person fields to be included in the response. Each
    /// path should start with `person.`: for example, `person.names` or
    /// `person.photos`.
    ///
    /// Sets the *request mask.include field* query property to the given value.
    pub fn request_mask_include_field(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._request_mask_include_field = Some(new_value.to_string());
        self
    }
    /// Required. A field mask to restrict which fields on each person are returned. Multiple
    /// fields can be specified by separating them with commas. Valid values are:
    /// 
    /// * addresses
    /// * ageRanges
    /// * biographies
    /// * birthdays
    /// * coverPhotos
    /// * emailAddresses
    /// * events
    /// * genders
    /// * imClients
    /// * interests
    /// * locales
    /// * memberships
    /// * metadata
    /// * names
    /// * nicknames
    /// * occupations
    /// * organizations
    /// * phoneNumbers
    /// * photos
    /// * relations
    /// * residences
    /// * sipAddresses
    /// * skills
    /// * urls
    /// * userDefined
    ///
    /// Sets the *person fields* query property to the given value.
    pub fn person_fields(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._person_fields = Some(new_value.to_string());
        self
    }
    /// Optional. A page token, received from a previous `ListConnections` call.
    /// Provide this to retrieve the subsequent page.
    /// 
    /// When paginating, all other parameters provided to `ListConnections`
    /// must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The number of connections to include in the response. Valid values are
    /// between 1 and 2000, inclusive. Defaults to 100 if not set or set to 0.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> PeopleConnectionListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleConnectionListCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleConnectionListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ContactReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleConnectionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Create a new contact and return the person resource for that contact.
///
/// A builder for the *createContact* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// use people1::Person;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Person::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().create_contact(req)
///              .doit();
/// # }
/// ```
pub struct PeopleCreateContactCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _request: Person,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleCreateContactCall<'a, C, A> {}

impl<'a, C, A> PeopleCreateContactCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Person)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.createContact",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/people:createContact";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Person) -> PeopleCreateContactCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleCreateContactCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleCreateContactCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleCreateContactCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Delete a contact person. Any non-contact data will not be deleted.
///
/// A builder for the *deleteContact* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().delete_contact("resourceName")
///              .doit();
/// # }
/// ```
pub struct PeopleDeleteContactCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleDeleteContactCall<'a, C, A> {}

impl<'a, C, A> PeopleDeleteContactCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.deleteContact",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        for &field in ["alt", "resourceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}:deleteContact";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name of the contact to delete.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleDeleteContactCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleDeleteContactCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleDeleteContactCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleDeleteContactCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Provides information about a person by specifying a resource name. Use
/// `people/me` to indicate the authenticated user.
/// 
/// The request throws a 400 error if 'personFields' is not specified.
///
/// A builder for the *get* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().get("resourceName")
///              .request_mask_include_field("Lorem")
///              .person_fields("eos")
///              .doit();
/// # }
/// ```
pub struct PeopleGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _request_mask_include_field: Option<String>,
    _person_fields: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleGetCall<'a, C, A> {}

impl<'a, C, A> PeopleGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Person)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._request_mask_include_field {
            params.push(("requestMask.includeField", value.to_string()));
        }
        if let Some(value) = self._person_fields {
            params.push(("personFields", value.to_string()));
        }
        for &field in ["alt", "resourceName", "requestMask.includeField", "personFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ContactReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name of the person to provide information about.
    /// 
    /// - To get information about the authenticated user, specify `people/me`.
    /// - To get information about a google account, specify
    ///  `people/{account_id}`.
    /// - To get information about a contact, specify the resource name that
    ///   identifies the contact as returned by
    /// [`people.connections.list`](/people/api/rest/v1/people.connections/list).
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleGetCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Required. Comma-separated list of person fields to be included in the response. Each
    /// path should start with `person.`: for example, `person.names` or
    /// `person.photos`.
    ///
    /// Sets the *request mask.include field* query property to the given value.
    pub fn request_mask_include_field(mut self, new_value: &str) -> PeopleGetCall<'a, C, A> {
        self._request_mask_include_field = Some(new_value.to_string());
        self
    }
    /// Required. A field mask to restrict which fields on the person are returned. Multiple
    /// fields can be specified by separating them with commas. Valid values are:
    /// 
    /// * addresses
    /// * ageRanges
    /// * biographies
    /// * birthdays
    /// * coverPhotos
    /// * emailAddresses
    /// * events
    /// * genders
    /// * imClients
    /// * interests
    /// * locales
    /// * memberships
    /// * metadata
    /// * names
    /// * nicknames
    /// * occupations
    /// * organizations
    /// * phoneNumbers
    /// * photos
    /// * relations
    /// * residences
    /// * sipAddresses
    /// * skills
    /// * urls
    /// * userDefined
    ///
    /// Sets the *person fields* query property to the given value.
    pub fn person_fields(mut self, new_value: &str) -> PeopleGetCall<'a, C, A> {
        self._person_fields = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleGetCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ContactReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Update contact data for an existing contact person. Any non-contact data
/// will not be modified.
/// 
/// The request throws a 400 error if `updatePersonFields` is not specified.
/// 
/// The request throws a 400 error if `person.metadata.sources` is not
/// specified for the contact to be updated.
/// 
/// The request throws a 400 error with an error with reason
/// `"failedPrecondition"` if `person.metadata.sources.etag` is different than
/// the contact's etag, which indicates the contact has changed since its data
/// was read. Clients should get the latest person and re-apply their updates
/// to the latest person.
///
/// A builder for the *updateContact* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// use people1::Person;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Person::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().update_contact(req, "resourceName")
///              .update_person_fields("sadipscing")
///              .doit();
/// # }
/// ```
pub struct PeopleUpdateContactCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _request: Person,
    _resource_name: String,
    _update_person_fields: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleUpdateContactCall<'a, C, A> {}

impl<'a, C, A> PeopleUpdateContactCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Person)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.updateContact",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._update_person_fields {
            params.push(("updatePersonFields", value.to_string()));
        }
        for &field in ["alt", "resourceName", "updatePersonFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}:updateContact";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Person) -> PeopleUpdateContactCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name for the person, assigned by the server. An ASCII string
    /// with a max length of 27 characters, in the form of
    /// `people/{person_id}`.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleUpdateContactCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Required. A field mask to restrict which fields on the person are updated. Multiple
    /// fields can be specified by separating them with commas.
    /// All updated fields will be replaced. Valid values are:
    /// 
    /// * addresses
    /// * biographies
    /// * birthdays
    /// * emailAddresses
    /// * events
    /// * genders
    /// * imClients
    /// * interests
    /// * locales
    /// * memberships
    /// * names
    /// * nicknames
    /// * occupations
    /// * organizations
    /// * phoneNumbers
    /// * relations
    /// * residences
    /// * sipAddresses
    /// * urls
    /// * userDefined
    ///
    /// Sets the *update person fields* query property to the given value.
    pub fn update_person_fields(mut self, new_value: &str) -> PeopleUpdateContactCall<'a, C, A> {
        self._update_person_fields = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleUpdateContactCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleUpdateContactCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleUpdateContactCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Update a contact's photo.
///
/// A builder for the *updateContactPhoto* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// use people1::UpdateContactPhotoRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UpdateContactPhotoRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().update_contact_photo(req, "resourceName")
///              .doit();
/// # }
/// ```
pub struct PeopleUpdateContactPhotoCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _request: UpdateContactPhotoRequest,
    _resource_name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleUpdateContactPhotoCall<'a, C, A> {}

impl<'a, C, A> PeopleUpdateContactPhotoCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UpdateContactPhotoResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.updateContactPhoto",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resourceName", self._resource_name.to_string()));
        for &field in ["alt", "resourceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+resourceName}:updateContactPhoto";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Contact.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["resourceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: UpdateContactPhotoRequest) -> PeopleUpdateContactPhotoCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required. Person resource name
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleUpdateContactPhotoCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleUpdateContactPhotoCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleUpdateContactPhotoCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Contact`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleUpdateContactPhotoCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Provides information about a list of specific people by specifying a list
/// of requested resource names. Use `people/me` to indicate the authenticated
/// user.
/// 
/// The request throws a 400 error if 'personFields' is not specified.
///
/// A builder for the *getBatchGet* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_people1 as people1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use people1::PeopleService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PeopleService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().get_batch_get()
///              .add_resource_names("eirmod")
///              .request_mask_include_field("elitr")
///              .person_fields("amet")
///              .doit();
/// # }
/// ```
pub struct PeopleGetBatchGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_names: Vec<String>,
    _request_mask_include_field: Option<String>,
    _person_fields: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleGetBatchGetCall<'a, C, A> {}

impl<'a, C, A> PeopleGetBatchGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetPeopleResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.getBatchGet",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if self._resource_names.len() > 0 {
            for f in self._resource_names.iter() {
                params.push(("resourceNames", f.to_string()));
            }
        }
        if let Some(value) = self._request_mask_include_field {
            params.push(("requestMask.includeField", value.to_string()));
        }
        if let Some(value) = self._person_fields {
            params.push(("personFields", value.to_string()));
        }
        for &field in ["alt", "resourceNames", "requestMask.includeField", "personFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/people:batchGet";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ContactReadonly.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource names of the people to provide information about.
    /// 
    /// - To get information about the authenticated user, specify `people/me`.
    /// - To get information about a google account, specify
    ///   `people/{account_id}`.
    /// - To get information about a contact, specify the resource name that
    ///   identifies the contact as returned by
    /// [`people.connections.list`](/people/api/rest/v1/people.connections/list).
    /// 
    /// You can include up to 50 resource names in one request.
    ///
    /// Append the given value to the *resource names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_resource_names(mut self, new_value: &str) -> PeopleGetBatchGetCall<'a, C, A> {
        self._resource_names.push(new_value.to_string());
        self
    }
    /// Required. Comma-separated list of person fields to be included in the response. Each
    /// path should start with `person.`: for example, `person.names` or
    /// `person.photos`.
    ///
    /// Sets the *request mask.include field* query property to the given value.
    pub fn request_mask_include_field(mut self, new_value: &str) -> PeopleGetBatchGetCall<'a, C, A> {
        self._request_mask_include_field = Some(new_value.to_string());
        self
    }
    /// Required. A field mask to restrict which fields on each person are returned. Multiple
    /// fields can be specified by separating them with commas. Valid values are:
    /// 
    /// * addresses
    /// * ageRanges
    /// * biographies
    /// * birthdays
    /// * coverPhotos
    /// * emailAddresses
    /// * events
    /// * genders
    /// * imClients
    /// * interests
    /// * locales
    /// * memberships
    /// * metadata
    /// * names
    /// * nicknames
    /// * occupations
    /// * organizations
    /// * phoneNumbers
    /// * photos
    /// * relations
    /// * residences
    /// * sipAddresses
    /// * skills
    /// * urls
    /// * userDefined
    ///
    /// Sets the *person fields* query property to the given value.
    pub fn person_fields(mut self, new_value: &str) -> PeopleGetBatchGetCall<'a, C, A> {
        self._person_fields = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PeopleGetBatchGetCall<'a, C, A> {
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
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleGetBatchGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ContactReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PeopleGetBatchGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


