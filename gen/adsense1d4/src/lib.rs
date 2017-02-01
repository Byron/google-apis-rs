// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *AdSense* crate version *1.0.3+20161206*, where *20161206* is the exact revision of the *adsense:v1.4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.3*.
//! 
//! Everything else about the *AdSense* *v1d4* API can be found at the
//! [official documentation site](https://developers.google.com/adsense/management/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/adsense1d4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.AdSense.html) ... 
//! 
//! * [accounts](struct.Account.html)
//!  * [*adclients list*](struct.AccountAdclientListCall.html), [*adunits customchannels list*](struct.AccountAdunitCustomchannelListCall.html), [*adunits get*](struct.AccountAdunitGetCall.html), [*adunits get ad code*](struct.AccountAdunitGetAdCodeCall.html), [*adunits list*](struct.AccountAdunitListCall.html), [*alerts delete*](struct.AccountAlertDeleteCall.html), [*alerts list*](struct.AccountAlertListCall.html), [*customchannels adunits list*](struct.AccountCustomchannelAdunitListCall.html), [*customchannels get*](struct.AccountCustomchannelGetCall.html), [*customchannels list*](struct.AccountCustomchannelListCall.html), [*get*](struct.AccountGetCall.html), [*list*](struct.AccountListCall.html), [*payments list*](struct.AccountPaymentListCall.html), [*reports generate*](struct.AccountReportGenerateCall.html), [*reports saved generate*](struct.AccountReportSavedGenerateCall.html), [*reports saved list*](struct.AccountReportSavedListCall.html), [*savedadstyles get*](struct.AccountSavedadstyleGetCall.html), [*savedadstyles list*](struct.AccountSavedadstyleListCall.html) and [*urlchannels list*](struct.AccountUrlchannelListCall.html)
//! * adclients
//!  * [*list*](struct.AdclientListCall.html)
//! * adunits
//!  * [*customchannels list*](struct.AdunitCustomchannelListCall.html), [*get*](struct.AdunitGetCall.html), [*get ad code*](struct.AdunitGetAdCodeCall.html) and [*list*](struct.AdunitListCall.html)
//! * [alerts](struct.Alert.html)
//!  * [*delete*](struct.AlertDeleteCall.html) and [*list*](struct.AlertListCall.html)
//! * customchannels
//!  * [*adunits list*](struct.CustomchannelAdunitListCall.html), [*get*](struct.CustomchannelGetCall.html) and [*list*](struct.CustomchannelListCall.html)
//! * [metadata](struct.Metadata.html)
//!  * [*dimensions list*](struct.MetadataDimensionListCall.html) and [*metrics list*](struct.MetadataMetricListCall.html)
//! * [payments](struct.Payment.html)
//!  * [*list*](struct.PaymentListCall.html)
//! * reports
//!  * [*generate*](struct.ReportGenerateCall.html), [*saved generate*](struct.ReportSavedGenerateCall.html) and [*saved list*](struct.ReportSavedListCall.html)
//! * savedadstyles
//!  * [*get*](struct.SavedadstyleGetCall.html) and [*list*](struct.SavedadstyleListCall.html)
//! * urlchannels
//!  * [*list*](struct.UrlchannelListCall.html)
//! 
//! 
//! Download supported by ...
//! 
//! * [*generate reports*](struct.ReportGenerateCall.html)
//! * [*reports generate accounts*](struct.AccountReportGenerateCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.AdSense.html)**
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
//! let r = hub.accounts().adunits_list(...).doit()
//! let r = hub.accounts().customchannels_list(...).doit()
//! let r = hub.accounts().adunits_customchannels_list(...).doit()
//! let r = hub.accounts().alerts_delete(...).doit()
//! let r = hub.accounts().savedadstyles_get(...).doit()
//! let r = hub.accounts().reports_generate(...).doit()
//! let r = hub.accounts().list(...).doit()
//! let r = hub.accounts().adclients_list(...).doit()
//! let r = hub.accounts().reports_saved_generate(...).doit()
//! let r = hub.accounts().adunits_get(...).doit()
//! let r = hub.accounts().alerts_list(...).doit()
//! let r = hub.accounts().savedadstyles_list(...).doit()
//! let r = hub.accounts().customchannels_get(...).doit()
//! let r = hub.accounts().get(...).doit()
//! let r = hub.accounts().customchannels_adunits_list(...).doit()
//! let r = hub.accounts().payments_list(...).doit()
//! let r = hub.accounts().adunits_get_ad_code(...).doit()
//! let r = hub.accounts().reports_saved_list(...).doit()
//! let r = hub.accounts().urlchannels_list(...).doit()
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
//! google-adsense1d4 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_adsense1d4 as adsense1d4;
//! use adsense1d4::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use adsense1d4::AdSense;
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
//! let mut hub = AdSense::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().reports_generate("accountId", "startDate", "endDate")
//!              .use_timezone_reporting(false)
//!              .start_index(-37)
//!              .add_sort("eirmod")
//!              .add_metric("At")
//!              .max_results(-96)
//!              .locale("et")
//!              .add_filter("sed")
//!              .add_dimension("sit")
//!              .currency("takimata")
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
#![cfg_attr(feature = "nightly", feature(proc_macro))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));