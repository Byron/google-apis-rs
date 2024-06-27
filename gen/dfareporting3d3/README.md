<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dfareporting3d3` library allows access to all features of the *Google Dfareporting* service.

This documentation was generated from *Dfareporting* crate version *5.0.5+20220104*, where *20220104* is the exact revision of the *dfareporting:v3.3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Dfareporting* *v3d3* API can be found at the
[official documentation site](https://developers.google.com/doubleclick-advertisers/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/Dfareporting) ... 

* [account active ad summaries](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountActiveAdSummary)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountActiveAdSummaryGetCall)
* [account permission groups](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPermissionGroup)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPermissionGroupGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPermissionGroupListCall)
* [account permissions](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPermission)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPermissionGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPermissionListCall)
* [account user profiles](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUserProfile)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUserProfileGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUserProfileInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUserProfileListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUserProfilePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUserProfileUpdateCall)
* [accounts](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Account)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountGetCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AccountUpdateCall)
* [ads](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Ad)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdUpdateCall)
* [advertiser groups](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroup)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroupDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroupGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroupInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroupListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroupPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGroupUpdateCall)
* advertiser landing pages
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserLandingPageGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserLandingPageInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserLandingPageListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserLandingPagePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserLandingPageUpdateCall)
* [advertisers](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Advertiser)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::AdvertiserUpdateCall)
* [browsers](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Browser)
 * [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::BrowserListCall)
* [campaign creative associations](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignCreativeAssociation)
 * [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignCreativeAssociationInsertCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignCreativeAssociationListCall)
* [campaigns](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Campaign)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CampaignUpdateCall)
* [change logs](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ChangeLog)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ChangeLogGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ChangeLogListCall)
* [cities](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::City)
 * [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CityListCall)
* [connection types](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ConnectionType)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ConnectionTypeGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ConnectionTypeListCall)
* [content categories](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategory)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategoryDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategoryGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategoryInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategoryListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategoryPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ContentCategoryUpdateCall)
* [conversions](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Conversion)
 * [*batchinsert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ConversionBatchinsertCall) and [*batchupdate*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ConversionBatchupdateCall)
* [countries](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Country)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CountryGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CountryListCall)
* [creative assets](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeAsset)
 * [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeAssetInsertCall)
* [creative field values](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValue)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValueDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValueGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValueInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValueListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValuePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldValueUpdateCall)
* [creative fields](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeField)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeFieldUpdateCall)
* [creative groups](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGroup)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGroupGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGroupInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGroupListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGroupPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGroupUpdateCall)
* [creatives](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Creative)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeUpdateCall)
* [dimension values](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DimensionValue)
 * [*query*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DimensionValueQueryCall)
* [directory sites](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DirectorySite)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DirectorySiteGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DirectorySiteInsertCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DirectorySiteListCall)
* [dynamic targeting keys](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DynamicTargetingKey)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DynamicTargetingKeyDeleteCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DynamicTargetingKeyInsertCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::DynamicTargetingKeyListCall)
* [event tags](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTag)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTagDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTagGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTagInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTagListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTagPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::EventTagUpdateCall)
* [files](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::File)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FileGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FileListCall)
* [floodlight activities](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivity)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityDeleteCall), [*generatetag*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGeneratetagCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityUpdateCall)
* [floodlight activity groups](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGroup)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGroupGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGroupInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGroupListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGroupPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightActivityGroupUpdateCall)
* [floodlight configurations](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightConfiguration)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightConfigurationGetCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightConfigurationListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightConfigurationPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FloodlightConfigurationUpdateCall)
* [inventory items](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::InventoryItem)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::InventoryItemGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::InventoryItemListCall)
* [languages](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Language)
 * [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::LanguageListCall)
* [metros](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Metro)
 * [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MetroListCall)
* [mobile apps](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MobileApp)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MobileAppGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MobileAppListCall)
* [mobile carriers](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MobileCarrier)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MobileCarrierGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::MobileCarrierListCall)
* [operating system versions](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OperatingSystemVersion)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OperatingSystemVersionGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OperatingSystemVersionListCall)
* [operating systems](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OperatingSystem)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OperatingSystemGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OperatingSystemListCall)
* [order documents](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OrderDocument)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OrderDocumentGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OrderDocumentListCall)
* [orders](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Order)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OrderGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::OrderListCall)
* [placement groups](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGroup)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGroupGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGroupInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGroupListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGroupPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGroupUpdateCall)
* [placement strategies](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategy)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategyDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategyGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategyInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategyListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategyPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementStrategyUpdateCall)
* [placements](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Placement)
 * [*generatetags*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGeneratetagCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlacementUpdateCall)
* [platform types](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlatformType)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlatformTypeGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PlatformTypeListCall)
* [postal codes](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PostalCode)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PostalCodeGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::PostalCodeListCall)
* [projects](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Project)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ProjectGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ProjectListCall)
* [regions](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Region)
 * [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RegionListCall)
* [remarketing list shares](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListShare)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListShareGetCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListSharePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListShareUpdateCall)
* [remarketing lists](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingList)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::RemarketingListUpdateCall)
* [reports](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Report)
 * [*compatible fields query*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportCompatibleFieldQueryCall), [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportDeleteCall), [*files get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportFileGetCall), [*files list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportFileListCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportPatchCall), [*run*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportRunCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportUpdateCall)
* [sites](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Site)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SiteGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SiteInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SiteListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SitePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SiteUpdateCall)
* [sizes](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Size)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SizeGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SizeInsertCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SizeListCall)
* [subaccounts](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::Subaccount)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SubaccountGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SubaccountInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SubaccountListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SubaccountPatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::SubaccountUpdateCall)
* [targetable remarketing lists](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetableRemarketingList)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetableRemarketingListGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetableRemarketingListListCall)
* [targeting templates](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetingTemplate)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetingTemplateGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetingTemplateInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetingTemplateListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetingTemplatePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::TargetingTemplateUpdateCall)
* [user profiles](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserProfile)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserProfileGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserProfileListCall)
* [user role permission groups](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePermissionGroup)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePermissionGroupGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePermissionGroupListCall)
* [user role permissions](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePermission)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePermissionGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePermissionListCall)
* [user roles](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRole)
 * [*delete*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRoleDeleteCall), [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRoleGetCall), [*insert*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRoleInsertCall), [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRoleListCall), [*patch*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRolePatchCall) and [*update*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::UserRoleUpdateCall)
* [video formats](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::VideoFormat)
 * [*get*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::VideoFormatGetCall) and [*list*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::VideoFormatListCall)


Upload supported by ...

* [*insert creative assets*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::CreativeAssetInsertCall)

Download supported by ...

* [*get files*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::FileGetCall)
* [*files get reports*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/api::ReportFileGetCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/Dfareporting)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::CallBuilder)
* **[Resources](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.reports().compatible_fields_query(...).doit().await
let r = hub.reports().files_get(...).doit().await
let r = hub.reports().files_list(...).doit().await
let r = hub.reports().delete(...).doit().await
let r = hub.reports().get(...).doit().await
let r = hub.reports().insert(...).doit().await
let r = hub.reports().list(...).doit().await
let r = hub.reports().patch(...).doit().await
let r = hub.reports().run(...).doit().await
let r = hub.reports().update(...).doit().await
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
google-dfareporting3d3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_dfareporting3d3 as dfareporting3d3;
use dfareporting3d3::{Result, Error};
use std::default::Default;
use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.reports().files_list(-22, -33)
             .sort_order("no")
             .sort_field("ipsum")
             .page_token("voluptua.")
             .max_results(-27)
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::RequestValue) and 
[decodable](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dfareporting3d3/5.0.5+20220104/google_dfareporting3d3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **dfareporting3d3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

