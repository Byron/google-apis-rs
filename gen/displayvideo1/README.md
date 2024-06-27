<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-displayvideo1` library allows access to all features of the *Google Display Video* service.

This documentation was generated from *Display Video* crate version *5.0.5+20240620*, where *20240620* is the exact revision of the *displayvideo:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Display Video* *v1* API can be found at the
[official documentation site](https://developers.google.com/display-video/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/DisplayVideo) ... 

* [advertisers](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::Advertiser)
 * [*assets upload*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserAssetUploadCall), [*audit*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserAuditCall), [*bulk edit advertiser assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall), [*bulk list advertiser assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserBulkListAdvertiserAssignedTargetingOptionCall), [*campaigns bulk list campaign assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignBulkListCampaignAssignedTargetingOptionCall), [*campaigns create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignCreateCall), [*campaigns delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignDeleteCall), [*campaigns get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignGetCall), [*campaigns list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignListCall), [*campaigns patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignPatchCall), [*campaigns targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignTargetingTypeAssignedTargetingOptionGetCall), [*campaigns targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCampaignTargetingTypeAssignedTargetingOptionListCall), [*channels create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelCreateCall), [*channels get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelGetCall), [*channels list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelListCall), [*channels patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelPatchCall), [*channels sites bulk edit*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelSiteBulkEditCall), [*channels sites create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelSiteCreateCall), [*channels sites delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelSiteDeleteCall), [*channels sites list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelSiteListCall), [*channels sites replace*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserChannelSiteReplaceCall), [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCreateCall), [*creatives create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCreativeCreateCall), [*creatives delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCreativeDeleteCall), [*creatives get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCreativeGetCall), [*creatives list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCreativeListCall), [*creatives patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserCreativePatchCall), [*delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserDeleteCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserGetCall), [*insertion orders bulk list insertion order assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderBulkListInsertionOrderAssignedTargetingOptionCall), [*insertion orders create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderCreateCall), [*insertion orders delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderDeleteCall), [*insertion orders get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderGetCall), [*insertion orders list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderListCall), [*insertion orders patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderPatchCall), [*insertion orders targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionGetCall), [*insertion orders targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionListCall), [*invoices list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInvoiceListCall), [*invoices lookup invoice currency*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserInvoiceLookupInvoiceCurrencyCall), [*line items bulk edit line item assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall), [*line items bulk list line item assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall), [*line items create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemCreateCall), [*line items delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemDeleteCall), [*line items generate default*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemGenerateDefaultCall), [*line items get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemGetCall), [*line items list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemListCall), [*line items patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemPatchCall), [*line items targeting types assigned targeting options create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall), [*line items targeting types assigned targeting options delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall), [*line items targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall), [*line items targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserListCall), [*location lists assigned locations bulk edit*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListAssignedLocationBulkEditCall), [*location lists assigned locations create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListAssignedLocationCreateCall), [*location lists assigned locations delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListAssignedLocationDeleteCall), [*location lists assigned locations list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListAssignedLocationListCall), [*location lists create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListCreateCall), [*location lists get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListGetCall), [*location lists list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListListCall), [*location lists patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserLocationListPatchCall), [*manual triggers activate*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserManualTriggerActivateCall), [*manual triggers create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserManualTriggerCreateCall), [*manual triggers deactivate*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserManualTriggerDeactivateCall), [*manual triggers get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserManualTriggerGetCall), [*manual triggers list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserManualTriggerListCall), [*manual triggers patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserManualTriggerPatchCall), [*negative keyword lists create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListCreateCall), [*negative keyword lists delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListDeleteCall), [*negative keyword lists get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListGetCall), [*negative keyword lists list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListListCall), [*negative keyword lists negative keywords bulk edit*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall), [*negative keyword lists negative keywords create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordCreateCall), [*negative keyword lists negative keywords delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordDeleteCall), [*negative keyword lists negative keywords list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordListCall), [*negative keyword lists negative keywords replace*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordReplaceCall), [*negative keyword lists patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserNegativeKeywordListPatchCall), [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserPatchCall), [*targeting types assigned targeting options create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionCreateCall), [*targeting types assigned targeting options delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall), [*targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionGetCall) and [*targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionListCall)
* [combined audiences](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CombinedAudience)
 * [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CombinedAudienceGetCall) and [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CombinedAudienceListCall)
* [custom bidding algorithms](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithm)
 * [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmCreateCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmListCall), [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmPatchCall), [*scripts create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmScriptCreateCall), [*scripts get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmScriptGetCall), [*scripts list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmScriptListCall) and [*upload script*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomBiddingAlgorithmUploadScriptCall)
* [custom lists](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomList)
 * [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomListGetCall) and [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::CustomListListCall)
* [first and third party audiences](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FirstAndThirdPartyAudience)
 * [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FirstAndThirdPartyAudienceCreateCall), [*edit customer match members*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FirstAndThirdPartyAudienceEditCustomerMatchMemberCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FirstAndThirdPartyAudienceGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FirstAndThirdPartyAudienceListCall) and [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FirstAndThirdPartyAudiencePatchCall)
* [floodlight groups](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FloodlightGroup)
 * [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FloodlightGroupGetCall) and [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::FloodlightGroupPatchCall)
* [google audiences](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GoogleAudience)
 * [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GoogleAudienceGetCall) and [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GoogleAudienceListCall)
* [guaranteed orders](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GuaranteedOrder)
 * [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GuaranteedOrderCreateCall), [*edit guaranteed order read accessors*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GuaranteedOrderEditGuaranteedOrderReadAccessorCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GuaranteedOrderGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GuaranteedOrderListCall) and [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::GuaranteedOrderPatchCall)
* [inventory source groups](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroup)
 * [*assigned inventory sources bulk edit*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceBulkEditCall), [*assigned inventory sources create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceCreateCall), [*assigned inventory sources delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceDeleteCall), [*assigned inventory sources list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceListCall), [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupCreateCall), [*delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupDeleteCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupListCall) and [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGroupPatchCall)
* [inventory sources](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySource)
 * [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceCreateCall), [*edit inventory source read write accessors*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceEditInventorySourceReadWriteAccessorCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourceListCall) and [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::InventorySourcePatchCall)
* media
 * [*download*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::MediaDownloadCall) and [*upload*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::MediaUploadCall)
* [partners](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::Partner)
 * [*bulk edit partner assigned targeting options*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerBulkEditPartnerAssignedTargetingOptionCall), [*channels create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelCreateCall), [*channels get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelGetCall), [*channels list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelListCall), [*channels patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelPatchCall), [*channels sites bulk edit*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelSiteBulkEditCall), [*channels sites create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelSiteCreateCall), [*channels sites delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelSiteDeleteCall), [*channels sites list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelSiteListCall), [*channels sites replace*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerChannelSiteReplaceCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerListCall), [*targeting types assigned targeting options create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerTargetingTypeAssignedTargetingOptionCreateCall), [*targeting types assigned targeting options delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerTargetingTypeAssignedTargetingOptionDeleteCall), [*targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerTargetingTypeAssignedTargetingOptionGetCall) and [*targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::PartnerTargetingTypeAssignedTargetingOptionListCall)
* sdfdownloadtasks
 * [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::SdfdownloadtaskCreateCall) and [*operations get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::SdfdownloadtaskOperationGetCall)
* targeting types
 * [*targeting options get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::TargetingTypeTargetingOptionGetCall), [*targeting options list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::TargetingTypeTargetingOptionListCall) and [*targeting options search*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::TargetingTypeTargetingOptionSearchCall)
* [users](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::User)
 * [*bulk edit assigned user roles*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::UserBulkEditAssignedUserRoleCall), [*create*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::UserCreateCall), [*delete*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::UserDeleteCall), [*get*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::UserGetCall), [*list*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::UserListCall) and [*patch*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::UserPatchCall)


Upload supported by ...

* [*assets upload advertisers*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::AdvertiserAssetUploadCall)
* [*upload media*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::MediaUploadCall)

Download supported by ...

* [*download media*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/api::MediaDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/DisplayVideo)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::CallBuilder)
* **[Resources](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.advertisers().assets_upload(...).doit().await
let r = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_get(...).doit().await
let r = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_list(...).doit().await
let r = hub.advertisers().campaigns_bulk_list_campaign_assigned_targeting_options(...).doit().await
let r = hub.advertisers().campaigns_create(...).doit().await
let r = hub.advertisers().campaigns_delete(...).doit().await
let r = hub.advertisers().campaigns_get(...).doit().await
let r = hub.advertisers().campaigns_list(...).doit().await
let r = hub.advertisers().campaigns_patch(...).doit().await
let r = hub.advertisers().channels_sites_bulk_edit(...).doit().await
let r = hub.advertisers().channels_sites_create(...).doit().await
let r = hub.advertisers().channels_sites_delete(...).doit().await
let r = hub.advertisers().channels_sites_list(...).doit().await
let r = hub.advertisers().channels_sites_replace(...).doit().await
let r = hub.advertisers().channels_create(...).doit().await
let r = hub.advertisers().channels_get(...).doit().await
let r = hub.advertisers().channels_list(...).doit().await
let r = hub.advertisers().channels_patch(...).doit().await
let r = hub.advertisers().creatives_create(...).doit().await
let r = hub.advertisers().creatives_delete(...).doit().await
let r = hub.advertisers().creatives_get(...).doit().await
let r = hub.advertisers().creatives_list(...).doit().await
let r = hub.advertisers().creatives_patch(...).doit().await
let r = hub.advertisers().insertion_orders_targeting_types_assigned_targeting_options_get(...).doit().await
let r = hub.advertisers().insertion_orders_targeting_types_assigned_targeting_options_list(...).doit().await
let r = hub.advertisers().insertion_orders_bulk_list_insertion_order_assigned_targeting_options(...).doit().await
let r = hub.advertisers().insertion_orders_create(...).doit().await
let r = hub.advertisers().insertion_orders_delete(...).doit().await
let r = hub.advertisers().insertion_orders_get(...).doit().await
let r = hub.advertisers().insertion_orders_list(...).doit().await
let r = hub.advertisers().insertion_orders_patch(...).doit().await
let r = hub.advertisers().invoices_list(...).doit().await
let r = hub.advertisers().invoices_lookup_invoice_currency(...).doit().await
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_create(...).doit().await
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_delete(...).doit().await
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_get(...).doit().await
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_list(...).doit().await
let r = hub.advertisers().line_items_bulk_edit_line_item_assigned_targeting_options(...).doit().await
let r = hub.advertisers().line_items_bulk_list_line_item_assigned_targeting_options(...).doit().await
let r = hub.advertisers().line_items_create(...).doit().await
let r = hub.advertisers().line_items_delete(...).doit().await
let r = hub.advertisers().line_items_generate_default(...).doit().await
let r = hub.advertisers().line_items_get(...).doit().await
let r = hub.advertisers().line_items_list(...).doit().await
let r = hub.advertisers().line_items_patch(...).doit().await
let r = hub.advertisers().location_lists_assigned_locations_bulk_edit(...).doit().await
let r = hub.advertisers().location_lists_assigned_locations_create(...).doit().await
let r = hub.advertisers().location_lists_assigned_locations_delete(...).doit().await
let r = hub.advertisers().location_lists_assigned_locations_list(...).doit().await
let r = hub.advertisers().location_lists_create(...).doit().await
let r = hub.advertisers().location_lists_get(...).doit().await
let r = hub.advertisers().location_lists_list(...).doit().await
let r = hub.advertisers().location_lists_patch(...).doit().await
let r = hub.advertisers().manual_triggers_activate(...).doit().await
let r = hub.advertisers().manual_triggers_create(...).doit().await
let r = hub.advertisers().manual_triggers_deactivate(...).doit().await
let r = hub.advertisers().manual_triggers_get(...).doit().await
let r = hub.advertisers().manual_triggers_list(...).doit().await
let r = hub.advertisers().manual_triggers_patch(...).doit().await
let r = hub.advertisers().negative_keyword_lists_negative_keywords_bulk_edit(...).doit().await
let r = hub.advertisers().negative_keyword_lists_negative_keywords_create(...).doit().await
let r = hub.advertisers().negative_keyword_lists_negative_keywords_delete(...).doit().await
let r = hub.advertisers().negative_keyword_lists_negative_keywords_list(...).doit().await
let r = hub.advertisers().negative_keyword_lists_negative_keywords_replace(...).doit().await
let r = hub.advertisers().negative_keyword_lists_create(...).doit().await
let r = hub.advertisers().negative_keyword_lists_delete(...).doit().await
let r = hub.advertisers().negative_keyword_lists_get(...).doit().await
let r = hub.advertisers().negative_keyword_lists_list(...).doit().await
let r = hub.advertisers().negative_keyword_lists_patch(...).doit().await
let r = hub.advertisers().targeting_types_assigned_targeting_options_create(...).doit().await
let r = hub.advertisers().targeting_types_assigned_targeting_options_delete(...).doit().await
let r = hub.advertisers().targeting_types_assigned_targeting_options_get(...).doit().await
let r = hub.advertisers().targeting_types_assigned_targeting_options_list(...).doit().await
let r = hub.advertisers().audit(...).doit().await
let r = hub.advertisers().bulk_edit_advertiser_assigned_targeting_options(...).doit().await
let r = hub.advertisers().bulk_list_advertiser_assigned_targeting_options(...).doit().await
let r = hub.advertisers().create(...).doit().await
let r = hub.advertisers().delete(...).doit().await
let r = hub.advertisers().get(...).doit().await
let r = hub.advertisers().list(...).doit().await
let r = hub.advertisers().patch(...).doit().await
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
google-displayvideo1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_displayvideo1 as displayvideo1;
use displayvideo1::{Result, Error};
use std::default::Default;
use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_list(-22, -33, "targetingType")
             .page_token("ipsum")
             .page_size(-28)
             .order_by("At")
             .filter("sanctus")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::RequestValue) and 
[decodable](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-displayvideo1/5.0.5+20240620/google_displayvideo1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **displayvideo1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

