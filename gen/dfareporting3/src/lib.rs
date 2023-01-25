// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *dfareporting* crate version *5.0.2-beta-1+20180830*, where *20180830* is the exact revision of the *dfareporting:v3.0* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.2-beta-1*.
//! 
//! Everything else about the *dfareporting* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/doubleclick-advertisers/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dfareporting3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Dfareporting) ... 
//! 
//! * [account active ad summaries](api::AccountActiveAdSummary)
//!  * [*get*](api::AccountActiveAdSummaryGetCall)
//! * [account permission groups](api::AccountPermissionGroup)
//!  * [*get*](api::AccountPermissionGroupGetCall) and [*list*](api::AccountPermissionGroupListCall)
//! * [account permissions](api::AccountPermission)
//!  * [*get*](api::AccountPermissionGetCall) and [*list*](api::AccountPermissionListCall)
//! * [account user profiles](api::AccountUserProfile)
//!  * [*get*](api::AccountUserProfileGetCall), [*insert*](api::AccountUserProfileInsertCall), [*list*](api::AccountUserProfileListCall), [*patch*](api::AccountUserProfilePatchCall) and [*update*](api::AccountUserProfileUpdateCall)
//! * [accounts](api::Account)
//!  * [*get*](api::AccountGetCall), [*list*](api::AccountListCall), [*patch*](api::AccountPatchCall) and [*update*](api::AccountUpdateCall)
//! * [ads](api::Ad)
//!  * [*get*](api::AdGetCall), [*insert*](api::AdInsertCall), [*list*](api::AdListCall), [*patch*](api::AdPatchCall) and [*update*](api::AdUpdateCall)
//! * [advertiser groups](api::AdvertiserGroup)
//!  * [*delete*](api::AdvertiserGroupDeleteCall), [*get*](api::AdvertiserGroupGetCall), [*insert*](api::AdvertiserGroupInsertCall), [*list*](api::AdvertiserGroupListCall), [*patch*](api::AdvertiserGroupPatchCall) and [*update*](api::AdvertiserGroupUpdateCall)
//! * advertiser landing pages
//!  * [*get*](api::AdvertiserLandingPageGetCall), [*insert*](api::AdvertiserLandingPageInsertCall), [*list*](api::AdvertiserLandingPageListCall), [*patch*](api::AdvertiserLandingPagePatchCall) and [*update*](api::AdvertiserLandingPageUpdateCall)
//! * [advertisers](api::Advertiser)
//!  * [*get*](api::AdvertiserGetCall), [*insert*](api::AdvertiserInsertCall), [*list*](api::AdvertiserListCall), [*patch*](api::AdvertiserPatchCall) and [*update*](api::AdvertiserUpdateCall)
//! * [browsers](api::Browser)
//!  * [*list*](api::BrowserListCall)
//! * [campaign creative associations](api::CampaignCreativeAssociation)
//!  * [*insert*](api::CampaignCreativeAssociationInsertCall) and [*list*](api::CampaignCreativeAssociationListCall)
//! * [campaigns](api::Campaign)
//!  * [*get*](api::CampaignGetCall), [*insert*](api::CampaignInsertCall), [*list*](api::CampaignListCall), [*patch*](api::CampaignPatchCall) and [*update*](api::CampaignUpdateCall)
//! * [change logs](api::ChangeLog)
//!  * [*get*](api::ChangeLogGetCall) and [*list*](api::ChangeLogListCall)
//! * [cities](api::City)
//!  * [*list*](api::CityListCall)
//! * [connection types](api::ConnectionType)
//!  * [*get*](api::ConnectionTypeGetCall) and [*list*](api::ConnectionTypeListCall)
//! * [content categories](api::ContentCategory)
//!  * [*delete*](api::ContentCategoryDeleteCall), [*get*](api::ContentCategoryGetCall), [*insert*](api::ContentCategoryInsertCall), [*list*](api::ContentCategoryListCall), [*patch*](api::ContentCategoryPatchCall) and [*update*](api::ContentCategoryUpdateCall)
//! * [conversions](api::Conversion)
//!  * [*batchinsert*](api::ConversionBatchinsertCall) and [*batchupdate*](api::ConversionBatchupdateCall)
//! * [countries](api::Country)
//!  * [*get*](api::CountryGetCall) and [*list*](api::CountryListCall)
//! * [creative assets](api::CreativeAsset)
//!  * [*insert*](api::CreativeAssetInsertCall)
//! * [creative field values](api::CreativeFieldValue)
//!  * [*delete*](api::CreativeFieldValueDeleteCall), [*get*](api::CreativeFieldValueGetCall), [*insert*](api::CreativeFieldValueInsertCall), [*list*](api::CreativeFieldValueListCall), [*patch*](api::CreativeFieldValuePatchCall) and [*update*](api::CreativeFieldValueUpdateCall)
//! * [creative fields](api::CreativeField)
//!  * [*delete*](api::CreativeFieldDeleteCall), [*get*](api::CreativeFieldGetCall), [*insert*](api::CreativeFieldInsertCall), [*list*](api::CreativeFieldListCall), [*patch*](api::CreativeFieldPatchCall) and [*update*](api::CreativeFieldUpdateCall)
//! * [creative groups](api::CreativeGroup)
//!  * [*get*](api::CreativeGroupGetCall), [*insert*](api::CreativeGroupInsertCall), [*list*](api::CreativeGroupListCall), [*patch*](api::CreativeGroupPatchCall) and [*update*](api::CreativeGroupUpdateCall)
//! * [creatives](api::Creative)
//!  * [*get*](api::CreativeGetCall), [*insert*](api::CreativeInsertCall), [*list*](api::CreativeListCall), [*patch*](api::CreativePatchCall) and [*update*](api::CreativeUpdateCall)
//! * [dimension values](api::DimensionValue)
//!  * [*query*](api::DimensionValueQueryCall)
//! * [directory site contacts](api::DirectorySiteContact)
//!  * [*get*](api::DirectorySiteContactGetCall) and [*list*](api::DirectorySiteContactListCall)
//! * [directory sites](api::DirectorySite)
//!  * [*get*](api::DirectorySiteGetCall), [*insert*](api::DirectorySiteInsertCall) and [*list*](api::DirectorySiteListCall)
//! * [dynamic targeting keys](api::DynamicTargetingKey)
//!  * [*delete*](api::DynamicTargetingKeyDeleteCall), [*insert*](api::DynamicTargetingKeyInsertCall) and [*list*](api::DynamicTargetingKeyListCall)
//! * [event tags](api::EventTag)
//!  * [*delete*](api::EventTagDeleteCall), [*get*](api::EventTagGetCall), [*insert*](api::EventTagInsertCall), [*list*](api::EventTagListCall), [*patch*](api::EventTagPatchCall) and [*update*](api::EventTagUpdateCall)
//! * [files](api::File)
//!  * [*get*](api::FileGetCall) and [*list*](api::FileListCall)
//! * [floodlight activities](api::FloodlightActivity)
//!  * [*delete*](api::FloodlightActivityDeleteCall), [*generatetag*](api::FloodlightActivityGeneratetagCall), [*get*](api::FloodlightActivityGetCall), [*insert*](api::FloodlightActivityInsertCall), [*list*](api::FloodlightActivityListCall), [*patch*](api::FloodlightActivityPatchCall) and [*update*](api::FloodlightActivityUpdateCall)
//! * [floodlight activity groups](api::FloodlightActivityGroup)
//!  * [*get*](api::FloodlightActivityGroupGetCall), [*insert*](api::FloodlightActivityGroupInsertCall), [*list*](api::FloodlightActivityGroupListCall), [*patch*](api::FloodlightActivityGroupPatchCall) and [*update*](api::FloodlightActivityGroupUpdateCall)
//! * [floodlight configurations](api::FloodlightConfiguration)
//!  * [*get*](api::FloodlightConfigurationGetCall), [*list*](api::FloodlightConfigurationListCall), [*patch*](api::FloodlightConfigurationPatchCall) and [*update*](api::FloodlightConfigurationUpdateCall)
//! * [inventory items](api::InventoryItem)
//!  * [*get*](api::InventoryItemGetCall) and [*list*](api::InventoryItemListCall)
//! * [languages](api::Language)
//!  * [*list*](api::LanguageListCall)
//! * [metros](api::Metro)
//!  * [*list*](api::MetroListCall)
//! * [mobile carriers](api::MobileCarrier)
//!  * [*get*](api::MobileCarrierGetCall) and [*list*](api::MobileCarrierListCall)
//! * [operating system versions](api::OperatingSystemVersion)
//!  * [*get*](api::OperatingSystemVersionGetCall) and [*list*](api::OperatingSystemVersionListCall)
//! * [operating systems](api::OperatingSystem)
//!  * [*get*](api::OperatingSystemGetCall) and [*list*](api::OperatingSystemListCall)
//! * [order documents](api::OrderDocument)
//!  * [*get*](api::OrderDocumentGetCall) and [*list*](api::OrderDocumentListCall)
//! * [orders](api::Order)
//!  * [*get*](api::OrderGetCall) and [*list*](api::OrderListCall)
//! * [placement groups](api::PlacementGroup)
//!  * [*get*](api::PlacementGroupGetCall), [*insert*](api::PlacementGroupInsertCall), [*list*](api::PlacementGroupListCall), [*patch*](api::PlacementGroupPatchCall) and [*update*](api::PlacementGroupUpdateCall)
//! * [placement strategies](api::PlacementStrategy)
//!  * [*delete*](api::PlacementStrategyDeleteCall), [*get*](api::PlacementStrategyGetCall), [*insert*](api::PlacementStrategyInsertCall), [*list*](api::PlacementStrategyListCall), [*patch*](api::PlacementStrategyPatchCall) and [*update*](api::PlacementStrategyUpdateCall)
//! * [placements](api::Placement)
//!  * [*generatetags*](api::PlacementGeneratetagCall), [*get*](api::PlacementGetCall), [*insert*](api::PlacementInsertCall), [*list*](api::PlacementListCall), [*patch*](api::PlacementPatchCall) and [*update*](api::PlacementUpdateCall)
//! * [platform types](api::PlatformType)
//!  * [*get*](api::PlatformTypeGetCall) and [*list*](api::PlatformTypeListCall)
//! * [postal codes](api::PostalCode)
//!  * [*get*](api::PostalCodeGetCall) and [*list*](api::PostalCodeListCall)
//! * [projects](api::Project)
//!  * [*get*](api::ProjectGetCall) and [*list*](api::ProjectListCall)
//! * [regions](api::Region)
//!  * [*list*](api::RegionListCall)
//! * [remarketing list shares](api::RemarketingListShare)
//!  * [*get*](api::RemarketingListShareGetCall), [*patch*](api::RemarketingListSharePatchCall) and [*update*](api::RemarketingListShareUpdateCall)
//! * [remarketing lists](api::RemarketingList)
//!  * [*get*](api::RemarketingListGetCall), [*insert*](api::RemarketingListInsertCall), [*list*](api::RemarketingListListCall), [*patch*](api::RemarketingListPatchCall) and [*update*](api::RemarketingListUpdateCall)
//! * [reports](api::Report)
//!  * [*compatible fields query*](api::ReportCompatibleFieldQueryCall), [*delete*](api::ReportDeleteCall), [*files get*](api::ReportFileGetCall), [*files list*](api::ReportFileListCall), [*get*](api::ReportGetCall), [*insert*](api::ReportInsertCall), [*list*](api::ReportListCall), [*patch*](api::ReportPatchCall), [*run*](api::ReportRunCall) and [*update*](api::ReportUpdateCall)
//! * [sites](api::Site)
//!  * [*get*](api::SiteGetCall), [*insert*](api::SiteInsertCall), [*list*](api::SiteListCall), [*patch*](api::SitePatchCall) and [*update*](api::SiteUpdateCall)
//! * [sizes](api::Size)
//!  * [*get*](api::SizeGetCall), [*insert*](api::SizeInsertCall) and [*list*](api::SizeListCall)
//! * [subaccounts](api::Subaccount)
//!  * [*get*](api::SubaccountGetCall), [*insert*](api::SubaccountInsertCall), [*list*](api::SubaccountListCall), [*patch*](api::SubaccountPatchCall) and [*update*](api::SubaccountUpdateCall)
//! * [targetable remarketing lists](api::TargetableRemarketingList)
//!  * [*get*](api::TargetableRemarketingListGetCall) and [*list*](api::TargetableRemarketingListListCall)
//! * [targeting templates](api::TargetingTemplate)
//!  * [*get*](api::TargetingTemplateGetCall), [*insert*](api::TargetingTemplateInsertCall), [*list*](api::TargetingTemplateListCall), [*patch*](api::TargetingTemplatePatchCall) and [*update*](api::TargetingTemplateUpdateCall)
//! * [user profiles](api::UserProfile)
//!  * [*get*](api::UserProfileGetCall) and [*list*](api::UserProfileListCall)
//! * [user role permission groups](api::UserRolePermissionGroup)
//!  * [*get*](api::UserRolePermissionGroupGetCall) and [*list*](api::UserRolePermissionGroupListCall)
//! * [user role permissions](api::UserRolePermission)
//!  * [*get*](api::UserRolePermissionGetCall) and [*list*](api::UserRolePermissionListCall)
//! * [user roles](api::UserRole)
//!  * [*delete*](api::UserRoleDeleteCall), [*get*](api::UserRoleGetCall), [*insert*](api::UserRoleInsertCall), [*list*](api::UserRoleListCall), [*patch*](api::UserRolePatchCall) and [*update*](api::UserRoleUpdateCall)
//! * [video formats](api::VideoFormat)
//!  * [*get*](api::VideoFormatGetCall) and [*list*](api::VideoFormatListCall)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*insert creative assets*](api::CreativeAssetInsertCall)
//! 
//! Download supported by ...
//! 
//! * [*get files*](api::FileGetCall)
//! * [*files get reports*](api::ReportFileGetCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Dfareporting)**
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
//! let r = hub.reports().compatible_fields_query(...).doit().await
//! let r = hub.reports().files_get(...).doit().await
//! let r = hub.reports().files_list(...).doit().await
//! let r = hub.reports().delete(...).doit().await
//! let r = hub.reports().get(...).doit().await
//! let r = hub.reports().insert(...).doit().await
//! let r = hub.reports().list(...).doit().await
//! let r = hub.reports().patch(...).doit().await
//! let r = hub.reports().run(...).doit().await
//! let r = hub.reports().update(...).doit().await
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
//! google-dfareporting3 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_dfareporting3 as dfareporting3;
//! use dfareporting3::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use dfareporting3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
//! let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.reports().files_list(-8, -80)
//!              .sort_order("amet.")
//!              .sort_field("takimata")
//!              .page_token("amet.")
//!              .max_results(-20)
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
pub use api::Dfareporting;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;