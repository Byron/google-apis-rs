<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-analyticsadmin1_alpha` library allows access to all features of the *Google Google Analytics Admin* service.

This documentation was generated from *Google Analytics Admin* crate version *5.0.3+20240303*, where *20240303* is the exact revision of the *analyticsadmin:v1alpha* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.3*.

Everything else about the *Google Analytics Admin* *v1_alpha* API can be found at the
[official documentation site](http://code.google.com/apis/analytics/docs/mgmt/home.html).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/GoogleAnalyticsAdmin) ... 

* account summaries
 * [*list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountSummaryListCall)
* accounts
 * [*access bindings batch create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingBatchCreateCall), [*access bindings batch delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingBatchDeleteCall), [*access bindings batch get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingBatchGetCall), [*access bindings batch update*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingBatchUpdateCall), [*access bindings create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingCreateCall), [*access bindings delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingDeleteCall), [*access bindings get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingGetCall), [*access bindings list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingListCall), [*access bindings patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountAccessBindingPatchCall), [*delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountDeleteCall), [*get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountGetCall), [*get data sharing settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountGetDataSharingSettingCall), [*list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountListCall), [*patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountPatchCall), [*provision account ticket*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountProvisionAccountTicketCall), [*run access report*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountRunAccessReportCall) and [*search change history events*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::AccountSearchChangeHistoryEventCall)
* properties
 * [*access bindings batch create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingBatchCreateCall), [*access bindings batch delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingBatchDeleteCall), [*access bindings batch get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingBatchGetCall), [*access bindings batch update*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingBatchUpdateCall), [*access bindings create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingCreateCall), [*access bindings delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingDeleteCall), [*access bindings get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingGetCall), [*access bindings list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingListCall), [*access bindings patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAccessBindingPatchCall), [*acknowledge user data collection*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAcknowledgeUserDataCollectionCall), [*ad sense links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAdSenseLinkCreateCall), [*ad sense links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAdSenseLinkDeleteCall), [*ad sense links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAdSenseLinkGetCall), [*ad sense links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAdSenseLinkListCall), [*audiences archive*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAudienceArchiveCall), [*audiences create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAudienceCreateCall), [*audiences get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAudienceGetCall), [*audiences list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAudienceListCall), [*audiences patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyAudiencePatchCall), [*big query links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyBigQueryLinkGetCall), [*big query links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyBigQueryLinkListCall), [*calculated metrics create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCalculatedMetricCreateCall), [*calculated metrics delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCalculatedMetricDeleteCall), [*calculated metrics get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCalculatedMetricGetCall), [*calculated metrics list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCalculatedMetricListCall), [*calculated metrics patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCalculatedMetricPatchCall), [*channel groups create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyChannelGroupCreateCall), [*channel groups delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyChannelGroupDeleteCall), [*channel groups get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyChannelGroupGetCall), [*channel groups list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyChannelGroupListCall), [*channel groups patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyChannelGroupPatchCall), [*conversion events create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyConversionEventCreateCall), [*conversion events delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyConversionEventDeleteCall), [*conversion events get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyConversionEventGetCall), [*conversion events list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyConversionEventListCall), [*conversion events patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyConversionEventPatchCall), [*create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCreateCall), [*create connected site tag*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCreateConnectedSiteTagCall), [*create rollup property*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCreateRollupPropertyCall), [*create subproperty*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCreateSubpropertyCall), [*custom dimensions archive*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomDimensionArchiveCall), [*custom dimensions create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomDimensionCreateCall), [*custom dimensions get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomDimensionGetCall), [*custom dimensions list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomDimensionListCall), [*custom dimensions patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomDimensionPatchCall), [*custom metrics archive*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomMetricArchiveCall), [*custom metrics create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomMetricCreateCall), [*custom metrics get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomMetricGetCall), [*custom metrics list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomMetricListCall), [*custom metrics patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyCustomMetricPatchCall), [*data streams create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamCreateCall), [*data streams delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamDeleteCall), [*data streams event create rules create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamEventCreateRuleCreateCall), [*data streams event create rules delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamEventCreateRuleDeleteCall), [*data streams event create rules get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamEventCreateRuleGetCall), [*data streams event create rules list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamEventCreateRuleListCall), [*data streams event create rules patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamEventCreateRulePatchCall), [*data streams get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamGetCall), [*data streams get data redaction settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamGetDataRedactionSettingCall), [*data streams get enhanced measurement settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamGetEnhancedMeasurementSettingCall), [*data streams get global site tag*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamGetGlobalSiteTagCall), [*data streams list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamListCall), [*data streams measurement protocol secrets create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretCreateCall), [*data streams measurement protocol secrets delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretDeleteCall), [*data streams measurement protocol secrets get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretGetCall), [*data streams measurement protocol secrets list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretListCall), [*data streams measurement protocol secrets patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretPatchCall), [*data streams patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamPatchCall), [*data streams s k ad network conversion value schema create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamSKAdNetworkConversionValueSchemaCreateCall), [*data streams s k ad network conversion value schema delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamSKAdNetworkConversionValueSchemaDeleteCall), [*data streams s k ad network conversion value schema get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamSKAdNetworkConversionValueSchemaGetCall), [*data streams s k ad network conversion value schema list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamSKAdNetworkConversionValueSchemaListCall), [*data streams s k ad network conversion value schema patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamSKAdNetworkConversionValueSchemaPatchCall), [*data streams update data redaction settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamUpdateDataRedactionSettingCall), [*data streams update enhanced measurement settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDataStreamUpdateEnhancedMeasurementSettingCall), [*delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDeleteCall), [*delete connected site tag*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDeleteConnectedSiteTagCall), [*display video360 advertiser link proposals approve*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalApproveCall), [*display video360 advertiser link proposals cancel*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalCancelCall), [*display video360 advertiser link proposals create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalCreateCall), [*display video360 advertiser link proposals delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalDeleteCall), [*display video360 advertiser link proposals get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalGetCall), [*display video360 advertiser link proposals list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalListCall), [*display video360 advertiser links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkCreateCall), [*display video360 advertiser links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkDeleteCall), [*display video360 advertiser links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkGetCall), [*display video360 advertiser links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkListCall), [*display video360 advertiser links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkPatchCall), [*expanded data sets create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyExpandedDataSetCreateCall), [*expanded data sets delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyExpandedDataSetDeleteCall), [*expanded data sets get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyExpandedDataSetGetCall), [*expanded data sets list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyExpandedDataSetListCall), [*expanded data sets patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyExpandedDataSetPatchCall), [*fetch automated ga4 configuration opt out*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyFetchAutomatedGa4ConfigurationOptOutCall), [*fetch connected ga4 property*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyFetchConnectedGa4PropertyCall), [*firebase links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyFirebaseLinkCreateCall), [*firebase links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyFirebaseLinkDeleteCall), [*firebase links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyFirebaseLinkListCall), [*get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGetCall), [*get attribution settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGetAttributionSettingCall), [*get data retention settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGetDataRetentionSettingCall), [*get google signals settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGetGoogleSignalsSettingCall), [*google ads links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkCreateCall), [*google ads links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkDeleteCall), [*google ads links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkListCall), [*google ads links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkPatchCall), [*list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyListCall), [*list connected site tags*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyListConnectedSiteTagCall), [*patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyPatchCall), [*rollup property source links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyRollupPropertySourceLinkCreateCall), [*rollup property source links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyRollupPropertySourceLinkDeleteCall), [*rollup property source links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyRollupPropertySourceLinkGetCall), [*rollup property source links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyRollupPropertySourceLinkListCall), [*run access report*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyRunAccessReportCall), [*search ads360 links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySearchAds360LinkCreateCall), [*search ads360 links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySearchAds360LinkDeleteCall), [*search ads360 links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySearchAds360LinkGetCall), [*search ads360 links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySearchAds360LinkListCall), [*search ads360 links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySearchAds360LinkPatchCall), [*set automated ga4 configuration opt out*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySetAutomatedGa4ConfigurationOptOutCall), [*subproperty event filters create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySubpropertyEventFilterCreateCall), [*subproperty event filters delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySubpropertyEventFilterDeleteCall), [*subproperty event filters get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySubpropertyEventFilterGetCall), [*subproperty event filters list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySubpropertyEventFilterListCall), [*subproperty event filters patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertySubpropertyEventFilterPatchCall), [*update attribution settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyUpdateAttributionSettingCall), [*update data retention settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyUpdateDataRetentionSettingCall) and [*update google signals settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/api::PropertyUpdateGoogleSignalsSettingCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/GoogleAnalyticsAdmin)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::CallBuilder)
* **[Resources](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().access_bindings_batch_delete(...).doit().await
let r = hub.accounts().access_bindings_delete(...).doit().await
let r = hub.accounts().delete(...).doit().await
let r = hub.properties().access_bindings_batch_delete(...).doit().await
let r = hub.properties().access_bindings_delete(...).doit().await
let r = hub.properties().ad_sense_links_delete(...).doit().await
let r = hub.properties().audiences_archive(...).doit().await
let r = hub.properties().calculated_metrics_delete(...).doit().await
let r = hub.properties().channel_groups_delete(...).doit().await
let r = hub.properties().conversion_events_delete(...).doit().await
let r = hub.properties().custom_dimensions_archive(...).doit().await
let r = hub.properties().custom_metrics_archive(...).doit().await
let r = hub.properties().data_streams_event_create_rules_delete(...).doit().await
let r = hub.properties().data_streams_measurement_protocol_secrets_delete(...).doit().await
let r = hub.properties().data_streams_s_k_ad_network_conversion_value_schema_delete(...).doit().await
let r = hub.properties().data_streams_delete(...).doit().await
let r = hub.properties().display_video360_advertiser_link_proposals_delete(...).doit().await
let r = hub.properties().display_video360_advertiser_links_delete(...).doit().await
let r = hub.properties().expanded_data_sets_delete(...).doit().await
let r = hub.properties().firebase_links_delete(...).doit().await
let r = hub.properties().google_ads_links_delete(...).doit().await
let r = hub.properties().rollup_property_source_links_delete(...).doit().await
let r = hub.properties().search_ads360_links_delete(...).doit().await
let r = hub.properties().subproperty_event_filters_delete(...).doit().await
let r = hub.properties().delete_connected_site_tag(...).doit().await
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
google-analyticsadmin1_alpha = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_analyticsadmin1_alpha as analyticsadmin1_alpha;
use analyticsadmin1_alpha::api::GoogleAnalyticsAdminV1alphaBatchDeleteAccessBindingsRequest;
use analyticsadmin1_alpha::{Result, Error};
use std::default::Default;
use analyticsadmin1_alpha::{GoogleAnalyticsAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = GoogleAnalyticsAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleAnalyticsAdminV1alphaBatchDeleteAccessBindingsRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().access_bindings_batch_delete(req, "parent")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Delegate) to the 
[Method Builder](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::RequestValue) and 
[decodable](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-analyticsadmin1_alpha/5.0.3+20240303/google_analyticsadmin1_alpha/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **analyticsadmin1_alpha** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

