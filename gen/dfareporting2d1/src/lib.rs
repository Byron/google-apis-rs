// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *dfareporting* crate version *1.0.3+20160323*, where *20160323* is the exact revision of the *dfareporting:v2.1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.3*.
//! 
//! Everything else about the *dfareporting* *v2d1* API can be found at the
//! [official documentation site](https://developers.google.com/doubleclick-advertisers/reporting/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/dfareporting2d1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Dfareporting.html) ... 
//! 
//! * [account active ad summaries](struct.AccountActiveAdSummary.html)
//!  * [*get*](struct.AccountActiveAdSummaryGetCall.html)
//! * [account permission groups](struct.AccountPermissionGroup.html)
//!  * [*get*](struct.AccountPermissionGroupGetCall.html) and [*list*](struct.AccountPermissionGroupListCall.html)
//! * [account permissions](struct.AccountPermission.html)
//!  * [*get*](struct.AccountPermissionGetCall.html) and [*list*](struct.AccountPermissionListCall.html)
//! * [account user profiles](struct.AccountUserProfile.html)
//!  * [*get*](struct.AccountUserProfileGetCall.html), [*insert*](struct.AccountUserProfileInsertCall.html), [*list*](struct.AccountUserProfileListCall.html), [*patch*](struct.AccountUserProfilePatchCall.html) and [*update*](struct.AccountUserProfileUpdateCall.html)
//! * [accounts](struct.Account.html)
//!  * [*get*](struct.AccountGetCall.html), [*list*](struct.AccountListCall.html), [*patch*](struct.AccountPatchCall.html) and [*update*](struct.AccountUpdateCall.html)
//! * [ads](struct.Ad.html)
//!  * [*get*](struct.AdGetCall.html), [*insert*](struct.AdInsertCall.html), [*list*](struct.AdListCall.html), [*patch*](struct.AdPatchCall.html) and [*update*](struct.AdUpdateCall.html)
//! * [advertiser groups](struct.AdvertiserGroup.html)
//!  * [*delete*](struct.AdvertiserGroupDeleteCall.html), [*get*](struct.AdvertiserGroupGetCall.html), [*insert*](struct.AdvertiserGroupInsertCall.html), [*list*](struct.AdvertiserGroupListCall.html), [*patch*](struct.AdvertiserGroupPatchCall.html) and [*update*](struct.AdvertiserGroupUpdateCall.html)
//! * [advertisers](struct.Advertiser.html)
//!  * [*get*](struct.AdvertiserGetCall.html), [*insert*](struct.AdvertiserInsertCall.html), [*list*](struct.AdvertiserListCall.html), [*patch*](struct.AdvertiserPatchCall.html) and [*update*](struct.AdvertiserUpdateCall.html)
//! * [browsers](struct.Browser.html)
//!  * [*list*](struct.BrowserListCall.html)
//! * [campaign creative associations](struct.CampaignCreativeAssociation.html)
//!  * [*insert*](struct.CampaignCreativeAssociationInsertCall.html) and [*list*](struct.CampaignCreativeAssociationListCall.html)
//! * [campaigns](struct.Campaign.html)
//!  * [*get*](struct.CampaignGetCall.html), [*insert*](struct.CampaignInsertCall.html), [*list*](struct.CampaignListCall.html), [*patch*](struct.CampaignPatchCall.html) and [*update*](struct.CampaignUpdateCall.html)
//! * [change logs](struct.ChangeLog.html)
//!  * [*get*](struct.ChangeLogGetCall.html) and [*list*](struct.ChangeLogListCall.html)
//! * [cities](struct.City.html)
//!  * [*list*](struct.CityListCall.html)
//! * [connection types](struct.ConnectionType.html)
//!  * [*get*](struct.ConnectionTypeGetCall.html) and [*list*](struct.ConnectionTypeListCall.html)
//! * [content categories](struct.ContentCategory.html)
//!  * [*delete*](struct.ContentCategoryDeleteCall.html), [*get*](struct.ContentCategoryGetCall.html), [*insert*](struct.ContentCategoryInsertCall.html), [*list*](struct.ContentCategoryListCall.html), [*patch*](struct.ContentCategoryPatchCall.html) and [*update*](struct.ContentCategoryUpdateCall.html)
//! * [countries](struct.Country.html)
//!  * [*get*](struct.CountryGetCall.html) and [*list*](struct.CountryListCall.html)
//! * [creative assets](struct.CreativeAsset.html)
//!  * [*insert*](struct.CreativeAssetInsertCall.html)
//! * [creative field values](struct.CreativeFieldValue.html)
//!  * [*delete*](struct.CreativeFieldValueDeleteCall.html), [*get*](struct.CreativeFieldValueGetCall.html), [*insert*](struct.CreativeFieldValueInsertCall.html), [*list*](struct.CreativeFieldValueListCall.html), [*patch*](struct.CreativeFieldValuePatchCall.html) and [*update*](struct.CreativeFieldValueUpdateCall.html)
//! * [creative fields](struct.CreativeField.html)
//!  * [*delete*](struct.CreativeFieldDeleteCall.html), [*get*](struct.CreativeFieldGetCall.html), [*insert*](struct.CreativeFieldInsertCall.html), [*list*](struct.CreativeFieldListCall.html), [*patch*](struct.CreativeFieldPatchCall.html) and [*update*](struct.CreativeFieldUpdateCall.html)
//! * [creative groups](struct.CreativeGroup.html)
//!  * [*get*](struct.CreativeGroupGetCall.html), [*insert*](struct.CreativeGroupInsertCall.html), [*list*](struct.CreativeGroupListCall.html), [*patch*](struct.CreativeGroupPatchCall.html) and [*update*](struct.CreativeGroupUpdateCall.html)
//! * [creatives](struct.Creative.html)
//!  * [*get*](struct.CreativeGetCall.html), [*insert*](struct.CreativeInsertCall.html), [*list*](struct.CreativeListCall.html), [*patch*](struct.CreativePatchCall.html) and [*update*](struct.CreativeUpdateCall.html)
//! * [dimension values](struct.DimensionValue.html)
//!  * [*query*](struct.DimensionValueQueryCall.html)
//! * [directory site contacts](struct.DirectorySiteContact.html)
//!  * [*get*](struct.DirectorySiteContactGetCall.html) and [*list*](struct.DirectorySiteContactListCall.html)
//! * [directory sites](struct.DirectorySite.html)
//!  * [*get*](struct.DirectorySiteGetCall.html), [*insert*](struct.DirectorySiteInsertCall.html) and [*list*](struct.DirectorySiteListCall.html)
//! * [event tags](struct.EventTag.html)
//!  * [*delete*](struct.EventTagDeleteCall.html), [*get*](struct.EventTagGetCall.html), [*insert*](struct.EventTagInsertCall.html), [*list*](struct.EventTagListCall.html), [*patch*](struct.EventTagPatchCall.html) and [*update*](struct.EventTagUpdateCall.html)
//! * [files](struct.File.html)
//!  * [*get*](struct.FileGetCall.html) and [*list*](struct.FileListCall.html)
//! * [floodlight activities](struct.FloodlightActivity.html)
//!  * [*delete*](struct.FloodlightActivityDeleteCall.html), [*generatetag*](struct.FloodlightActivityGeneratetagCall.html), [*get*](struct.FloodlightActivityGetCall.html), [*insert*](struct.FloodlightActivityInsertCall.html), [*list*](struct.FloodlightActivityListCall.html), [*patch*](struct.FloodlightActivityPatchCall.html) and [*update*](struct.FloodlightActivityUpdateCall.html)
//! * [floodlight activity groups](struct.FloodlightActivityGroup.html)
//!  * [*delete*](struct.FloodlightActivityGroupDeleteCall.html), [*get*](struct.FloodlightActivityGroupGetCall.html), [*insert*](struct.FloodlightActivityGroupInsertCall.html), [*list*](struct.FloodlightActivityGroupListCall.html), [*patch*](struct.FloodlightActivityGroupPatchCall.html) and [*update*](struct.FloodlightActivityGroupUpdateCall.html)
//! * [floodlight configurations](struct.FloodlightConfiguration.html)
//!  * [*get*](struct.FloodlightConfigurationGetCall.html), [*list*](struct.FloodlightConfigurationListCall.html), [*patch*](struct.FloodlightConfigurationPatchCall.html) and [*update*](struct.FloodlightConfigurationUpdateCall.html)
//! * [inventory items](struct.InventoryItem.html)
//!  * [*get*](struct.InventoryItemGetCall.html) and [*list*](struct.InventoryItemListCall.html)
//! * [landing pages](struct.LandingPage.html)
//!  * [*delete*](struct.LandingPageDeleteCall.html), [*get*](struct.LandingPageGetCall.html), [*insert*](struct.LandingPageInsertCall.html), [*list*](struct.LandingPageListCall.html), [*patch*](struct.LandingPagePatchCall.html) and [*update*](struct.LandingPageUpdateCall.html)
//! * [metros](struct.Metro.html)
//!  * [*list*](struct.MetroListCall.html)
//! * [mobile carriers](struct.MobileCarrier.html)
//!  * [*get*](struct.MobileCarrierGetCall.html) and [*list*](struct.MobileCarrierListCall.html)
//! * [operating system versions](struct.OperatingSystemVersion.html)
//!  * [*get*](struct.OperatingSystemVersionGetCall.html) and [*list*](struct.OperatingSystemVersionListCall.html)
//! * [operating systems](struct.OperatingSystem.html)
//!  * [*get*](struct.OperatingSystemGetCall.html) and [*list*](struct.OperatingSystemListCall.html)
//! * [order documents](struct.OrderDocument.html)
//!  * [*get*](struct.OrderDocumentGetCall.html) and [*list*](struct.OrderDocumentListCall.html)
//! * [orders](struct.Order.html)
//!  * [*get*](struct.OrderGetCall.html) and [*list*](struct.OrderListCall.html)
//! * [placement groups](struct.PlacementGroup.html)
//!  * [*get*](struct.PlacementGroupGetCall.html), [*insert*](struct.PlacementGroupInsertCall.html), [*list*](struct.PlacementGroupListCall.html), [*patch*](struct.PlacementGroupPatchCall.html) and [*update*](struct.PlacementGroupUpdateCall.html)
//! * [placement strategies](struct.PlacementStrategy.html)
//!  * [*delete*](struct.PlacementStrategyDeleteCall.html), [*get*](struct.PlacementStrategyGetCall.html), [*insert*](struct.PlacementStrategyInsertCall.html), [*list*](struct.PlacementStrategyListCall.html), [*patch*](struct.PlacementStrategyPatchCall.html) and [*update*](struct.PlacementStrategyUpdateCall.html)
//! * [placements](struct.Placement.html)
//!  * [*generatetags*](struct.PlacementGeneratetagCall.html), [*get*](struct.PlacementGetCall.html), [*insert*](struct.PlacementInsertCall.html), [*list*](struct.PlacementListCall.html), [*patch*](struct.PlacementPatchCall.html) and [*update*](struct.PlacementUpdateCall.html)
//! * [platform types](struct.PlatformType.html)
//!  * [*get*](struct.PlatformTypeGetCall.html) and [*list*](struct.PlatformTypeListCall.html)
//! * [postal codes](struct.PostalCode.html)
//!  * [*get*](struct.PostalCodeGetCall.html) and [*list*](struct.PostalCodeListCall.html)
//! * [projects](struct.Project.html)
//!  * [*get*](struct.ProjectGetCall.html) and [*list*](struct.ProjectListCall.html)
//! * [regions](struct.Region.html)
//!  * [*list*](struct.RegionListCall.html)
//! * [remarketing list shares](struct.RemarketingListShare.html)
//!  * [*get*](struct.RemarketingListShareGetCall.html), [*patch*](struct.RemarketingListSharePatchCall.html) and [*update*](struct.RemarketingListShareUpdateCall.html)
//! * [remarketing lists](struct.RemarketingList.html)
//!  * [*get*](struct.RemarketingListGetCall.html), [*insert*](struct.RemarketingListInsertCall.html), [*list*](struct.RemarketingListListCall.html), [*patch*](struct.RemarketingListPatchCall.html) and [*update*](struct.RemarketingListUpdateCall.html)
//! * [reports](struct.Report.html)
//!  * [*compatible fields query*](struct.ReportCompatibleFieldQueryCall.html), [*delete*](struct.ReportDeleteCall.html), [*files get*](struct.ReportFileGetCall.html), [*files list*](struct.ReportFileListCall.html), [*get*](struct.ReportGetCall.html), [*insert*](struct.ReportInsertCall.html), [*list*](struct.ReportListCall.html), [*patch*](struct.ReportPatchCall.html), [*run*](struct.ReportRunCall.html) and [*update*](struct.ReportUpdateCall.html)
//! * [sites](struct.Site.html)
//!  * [*get*](struct.SiteGetCall.html), [*insert*](struct.SiteInsertCall.html), [*list*](struct.SiteListCall.html), [*patch*](struct.SitePatchCall.html) and [*update*](struct.SiteUpdateCall.html)
//! * [sizes](struct.Size.html)
//!  * [*get*](struct.SizeGetCall.html), [*insert*](struct.SizeInsertCall.html) and [*list*](struct.SizeListCall.html)
//! * [subaccounts](struct.Subaccount.html)
//!  * [*get*](struct.SubaccountGetCall.html), [*insert*](struct.SubaccountInsertCall.html), [*list*](struct.SubaccountListCall.html), [*patch*](struct.SubaccountPatchCall.html) and [*update*](struct.SubaccountUpdateCall.html)
//! * [targetable remarketing lists](struct.TargetableRemarketingList.html)
//!  * [*get*](struct.TargetableRemarketingListGetCall.html) and [*list*](struct.TargetableRemarketingListListCall.html)
//! * [user profiles](struct.UserProfile.html)
//!  * [*get*](struct.UserProfileGetCall.html) and [*list*](struct.UserProfileListCall.html)
//! * [user role permission groups](struct.UserRolePermissionGroup.html)
//!  * [*get*](struct.UserRolePermissionGroupGetCall.html) and [*list*](struct.UserRolePermissionGroupListCall.html)
//! * [user role permissions](struct.UserRolePermission.html)
//!  * [*get*](struct.UserRolePermissionGetCall.html) and [*list*](struct.UserRolePermissionListCall.html)
//! * [user roles](struct.UserRole.html)
//!  * [*delete*](struct.UserRoleDeleteCall.html), [*get*](struct.UserRoleGetCall.html), [*insert*](struct.UserRoleInsertCall.html), [*list*](struct.UserRoleListCall.html), [*patch*](struct.UserRolePatchCall.html) and [*update*](struct.UserRoleUpdateCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*insert creative assets*](struct.CreativeAssetInsertCall.html)
//! 
//! Download supported by ...
//! 
//! * [*get files*](struct.FileGetCall.html)
//! * [*files get reports*](struct.ReportFileGetCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Dfareporting.html)**
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
//! let r = hub.reports().run(...).doit()
//! let r = hub.reports().get(...).doit()
//! let r = hub.reports().list(...).doit()
//! let r = hub.reports().delete(...).doit()
//! let r = hub.reports().files_list(...).doit()
//! let r = hub.reports().insert(...).doit()
//! let r = hub.reports().patch(...).doit()
//! let r = hub.reports().compatible_fields_query(...).doit()
//! let r = hub.reports().update(...).doit()
//! let r = hub.reports().files_get(...).doit()
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
//! google-dfareporting2d1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_dfareporting2d1 as dfareporting2d1;
//! use dfareporting2d1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use dfareporting2d1::Dfareporting;
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
//! let mut hub = Dfareporting::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.reports().list("profileId")
//!              .sort_order("Stet")
//!              .sort_field("sed")
//!              .scope("eirmod")
//!              .page_token("elitr")
//!              .max_results(-61)
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