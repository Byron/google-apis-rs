// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *licensing* crate version *0.1.4+20140122*, where *20140122* is the exact revision of the *licensing:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.4*.
//! 
//! Everything else about the *licensing* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/google-apps/licensing/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/licensing1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Licensing.html) ... 
//! 
//! * [license assignments](struct.LicenseAssignment.html)
//!  * [*delete*](struct.LicenseAssignmentDeleteCall.html), [*get*](struct.LicenseAssignmentGetCall.html), [*insert*](struct.LicenseAssignmentInsertCall.html), [*list for product*](struct.LicenseAssignmentListForProductCall.html), [*list for product and sku*](struct.LicenseAssignmentListForProductAndSkuCall.html), [*patch*](struct.LicenseAssignmentPatchCall.html) and [*update*](struct.LicenseAssignmentUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Licensing.html)**
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
//! let r = hub.license_assignments().delete(...).doit()
//! let r = hub.license_assignments().list_for_product(...).doit()
//! let r = hub.license_assignments().get(...).doit()
//! let r = hub.license_assignments().list_for_product_and_sku(...).doit()
//! let r = hub.license_assignments().update(...).doit()
//! let r = hub.license_assignments().patch(...).doit()
//! let r = hub.license_assignments().insert(...).doit()
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
//! google-licensing1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_licensing1 as licensing1;
//! use licensing1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use licensing1::Licensing;
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
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Licensing::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.license_assignments().list_for_product_and_sku("productId", "skuId", "customerId")
//!              .page_token("takimata")
//!              .max_results(31)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!         Error::MissingToken => println!("OAuth2: Missing Token"),
//!         Error::Cancelled => println!("Operation canceled by user"),
//!         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     },
//!     Ok(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
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
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(std_misc)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin, slice_patterns)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage Google Apps licenses for your domain
    AppLicensing,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppLicensing => "https://www.googleapis.com/auth/apps.licensing",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppLicensing
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Licensing related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_licensing1 as licensing1;
/// use licensing1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use licensing1::Licensing;
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
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().list_for_product_and_sku("productId", "skuId", "customerId")
///              .page_token("sea")
///              .max_results(11)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
///         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///         Error::MissingToken => println!("OAuth2: Missing Token"),
///         Error::Cancelled => println!("Operation canceled by user"),
///         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     },
///     Ok(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Licensing<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Licensing<C, NC, A> {}

impl<'a, C, NC, A> Licensing<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Licensing<C, NC, A> {
        Licensing {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.4".to_string(),
            _m: PhantomData
        }
    }

    pub fn license_assignments(&'a self) -> LicenseAssignmentMethods<'a, C, NC, A> {
        LicenseAssignmentMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.4`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// Template for LiscenseAssignment Resource
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete license assignments](struct.LicenseAssignmentDeleteCall.html) (none)
/// * [list for product license assignments](struct.LicenseAssignmentListForProductCall.html) (none)
/// * [get license assignments](struct.LicenseAssignmentGetCall.html) (response)
/// * [list for product and sku license assignments](struct.LicenseAssignmentListForProductAndSkuCall.html) (none)
/// * [update license assignments](struct.LicenseAssignmentUpdateCall.html) (request|response)
/// * [patch license assignments](struct.LicenseAssignmentPatchCall.html) (request|response)
/// * [insert license assignments](struct.LicenseAssignmentInsertCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LicenseAssignment {
    /// Name of the sku of the product.
    #[serde(rename="skuId")]
    pub sku_id: Option<String>,
    /// Identifies the resource as a LicenseAssignment.
    pub kind: Option<String>,
    /// ETag of the resource.
    pub etags: Option<String>,
    /// Email id of the user.
    #[serde(rename="userId")]
    pub user_id: Option<String>,
    /// Link to this page.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// Name of the product.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl RequestValue for LicenseAssignment {}
impl Resource for LicenseAssignment {}
impl ResponseResult for LicenseAssignment {}


/// Template for LicenseAssignment Insert request
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert license assignments](struct.LicenseAssignmentInsertCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct LicenseAssignmentInsert {
    /// Email id of the user
    #[serde(rename="userId")]
    pub user_id: Option<String>,
}

impl RequestValue for LicenseAssignmentInsert {}


/// LicesnseAssignment List for a given product/sku for a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list for product and sku license assignments](struct.LicenseAssignmentListForProductAndSkuCall.html) (response)
/// * [list for product license assignments](struct.LicenseAssignmentListForProductCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LicenseAssignmentList {
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: String,
    /// The LicenseAssignments in this page of results.
    pub items: Vec<LicenseAssignment>,
    /// Identifies the resource as a collection of LicenseAssignments.
    pub kind: String,
    /// ETag of the resource.
    pub etag: String,
}

impl ResponseResult for LicenseAssignmentList {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *licenseAssignment* resources.
/// It is not used directly, but through the `Licensing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_licensing1 as licensing1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use licensing1::Licensing;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list_for_product(...)`, `list_for_product_and_sku(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.license_assignments();
/// # }
/// ```
pub struct LicenseAssignmentMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for LicenseAssignmentMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Revoke License.
    /// 
    /// # Arguments
    ///
    /// * `productId` - Name for product
    /// * `skuId` - Name for sku
    /// * `userId` - email id or unique Id of the user
    pub fn delete(&self, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentDeleteCall<'a, C, NC, A> {
        LicenseAssignmentDeleteCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List license assignments for given product of the customer.
    /// 
    /// # Arguments
    ///
    /// * `productId` - Name for product
    /// * `customerId` - CustomerId represents the customer for whom licenseassignments are queried
    pub fn list_for_product(&self, product_id: &str, customer_id: &str) -> LicenseAssignmentListForProductCall<'a, C, NC, A> {
        LicenseAssignmentListForProductCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _customer_id: customer_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get license assignment of a particular product and sku for a user
    /// 
    /// # Arguments
    ///
    /// * `productId` - Name for product
    /// * `skuId` - Name for sku
    /// * `userId` - email id or unique Id of the user
    pub fn get(&self, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentGetCall<'a, C, NC, A> {
        LicenseAssignmentGetCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List license assignments for given product and sku of the customer.
    /// 
    /// # Arguments
    ///
    /// * `productId` - Name for product
    /// * `skuId` - Name for sku
    /// * `customerId` - CustomerId represents the customer for whom licenseassignments are queried
    pub fn list_for_product_and_sku(&self, product_id: &str, sku_id: &str, customer_id: &str) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
        LicenseAssignmentListForProductAndSkuCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _customer_id: customer_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assign License.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `productId` - Name for product
    /// * `skuId` - Name for sku for which license would be revoked
    /// * `userId` - email id or unique Id of the user
    pub fn update(&self, request: &LicenseAssignment, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentUpdateCall<'a, C, NC, A> {
        LicenseAssignmentUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assign License. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `productId` - Name for product
    /// * `skuId` - Name for sku for which license would be revoked
    /// * `userId` - email id or unique Id of the user
    pub fn patch(&self, request: &LicenseAssignment, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentPatchCall<'a, C, NC, A> {
        LicenseAssignmentPatchCall {
            hub: self.hub,
            _request: request.clone(),
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assign License.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `productId` - Name for product
    /// * `skuId` - Name for sku
    pub fn insert(&self, request: &LicenseAssignmentInsert, product_id: &str, sku_id: &str) -> LicenseAssignmentInsertCall<'a, C, NC, A> {
        LicenseAssignmentInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Revoke License.
///
/// A builder for the *delete* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().delete("productId", "skuId", "userId")
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _product_id: String,
    _sku_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("skuId", self._sku_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["productId", "skuId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/sku/{skuId}/user/{userId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId"), ("{skuId}", "skuId"), ("{userId}", "userId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["productId", "skuId", "userId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentDeleteCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *sku id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for sku
    pub fn sku_id(mut self, new_value: &str) -> LicenseAssignmentDeleteCall<'a, C, NC, A> {
        self._sku_id = new_value.to_string();
        self
    }
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// email id or unique Id of the user
    pub fn user_id(mut self, new_value: &str) -> LicenseAssignmentDeleteCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentDeleteCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentDeleteCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentDeleteCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List license assignments for given product of the customer.
///
/// A builder for the *listForProduct* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().list_for_product("productId", "customerId")
///              .page_token("no")
///              .max_results(80)
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentListForProductCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _product_id: String,
    _customer_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentListForProductCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentListForProductCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LicenseAssignmentList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.listForProduct", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("customerId", self._customer_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "productId", "customerId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/users".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["productId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentListForProductCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *customer id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// CustomerId represents the customer for whom licenseassignments are queried
    pub fn customer_id(mut self, new_value: &str) -> LicenseAssignmentListForProductCall<'a, C, NC, A> {
        self._customer_id = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// Token to fetch the next page.Optional. By default server will return first page
    pub fn page_token(mut self, new_value: &str) -> LicenseAssignmentListForProductCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of campaigns to return at one time. Must be positive. Optional. Default value is 100.
    pub fn max_results(mut self, new_value: u32) -> LicenseAssignmentListForProductCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentListForProductCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentListForProductCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentListForProductCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get license assignment of a particular product and sku for a user
///
/// A builder for the *get* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().get("productId", "skuId", "userId")
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _product_id: String,
    _sku_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LicenseAssignment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("skuId", self._sku_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "productId", "skuId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/sku/{skuId}/user/{userId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId"), ("{skuId}", "skuId"), ("{userId}", "userId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["productId", "skuId", "userId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentGetCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *sku id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for sku
    pub fn sku_id(mut self, new_value: &str) -> LicenseAssignmentGetCall<'a, C, NC, A> {
        self._sku_id = new_value.to_string();
        self
    }
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// email id or unique Id of the user
    pub fn user_id(mut self, new_value: &str) -> LicenseAssignmentGetCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentGetCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentGetCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentGetCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List license assignments for given product and sku of the customer.
///
/// A builder for the *listForProductAndSku* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().list_for_product_and_sku("productId", "skuId", "customerId")
///              .page_token("et")
///              .max_results(31)
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _product_id: String,
    _sku_id: String,
    _customer_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LicenseAssignmentList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.listForProductAndSku", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("skuId", self._sku_id.to_string()));
        params.push(("customerId", self._customer_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "productId", "skuId", "customerId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/sku/{skuId}/users".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId"), ("{skuId}", "skuId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["productId", "skuId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *sku id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for sku
    pub fn sku_id(mut self, new_value: &str) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
        self._sku_id = new_value.to_string();
        self
    }
    /// Sets the *customer id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// CustomerId represents the customer for whom licenseassignments are queried
    pub fn customer_id(mut self, new_value: &str) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
        self._customer_id = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// Token to fetch the next page.Optional. By default server will return first page
    pub fn page_token(mut self, new_value: &str) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of campaigns to return at one time. Must be positive. Optional. Default value is 100.
    pub fn max_results(mut self, new_value: u32) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentListForProductAndSkuCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Assign License.
///
/// A builder for the *update* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// use licensing1::LicenseAssignment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LicenseAssignment = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().update(&req, "productId", "skuId", "userId")
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _request: LicenseAssignment,
    _product_id: String,
    _sku_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LicenseAssignment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("skuId", self._sku_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "productId", "skuId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/sku/{skuId}/user/{userId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId"), ("{skuId}", "skuId"), ("{userId}", "userId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["productId", "skuId", "userId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_ref())
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
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &LicenseAssignment) -> LicenseAssignmentUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentUpdateCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *sku id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for sku for which license would be revoked
    pub fn sku_id(mut self, new_value: &str) -> LicenseAssignmentUpdateCall<'a, C, NC, A> {
        self._sku_id = new_value.to_string();
        self
    }
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// email id or unique Id of the user
    pub fn user_id(mut self, new_value: &str) -> LicenseAssignmentUpdateCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentUpdateCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentUpdateCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentUpdateCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Assign License. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// use licensing1::LicenseAssignment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LicenseAssignment = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().patch(&req, "productId", "skuId", "userId")
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentPatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _request: LicenseAssignment,
    _product_id: String,
    _sku_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentPatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentPatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LicenseAssignment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.patch", 
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("skuId", self._sku_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "productId", "skuId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/sku/{skuId}/user/{userId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId"), ("{skuId}", "skuId"), ("{userId}", "userId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["productId", "skuId", "userId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.as_ref())
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
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &LicenseAssignment) -> LicenseAssignmentPatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentPatchCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *sku id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for sku for which license would be revoked
    pub fn sku_id(mut self, new_value: &str) -> LicenseAssignmentPatchCall<'a, C, NC, A> {
        self._sku_id = new_value.to_string();
        self
    }
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// email id or unique Id of the user
    pub fn user_id(mut self, new_value: &str) -> LicenseAssignmentPatchCall<'a, C, NC, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentPatchCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentPatchCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentPatchCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Assign License.
///
/// A builder for the *insert* method supported by a *licenseAssignment* resource.
/// It is not used directly, but through a `LicenseAssignmentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_licensing1 as licensing1;
/// use licensing1::LicenseAssignmentInsert;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use licensing1::Licensing;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Licensing::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LicenseAssignmentInsert = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.license_assignments().insert(&req, "productId", "skuId")
///              .doit();
/// # }
/// ```
pub struct LicenseAssignmentInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Licensing<C, NC, A>,
    _request: LicenseAssignmentInsert,
    _product_id: String,
    _sku_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LicenseAssignmentInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LicenseAssignmentInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LicenseAssignment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "licensing.licenseAssignments.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("productId", self._product_id.to_string()));
        params.push(("skuId", self._sku_id.to_string()));
        for &field in ["alt", "productId", "skuId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/licensing/v1/product/{productId}/sku/{skuId}/user".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppLicensing.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{productId}", "productId"), ("{skuId}", "skuId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["productId", "skuId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
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
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &LicenseAssignmentInsert) -> LicenseAssignmentInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *product id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for product
    pub fn product_id(mut self, new_value: &str) -> LicenseAssignmentInsertCall<'a, C, NC, A> {
        self._product_id = new_value.to_string();
        self
    }
    /// Sets the *sku id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name for sku
    pub fn sku_id(mut self, new_value: &str) -> LicenseAssignmentInsertCall<'a, C, NC, A> {
        self._sku_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LicenseAssignmentInsertCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LicenseAssignmentInsertCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LicenseAssignmentInsertCall<'a, C, NC, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


