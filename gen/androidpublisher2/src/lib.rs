// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Android Publisher* crate version *0.1.13+20160324*, where *20160324* is the exact revision of the *androidpublisher:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.13*.
//! 
//! Everything else about the *Android Publisher* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/android-publisher).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/androidpublisher2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.AndroidPublisher.html) ... 
//! 
//! * edits
//!  * [*apklistings delete*](struct.EditApklistingDeleteCall.html), [*apklistings deleteall*](struct.EditApklistingDeleteallCall.html), [*apklistings get*](struct.EditApklistingGetCall.html), [*apklistings list*](struct.EditApklistingListCall.html), [*apklistings patch*](struct.EditApklistingPatchCall.html), [*apklistings update*](struct.EditApklistingUpdateCall.html), [*apks addexternallyhosted*](struct.EditApkAddexternallyhostedCall.html), [*apks list*](struct.EditApkListCall.html), [*apks upload*](struct.EditApkUploadCall.html), [*commit*](struct.EditCommitCall.html), [*delete*](struct.EditDeleteCall.html), [*details get*](struct.EditDetailGetCall.html), [*details patch*](struct.EditDetailPatchCall.html), [*details update*](struct.EditDetailUpdateCall.html), [*expansionfiles get*](struct.EditExpansionfileGetCall.html), [*expansionfiles patch*](struct.EditExpansionfilePatchCall.html), [*expansionfiles update*](struct.EditExpansionfileUpdateCall.html), [*expansionfiles upload*](struct.EditExpansionfileUploadCall.html), [*get*](struct.EditGetCall.html), [*images delete*](struct.EditImageDeleteCall.html), [*images deleteall*](struct.EditImageDeleteallCall.html), [*images list*](struct.EditImageListCall.html), [*images upload*](struct.EditImageUploadCall.html), [*insert*](struct.EditInsertCall.html), [*listings delete*](struct.EditListingDeleteCall.html), [*listings deleteall*](struct.EditListingDeleteallCall.html), [*listings get*](struct.EditListingGetCall.html), [*listings list*](struct.EditListingListCall.html), [*listings patch*](struct.EditListingPatchCall.html), [*listings update*](struct.EditListingUpdateCall.html), [*testers get*](struct.EditTesterGetCall.html), [*testers patch*](struct.EditTesterPatchCall.html), [*testers update*](struct.EditTesterUpdateCall.html), [*tracks get*](struct.EditTrackGetCall.html), [*tracks list*](struct.EditTrackListCall.html), [*tracks patch*](struct.EditTrackPatchCall.html), [*tracks update*](struct.EditTrackUpdateCall.html) and [*validate*](struct.EditValidateCall.html)
//! * [entitlements](struct.Entitlement.html)
//!  * [*list*](struct.EntitlementListCall.html)
//! * inappproducts
//!  * [*batch*](struct.InappproductBatchCall.html), [*delete*](struct.InappproductDeleteCall.html), [*get*](struct.InappproductGetCall.html), [*insert*](struct.InappproductInsertCall.html), [*list*](struct.InappproductListCall.html), [*patch*](struct.InappproductPatchCall.html) and [*update*](struct.InappproductUpdateCall.html)
//! * purchases
//!  * [*products get*](struct.PurchaseProductGetCall.html), [*subscriptions cancel*](struct.PurchaseSubscriptionCancelCall.html), [*subscriptions defer*](struct.PurchaseSubscriptionDeferCall.html), [*subscriptions get*](struct.PurchaseSubscriptionGetCall.html), [*subscriptions refund*](struct.PurchaseSubscriptionRefundCall.html) and [*subscriptions revoke*](struct.PurchaseSubscriptionRevokeCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*images upload edits*](struct.EditImageUploadCall.html)
//! * [*expansionfiles upload edits*](struct.EditExpansionfileUploadCall.html)
//! * [*apks upload edits*](struct.EditApkUploadCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.AndroidPublisher.html)**
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
//! let r = hub.inappproducts().patch(...).doit()
//! let r = hub.inappproducts().insert(...).doit()
//! let r = hub.inappproducts().get(...).doit()
//! let r = hub.inappproducts().update(...).doit()
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
//! google-androidpublisher2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_androidpublisher2 as androidpublisher2;
//! use androidpublisher2::InAppProduct;
//! use androidpublisher2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use androidpublisher2::AndroidPublisher;
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
//! let mut hub = AndroidPublisher::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = InAppProduct::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.inappproducts().patch(req, "packageName", "sku")
//!              .auto_convert_missing_prices(true)
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
#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));