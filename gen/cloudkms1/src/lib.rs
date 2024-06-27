// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud KMS* crate version *5.0.5+20240621*, where *20240621* is the exact revision of the *cloudkms:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Cloud KMS* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/kms/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/cloudkms1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](CloudKMS) ... 
//! 
//! * folders
//!  * [*get autokey config*](api::FolderGetAutokeyConfigCall) and [*update autokey config*](api::FolderUpdateAutokeyConfigCall)
//! * projects
//!  * [*locations ekm config get iam policy*](api::ProjectLocationEkmConfigGetIamPolicyCall), [*locations ekm config set iam policy*](api::ProjectLocationEkmConfigSetIamPolicyCall), [*locations ekm config test iam permissions*](api::ProjectLocationEkmConfigTestIamPermissionCall), [*locations ekm connections create*](api::ProjectLocationEkmConnectionCreateCall), [*locations ekm connections get*](api::ProjectLocationEkmConnectionGetCall), [*locations ekm connections get iam policy*](api::ProjectLocationEkmConnectionGetIamPolicyCall), [*locations ekm connections list*](api::ProjectLocationEkmConnectionListCall), [*locations ekm connections patch*](api::ProjectLocationEkmConnectionPatchCall), [*locations ekm connections set iam policy*](api::ProjectLocationEkmConnectionSetIamPolicyCall), [*locations ekm connections test iam permissions*](api::ProjectLocationEkmConnectionTestIamPermissionCall), [*locations ekm connections verify connectivity*](api::ProjectLocationEkmConnectionVerifyConnectivityCall), [*locations generate random bytes*](api::ProjectLocationGenerateRandomByteCall), [*locations get*](api::ProjectLocationGetCall), [*locations get ekm config*](api::ProjectLocationGetEkmConfigCall), [*locations key handles create*](api::ProjectLocationKeyHandleCreateCall), [*locations key handles get*](api::ProjectLocationKeyHandleGetCall), [*locations key handles list*](api::ProjectLocationKeyHandleListCall), [*locations key rings create*](api::ProjectLocationKeyRingCreateCall), [*locations key rings crypto keys create*](api::ProjectLocationKeyRingCryptoKeyCreateCall), [*locations key rings crypto keys crypto key versions asymmetric decrypt*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricDecryptCall), [*locations key rings crypto keys crypto key versions asymmetric sign*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionAsymmetricSignCall), [*locations key rings crypto keys crypto key versions create*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionCreateCall), [*locations key rings crypto keys crypto key versions destroy*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionDestroyCall), [*locations key rings crypto keys crypto key versions get*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetCall), [*locations key rings crypto keys crypto key versions get public key*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionGetPublicKeyCall), [*locations key rings crypto keys crypto key versions import*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionImportCall), [*locations key rings crypto keys crypto key versions list*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionListCall), [*locations key rings crypto keys crypto key versions mac sign*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacSignCall), [*locations key rings crypto keys crypto key versions mac verify*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionMacVerifyCall), [*locations key rings crypto keys crypto key versions patch*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionPatchCall), [*locations key rings crypto keys crypto key versions raw decrypt*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRawDecryptCall), [*locations key rings crypto keys crypto key versions raw encrypt*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRawEncryptCall), [*locations key rings crypto keys crypto key versions restore*](api::ProjectLocationKeyRingCryptoKeyCryptoKeyVersionRestoreCall), [*locations key rings crypto keys decrypt*](api::ProjectLocationKeyRingCryptoKeyDecryptCall), [*locations key rings crypto keys encrypt*](api::ProjectLocationKeyRingCryptoKeyEncryptCall), [*locations key rings crypto keys get*](api::ProjectLocationKeyRingCryptoKeyGetCall), [*locations key rings crypto keys get iam policy*](api::ProjectLocationKeyRingCryptoKeyGetIamPolicyCall), [*locations key rings crypto keys list*](api::ProjectLocationKeyRingCryptoKeyListCall), [*locations key rings crypto keys patch*](api::ProjectLocationKeyRingCryptoKeyPatchCall), [*locations key rings crypto keys set iam policy*](api::ProjectLocationKeyRingCryptoKeySetIamPolicyCall), [*locations key rings crypto keys test iam permissions*](api::ProjectLocationKeyRingCryptoKeyTestIamPermissionCall), [*locations key rings crypto keys update primary version*](api::ProjectLocationKeyRingCryptoKeyUpdatePrimaryVersionCall), [*locations key rings get*](api::ProjectLocationKeyRingGetCall), [*locations key rings get iam policy*](api::ProjectLocationKeyRingGetIamPolicyCall), [*locations key rings import jobs create*](api::ProjectLocationKeyRingImportJobCreateCall), [*locations key rings import jobs get*](api::ProjectLocationKeyRingImportJobGetCall), [*locations key rings import jobs get iam policy*](api::ProjectLocationKeyRingImportJobGetIamPolicyCall), [*locations key rings import jobs list*](api::ProjectLocationKeyRingImportJobListCall), [*locations key rings import jobs set iam policy*](api::ProjectLocationKeyRingImportJobSetIamPolicyCall), [*locations key rings import jobs test iam permissions*](api::ProjectLocationKeyRingImportJobTestIamPermissionCall), [*locations key rings list*](api::ProjectLocationKeyRingListCall), [*locations key rings set iam policy*](api::ProjectLocationKeyRingSetIamPolicyCall), [*locations key rings test iam permissions*](api::ProjectLocationKeyRingTestIamPermissionCall), [*locations list*](api::ProjectLocationListCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations update ekm config*](api::ProjectLocationUpdateEkmConfigCall) and [*show effective autokey config*](api::ProjectShowEffectiveAutokeyConfigCall)
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
//! * **[Hub](CloudKMS)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.projects().locations_ekm_config_get_iam_policy(...).doit().await
//! let r = hub.projects().locations_ekm_config_set_iam_policy(...).doit().await
//! let r = hub.projects().locations_ekm_connections_get_iam_policy(...).doit().await
//! let r = hub.projects().locations_ekm_connections_set_iam_policy(...).doit().await
//! let r = hub.projects().locations_key_rings_crypto_keys_get_iam_policy(...).doit().await
//! let r = hub.projects().locations_key_rings_crypto_keys_set_iam_policy(...).doit().await
//! let r = hub.projects().locations_key_rings_import_jobs_get_iam_policy(...).doit().await
//! let r = hub.projects().locations_key_rings_import_jobs_set_iam_policy(...).doit().await
//! let r = hub.projects().locations_key_rings_get_iam_policy(...).doit().await
//! let r = hub.projects().locations_key_rings_set_iam_policy(...).doit().await
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
//! google-cloudkms1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_cloudkms1 as cloudkms1;
//! use cloudkms1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use cloudkms1::{CloudKMS, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = CloudKMS::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_ekm_config_get_iam_policy("resource")
//!              .options_requested_policy_version(-55)
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
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
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! ## Cargo Features
//! 
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//! 
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::CloudKMS;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;