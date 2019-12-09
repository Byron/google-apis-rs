<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dfareporting3d2` library allows access to all features of the *Google dfareporting* service.

This documentation was generated from *dfareporting* crate version *1.0.12+20190531*, where *20190531* is the exact revision of the *dfareporting:v3.2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.12*.

Everything else about the *dfareporting* *v3d2* API can be found at the
[official documentation site](https://developers.google.com/doubleclick-advertisers/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Dfareporting.html) ... 

* [account active ad summaries](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountActiveAdSummary.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountActiveAdSummaryGetCall.html)
* [account permission groups](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPermissionGroup.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPermissionGroupGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPermissionGroupListCall.html)
* [account permissions](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPermission.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPermissionGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPermissionListCall.html)
* [account user profiles](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUserProfile.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUserProfileGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUserProfileInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUserProfileListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUserProfilePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUserProfileUpdateCall.html)
* [accounts](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Account.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountGetCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AccountUpdateCall.html)
* [ads](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Ad.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdUpdateCall.html)
* [advertiser groups](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroup.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroupDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGroupUpdateCall.html)
* advertiser landing pages
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserLandingPageGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserLandingPageInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserLandingPageListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserLandingPagePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserLandingPageUpdateCall.html)
* [advertisers](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Advertiser.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.AdvertiserUpdateCall.html)
* [browsers](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Browser.html)
 * [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.BrowserListCall.html)
* [campaign creative associations](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignCreativeAssociation.html)
 * [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignCreativeAssociationInsertCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignCreativeAssociationListCall.html)
* [campaigns](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Campaign.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CampaignUpdateCall.html)
* [change logs](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ChangeLog.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ChangeLogGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ChangeLogListCall.html)
* [cities](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.City.html)
 * [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CityListCall.html)
* [connection types](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ConnectionType.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ConnectionTypeGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ConnectionTypeListCall.html)
* [content categories](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategory.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategoryDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategoryGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategoryInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategoryListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategoryPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ContentCategoryUpdateCall.html)
* [conversions](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Conversion.html)
 * [*batchinsert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ConversionBatchinsertCall.html) and [*batchupdate*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ConversionBatchupdateCall.html)
* [countries](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Country.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CountryGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CountryListCall.html)
* [creative assets](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeAsset.html)
 * [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeAssetInsertCall.html)
* [creative field values](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValue.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValueDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValueGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValueInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValueListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValuePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldValueUpdateCall.html)
* [creative fields](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeField.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeFieldUpdateCall.html)
* [creative groups](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGroup.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGroupUpdateCall.html)
* [creatives](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Creative.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeUpdateCall.html)
* [dimension values](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DimensionValue.html)
 * [*query*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DimensionValueQueryCall.html)
* [directory site contacts](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySiteContact.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySiteContactGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySiteContactListCall.html)
* [directory sites](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySite.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySiteGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySiteInsertCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DirectorySiteListCall.html)
* [dynamic targeting keys](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DynamicTargetingKey.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DynamicTargetingKeyDeleteCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DynamicTargetingKeyInsertCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.DynamicTargetingKeyListCall.html)
* [event tags](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTag.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTagDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTagGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTagInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTagListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTagPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.EventTagUpdateCall.html)
* [files](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.File.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FileGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FileListCall.html)
* [floodlight activities](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivity.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityDeleteCall.html), [*generatetag*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGeneratetagCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityUpdateCall.html)
* [floodlight activity groups](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGroup.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightActivityGroupUpdateCall.html)
* [floodlight configurations](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightConfiguration.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightConfigurationGetCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightConfigurationListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightConfigurationPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FloodlightConfigurationUpdateCall.html)
* [inventory items](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.InventoryItem.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.InventoryItemGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.InventoryItemListCall.html)
* [languages](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Language.html)
 * [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.LanguageListCall.html)
* [metros](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Metro.html)
 * [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MetroListCall.html)
* [mobile apps](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MobileApp.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MobileAppGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MobileAppListCall.html)
* [mobile carriers](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MobileCarrier.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MobileCarrierGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.MobileCarrierListCall.html)
* [operating system versions](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OperatingSystemVersion.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OperatingSystemVersionGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OperatingSystemVersionListCall.html)
* [operating systems](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OperatingSystem.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OperatingSystemGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OperatingSystemListCall.html)
* [order documents](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OrderDocument.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OrderDocumentGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OrderDocumentListCall.html)
* [orders](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Order.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OrderGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.OrderListCall.html)
* [placement groups](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGroup.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGroupGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGroupInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGroupListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGroupPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGroupUpdateCall.html)
* [placement strategies](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategy.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategyDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategyGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategyInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategyListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategyPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementStrategyUpdateCall.html)
* [placements](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Placement.html)
 * [*generatetags*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGeneratetagCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlacementUpdateCall.html)
* [platform types](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlatformType.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlatformTypeGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PlatformTypeListCall.html)
* [postal codes](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PostalCode.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PostalCodeGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.PostalCodeListCall.html)
* [projects](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Project.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ProjectGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ProjectListCall.html)
* [regions](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Region.html)
 * [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RegionListCall.html)
* [remarketing list shares](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListShare.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListShareGetCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListSharePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListShareUpdateCall.html)
* [remarketing lists](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingList.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.RemarketingListUpdateCall.html)
* [reports](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Report.html)
 * [*compatible fields query*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportCompatibleFieldQueryCall.html), [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportDeleteCall.html), [*files get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportFileGetCall.html), [*files list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportFileListCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportPatchCall.html), [*run*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportRunCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportUpdateCall.html)
* [sites](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Site.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SiteGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SiteInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SiteListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SitePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SiteUpdateCall.html)
* [sizes](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Size.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SizeGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SizeInsertCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SizeListCall.html)
* [subaccounts](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Subaccount.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SubaccountGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SubaccountInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SubaccountListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SubaccountPatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.SubaccountUpdateCall.html)
* [targetable remarketing lists](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetableRemarketingList.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetableRemarketingListGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetableRemarketingListListCall.html)
* [targeting templates](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetingTemplate.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetingTemplateGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetingTemplateInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetingTemplateListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetingTemplatePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.TargetingTemplateUpdateCall.html)
* [user profiles](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserProfile.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserProfileGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserProfileListCall.html)
* [user role permission groups](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePermissionGroup.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePermissionGroupGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePermissionGroupListCall.html)
* [user role permissions](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePermission.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePermissionGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePermissionListCall.html)
* [user roles](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRole.html)
 * [*delete*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRoleDeleteCall.html), [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRoleGetCall.html), [*insert*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRoleInsertCall.html), [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRoleListCall.html), [*patch*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRolePatchCall.html) and [*update*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.UserRoleUpdateCall.html)
* [video formats](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.VideoFormat.html)
 * [*get*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.VideoFormatGetCall.html) and [*list*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.VideoFormatListCall.html)


Upload supported by ...

* [*insert creative assets*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.CreativeAssetInsertCall.html)

Download supported by ...

* [*get files*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.FileGetCall.html)
* [*files get reports*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.ReportFileGetCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/struct.Dfareporting.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.CallBuilder.html)**
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
google-dfareporting3d2 = "*"
# This project intentionally uses an old version of Hyper. See
# https://github.com/Byron/google-apis-rs/issues/173 for more
# information.
hyper = "^0.10"
hyper-rustls = "^0.6"
serde = "^1.0"
serde_json = "^1.0"
yup-oauth2 = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_dfareporting3d2 as dfareporting3d2;
use dfareporting3d2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use dfareporting3d2::Dfareporting;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Dfareporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dfareporting3d2/1.0.12+20190531/google_dfareporting3d2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **dfareporting3d2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
