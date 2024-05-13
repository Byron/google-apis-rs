// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Google Analytics Admin* crate version *5.0.3+20240303*, where *20240303* is the exact revision of the *analyticsadmin:v1alpha* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.3*.
//! 
//! Everything else about the *Google Analytics Admin* *v1_alpha* API can be found at the
//! [official documentation site](http://code.google.com/apis/analytics/docs/mgmt/home.html).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/analyticsadmin1_alpha).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](GoogleAnalyticsAdmin) ... 
//! 
//! * account summaries
//!  * [*list*](api::AccountSummaryListCall)
//! * accounts
//!  * [*access bindings batch create*](api::AccountAccessBindingBatchCreateCall), [*access bindings batch delete*](api::AccountAccessBindingBatchDeleteCall), [*access bindings batch get*](api::AccountAccessBindingBatchGetCall), [*access bindings batch update*](api::AccountAccessBindingBatchUpdateCall), [*access bindings create*](api::AccountAccessBindingCreateCall), [*access bindings delete*](api::AccountAccessBindingDeleteCall), [*access bindings get*](api::AccountAccessBindingGetCall), [*access bindings list*](api::AccountAccessBindingListCall), [*access bindings patch*](api::AccountAccessBindingPatchCall), [*delete*](api::AccountDeleteCall), [*get*](api::AccountGetCall), [*get data sharing settings*](api::AccountGetDataSharingSettingCall), [*list*](api::AccountListCall), [*patch*](api::AccountPatchCall), [*provision account ticket*](api::AccountProvisionAccountTicketCall), [*run access report*](api::AccountRunAccessReportCall) and [*search change history events*](api::AccountSearchChangeHistoryEventCall)
//! * properties
//!  * [*access bindings batch create*](api::PropertyAccessBindingBatchCreateCall), [*access bindings batch delete*](api::PropertyAccessBindingBatchDeleteCall), [*access bindings batch get*](api::PropertyAccessBindingBatchGetCall), [*access bindings batch update*](api::PropertyAccessBindingBatchUpdateCall), [*access bindings create*](api::PropertyAccessBindingCreateCall), [*access bindings delete*](api::PropertyAccessBindingDeleteCall), [*access bindings get*](api::PropertyAccessBindingGetCall), [*access bindings list*](api::PropertyAccessBindingListCall), [*access bindings patch*](api::PropertyAccessBindingPatchCall), [*acknowledge user data collection*](api::PropertyAcknowledgeUserDataCollectionCall), [*ad sense links create*](api::PropertyAdSenseLinkCreateCall), [*ad sense links delete*](api::PropertyAdSenseLinkDeleteCall), [*ad sense links get*](api::PropertyAdSenseLinkGetCall), [*ad sense links list*](api::PropertyAdSenseLinkListCall), [*audiences archive*](api::PropertyAudienceArchiveCall), [*audiences create*](api::PropertyAudienceCreateCall), [*audiences get*](api::PropertyAudienceGetCall), [*audiences list*](api::PropertyAudienceListCall), [*audiences patch*](api::PropertyAudiencePatchCall), [*big query links get*](api::PropertyBigQueryLinkGetCall), [*big query links list*](api::PropertyBigQueryLinkListCall), [*calculated metrics create*](api::PropertyCalculatedMetricCreateCall), [*calculated metrics delete*](api::PropertyCalculatedMetricDeleteCall), [*calculated metrics get*](api::PropertyCalculatedMetricGetCall), [*calculated metrics list*](api::PropertyCalculatedMetricListCall), [*calculated metrics patch*](api::PropertyCalculatedMetricPatchCall), [*channel groups create*](api::PropertyChannelGroupCreateCall), [*channel groups delete*](api::PropertyChannelGroupDeleteCall), [*channel groups get*](api::PropertyChannelGroupGetCall), [*channel groups list*](api::PropertyChannelGroupListCall), [*channel groups patch*](api::PropertyChannelGroupPatchCall), [*conversion events create*](api::PropertyConversionEventCreateCall), [*conversion events delete*](api::PropertyConversionEventDeleteCall), [*conversion events get*](api::PropertyConversionEventGetCall), [*conversion events list*](api::PropertyConversionEventListCall), [*conversion events patch*](api::PropertyConversionEventPatchCall), [*create*](api::PropertyCreateCall), [*create connected site tag*](api::PropertyCreateConnectedSiteTagCall), [*create rollup property*](api::PropertyCreateRollupPropertyCall), [*create subproperty*](api::PropertyCreateSubpropertyCall), [*custom dimensions archive*](api::PropertyCustomDimensionArchiveCall), [*custom dimensions create*](api::PropertyCustomDimensionCreateCall), [*custom dimensions get*](api::PropertyCustomDimensionGetCall), [*custom dimensions list*](api::PropertyCustomDimensionListCall), [*custom dimensions patch*](api::PropertyCustomDimensionPatchCall), [*custom metrics archive*](api::PropertyCustomMetricArchiveCall), [*custom metrics create*](api::PropertyCustomMetricCreateCall), [*custom metrics get*](api::PropertyCustomMetricGetCall), [*custom metrics list*](api::PropertyCustomMetricListCall), [*custom metrics patch*](api::PropertyCustomMetricPatchCall), [*data streams create*](api::PropertyDataStreamCreateCall), [*data streams delete*](api::PropertyDataStreamDeleteCall), [*data streams event create rules create*](api::PropertyDataStreamEventCreateRuleCreateCall), [*data streams event create rules delete*](api::PropertyDataStreamEventCreateRuleDeleteCall), [*data streams event create rules get*](api::PropertyDataStreamEventCreateRuleGetCall), [*data streams event create rules list*](api::PropertyDataStreamEventCreateRuleListCall), [*data streams event create rules patch*](api::PropertyDataStreamEventCreateRulePatchCall), [*data streams get*](api::PropertyDataStreamGetCall), [*data streams get data redaction settings*](api::PropertyDataStreamGetDataRedactionSettingCall), [*data streams get enhanced measurement settings*](api::PropertyDataStreamGetEnhancedMeasurementSettingCall), [*data streams get global site tag*](api::PropertyDataStreamGetGlobalSiteTagCall), [*data streams list*](api::PropertyDataStreamListCall), [*data streams measurement protocol secrets create*](api::PropertyDataStreamMeasurementProtocolSecretCreateCall), [*data streams measurement protocol secrets delete*](api::PropertyDataStreamMeasurementProtocolSecretDeleteCall), [*data streams measurement protocol secrets get*](api::PropertyDataStreamMeasurementProtocolSecretGetCall), [*data streams measurement protocol secrets list*](api::PropertyDataStreamMeasurementProtocolSecretListCall), [*data streams measurement protocol secrets patch*](api::PropertyDataStreamMeasurementProtocolSecretPatchCall), [*data streams patch*](api::PropertyDataStreamPatchCall), [*data streams s k ad network conversion value schema create*](api::PropertyDataStreamSKAdNetworkConversionValueSchemaCreateCall), [*data streams s k ad network conversion value schema delete*](api::PropertyDataStreamSKAdNetworkConversionValueSchemaDeleteCall), [*data streams s k ad network conversion value schema get*](api::PropertyDataStreamSKAdNetworkConversionValueSchemaGetCall), [*data streams s k ad network conversion value schema list*](api::PropertyDataStreamSKAdNetworkConversionValueSchemaListCall), [*data streams s k ad network conversion value schema patch*](api::PropertyDataStreamSKAdNetworkConversionValueSchemaPatchCall), [*data streams update data redaction settings*](api::PropertyDataStreamUpdateDataRedactionSettingCall), [*data streams update enhanced measurement settings*](api::PropertyDataStreamUpdateEnhancedMeasurementSettingCall), [*delete*](api::PropertyDeleteCall), [*delete connected site tag*](api::PropertyDeleteConnectedSiteTagCall), [*display video360 advertiser link proposals approve*](api::PropertyDisplayVideo360AdvertiserLinkProposalApproveCall), [*display video360 advertiser link proposals cancel*](api::PropertyDisplayVideo360AdvertiserLinkProposalCancelCall), [*display video360 advertiser link proposals create*](api::PropertyDisplayVideo360AdvertiserLinkProposalCreateCall), [*display video360 advertiser link proposals delete*](api::PropertyDisplayVideo360AdvertiserLinkProposalDeleteCall), [*display video360 advertiser link proposals get*](api::PropertyDisplayVideo360AdvertiserLinkProposalGetCall), [*display video360 advertiser link proposals list*](api::PropertyDisplayVideo360AdvertiserLinkProposalListCall), [*display video360 advertiser links create*](api::PropertyDisplayVideo360AdvertiserLinkCreateCall), [*display video360 advertiser links delete*](api::PropertyDisplayVideo360AdvertiserLinkDeleteCall), [*display video360 advertiser links get*](api::PropertyDisplayVideo360AdvertiserLinkGetCall), [*display video360 advertiser links list*](api::PropertyDisplayVideo360AdvertiserLinkListCall), [*display video360 advertiser links patch*](api::PropertyDisplayVideo360AdvertiserLinkPatchCall), [*expanded data sets create*](api::PropertyExpandedDataSetCreateCall), [*expanded data sets delete*](api::PropertyExpandedDataSetDeleteCall), [*expanded data sets get*](api::PropertyExpandedDataSetGetCall), [*expanded data sets list*](api::PropertyExpandedDataSetListCall), [*expanded data sets patch*](api::PropertyExpandedDataSetPatchCall), [*fetch automated ga4 configuration opt out*](api::PropertyFetchAutomatedGa4ConfigurationOptOutCall), [*fetch connected ga4 property*](api::PropertyFetchConnectedGa4PropertyCall), [*firebase links create*](api::PropertyFirebaseLinkCreateCall), [*firebase links delete*](api::PropertyFirebaseLinkDeleteCall), [*firebase links list*](api::PropertyFirebaseLinkListCall), [*get*](api::PropertyGetCall), [*get attribution settings*](api::PropertyGetAttributionSettingCall), [*get data retention settings*](api::PropertyGetDataRetentionSettingCall), [*get google signals settings*](api::PropertyGetGoogleSignalsSettingCall), [*google ads links create*](api::PropertyGoogleAdsLinkCreateCall), [*google ads links delete*](api::PropertyGoogleAdsLinkDeleteCall), [*google ads links list*](api::PropertyGoogleAdsLinkListCall), [*google ads links patch*](api::PropertyGoogleAdsLinkPatchCall), [*list*](api::PropertyListCall), [*list connected site tags*](api::PropertyListConnectedSiteTagCall), [*patch*](api::PropertyPatchCall), [*rollup property source links create*](api::PropertyRollupPropertySourceLinkCreateCall), [*rollup property source links delete*](api::PropertyRollupPropertySourceLinkDeleteCall), [*rollup property source links get*](api::PropertyRollupPropertySourceLinkGetCall), [*rollup property source links list*](api::PropertyRollupPropertySourceLinkListCall), [*run access report*](api::PropertyRunAccessReportCall), [*search ads360 links create*](api::PropertySearchAds360LinkCreateCall), [*search ads360 links delete*](api::PropertySearchAds360LinkDeleteCall), [*search ads360 links get*](api::PropertySearchAds360LinkGetCall), [*search ads360 links list*](api::PropertySearchAds360LinkListCall), [*search ads360 links patch*](api::PropertySearchAds360LinkPatchCall), [*set automated ga4 configuration opt out*](api::PropertySetAutomatedGa4ConfigurationOptOutCall), [*subproperty event filters create*](api::PropertySubpropertyEventFilterCreateCall), [*subproperty event filters delete*](api::PropertySubpropertyEventFilterDeleteCall), [*subproperty event filters get*](api::PropertySubpropertyEventFilterGetCall), [*subproperty event filters list*](api::PropertySubpropertyEventFilterListCall), [*subproperty event filters patch*](api::PropertySubpropertyEventFilterPatchCall), [*update attribution settings*](api::PropertyUpdateAttributionSettingCall), [*update data retention settings*](api::PropertyUpdateDataRetentionSettingCall) and [*update google signals settings*](api::PropertyUpdateGoogleSignalsSettingCall)
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
//! * **[Hub](GoogleAnalyticsAdmin)**
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
//! let r = hub.accounts().access_bindings_batch_delete(...).doit().await
//! let r = hub.accounts().access_bindings_delete(...).doit().await
//! let r = hub.accounts().delete(...).doit().await
//! let r = hub.properties().access_bindings_batch_delete(...).doit().await
//! let r = hub.properties().access_bindings_delete(...).doit().await
//! let r = hub.properties().ad_sense_links_delete(...).doit().await
//! let r = hub.properties().audiences_archive(...).doit().await
//! let r = hub.properties().calculated_metrics_delete(...).doit().await
//! let r = hub.properties().channel_groups_delete(...).doit().await
//! let r = hub.properties().conversion_events_delete(...).doit().await
//! let r = hub.properties().custom_dimensions_archive(...).doit().await
//! let r = hub.properties().custom_metrics_archive(...).doit().await
//! let r = hub.properties().data_streams_event_create_rules_delete(...).doit().await
//! let r = hub.properties().data_streams_measurement_protocol_secrets_delete(...).doit().await
//! let r = hub.properties().data_streams_s_k_ad_network_conversion_value_schema_delete(...).doit().await
//! let r = hub.properties().data_streams_delete(...).doit().await
//! let r = hub.properties().display_video360_advertiser_link_proposals_delete(...).doit().await
//! let r = hub.properties().display_video360_advertiser_links_delete(...).doit().await
//! let r = hub.properties().expanded_data_sets_delete(...).doit().await
//! let r = hub.properties().firebase_links_delete(...).doit().await
//! let r = hub.properties().google_ads_links_delete(...).doit().await
//! let r = hub.properties().rollup_property_source_links_delete(...).doit().await
//! let r = hub.properties().search_ads360_links_delete(...).doit().await
//! let r = hub.properties().subproperty_event_filters_delete(...).doit().await
//! let r = hub.properties().delete_connected_site_tag(...).doit().await
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
//! google-analyticsadmin1_alpha = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_analyticsadmin1_alpha as analyticsadmin1_alpha;
//! use analyticsadmin1_alpha::api::GoogleAnalyticsAdminV1alphaBatchDeleteAccessBindingsRequest;
//! use analyticsadmin1_alpha::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use analyticsadmin1_alpha::{GoogleAnalyticsAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = GoogleAnalyticsAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleAnalyticsAdminV1alphaBatchDeleteAccessBindingsRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().access_bindings_batch_delete(req, "parent")
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
pub use api::GoogleAnalyticsAdmin;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;