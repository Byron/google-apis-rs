// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Ad Exchange Buyer* crate version *0.1.13+20160405*, where *20160405* is the exact revision of the *adexchangebuyer:v1.4* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.13*.
//! 
//! Everything else about the *Ad Exchange Buyer* *v1d4* API can be found at the
//! [official documentation site](https://developers.google.com/ad-exchange/buyer-rest).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/adexchangebuyer1d4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.AdExchangeBuyer.html) ... 
//! 
//! * [accounts](struct.Account.html)
//!  * [*get*](struct.AccountGetCall.html), [*list*](struct.AccountListCall.html), [*patch*](struct.AccountPatchCall.html) and [*update*](struct.AccountUpdateCall.html)
//! * [billing info](struct.BillingInfo.html)
//!  * [*get*](struct.BillingInfoGetCall.html) and [*list*](struct.BillingInfoListCall.html)
//! * [budget](struct.Budget.html)
//!  * [*get*](struct.BudgetGetCall.html), [*patch*](struct.BudgetPatchCall.html) and [*update*](struct.BudgetUpdateCall.html)
//! * [creatives](struct.Creative.html)
//!  * [*add deal*](struct.CreativeAddDealCall.html), [*get*](struct.CreativeGetCall.html), [*insert*](struct.CreativeInsertCall.html), [*list*](struct.CreativeListCall.html) and [*remove deal*](struct.CreativeRemoveDealCall.html)
//! * marketplacedeals
//!  * [*delete*](struct.MarketplacedealDeleteCall.html), [*insert*](struct.MarketplacedealInsertCall.html), [*list*](struct.MarketplacedealListCall.html) and [*update*](struct.MarketplacedealUpdateCall.html)
//! * marketplacenotes
//!  * [*insert*](struct.MarketplacenoteInsertCall.html) and [*list*](struct.MarketplacenoteListCall.html)
//! * marketplaceprivateauction
//!  * [*updateproposal*](struct.MarketplaceprivateauctionUpdateproposalCall.html)
//! * [performance report](struct.PerformanceReport.html)
//!  * [*list*](struct.PerformanceReportListCall.html)
//! * [pretargeting config](struct.PretargetingConfig.html)
//!  * [*delete*](struct.PretargetingConfigDeleteCall.html), [*get*](struct.PretargetingConfigGetCall.html), [*insert*](struct.PretargetingConfigInsertCall.html), [*list*](struct.PretargetingConfigListCall.html), [*patch*](struct.PretargetingConfigPatchCall.html) and [*update*](struct.PretargetingConfigUpdateCall.html)
//! * [products](struct.Product.html)
//!  * [*get*](struct.ProductGetCall.html) and [*search*](struct.ProductSearchCall.html)
//! * [proposals](struct.Proposal.html)
//!  * [*get*](struct.ProposalGetCall.html), [*insert*](struct.ProposalInsertCall.html), [*patch*](struct.ProposalPatchCall.html), [*search*](struct.ProposalSearchCall.html), [*setupcomplete*](struct.ProposalSetupcompleteCall.html) and [*update*](struct.ProposalUpdateCall.html)
//! * pubprofiles
//!  * [*list*](struct.PubprofileListCall.html)
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
//! * **[Hub](struct.AdExchangeBuyer.html)**
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
//! let r = hub.proposals().update(...).doit()
//! let r = hub.proposals().search(...).doit()
//! let r = hub.proposals().insert(...).doit()
//! let r = hub.proposals().setupcomplete(...).doit()
//! let r = hub.proposals().patch(...).doit()
//! let r = hub.proposals().get(...).doit()
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
//! google-adexchangebuyer1d4 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
//! use adexchangebuyer1d4::Proposal;
//! use adexchangebuyer1d4::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use adexchangebuyer1d4::AdExchangeBuyer;
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
//! let mut hub = AdExchangeBuyer::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Proposal::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.proposals().update(req, "proposalId", "revisionNumber", "updateAction")
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