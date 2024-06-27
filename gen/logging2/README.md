<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-logging2` library allows access to all features of the *Google Logging* service.

This documentation was generated from *Logging* crate version *5.0.5+20240531*, where *20240531* is the exact revision of the *logging:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Logging* *v2* API can be found at the
[official documentation site](https://cloud.google.com/logging/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/Logging) ... 

* billing accounts
 * [*exclusions create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountExclusionCreateCall), [*exclusions delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountExclusionDeleteCall), [*exclusions get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountExclusionGetCall), [*exclusions list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountExclusionListCall), [*exclusions patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountExclusionPatchCall), [*get cmek settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountGetCmekSettingCall), [*get settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountGetSettingCall), [*locations buckets create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketCreateCall), [*locations buckets create async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketCreateAsyncCall), [*locations buckets delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketDeleteCall), [*locations buckets get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketGetCall), [*locations buckets links create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketLinkCreateCall), [*locations buckets links delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketLinkDeleteCall), [*locations buckets links get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketLinkGetCall), [*locations buckets links list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketLinkListCall), [*locations buckets list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketListCall), [*locations buckets patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketPatchCall), [*locations buckets undelete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketUndeleteCall), [*locations buckets update async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketUpdateAsyncCall), [*locations buckets views create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketViewCreateCall), [*locations buckets views delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketViewDeleteCall), [*locations buckets views get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketViewGetCall), [*locations buckets views list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketViewListCall), [*locations buckets views logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketViewLogListCall), [*locations buckets views patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationBucketViewPatchCall), [*locations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationGetCall), [*locations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationListCall), [*locations operations cancel*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationOperationListCall), [*locations recent queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationRecentQueryListCall), [*locations saved queries create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationSavedQueryCreateCall), [*locations saved queries delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationSavedQueryDeleteCall), [*locations saved queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLocationSavedQueryListCall), [*logs delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLogDeleteCall), [*logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountLogListCall), [*sinks create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountSinkCreateCall), [*sinks delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountSinkDeleteCall), [*sinks get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountSinkGetCall), [*sinks list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountSinkListCall), [*sinks patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountSinkPatchCall) and [*sinks update*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::BillingAccountSinkUpdateCall)
* entries
 * [*copy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::EntryCopyCall), [*list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::EntryListCall), [*tail*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::EntryTailCall) and [*write*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::EntryWriteCall)
* exclusions
 * [*create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ExclusionCreateCall), [*delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ExclusionDeleteCall), [*get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ExclusionGetCall), [*list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ExclusionListCall) and [*patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ExclusionPatchCall)
* folders
 * [*exclusions create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderExclusionCreateCall), [*exclusions delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderExclusionDeleteCall), [*exclusions get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderExclusionGetCall), [*exclusions list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderExclusionListCall), [*exclusions patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderExclusionPatchCall), [*get cmek settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderGetCmekSettingCall), [*get settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderGetSettingCall), [*locations buckets create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketCreateCall), [*locations buckets create async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketCreateAsyncCall), [*locations buckets delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketDeleteCall), [*locations buckets get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketGetCall), [*locations buckets links create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketLinkCreateCall), [*locations buckets links delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketLinkDeleteCall), [*locations buckets links get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketLinkGetCall), [*locations buckets links list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketLinkListCall), [*locations buckets list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketListCall), [*locations buckets patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketPatchCall), [*locations buckets undelete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketUndeleteCall), [*locations buckets update async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketUpdateAsyncCall), [*locations buckets views create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewCreateCall), [*locations buckets views delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewDeleteCall), [*locations buckets views get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewGetCall), [*locations buckets views get iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewGetIamPolicyCall), [*locations buckets views list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewListCall), [*locations buckets views logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewLogListCall), [*locations buckets views patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewPatchCall), [*locations buckets views set iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewSetIamPolicyCall), [*locations buckets views test iam permissions*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationBucketViewTestIamPermissionCall), [*locations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationGetCall), [*locations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationListCall), [*locations operations cancel*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationOperationListCall), [*locations recent queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationRecentQueryListCall), [*locations saved queries create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationSavedQueryCreateCall), [*locations saved queries delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationSavedQueryDeleteCall), [*locations saved queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLocationSavedQueryListCall), [*logs delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLogDeleteCall), [*logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderLogListCall), [*sinks create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderSinkCreateCall), [*sinks delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderSinkDeleteCall), [*sinks get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderSinkGetCall), [*sinks list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderSinkListCall), [*sinks patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderSinkPatchCall), [*sinks update*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderSinkUpdateCall) and [*update settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::FolderUpdateSettingCall)
* [locations](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::Location)
 * [*buckets create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketCreateCall), [*buckets create async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketCreateAsyncCall), [*buckets delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketDeleteCall), [*buckets get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketGetCall), [*buckets links create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketLinkCreateCall), [*buckets links delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketLinkDeleteCall), [*buckets links get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketLinkGetCall), [*buckets links list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketLinkListCall), [*buckets list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketListCall), [*buckets patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketPatchCall), [*buckets undelete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketUndeleteCall), [*buckets update async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketUpdateAsyncCall), [*buckets views create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewCreateCall), [*buckets views delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewDeleteCall), [*buckets views get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewGetCall), [*buckets views get iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewGetIamPolicyCall), [*buckets views list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewListCall), [*buckets views patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewPatchCall), [*buckets views set iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewSetIamPolicyCall), [*buckets views test iam permissions*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationBucketViewTestIamPermissionCall), [*get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationGetCall), [*list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationListCall), [*operations cancel*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationOperationCancelCall), [*operations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationOperationGetCall) and [*operations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LocationOperationListCall)
* logs
 * [*delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LogDeleteCall) and [*list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::LogListCall)
* [monitored resource descriptors](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::MonitoredResourceDescriptor)
 * [*list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::MonitoredResourceDescriptorListCall)
* organizations
 * [*exclusions create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationExclusionCreateCall), [*exclusions delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationExclusionDeleteCall), [*exclusions get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationExclusionGetCall), [*exclusions list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationExclusionListCall), [*exclusions patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationExclusionPatchCall), [*get cmek settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationGetCmekSettingCall), [*get settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationGetSettingCall), [*locations buckets create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketCreateCall), [*locations buckets create async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketCreateAsyncCall), [*locations buckets delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketDeleteCall), [*locations buckets get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketGetCall), [*locations buckets links create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketLinkCreateCall), [*locations buckets links delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketLinkDeleteCall), [*locations buckets links get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketLinkGetCall), [*locations buckets links list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketLinkListCall), [*locations buckets list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketListCall), [*locations buckets patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketPatchCall), [*locations buckets undelete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketUndeleteCall), [*locations buckets update async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketUpdateAsyncCall), [*locations buckets views create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewCreateCall), [*locations buckets views delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewDeleteCall), [*locations buckets views get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewGetCall), [*locations buckets views get iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewGetIamPolicyCall), [*locations buckets views list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewListCall), [*locations buckets views logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewLogListCall), [*locations buckets views patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewPatchCall), [*locations buckets views set iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewSetIamPolicyCall), [*locations buckets views test iam permissions*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationBucketViewTestIamPermissionCall), [*locations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationGetCall), [*locations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationListCall), [*locations operations cancel*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationOperationListCall), [*locations recent queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationRecentQueryListCall), [*locations saved queries create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationSavedQueryCreateCall), [*locations saved queries delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationSavedQueryDeleteCall), [*locations saved queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLocationSavedQueryListCall), [*logs delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLogDeleteCall), [*logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationLogListCall), [*sinks create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationSinkCreateCall), [*sinks delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationSinkDeleteCall), [*sinks get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationSinkGetCall), [*sinks list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationSinkListCall), [*sinks patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationSinkPatchCall), [*sinks update*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationSinkUpdateCall), [*update cmek settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationUpdateCmekSettingCall) and [*update settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::OrganizationUpdateSettingCall)
* projects
 * [*exclusions create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectExclusionCreateCall), [*exclusions delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectExclusionDeleteCall), [*exclusions get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectExclusionGetCall), [*exclusions list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectExclusionListCall), [*exclusions patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectExclusionPatchCall), [*get cmek settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectGetCmekSettingCall), [*get settings*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectGetSettingCall), [*locations buckets create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketCreateCall), [*locations buckets create async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketCreateAsyncCall), [*locations buckets delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketDeleteCall), [*locations buckets get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketGetCall), [*locations buckets links create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketLinkCreateCall), [*locations buckets links delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketLinkDeleteCall), [*locations buckets links get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketLinkGetCall), [*locations buckets links list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketLinkListCall), [*locations buckets list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketListCall), [*locations buckets patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketPatchCall), [*locations buckets undelete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketUndeleteCall), [*locations buckets update async*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketUpdateAsyncCall), [*locations buckets views create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewCreateCall), [*locations buckets views delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewDeleteCall), [*locations buckets views get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewGetCall), [*locations buckets views get iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewGetIamPolicyCall), [*locations buckets views list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewListCall), [*locations buckets views logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewLogListCall), [*locations buckets views patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewPatchCall), [*locations buckets views set iam policy*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewSetIamPolicyCall), [*locations buckets views test iam permissions*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationBucketViewTestIamPermissionCall), [*locations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationGetCall), [*locations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationOperationListCall), [*locations recent queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationRecentQueryListCall), [*locations saved queries create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationSavedQueryCreateCall), [*locations saved queries delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationSavedQueryDeleteCall), [*locations saved queries list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLocationSavedQueryListCall), [*logs delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLogDeleteCall), [*logs list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectLogListCall), [*metrics create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectMetricCreateCall), [*metrics delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectMetricDeleteCall), [*metrics get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectMetricGetCall), [*metrics list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectMetricListCall), [*metrics update*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectMetricUpdateCall), [*sinks create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectSinkCreateCall), [*sinks delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectSinkDeleteCall), [*sinks get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectSinkGetCall), [*sinks list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectSinkListCall), [*sinks patch*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectSinkPatchCall) and [*sinks update*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::ProjectSinkUpdateCall)
* sinks
 * [*create*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::SinkCreateCall), [*delete*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::SinkDeleteCall), [*get*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::SinkGetCall), [*list*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::SinkListCall) and [*update*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::SinkUpdateCall)

Other activities are ...

* [get cmek settings](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::MethodGetCmekSettingCall)
* [get settings](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::MethodGetSettingCall)
* [update cmek settings](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::MethodUpdateCmekSettingCall)
* [update settings](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/api::MethodUpdateSettingCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/Logging)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::CallBuilder)
* **[Resources](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.billing_accounts().exclusions_delete(...).doit().await
let r = hub.billing_accounts().locations_buckets_views_delete(...).doit().await
let r = hub.billing_accounts().locations_buckets_delete(...).doit().await
let r = hub.billing_accounts().locations_buckets_undelete(...).doit().await
let r = hub.billing_accounts().locations_operations_cancel(...).doit().await
let r = hub.billing_accounts().locations_saved_queries_delete(...).doit().await
let r = hub.billing_accounts().logs_delete(...).doit().await
let r = hub.billing_accounts().sinks_delete(...).doit().await
let r = hub.exclusions().delete(...).doit().await
let r = hub.folders().exclusions_delete(...).doit().await
let r = hub.folders().locations_buckets_views_delete(...).doit().await
let r = hub.folders().locations_buckets_delete(...).doit().await
let r = hub.folders().locations_buckets_undelete(...).doit().await
let r = hub.folders().locations_operations_cancel(...).doit().await
let r = hub.folders().locations_saved_queries_delete(...).doit().await
let r = hub.folders().logs_delete(...).doit().await
let r = hub.folders().sinks_delete(...).doit().await
let r = hub.locations().buckets_views_delete(...).doit().await
let r = hub.locations().buckets_delete(...).doit().await
let r = hub.locations().buckets_undelete(...).doit().await
let r = hub.locations().operations_cancel(...).doit().await
let r = hub.logs().delete(...).doit().await
let r = hub.organizations().exclusions_delete(...).doit().await
let r = hub.organizations().locations_buckets_views_delete(...).doit().await
let r = hub.organizations().locations_buckets_delete(...).doit().await
let r = hub.organizations().locations_buckets_undelete(...).doit().await
let r = hub.organizations().locations_operations_cancel(...).doit().await
let r = hub.organizations().locations_saved_queries_delete(...).doit().await
let r = hub.organizations().logs_delete(...).doit().await
let r = hub.organizations().sinks_delete(...).doit().await
let r = hub.projects().exclusions_delete(...).doit().await
let r = hub.projects().locations_buckets_views_delete(...).doit().await
let r = hub.projects().locations_buckets_delete(...).doit().await
let r = hub.projects().locations_buckets_undelete(...).doit().await
let r = hub.projects().locations_operations_cancel(...).doit().await
let r = hub.projects().locations_saved_queries_delete(...).doit().await
let r = hub.projects().logs_delete(...).doit().await
let r = hub.projects().metrics_delete(...).doit().await
let r = hub.projects().sinks_delete(...).doit().await
let r = hub.sinks().delete(...).doit().await
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
google-logging2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_logging2 as logging2;
use logging2::api::UndeleteBucketRequest;
use logging2::{Result, Error};
use std::default::Default;
use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = UndeleteBucketRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.billing_accounts().locations_buckets_undelete(req, "name")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::RequestValue) and 
[decodable](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-logging2/5.0.5+20240531/google_logging2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **logging2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

