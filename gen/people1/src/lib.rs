// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *People Service* crate version *1.0.4+20170518*, where *20170518* is the exact revision of the *people:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.4*.
//! 
//! Everything else about the *People Service* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/people/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/people1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.PeopleService.html) ... 
//! 
//! * people
//!  * [*connections list*](struct.PeopleConnectionListCall.html), [*get*](struct.PeopleGetCall.html) and [*get batch get*](struct.PeopleGetBatchGetCall.html)
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
//! let r = hub.people().get(...).doit()
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
//! let result = hub.people().get("resourceName")
//!              .request_mask_include_field("sed")
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
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
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

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Know the list of people in your circles, your age range, and language
    PluLogin,

    /// View your complete date of birth
    UserBirthdayRead,

    /// View your contacts
    ContactReadonly,

    /// View your email addresses
    UserEmailRead,

    /// View your street addresses
    UserAddresseRead,

    /// Manage your contacts
    Contact,

    /// View your email address
    UserinfoEmail,

    /// View your basic profile info
    UserinfoProfile,

    /// View your phone numbers
    UserPhonenumberRead,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PluLogin => "https://www.googleapis.com/auth/plus.login",
            Scope::UserBirthdayRead => "https://www.googleapis.com/auth/user.birthday.read",
            Scope::ContactReadonly => "https://www.googleapis.com/auth/contacts.readonly",
            Scope::UserEmailRead => "https://www.googleapis.com/auth/user.emails.read",
            Scope::UserAddresseRead => "https://www.googleapis.com/auth/user.addresses.read",
            Scope::Contact => "https://www.googleapis.com/auth/contacts",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
            Scope::UserPhonenumberRead => "https://www.googleapis.com/auth/user.phonenumbers.read",
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
/// let result = hub.people().get("resourceName")
///              .request_mask_include_field("dolores")
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
            _user_agent: "google-api-rust-client/1.0.4".to_string(),
            _base_url: "https://people.googleapis.com/".to_string(),
            _root_url: "https://people.googleapis.com/".to_string(),
        }
    }

    pub fn people(&'a self) -> PeopleMethods<'a, C, A> {
        PeopleMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.4`.
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
/// There is no detailed description.
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
    /// The token that can be used to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of people that the requestor is connected to.
    pub connections: Option<Vec<Person>>,
    /// The token that can be used to retrieve changes since the last request.
    #[serde(rename="nextSyncToken")]
    pub next_sync_token: Option<String>,
    /// The total number of items in the list without pagination.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
    /// DEPRECATED(Please use total_items). The total number of people in the list
    /// without pagination.
    #[serde(rename="totalPeople")]
    pub total_people: Option<i32>,
}

impl ResponseResult for ListConnectionsResponse {}


/// A person's read-only relationship status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipStatus {
    /// The read-only value of the relationship status translated and formatted in
    /// the viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The relationship status. The value can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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
    /// The status of the response.
    pub status: Option<Status>,
    /// The person.
    pub person: Option<Person>,
    /// DEPRECATED(Please use status instead).
    /// [HTTP 1.1 status
    /// code](http://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html).
    #[serde(rename="httpStatusCode")]
    pub http_status_code: Option<i32>,
    /// The original requested resource name. May be different than the resource
    /// name on the returned person.
    /// 
    /// The resource name can change when adding or removing fields that link a
    /// contact and profile such as a verified email, verified phone number, or a
    /// profile URL.
    #[serde(rename="requestedResourceName")]
    pub requested_resource_name: Option<String>,
}

impl Part for PersonResponse {}


/// A read-only brief one-line description of the person.
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
    /// The contact group ID for the contact group membership. The contact group
    /// ID can be custom or predefined. Possible values include, but are not
    /// limited to, the following:
    /// 
    /// *  `myContacts`
    /// *  `starred`
    /// *  A numerical ID for user-created groups.
    #[serde(rename="contactGroupId")]
    pub contact_group_id: Option<String>,
}

impl Part for ContactGroupMembership {}


/// The source of a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the
    /// source. Used for web cache validation. Only populated in
    /// person.metadata.sources.
    pub etag: Option<String>,
    /// The source type.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The unique identifier within the source type generated by the server.
    pub id: Option<String>,
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


/// A person's read-only cover photo. A large image shown on the person's
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
    /// True if the field is verified; false if the field is unverified. A
    /// verified field is typically a name, email address, phone number, or
    /// website that has been confirmed to be owned by the person.
    pub verified: Option<bool>,
    /// True if the field is the primary field; false if the field is a secondary
    /// field.
    pub primary: Option<bool>,
}

impl Part for FieldMetadata {}


/// A person's name. If the name is a mononym, the family name is empty.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    /// The middle name(s) spelled as they sound.
    #[serde(rename="phoneticMiddleName")]
    pub phonetic_middle_name: Option<String>,
    /// The honorific prefixes, such as `Mrs.` or `Dr.`
    #[serde(rename="honorificPrefix")]
    pub honorific_prefix: Option<String>,
    /// The read-only display name with the last name first formatted according to
    /// the locale specified by the viewer's account or the
    /// <code>Accept-Language</code> HTTP header.
    #[serde(rename="displayNameLastFirst")]
    pub display_name_last_first: Option<String>,
    /// The read-only display name formatted according to the locale specified by
    /// the viewer's account or the <code>Accept-Language</code> HTTP header.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The family name spelled as it sounds.
    #[serde(rename="phoneticFamilyName")]
    pub phonetic_family_name: Option<String>,
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
    /// The given name spelled as it sounds.
    #[serde(rename="phoneticGivenName")]
    pub phonetic_given_name: Option<String>,
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


/// A person's read-only relationship interest .
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipInterest {
    /// The value of the relationship interest translated and formatted in the
    /// viewer's account locale or the locale specified in the Accept-Language
    /// HTTP header.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The kind of relationship the person is looking for. The value can be custom
    /// or predefined. Possible values include, but are not limited to, the
    /// following values:
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


/// A person's read-only membership in a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Membership {
    /// The contact group membership.
    #[serde(rename="contactGroupMembership")]
    pub contact_group_membership: Option<ContactGroupMembership>,
    /// The domain membership.
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


/// A person's physical address. May be a P.O. box or street address. All fields
/// are optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// The read-only type of the address translated and formatted in the viewer's
    /// account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The [ISO 3166-1 alpha-2](http://www.iso.org/iso/country_codes.htm) country
    /// code of the address.
    #[serde(rename="countryCode")]
    pub country_code: Option<String>,
    /// The city of the address.
    pub city: Option<String>,
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
    /// The type of the address. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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


/// Represents a whole calendar date, for example a date of birth. The time
/// of day and time zone are either specified elsewhere or are not
/// significant. The date is relative to the
/// [Proleptic Gregorian Calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar).
/// The day may be 0 to represent a year and month where the day is not
/// significant. The year may be 0 to represent a month and day independent
/// of year; for example, anniversary date.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Month of year. Must be from 1 to 12.
    pub month: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year/month where the day is not significant.
    pub day: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    pub year: Option<i32>,
}

impl Part for Date {}


/// A person's relation to another person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Relation {
    /// The name of the other person this relation refers to.
    pub person: Option<String>,
    /// The type of the relation translated and formatted in the viewer's account
    /// locale or the locale specified in the Accept-Language HTTP header.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The person's relation to the other person. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following values:
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
    /// Metadata about the relation.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Relation {}


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


/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// - Simple to use and understand for most users
/// - Flexible enough to meet unexpected needs
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
/// - Partial errors. If a service needs to return partial errors to the client,
///     it may embed the `Status` in the normal response to indicate the partial
///     errors.
/// 
/// - Workflow errors. A typical workflow has multiple steps. Each step may
///     have a `Status` message for error reporting.
/// 
/// - Batch operations. If a client uses batch request and batch response, the
///     `Status` message should be used directly inside batch response, one for
///     each error sub-response.
/// 
/// - Asynchronous operations. If an API call embeds asynchronous operation
///     results in its response, the status of those operations should be
///     represented directly using the `Status` message.
/// 
/// - Logging. If some API errors are stored in logs, the message `Status` could
///     be used directly after any stripping needed for security/privacy reasons.
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
    /// A list of messages that carry the error details.  There will be a
    /// common set of message types for APIs to use.
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
    /// The read-only type of the IM client translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The protocol of the IM client. The protocol can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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
    /// The read-only protocol of the IM client formatted in the viewer's account
    /// locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedProtocol")]
    pub formatted_protocol: Option<String>,
    /// The type of the IM client. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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


/// There is no detailed description.
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


/// A person's associated URLs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    /// The read-only type of the URL translated and formatted in the viewer's
    /// account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the URL. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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
/// * [get people](struct.PeopleGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    /// The person's interests.
    pub interests: Option<Vec<Interest>>,
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
    /// The person's photos.
    pub photos: Option<Vec<Photo>>,
    /// The person's names.
    pub names: Option<Vec<Name>>,
    /// DEPRECATED(Please read person.age_ranges instead). The person's age range.
    #[serde(rename="ageRange")]
    pub age_range: Option<String>,
    /// The person's residences.
    pub residences: Option<Vec<Residence>>,
    /// The resource name for the person, assigned by the server. An ASCII string
    /// with a max length of 27 characters, in the form of `people/<person_id>`.
    #[serde(rename="resourceName")]
    pub resource_name: Option<String>,
    /// The kind of relationship the person is looking for.
    #[serde(rename="relationshipInterests")]
    pub relationship_interests: Option<Vec<RelationshipInterest>>,
    /// The person's cover photos.
    #[serde(rename="coverPhotos")]
    pub cover_photos: Option<Vec<CoverPhoto>>,
    /// The person's locale preferences.
    pub locales: Option<Vec<Locale>>,
    /// The person's past or current organizations.
    pub organizations: Option<Vec<Organization>>,
    /// The person's taglines.
    pub taglines: Option<Vec<Tagline>>,
    /// The person's skills.
    pub skills: Option<Vec<Skill>>,
    /// The person's age ranges.
    #[serde(rename="ageRanges")]
    pub age_ranges: Option<Vec<AgeRangeType>>,
    /// The person's birthdays.
    pub birthdays: Option<Vec<Birthday>>,
    /// The person's relations.
    pub relations: Option<Vec<Relation>>,
    /// The person's group memberships.
    pub memberships: Option<Vec<Membership>>,
    /// The person's biographies.
    pub biographies: Option<Vec<Biography>>,
    /// The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the
    /// resource. Used for web cache validation.
    pub etag: Option<String>,
    /// The person's associated URLs.
    pub urls: Option<Vec<Url>>,
    /// The person's relationship statuses.
    #[serde(rename="relationshipStatuses")]
    pub relationship_statuses: Option<Vec<RelationshipStatus>>,
    /// The person's genders.
    pub genders: Option<Vec<Gender>>,
    /// The person's email addresses.
    #[serde(rename="emailAddresses")]
    pub email_addresses: Option<Vec<EmailAddress>>,
    /// The person's instant messaging clients.
    #[serde(rename="imClients")]
    pub im_clients: Option<Vec<ImClient>>,
    /// The person's events.
    pub events: Option<Vec<Event>>,
    /// Metadata about the person.
    pub metadata: Option<PersonMetadata>,
}

impl ResponseResult for Person {}


/// A person's gender.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Gender {
    /// The read-only value of the gender translated and formatted in the viewer's
    /// account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The gender for the person. The gender can be custom or predefined.
    /// Possible values include, but are not limited to, the
    /// following:
    /// 
    /// * `male`
    /// * `female`
    /// * `other`
    /// * `unknown`
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
    /// The read-only type of the phone number translated and formatted in the
    /// viewer's account locale or the the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// Metadata about the phone number.
    pub metadata: Option<FieldMetadata>,
    /// The type of the phone number. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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
    /// The read-only canonicalized [ITU-T E.164](https://law.resource.org/pub/us/cfr/ibr/004/itu-t.E.164.1.2008.pdf)
    /// form of the phone number.
    #[serde(rename="canonicalForm")]
    pub canonical_form: Option<String>,
}

impl Part for PhoneNumber {}


/// A person's past or current organization. Overlapping date ranges are
/// permitted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// The read-only type of the organization translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The domain name associated with the organization; for example, `google.com`.
    pub domain: Option<String>,
    /// The end date when the person left the organization.
    #[serde(rename="endDate")]
    pub end_date: Option<Date>,
    /// The name of the organization.
    pub name: Option<String>,
    /// The person's job title at the organization.
    pub title: Option<String>,
    /// The type of the organization. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
    /// 
    /// * `work`
    /// * `school`
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The symbol associated with the organization; for example, a stock ticker
    /// symbol, abbreviation, or acronym.
    pub symbol: Option<String>,
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
    /// The start date when the person joined the organization.
    #[serde(rename="startDate")]
    pub start_date: Option<Date>,
    /// The phonetic name of the organization.
    #[serde(rename="phoneticName")]
    pub phonetic_name: Option<String>,
    /// Metadata about the organization.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Organization {}


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


/// A person's read-only photo. A picture shown next to the person's name to
/// help others recognize the person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// The URL of the photo.
    pub url: Option<String>,
    /// Metadata about the photo.
    pub metadata: Option<FieldMetadata>,
}

impl Part for Photo {}


/// The read-only metadata about a person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonMetadata {
    /// Any former resource names this person has had. Populated only for
    /// [`connections.list`](/people/api/rest/v1/people.connections/list) requests
    /// that include a sync token.
    /// 
    /// The resource name may change when adding or removing fields that link a
    /// contact and profile such as a verified email, verified phone number, or
    /// profile URL.
    #[serde(rename="previousResourceNames")]
    pub previous_resource_names: Option<Vec<String>>,
    /// The sources of data for the person.
    pub sources: Option<Vec<Source>>,
    /// Resource names of people linked to this resource.
    #[serde(rename="linkedPeopleResourceNames")]
    pub linked_people_resource_names: Option<Vec<String>>,
    /// True if the person resource has been deleted. Populated only for
    /// [`connections.list`](/people/api/rest/v1/people.connections/list) requests
    /// that include a sync token.
    pub deleted: Option<bool>,
    /// DEPRECATED(Please read person.metadata.sources.profile_metadata instead).
    /// The type of the person object.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
}

impl Part for PersonMetadata {}


/// A person's email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    /// The read-only type of the email address translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the email address. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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


/// A Google Apps Domain membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainMembership {
    /// True if the person is in the viewer's Google Apps domain.
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
    /// The read-only type of the event translated and formatted in the
    /// viewer's account locale or the `Accept-Language` HTTP header locale.
    #[serde(rename="formattedType")]
    pub formatted_type: Option<String>,
    /// The type of the event. The type can be custom or predefined.
    /// Possible values include, but are not limited to, the following:
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


/// The read-only metadata about a profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// The profile object type.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
}

impl Part for ProfileMetadata {}



// ###################
// MethodBuilders ###
// #################

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
/// // like `connections_list(...)`, `get(...)` and `get_batch_get(...)`
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
    /// Provides information about a person for a resource name. Use
    /// `people/me` to indicate the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - The resource name of the person to provide information about.
    ///                    - To get information about the authenticated user, specify `people/me`.
    ///                    - To get information about any user, specify the resource name that
    ///                      identifies the user, such as the resource names returned by
    ///                      [`people.connections.list`](/people/api/rest/v1/people.connections/list).
    pub fn get(&self, resource_name: &str) -> PeopleGetCall<'a, C, A> {
        PeopleGetCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _request_mask_include_field: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of the authenticated user's contacts merged with any
    /// linked profiles.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - The resource name to return connections for. Only `people/me` is valid.
    pub fn connections_list(&self, resource_name: &str) -> PeopleConnectionListCall<'a, C, A> {
        PeopleConnectionListCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _sync_token: Default::default(),
            _sort_order: Default::default(),
            _request_sync_token: Default::default(),
            _request_mask_include_field: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
    pub fn get_batch_get(&self) -> PeopleGetBatchGetCall<'a, C, A> {
        PeopleGetBatchGetCall {
            hub: self.hub,
            _resource_names: Default::default(),
            _request_mask_include_field: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Provides information about a person for a resource name. Use
/// `people/me` to indicate the authenticated user.
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
///              .request_mask_include_field("accusam")
///              .doit();
/// # }
/// ```
pub struct PeopleGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_name: String,
    _request_mask_include_field: Option<String>,
    _delegate: Option<&'a mut Delegate>,
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
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("resourceName", self._resource_name.to_string()));
        if let Some(value) = self._request_mask_include_field {
            params.push(("requestMask.includeField", value.to_string()));
        }
        for &field in ["alt", "resourceName", "requestMask.includeField"].iter() {
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// The resource name of the person to provide information about.
    /// 
    /// - To get information about the authenticated user, specify `people/me`.
    /// - To get information about any user, specify the resource name that
    ///   identifies the user, such as the resource names returned by
    ///   [`people.connections.list`](/people/api/rest/v1/people.connections/list).
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleGetCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// Comma-separated list of fields to be included in the response. Omitting
    /// this field will include all fields except for connections.list requests,
    /// which have a default mask that includes common fields like metadata, name,
    /// photo, and profile url.
    /// Each path should start with `person.`: for example, `person.names` or
    /// `person.photos`.
    ///
    /// Sets the *request mask.include field* query property to the given value.
    pub fn request_mask_include_field(mut self, new_value: &str) -> PeopleGetCall<'a, C, A> {
        self._request_mask_include_field = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
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
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Provides a list of the authenticated user's contacts merged with any
/// linked profiles.
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
///              .sync_token("justo")
///              .sort_order("amet.")
///              .request_sync_token(false)
///              .request_mask_include_field("labore")
///              .page_token("sea")
///              .page_size(-90)
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
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
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
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.connections.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
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
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "resourceName", "syncToken", "sortOrder", "requestSyncToken", "requestMask.includeField", "pageToken", "pageSize"].iter() {
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// The resource name to return connections for. Only `people/me` is valid.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._resource_name = new_value.to_string();
        self
    }
    /// A sync token, returned by a previous call to `people.connections.list`.
    /// Only resources changed since the sync token was created will be returned.
    ///
    /// Sets the *sync token* query property to the given value.
    pub fn sync_token(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._sync_token = Some(new_value.to_string());
        self
    }
    /// The order in which the connections should be sorted. Defaults to
    /// `LAST_MODIFIED_ASCENDING`.
    ///
    /// Sets the *sort order* query property to the given value.
    pub fn sort_order(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._sort_order = Some(new_value.to_string());
        self
    }
    /// Whether the response should include a sync token, which can be used to get
    /// all changes since the last request.
    ///
    /// Sets the *request sync token* query property to the given value.
    pub fn request_sync_token(mut self, new_value: bool) -> PeopleConnectionListCall<'a, C, A> {
        self._request_sync_token = Some(new_value);
        self
    }
    /// Comma-separated list of fields to be included in the response. Omitting
    /// this field will include all fields except for connections.list requests,
    /// which have a default mask that includes common fields like metadata, name,
    /// photo, and profile url.
    /// Each path should start with `person.`: for example, `person.names` or
    /// `person.photos`.
    ///
    /// Sets the *request mask.include field* query property to the given value.
    pub fn request_mask_include_field(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._request_mask_include_field = Some(new_value.to_string());
        self
    }
    /// The token of the page to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PeopleConnectionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The number of connections to include in the response. Valid values are
    /// between 1 and 2000, inclusive. Defaults to 100.
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleConnectionListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
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
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleConnectionListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Provides information about a list of specific people by specifying a list
/// of requested resource names. Use `people/me` to indicate the authenticated
/// user.
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
///              .add_resource_names("dolores")
///              .request_mask_include_field("gubergren")
///              .doit();
/// # }
/// ```
pub struct PeopleGetBatchGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PeopleService<C, A>,
    _resource_names: Vec<String>,
    _request_mask_include_field: Option<String>,
    _delegate: Option<&'a mut Delegate>,
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
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "people.people.getBatchGet",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if self._resource_names.len() > 0 {
            for f in self._resource_names.iter() {
                params.push(("resourceNames", f.to_string()));
            }
        }
        if let Some(value) = self._request_mask_include_field {
            params.push(("requestMask.includeField", value.to_string()));
        }
        for &field in ["alt", "resourceNames", "requestMask.includeField"].iter() {
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


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// The resource name, such as one returned by
    /// [`people.connections.list`](/people/api/rest/v1/people.connections/list),
    /// of one of the people to provide information about. You can include this
    /// parameter up to 50 times in one request.
    ///
    /// Append the given value to the *resource names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_resource_names(mut self, new_value: &str) -> PeopleGetBatchGetCall<'a, C, A> {
        self._resource_names.push(new_value.to_string());
        self
    }
    /// Comma-separated list of fields to be included in the response. Omitting
    /// this field will include all fields except for connections.list requests,
    /// which have a default mask that includes common fields like metadata, name,
    /// photo, and profile url.
    /// Each path should start with `person.`: for example, `person.names` or
    /// `person.photos`.
    ///
    /// Sets the *request mask.include field* query property to the given value.
    pub fn request_mask_include_field(mut self, new_value: &str) -> PeopleGetBatchGetCall<'a, C, A> {
        self._request_mask_include_field = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleGetBatchGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
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
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleGetBatchGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


