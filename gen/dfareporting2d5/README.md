<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dfareporting2d5` library allows access to all features of the *Google dfareporting* service.

This documentation was generated from *dfareporting* crate version *1.0.4+20170428*, where *20170428* is the exact revision of the *dfareporting:v2.5* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.4*.

Everything else about the *dfareporting* *v2d5* API can be found at the
[official documentation site](https://developers.google.com/doubleclick-advertisers/reporting/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Dfareporting.html) ... 

* [account active ad summaries](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountActiveAdSummary.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountActiveAdSummaryGetCall.html)
* [account permission groups](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPermissionGroup.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPermissionGroupGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPermissionGroupListCall.html)
* [account permissions](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPermission.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPermissionGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPermissionListCall.html)
* [account user profiles](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUserProfile.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUserProfileGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUserProfileInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUserProfileListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUserProfilePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUserProfileUpdateCall.html)
* [accounts](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Account.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountGetCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AccountUpdateCall.html)
* [ads](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Ad.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdUpdateCall.html)
* [advertiser groups](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroup.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroupDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGroupUpdateCall.html)
* [advertisers](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Advertiser.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.AdvertiserUpdateCall.html)
* [browsers](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Browser.html)
 * [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.BrowserListCall.html)
* [campaign creative associations](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignCreativeAssociation.html)
 * [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignCreativeAssociationInsertCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignCreativeAssociationListCall.html)
* [campaigns](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Campaign.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CampaignUpdateCall.html)
* [change logs](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ChangeLog.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ChangeLogGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ChangeLogListCall.html)
* [cities](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.City.html)
 * [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CityListCall.html)
* [connection types](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ConnectionType.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ConnectionTypeGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ConnectionTypeListCall.html)
* [content categories](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategory.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategoryDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategoryGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategoryInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategoryListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategoryPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ContentCategoryUpdateCall.html)
* [conversions](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Conversion.html)
 * [*batchinsert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ConversionBatchinsertCall.html)
* [countries](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Country.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CountryGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CountryListCall.html)
* [creative assets](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeAsset.html)
 * [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeAssetInsertCall.html)
* [creative field values](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValue.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValueDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValueGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValueInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValueListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValuePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldValueUpdateCall.html)
* [creative fields](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeField.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeFieldUpdateCall.html)
* [creative groups](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGroup.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGroupUpdateCall.html)
* [creatives](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Creative.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeUpdateCall.html)
* [dimension values](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DimensionValue.html)
 * [*query*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DimensionValueQueryCall.html)
* [directory site contacts](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySiteContact.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySiteContactGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySiteContactListCall.html)
* [directory sites](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySite.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySiteGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySiteInsertCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DirectorySiteListCall.html)
* [dynamic targeting keys](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DynamicTargetingKey.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DynamicTargetingKeyDeleteCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DynamicTargetingKeyInsertCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.DynamicTargetingKeyListCall.html)
* [event tags](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTag.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTagDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTagGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTagInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTagListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTagPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.EventTagUpdateCall.html)
* [files](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.File.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FileGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FileListCall.html)
* [floodlight activities](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivity.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityDeleteCall.html), [*generatetag*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGeneratetagCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityUpdateCall.html)
* [floodlight activity groups](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGroup.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightActivityGroupUpdateCall.html)
* [floodlight configurations](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightConfiguration.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightConfigurationGetCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightConfigurationListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightConfigurationPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FloodlightConfigurationUpdateCall.html)
* [inventory items](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.InventoryItem.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.InventoryItemGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.InventoryItemListCall.html)
* [landing pages](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPage.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPageDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPageGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPageInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPageListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPagePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.LandingPageUpdateCall.html)
* [metros](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Metro.html)
 * [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.MetroListCall.html)
* [mobile carriers](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.MobileCarrier.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.MobileCarrierGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.MobileCarrierListCall.html)
* [operating system versions](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OperatingSystemVersion.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OperatingSystemVersionGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OperatingSystemVersionListCall.html)
* [operating systems](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OperatingSystem.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OperatingSystemGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OperatingSystemListCall.html)
* [order documents](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OrderDocument.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OrderDocumentGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OrderDocumentListCall.html)
* [orders](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Order.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OrderGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.OrderListCall.html)
* [placement groups](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGroup.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGroupUpdateCall.html)
* [placement strategies](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategy.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategyDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategyGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategyInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategyListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategyPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementStrategyUpdateCall.html)
* [placements](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Placement.html)
 * [*generatetags*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGeneratetagCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlacementUpdateCall.html)
* [platform types](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlatformType.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlatformTypeGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PlatformTypeListCall.html)
* [postal codes](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PostalCode.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PostalCodeGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.PostalCodeListCall.html)
* [projects](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Project.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ProjectGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ProjectListCall.html)
* [regions](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Region.html)
 * [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RegionListCall.html)
* [remarketing list shares](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListShare.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListShareGetCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListSharePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListShareUpdateCall.html)
* [remarketing lists](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingList.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.RemarketingListUpdateCall.html)
* [reports](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Report.html)
 * [*compatible fields query*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportCompatibleFieldQueryCall.html), [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportDeleteCall.html), [*files get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportFileGetCall.html), [*files list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportFileListCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportPatchCall.html), [*run*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportRunCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportUpdateCall.html)
* [sites](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Site.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SiteGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SiteInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SiteListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SitePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SiteUpdateCall.html)
* [sizes](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Size.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SizeGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SizeInsertCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SizeListCall.html)
* [subaccounts](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Subaccount.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SubaccountGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SubaccountInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SubaccountListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SubaccountPatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.SubaccountUpdateCall.html)
* [targetable remarketing lists](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.TargetableRemarketingList.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.TargetableRemarketingListGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.TargetableRemarketingListListCall.html)
* [user profiles](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserProfile.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserProfileGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserProfileListCall.html)
* [user role permission groups](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePermissionGroup.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePermissionGroupGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePermissionGroupListCall.html)
* [user role permissions](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePermission.html)
 * [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePermissionGetCall.html) and [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePermissionListCall.html)
* [user roles](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRole.html)
 * [*delete*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRoleDeleteCall.html), [*get*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRoleGetCall.html), [*insert*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRoleInsertCall.html), [*list*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRoleListCall.html), [*patch*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRolePatchCall.html) and [*update*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.UserRoleUpdateCall.html)


Upload supported by ...

* [*insert creative assets*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.CreativeAssetInsertCall.html)

Download supported by ...

* [*get files*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.FileGetCall.html)
* [*files get reports*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.ReportFileGetCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/struct.Dfareporting.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.CallBuilder.html)**
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
google-dfareporting2d5 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_dfareporting2d5 as dfareporting2d5;
use dfareporting2d5::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use dfareporting2d5::Dfareporting;

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
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dfareporting2d5/1.0.4+20170428/google_dfareporting2d5/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **dfareporting2d5** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
