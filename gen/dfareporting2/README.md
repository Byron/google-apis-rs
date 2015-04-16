<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dfareporting2` library allows access to all features of the *Google dfareporting* service.

This documentation was generated from *dfareporting* crate version *0.1.5+20150326*, where *20150326* is the exact revision of the *dfareporting:v2.0* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.5*.

Everything else about the *dfareporting* *v2* API can be found at the
[official documentation site](https://developers.google.com/doubleclick-advertisers/reporting/).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Dfareporting.html) ... 

* [account active ad summaries](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountActiveAdSummary.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountActiveAdSummaryGetCall.html)
* [account permission groups](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPermissionGroup.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPermissionGroupGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPermissionGroupListCall.html)
* [account permissions](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPermission.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPermissionGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPermissionListCall.html)
* [account user profiles](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountUserProfile.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountUserProfileGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountUserProfileListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountUserProfilePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountUserProfileUpdateCall.html)
* [accounts](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Account.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AccountUpdateCall.html)
* [ads](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Ad.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdUpdateCall.html)
* [advertiser groups](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroup.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroupDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroupGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroupInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroupListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroupPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGroupUpdateCall.html)
* [advertisers](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Advertiser.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.AdvertiserUpdateCall.html)
* [browsers](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Browser.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.BrowserListCall.html)
* [campaign creative associations](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignCreativeAssociation.html)
 * [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignCreativeAssociationInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignCreativeAssociationListCall.html)
* [campaigns](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Campaign.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CampaignUpdateCall.html)
* [change logs](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ChangeLog.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ChangeLogGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ChangeLogListCall.html)
* [cities](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.City.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CityListCall.html)
* [connection types](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ConnectionType.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ConnectionTypeListCall.html)
* [content categories](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategory.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategoryDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategoryGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategoryInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategoryListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategoryPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ContentCategoryUpdateCall.html)
* [countries](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Country.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CountryGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CountryListCall.html)
* [creative assets](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeAsset.html)
 * [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeAssetInsertCall.html)
* [creative field values](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValue.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValueDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValueGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValueInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValueListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValuePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldValueUpdateCall.html)
* [creative fields](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeField.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeFieldUpdateCall.html)
* [creative groups](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGroup.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGroupGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGroupInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGroupListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGroupPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGroupUpdateCall.html)
* [creatives](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Creative.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeUpdateCall.html)
* [dimension values](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DimensionValue.html)
 * [*query*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DimensionValueQueryCall.html)
* [directory site contacts](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DirectorySiteContact.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DirectorySiteContactGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DirectorySiteContactListCall.html)
* [directory sites](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DirectorySite.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DirectorySiteGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.DirectorySiteListCall.html)
* [event tags](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTag.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTagDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTagGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTagInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTagListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTagPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.EventTagUpdateCall.html)
* [files](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.File.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FileGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FileListCall.html)
* [floodlight activities](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivity.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityDeleteCall.html), [*generatetag*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGeneratetagCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityUpdateCall.html)
* [floodlight activity groups](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroup.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroupDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroupGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroupInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroupListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroupPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightActivityGroupUpdateCall.html)
* [floodlight configurations](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightConfiguration.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightConfigurationGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightConfigurationListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightConfigurationPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FloodlightConfigurationUpdateCall.html)
* [landing pages](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPage.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPageDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPageGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPageInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPageListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPagePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.LandingPageUpdateCall.html)
* [metros](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Metro.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.MetroListCall.html)
* [mobile carriers](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.MobileCarrier.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.MobileCarrierListCall.html)
* [operating system versions](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.OperatingSystemVersion.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.OperatingSystemVersionListCall.html)
* [operating systems](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.OperatingSystem.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.OperatingSystemListCall.html)
* [placement groups](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGroup.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGroupGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGroupInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGroupListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGroupPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGroupUpdateCall.html)
* [placement strategies](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategy.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategyDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategyGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategyInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategyListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategyPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementStrategyUpdateCall.html)
* [placements](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Placement.html)
 * [*generatetags*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGeneratetagCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlacementUpdateCall.html)
* [platform types](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlatformType.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PlatformTypeListCall.html)
* [postal codes](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PostalCode.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.PostalCodeListCall.html)
* [regions](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Region.html)
 * [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.RegionListCall.html)
* [reports](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Report.html)
 * [*compatible fields query*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportCompatibleFieldQueryCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportDeleteCall.html), [*files get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportFileGetCall.html), [*files list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportFileListCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportPatchCall.html), [*run*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportRunCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportUpdateCall.html)
* [sites](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Site.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SiteGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SiteInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SiteListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SitePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SiteUpdateCall.html)
* [sizes](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Size.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SizeGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SizeInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SizeListCall.html)
* [subaccounts](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Subaccount.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SubaccountGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SubaccountInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SubaccountListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SubaccountPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.SubaccountUpdateCall.html)
* [user profiles](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserProfile.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserProfileGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserProfileListCall.html)
* [user role permission groups](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePermissionGroup.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePermissionGroupGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePermissionGroupListCall.html)
* [user role permissions](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePermission.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePermissionGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePermissionListCall.html)
* [user roles](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRole.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRoleDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRoleGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRoleInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRoleListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRolePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.UserRoleUpdateCall.html)


Upload supported by ...

* [*insert creative assets*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.CreativeAssetInsertCall.html)

Download supported by ...

* [*get files*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.FileGetCall.html)
* [*files get reports*](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.ReportFileGetCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_dfareporting2/struct.Dfareporting.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.reports().run(...).doit()
let r = hub.reports().get(...).doit()
let r = hub.reports().list(...).doit()
let r = hub.reports().delete(...).doit()
let r = hub.reports().files_list(...).doit()
let r = hub.reports().insert(...).doit()
let r = hub.reports().patch(...).doit()
let r = hub.reports().compatible_fields_query(...).doit()
let r = hub.reports().update(...).doit()
let r = hub.reports().files_get(...).doit()
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-dfareporting2 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_dfareporting2 as dfareporting2;
use dfareporting2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use dfareporting2::Dfareporting;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::new(),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Dfareporting::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.reports().list("profileId")
             .sort_order("sit")
             .sort_field("Stet")
             .scope("sed")
             .page_token("et")
             .max_results(-18)
             .doit();

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
        Error::HttpError(_)
        |Error::MissingAPIKey
        |Error::MissingToken
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_dfareporting2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_dfareporting2/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_dfareporting2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_dfareporting2/trait.RequestValue.html) are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **dfareporting2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
