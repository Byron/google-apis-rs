// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Shopping Content* crate version *5.0.5+20181009*, where *20181009* is the exact revision of the *content:v2sandbox* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Shopping Content* *v2_sandbox* API can be found at the
//! [official documentation site](https://developers.google.com/shopping-content).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/content2_sandbox).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](ShoppingContent) ... 
//! 
//! * orderinvoices
//!  * [*createchargeinvoice*](api::OrderinvoiceCreatechargeinvoiceCall) and [*createrefundinvoice*](api::OrderinvoiceCreaterefundinvoiceCall)
//! * orderpayments
//!  * [*notifyauthapproved*](api::OrderpaymentNotifyauthapprovedCall), [*notifyauthdeclined*](api::OrderpaymentNotifyauthdeclinedCall), [*notifycharge*](api::OrderpaymentNotifychargeCall) and [*notifyrefund*](api::OrderpaymentNotifyrefundCall)
//! * orderreturns
//!  * [*get*](api::OrderreturnGetCall) and [*list*](api::OrderreturnListCall)
//! * [orders](api::Order)
//!  * [*acknowledge*](api::OrderAcknowledgeCall), [*advancetestorder*](api::OrderAdvancetestorderCall), [*cancel*](api::OrderCancelCall), [*cancellineitem*](api::OrderCancellineitemCall), [*canceltestorderbycustomer*](api::OrderCanceltestorderbycustomerCall), [*createtestorder*](api::OrderCreatetestorderCall), [*createtestreturn*](api::OrderCreatetestreturnCall), [*custombatch*](api::OrderCustombatchCall), [*get*](api::OrderGetCall), [*getbymerchantorderid*](api::OrderGetbymerchantorderidCall), [*gettestordertemplate*](api::OrderGettestordertemplateCall), [*instorerefundlineitem*](api::OrderInstorerefundlineitemCall), [*list*](api::OrderListCall), [*refund*](api::OrderRefundCall), [*rejectreturnlineitem*](api::OrderRejectreturnlineitemCall), [*returnlineitem*](api::OrderReturnlineitemCall), [*returnrefundlineitem*](api::OrderReturnrefundlineitemCall), [*setlineitemmetadata*](api::OrderSetlineitemmetadataCall), [*shiplineitems*](api::OrderShiplineitemCall), [*updatelineitemshippingdetails*](api::OrderUpdatelineitemshippingdetailCall), [*updatemerchantorderid*](api::OrderUpdatemerchantorderidCall) and [*updateshipment*](api::OrderUpdateshipmentCall)
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
//! * **[Hub](ShoppingContent)**
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
//! let r = hub.orders().acknowledge(...).doit().await
//! let r = hub.orders().advancetestorder(...).doit().await
//! let r = hub.orders().cancel(...).doit().await
//! let r = hub.orders().cancellineitem(...).doit().await
//! let r = hub.orders().canceltestorderbycustomer(...).doit().await
//! let r = hub.orders().createtestorder(...).doit().await
//! let r = hub.orders().createtestreturn(...).doit().await
//! let r = hub.orders().custombatch(...).doit().await
//! let r = hub.orders().get(...).doit().await
//! let r = hub.orders().getbymerchantorderid(...).doit().await
//! let r = hub.orders().gettestordertemplate(...).doit().await
//! let r = hub.orders().instorerefundlineitem(...).doit().await
//! let r = hub.orders().list(...).doit().await
//! let r = hub.orders().refund(...).doit().await
//! let r = hub.orders().rejectreturnlineitem(...).doit().await
//! let r = hub.orders().returnlineitem(...).doit().await
//! let r = hub.orders().returnrefundlineitem(...).doit().await
//! let r = hub.orders().setlineitemmetadata(...).doit().await
//! let r = hub.orders().shiplineitems(...).doit().await
//! let r = hub.orders().updatelineitemshippingdetails(...).doit().await
//! let r = hub.orders().updatemerchantorderid(...).doit().await
//! let r = hub.orders().updateshipment(...).doit().await
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
//! google-content2_sandbox = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_content2_sandbox as content2_sandbox;
//! use content2_sandbox::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.orders().list(99)
//!              .add_statuses("takimata")
//!              .placed_date_start("amet.")
//!              .placed_date_end("duo")
//!              .page_token("ipsum")
//!              .order_by("gubergren")
//!              .max_results(50)
//!              .acknowledged(false)
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
pub use api::ShoppingContent;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;