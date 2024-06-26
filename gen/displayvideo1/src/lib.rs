// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Display Video* crate version *5.0.5+20240620*, where *20240620* is the exact revision of the *displayvideo:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Display Video* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/display-video/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/displayvideo1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](DisplayVideo) ... 
//! 
//! * [advertisers](api::Advertiser)
//!  * [*assets upload*](api::AdvertiserAssetUploadCall), [*audit*](api::AdvertiserAuditCall), [*bulk edit advertiser assigned targeting options*](api::AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall), [*bulk list advertiser assigned targeting options*](api::AdvertiserBulkListAdvertiserAssignedTargetingOptionCall), [*campaigns bulk list campaign assigned targeting options*](api::AdvertiserCampaignBulkListCampaignAssignedTargetingOptionCall), [*campaigns create*](api::AdvertiserCampaignCreateCall), [*campaigns delete*](api::AdvertiserCampaignDeleteCall), [*campaigns get*](api::AdvertiserCampaignGetCall), [*campaigns list*](api::AdvertiserCampaignListCall), [*campaigns patch*](api::AdvertiserCampaignPatchCall), [*campaigns targeting types assigned targeting options get*](api::AdvertiserCampaignTargetingTypeAssignedTargetingOptionGetCall), [*campaigns targeting types assigned targeting options list*](api::AdvertiserCampaignTargetingTypeAssignedTargetingOptionListCall), [*channels create*](api::AdvertiserChannelCreateCall), [*channels get*](api::AdvertiserChannelGetCall), [*channels list*](api::AdvertiserChannelListCall), [*channels patch*](api::AdvertiserChannelPatchCall), [*channels sites bulk edit*](api::AdvertiserChannelSiteBulkEditCall), [*channels sites create*](api::AdvertiserChannelSiteCreateCall), [*channels sites delete*](api::AdvertiserChannelSiteDeleteCall), [*channels sites list*](api::AdvertiserChannelSiteListCall), [*channels sites replace*](api::AdvertiserChannelSiteReplaceCall), [*create*](api::AdvertiserCreateCall), [*creatives create*](api::AdvertiserCreativeCreateCall), [*creatives delete*](api::AdvertiserCreativeDeleteCall), [*creatives get*](api::AdvertiserCreativeGetCall), [*creatives list*](api::AdvertiserCreativeListCall), [*creatives patch*](api::AdvertiserCreativePatchCall), [*delete*](api::AdvertiserDeleteCall), [*get*](api::AdvertiserGetCall), [*insertion orders bulk list insertion order assigned targeting options*](api::AdvertiserInsertionOrderBulkListInsertionOrderAssignedTargetingOptionCall), [*insertion orders create*](api::AdvertiserInsertionOrderCreateCall), [*insertion orders delete*](api::AdvertiserInsertionOrderDeleteCall), [*insertion orders get*](api::AdvertiserInsertionOrderGetCall), [*insertion orders list*](api::AdvertiserInsertionOrderListCall), [*insertion orders patch*](api::AdvertiserInsertionOrderPatchCall), [*insertion orders targeting types assigned targeting options get*](api::AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionGetCall), [*insertion orders targeting types assigned targeting options list*](api::AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionListCall), [*invoices list*](api::AdvertiserInvoiceListCall), [*invoices lookup invoice currency*](api::AdvertiserInvoiceLookupInvoiceCurrencyCall), [*line items bulk edit line item assigned targeting options*](api::AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall), [*line items bulk list line item assigned targeting options*](api::AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall), [*line items create*](api::AdvertiserLineItemCreateCall), [*line items delete*](api::AdvertiserLineItemDeleteCall), [*line items generate default*](api::AdvertiserLineItemGenerateDefaultCall), [*line items get*](api::AdvertiserLineItemGetCall), [*line items list*](api::AdvertiserLineItemListCall), [*line items patch*](api::AdvertiserLineItemPatchCall), [*line items targeting types assigned targeting options create*](api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall), [*line items targeting types assigned targeting options delete*](api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall), [*line items targeting types assigned targeting options get*](api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall), [*line items targeting types assigned targeting options list*](api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall), [*list*](api::AdvertiserListCall), [*location lists assigned locations bulk edit*](api::AdvertiserLocationListAssignedLocationBulkEditCall), [*location lists assigned locations create*](api::AdvertiserLocationListAssignedLocationCreateCall), [*location lists assigned locations delete*](api::AdvertiserLocationListAssignedLocationDeleteCall), [*location lists assigned locations list*](api::AdvertiserLocationListAssignedLocationListCall), [*location lists create*](api::AdvertiserLocationListCreateCall), [*location lists get*](api::AdvertiserLocationListGetCall), [*location lists list*](api::AdvertiserLocationListListCall), [*location lists patch*](api::AdvertiserLocationListPatchCall), [*manual triggers activate*](api::AdvertiserManualTriggerActivateCall), [*manual triggers create*](api::AdvertiserManualTriggerCreateCall), [*manual triggers deactivate*](api::AdvertiserManualTriggerDeactivateCall), [*manual triggers get*](api::AdvertiserManualTriggerGetCall), [*manual triggers list*](api::AdvertiserManualTriggerListCall), [*manual triggers patch*](api::AdvertiserManualTriggerPatchCall), [*negative keyword lists create*](api::AdvertiserNegativeKeywordListCreateCall), [*negative keyword lists delete*](api::AdvertiserNegativeKeywordListDeleteCall), [*negative keyword lists get*](api::AdvertiserNegativeKeywordListGetCall), [*negative keyword lists list*](api::AdvertiserNegativeKeywordListListCall), [*negative keyword lists negative keywords bulk edit*](api::AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall), [*negative keyword lists negative keywords create*](api::AdvertiserNegativeKeywordListNegativeKeywordCreateCall), [*negative keyword lists negative keywords delete*](api::AdvertiserNegativeKeywordListNegativeKeywordDeleteCall), [*negative keyword lists negative keywords list*](api::AdvertiserNegativeKeywordListNegativeKeywordListCall), [*negative keyword lists negative keywords replace*](api::AdvertiserNegativeKeywordListNegativeKeywordReplaceCall), [*negative keyword lists patch*](api::AdvertiserNegativeKeywordListPatchCall), [*patch*](api::AdvertiserPatchCall), [*targeting types assigned targeting options create*](api::AdvertiserTargetingTypeAssignedTargetingOptionCreateCall), [*targeting types assigned targeting options delete*](api::AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall), [*targeting types assigned targeting options get*](api::AdvertiserTargetingTypeAssignedTargetingOptionGetCall) and [*targeting types assigned targeting options list*](api::AdvertiserTargetingTypeAssignedTargetingOptionListCall)
//! * [combined audiences](api::CombinedAudience)
//!  * [*get*](api::CombinedAudienceGetCall) and [*list*](api::CombinedAudienceListCall)
//! * [custom bidding algorithms](api::CustomBiddingAlgorithm)
//!  * [*create*](api::CustomBiddingAlgorithmCreateCall), [*get*](api::CustomBiddingAlgorithmGetCall), [*list*](api::CustomBiddingAlgorithmListCall), [*patch*](api::CustomBiddingAlgorithmPatchCall), [*scripts create*](api::CustomBiddingAlgorithmScriptCreateCall), [*scripts get*](api::CustomBiddingAlgorithmScriptGetCall), [*scripts list*](api::CustomBiddingAlgorithmScriptListCall) and [*upload script*](api::CustomBiddingAlgorithmUploadScriptCall)
//! * [custom lists](api::CustomList)
//!  * [*get*](api::CustomListGetCall) and [*list*](api::CustomListListCall)
//! * [first and third party audiences](api::FirstAndThirdPartyAudience)
//!  * [*create*](api::FirstAndThirdPartyAudienceCreateCall), [*edit customer match members*](api::FirstAndThirdPartyAudienceEditCustomerMatchMemberCall), [*get*](api::FirstAndThirdPartyAudienceGetCall), [*list*](api::FirstAndThirdPartyAudienceListCall) and [*patch*](api::FirstAndThirdPartyAudiencePatchCall)
//! * [floodlight groups](api::FloodlightGroup)
//!  * [*get*](api::FloodlightGroupGetCall) and [*patch*](api::FloodlightGroupPatchCall)
//! * [google audiences](api::GoogleAudience)
//!  * [*get*](api::GoogleAudienceGetCall) and [*list*](api::GoogleAudienceListCall)
//! * [guaranteed orders](api::GuaranteedOrder)
//!  * [*create*](api::GuaranteedOrderCreateCall), [*edit guaranteed order read accessors*](api::GuaranteedOrderEditGuaranteedOrderReadAccessorCall), [*get*](api::GuaranteedOrderGetCall), [*list*](api::GuaranteedOrderListCall) and [*patch*](api::GuaranteedOrderPatchCall)
//! * [inventory source groups](api::InventorySourceGroup)
//!  * [*assigned inventory sources bulk edit*](api::InventorySourceGroupAssignedInventorySourceBulkEditCall), [*assigned inventory sources create*](api::InventorySourceGroupAssignedInventorySourceCreateCall), [*assigned inventory sources delete*](api::InventorySourceGroupAssignedInventorySourceDeleteCall), [*assigned inventory sources list*](api::InventorySourceGroupAssignedInventorySourceListCall), [*create*](api::InventorySourceGroupCreateCall), [*delete*](api::InventorySourceGroupDeleteCall), [*get*](api::InventorySourceGroupGetCall), [*list*](api::InventorySourceGroupListCall) and [*patch*](api::InventorySourceGroupPatchCall)
//! * [inventory sources](api::InventorySource)
//!  * [*create*](api::InventorySourceCreateCall), [*edit inventory source read write accessors*](api::InventorySourceEditInventorySourceReadWriteAccessorCall), [*get*](api::InventorySourceGetCall), [*list*](api::InventorySourceListCall) and [*patch*](api::InventorySourcePatchCall)
//! * media
//!  * [*download*](api::MediaDownloadCall) and [*upload*](api::MediaUploadCall)
//! * [partners](api::Partner)
//!  * [*bulk edit partner assigned targeting options*](api::PartnerBulkEditPartnerAssignedTargetingOptionCall), [*channels create*](api::PartnerChannelCreateCall), [*channels get*](api::PartnerChannelGetCall), [*channels list*](api::PartnerChannelListCall), [*channels patch*](api::PartnerChannelPatchCall), [*channels sites bulk edit*](api::PartnerChannelSiteBulkEditCall), [*channels sites create*](api::PartnerChannelSiteCreateCall), [*channels sites delete*](api::PartnerChannelSiteDeleteCall), [*channels sites list*](api::PartnerChannelSiteListCall), [*channels sites replace*](api::PartnerChannelSiteReplaceCall), [*get*](api::PartnerGetCall), [*list*](api::PartnerListCall), [*targeting types assigned targeting options create*](api::PartnerTargetingTypeAssignedTargetingOptionCreateCall), [*targeting types assigned targeting options delete*](api::PartnerTargetingTypeAssignedTargetingOptionDeleteCall), [*targeting types assigned targeting options get*](api::PartnerTargetingTypeAssignedTargetingOptionGetCall) and [*targeting types assigned targeting options list*](api::PartnerTargetingTypeAssignedTargetingOptionListCall)
//! * sdfdownloadtasks
//!  * [*create*](api::SdfdownloadtaskCreateCall) and [*operations get*](api::SdfdownloadtaskOperationGetCall)
//! * targeting types
//!  * [*targeting options get*](api::TargetingTypeTargetingOptionGetCall), [*targeting options list*](api::TargetingTypeTargetingOptionListCall) and [*targeting options search*](api::TargetingTypeTargetingOptionSearchCall)
//! * [users](api::User)
//!  * [*bulk edit assigned user roles*](api::UserBulkEditAssignedUserRoleCall), [*create*](api::UserCreateCall), [*delete*](api::UserDeleteCall), [*get*](api::UserGetCall), [*list*](api::UserListCall) and [*patch*](api::UserPatchCall)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*assets upload advertisers*](api::AdvertiserAssetUploadCall)
//! * [*upload media*](api::MediaUploadCall)
//! 
//! Download supported by ...
//! 
//! * [*download media*](api::MediaDownloadCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](DisplayVideo)**
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
//! let r = hub.advertisers().assets_upload(...).doit().await
//! let r = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_get(...).doit().await
//! let r = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_list(...).doit().await
//! let r = hub.advertisers().campaigns_bulk_list_campaign_assigned_targeting_options(...).doit().await
//! let r = hub.advertisers().campaigns_create(...).doit().await
//! let r = hub.advertisers().campaigns_delete(...).doit().await
//! let r = hub.advertisers().campaigns_get(...).doit().await
//! let r = hub.advertisers().campaigns_list(...).doit().await
//! let r = hub.advertisers().campaigns_patch(...).doit().await
//! let r = hub.advertisers().channels_sites_bulk_edit(...).doit().await
//! let r = hub.advertisers().channels_sites_create(...).doit().await
//! let r = hub.advertisers().channels_sites_delete(...).doit().await
//! let r = hub.advertisers().channels_sites_list(...).doit().await
//! let r = hub.advertisers().channels_sites_replace(...).doit().await
//! let r = hub.advertisers().channels_create(...).doit().await
//! let r = hub.advertisers().channels_get(...).doit().await
//! let r = hub.advertisers().channels_list(...).doit().await
//! let r = hub.advertisers().channels_patch(...).doit().await
//! let r = hub.advertisers().creatives_create(...).doit().await
//! let r = hub.advertisers().creatives_delete(...).doit().await
//! let r = hub.advertisers().creatives_get(...).doit().await
//! let r = hub.advertisers().creatives_list(...).doit().await
//! let r = hub.advertisers().creatives_patch(...).doit().await
//! let r = hub.advertisers().insertion_orders_targeting_types_assigned_targeting_options_get(...).doit().await
//! let r = hub.advertisers().insertion_orders_targeting_types_assigned_targeting_options_list(...).doit().await
//! let r = hub.advertisers().insertion_orders_bulk_list_insertion_order_assigned_targeting_options(...).doit().await
//! let r = hub.advertisers().insertion_orders_create(...).doit().await
//! let r = hub.advertisers().insertion_orders_delete(...).doit().await
//! let r = hub.advertisers().insertion_orders_get(...).doit().await
//! let r = hub.advertisers().insertion_orders_list(...).doit().await
//! let r = hub.advertisers().insertion_orders_patch(...).doit().await
//! let r = hub.advertisers().invoices_list(...).doit().await
//! let r = hub.advertisers().invoices_lookup_invoice_currency(...).doit().await
//! let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_create(...).doit().await
//! let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_delete(...).doit().await
//! let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_get(...).doit().await
//! let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_list(...).doit().await
//! let r = hub.advertisers().line_items_bulk_edit_line_item_assigned_targeting_options(...).doit().await
//! let r = hub.advertisers().line_items_bulk_list_line_item_assigned_targeting_options(...).doit().await
//! let r = hub.advertisers().line_items_create(...).doit().await
//! let r = hub.advertisers().line_items_delete(...).doit().await
//! let r = hub.advertisers().line_items_generate_default(...).doit().await
//! let r = hub.advertisers().line_items_get(...).doit().await
//! let r = hub.advertisers().line_items_list(...).doit().await
//! let r = hub.advertisers().line_items_patch(...).doit().await
//! let r = hub.advertisers().location_lists_assigned_locations_bulk_edit(...).doit().await
//! let r = hub.advertisers().location_lists_assigned_locations_create(...).doit().await
//! let r = hub.advertisers().location_lists_assigned_locations_delete(...).doit().await
//! let r = hub.advertisers().location_lists_assigned_locations_list(...).doit().await
//! let r = hub.advertisers().location_lists_create(...).doit().await
//! let r = hub.advertisers().location_lists_get(...).doit().await
//! let r = hub.advertisers().location_lists_list(...).doit().await
//! let r = hub.advertisers().location_lists_patch(...).doit().await
//! let r = hub.advertisers().manual_triggers_activate(...).doit().await
//! let r = hub.advertisers().manual_triggers_create(...).doit().await
//! let r = hub.advertisers().manual_triggers_deactivate(...).doit().await
//! let r = hub.advertisers().manual_triggers_get(...).doit().await
//! let r = hub.advertisers().manual_triggers_list(...).doit().await
//! let r = hub.advertisers().manual_triggers_patch(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_negative_keywords_bulk_edit(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_negative_keywords_create(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_negative_keywords_delete(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_negative_keywords_list(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_negative_keywords_replace(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_create(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_delete(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_get(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_list(...).doit().await
//! let r = hub.advertisers().negative_keyword_lists_patch(...).doit().await
//! let r = hub.advertisers().targeting_types_assigned_targeting_options_create(...).doit().await
//! let r = hub.advertisers().targeting_types_assigned_targeting_options_delete(...).doit().await
//! let r = hub.advertisers().targeting_types_assigned_targeting_options_get(...).doit().await
//! let r = hub.advertisers().targeting_types_assigned_targeting_options_list(...).doit().await
//! let r = hub.advertisers().audit(...).doit().await
//! let r = hub.advertisers().bulk_edit_advertiser_assigned_targeting_options(...).doit().await
//! let r = hub.advertisers().bulk_list_advertiser_assigned_targeting_options(...).doit().await
//! let r = hub.advertisers().create(...).doit().await
//! let r = hub.advertisers().delete(...).doit().await
//! let r = hub.advertisers().get(...).doit().await
//! let r = hub.advertisers().list(...).doit().await
//! let r = hub.advertisers().patch(...).doit().await
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
//! google-displayvideo1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_displayvideo1 as displayvideo1;
//! use displayvideo1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_list(-80, -2, "targetingType")
//!              .page_token("amet.")
//!              .page_size(-20)
//!              .order_by("ipsum")
//!              .filter("gubergren")
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
pub use api::DisplayVideo;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;