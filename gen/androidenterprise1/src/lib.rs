// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Android Enterprise* crate version *0.1.13+20160331*, where *20160331* is the exact revision of the *androidenterprise:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.13*.
//! 
//! Everything else about the *Android Enterprise* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/android/work/play/emm-api).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/androidenterprise1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.AndroidEnterprise.html) ... 
//! 
//! * [collections](struct.Collection.html)
//!  * [*delete*](struct.CollectionDeleteCall.html), [*get*](struct.CollectionGetCall.html), [*insert*](struct.CollectionInsertCall.html), [*list*](struct.CollectionListCall.html), [*patch*](struct.CollectionPatchCall.html) and [*update*](struct.CollectionUpdateCall.html)
//! * collectionviewers
//!  * [*delete*](struct.CollectionviewerDeleteCall.html), [*get*](struct.CollectionviewerGetCall.html), [*list*](struct.CollectionviewerListCall.html), [*patch*](struct.CollectionviewerPatchCall.html) and [*update*](struct.CollectionviewerUpdateCall.html)
//! * [devices](struct.Device.html)
//!  * [*get*](struct.DeviceGetCall.html), [*get state*](struct.DeviceGetStateCall.html), [*list*](struct.DeviceListCall.html) and [*set state*](struct.DeviceSetStateCall.html)
//! * [enterprises](struct.Enterprise.html)
//!  * [*delete*](struct.EnterpriseDeleteCall.html), [*enroll*](struct.EnterpriseEnrollCall.html), [*get*](struct.EnterpriseGetCall.html), [*get store layout*](struct.EnterpriseGetStoreLayoutCall.html), [*insert*](struct.EnterpriseInsertCall.html), [*list*](struct.EnterpriseListCall.html), [*send test push notification*](struct.EnterpriseSendTestPushNotificationCall.html), [*set account*](struct.EnterpriseSetAccountCall.html), [*set store layout*](struct.EnterpriseSetStoreLayoutCall.html) and [*unenroll*](struct.EnterpriseUnenrollCall.html)
//! * [entitlements](struct.Entitlement.html)
//!  * [*delete*](struct.EntitlementDeleteCall.html), [*get*](struct.EntitlementGetCall.html), [*list*](struct.EntitlementListCall.html), [*patch*](struct.EntitlementPatchCall.html) and [*update*](struct.EntitlementUpdateCall.html)
//! * grouplicenses
//!  * [*get*](struct.GrouplicenseGetCall.html) and [*list*](struct.GrouplicenseListCall.html)
//! * grouplicenseusers
//!  * [*list*](struct.GrouplicenseuserListCall.html)
//! * [installs](struct.Install.html)
//!  * [*delete*](struct.InstallDeleteCall.html), [*get*](struct.InstallGetCall.html), [*list*](struct.InstallListCall.html), [*patch*](struct.InstallPatchCall.html) and [*update*](struct.InstallUpdateCall.html)
//! * [permissions](struct.Permission.html)
//!  * [*get*](struct.PermissionGetCall.html)
//! * [products](struct.Product.html)
//!  * [*approve*](struct.ProductApproveCall.html), [*generate approval url*](struct.ProductGenerateApprovalUrlCall.html), [*get*](struct.ProductGetCall.html), [*get app restrictions schema*](struct.ProductGetAppRestrictionsSchemaCall.html), [*get permissions*](struct.ProductGetPermissionCall.html) and [*update permissions*](struct.ProductUpdatePermissionCall.html)
//! * storelayoutclusters
//!  * [*delete*](struct.StorelayoutclusterDeleteCall.html), [*get*](struct.StorelayoutclusterGetCall.html), [*insert*](struct.StorelayoutclusterInsertCall.html), [*list*](struct.StorelayoutclusterListCall.html), [*patch*](struct.StorelayoutclusterPatchCall.html) and [*update*](struct.StorelayoutclusterUpdateCall.html)
//! * storelayoutpages
//!  * [*delete*](struct.StorelayoutpageDeleteCall.html), [*get*](struct.StorelayoutpageGetCall.html), [*insert*](struct.StorelayoutpageInsertCall.html), [*list*](struct.StorelayoutpageListCall.html), [*patch*](struct.StorelayoutpagePatchCall.html) and [*update*](struct.StorelayoutpageUpdateCall.html)
//! * [users](struct.User.html)
//!  * [*generate token*](struct.UserGenerateTokenCall.html), [*get*](struct.UserGetCall.html), [*get available product set*](struct.UserGetAvailableProductSetCall.html), [*list*](struct.UserListCall.html), [*revoke token*](struct.UserRevokeTokenCall.html) and [*set available product set*](struct.UserSetAvailableProductSetCall.html)
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
//! * **[Hub](struct.AndroidEnterprise.html)**
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
//! let r = hub.enterprises().send_test_push_notification(...).doit()
//! let r = hub.enterprises().set_store_layout(...).doit()
//! let r = hub.enterprises().get_store_layout(...).doit()
//! let r = hub.enterprises().list(...).doit()
//! let r = hub.enterprises().unenroll(...).doit()
//! let r = hub.enterprises().set_account(...).doit()
//! let r = hub.enterprises().delete(...).doit()
//! let r = hub.enterprises().enroll(...).doit()
//! let r = hub.enterprises().insert(...).doit()
//! let r = hub.enterprises().get(...).doit()
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
//! google-androidenterprise1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_androidenterprise1 as androidenterprise1;
//! use androidenterprise1::StoreLayout;
//! use androidenterprise1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use androidenterprise1::AndroidEnterprise;
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
//! let mut hub = AndroidEnterprise::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = StoreLayout::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.enterprises().set_store_layout(req, "enterpriseId")
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