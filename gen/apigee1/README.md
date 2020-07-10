<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-apigee1` library allows access to all features of the *Google Apigee* service.

This documentation was generated from *Apigee* crate version *1.0.14+20200625*, where *20200625* is the exact revision of the *apigee:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.

Everything else about the *Apigee* *v1* API can be found at the
[official documentation site](https://cloud.google.com/apigee-api-management/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.Apigee.html) ... 

* hybrid
 * [*issuers list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.HybridIssuerListCall.html)
* organizations
 * [*apiproducts attributes*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductAttributeCall.html), [*apiproducts attributes delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductAttributeDeleteCall.html), [*apiproducts attributes get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductAttributeGetCall.html), [*apiproducts attributes list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductAttributeListCall.html), [*apiproducts attributes update api product attribute*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductAttributeUpdateApiProductAttributeCall.html), [*apiproducts create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductCreateCall.html), [*apiproducts delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductDeleteCall.html), [*apiproducts get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductGetCall.html), [*apiproducts list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductListCall.html), [*apiproducts update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiproductUpdateCall.html), [*apis create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiCreateCall.html), [*apis delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiDeleteCall.html), [*apis deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiDeploymentListCall.html), [*apis get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiGetCall.html), [*apis keyvaluemaps create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiKeyvaluemapCreateCall.html), [*apis keyvaluemaps delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiKeyvaluemapDeleteCall.html), [*apis list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiListCall.html), [*apis revisions delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiRevisionDeleteCall.html), [*apis revisions deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiRevisionDeploymentListCall.html), [*apis revisions get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiRevisionGetCall.html), [*apis revisions update api proxy revision*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationApiRevisionUpdateApiProxyRevisionCall.html), [*apps get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationAppGetCall.html), [*apps list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationAppListCall.html), [*create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationCreateCall.html), [*deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeploymentListCall.html), [*developers apps attributes*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppAttributeCall.html), [*developers apps attributes delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppAttributeDeleteCall.html), [*developers apps attributes get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppAttributeGetCall.html), [*developers apps attributes list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppAttributeListCall.html), [*developers apps attributes update developer app attribute*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppAttributeUpdateDeveloperAppAttributeCall.html), [*developers apps create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppCreateCall.html), [*developers apps delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppDeleteCall.html), [*developers apps generate key pair or update developer app status*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppGenerateKeyPairOrUpdateDeveloperAppStatuCall.html), [*developers apps get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppGetCall.html), [*developers apps keys apiproducts delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyApiproductDeleteCall.html), [*developers apps keys apiproducts update developer app key api product*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyApiproductUpdateDeveloperAppKeyApiProductCall.html), [*developers apps keys create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyCreateCall.html), [*developers apps keys create create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyCreateCreateCall.html), [*developers apps keys delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyDeleteCall.html), [*developers apps keys get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyGetCall.html), [*developers apps keys replace developer app key*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyReplaceDeveloperAppKeyCall.html), [*developers apps keys update developer app key*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppKeyUpdateDeveloperAppKeyCall.html), [*developers apps list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppListCall.html), [*developers apps update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAppUpdateCall.html), [*developers attributes*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAttributeCall.html), [*developers attributes delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAttributeDeleteCall.html), [*developers attributes get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAttributeGetCall.html), [*developers attributes list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAttributeListCall.html), [*developers attributes update developer attribute*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperAttributeUpdateDeveloperAttributeCall.html), [*developers create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperCreateCall.html), [*developers delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperDeleteCall.html), [*developers get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperGetCall.html), [*developers list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperListCall.html), [*developers set developer status*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperSetDeveloperStatuCall.html), [*developers update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationDeveloperUpdateCall.html), [*environments analytics admin get schemav2*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentAnalyticAdminGetSchemav2Call.html), [*environments apis deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiDeploymentListCall.html), [*environments apis revisions debugsessions create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionDebugsessionCreateCall.html), [*environments apis revisions debugsessions data get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionDebugsessionDataGetCall.html), [*environments apis revisions debugsessions delete data*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionDebugsessionDeleteDataCall.html), [*environments apis revisions debugsessions get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionDebugsessionGetCall.html), [*environments apis revisions debugsessions list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionDebugsessionListCall.html), [*environments apis revisions deployments*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionDeploymentCall.html), [*environments apis revisions get deployments*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentApiRevisionGetDeploymentCall.html), [*environments caches delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentCacheDeleteCall.html), [*environments create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentCreateCall.html), [*environments delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentDeleteCall.html), [*environments deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentDeploymentListCall.html), [*environments flowhooks attach shared flow to flow hook*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentFlowhookAttachSharedFlowToFlowHookCall.html), [*environments flowhooks detach shared flow from flow hook*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentFlowhookDetachSharedFlowFromFlowHookCall.html), [*environments flowhooks get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentFlowhookGetCall.html), [*environments get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentGetCall.html), [*environments get debugmask*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentGetDebugmaskCall.html), [*environments get deployed config*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentGetDeployedConfigCall.html), [*environments get iam policy*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentGetIamPolicyCall.html), [*environments keystores aliases create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreAliaseCreateCall.html), [*environments keystores aliases csr*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreAliaseCsrCall.html), [*environments keystores aliases delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreAliaseDeleteCall.html), [*environments keystores aliases get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreAliaseGetCall.html), [*environments keystores aliases get certificate*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreAliaseGetCertificateCall.html), [*environments keystores aliases update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreAliaseUpdateCall.html), [*environments keystores create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreCreateCall.html), [*environments keystores delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreDeleteCall.html), [*environments keystores get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeystoreGetCall.html), [*environments keyvaluemaps create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeyvaluemapCreateCall.html), [*environments keyvaluemaps delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentKeyvaluemapDeleteCall.html), [*environments optimized stats get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentOptimizedStatGetCall.html), [*environments queries create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentQueryCreateCall.html), [*environments queries get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentQueryGetCall.html), [*environments queries get result*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentQueryGetResultCall.html), [*environments queries list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentQueryListCall.html), [*environments references create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentReferenceCreateCall.html), [*environments references delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentReferenceDeleteCall.html), [*environments references get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentReferenceGetCall.html), [*environments references update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentReferenceUpdateCall.html), [*environments resourcefiles create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentResourcefileCreateCall.html), [*environments resourcefiles delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentResourcefileDeleteCall.html), [*environments resourcefiles get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentResourcefileGetCall.html), [*environments resourcefiles list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentResourcefileListCall.html), [*environments resourcefiles list environment resources*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentResourcefileListEnvironmentResourceCall.html), [*environments resourcefiles update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentResourcefileUpdateCall.html), [*environments set iam policy*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentSetIamPolicyCall.html), [*environments sharedflows deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentSharedflowDeploymentListCall.html), [*environments sharedflows revisions deployments*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentSharedflowRevisionDeploymentCall.html), [*environments sharedflows revisions get deployments*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentSharedflowRevisionGetDeploymentCall.html), [*environments stats get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentStatGetCall.html), [*environments subscribe*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentSubscribeCall.html), [*environments targetservers create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentTargetserverCreateCall.html), [*environments targetservers delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentTargetserverDeleteCall.html), [*environments targetservers get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentTargetserverGetCall.html), [*environments targetservers update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentTargetserverUpdateCall.html), [*environments test iam permissions*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentTestIamPermissionCall.html), [*environments unsubscribe*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentUnsubscribeCall.html), [*environments update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentUpdateCall.html), [*environments update debugmask*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentUpdateDebugmaskCall.html), [*environments update environment*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationEnvironmentUpdateEnvironmentCall.html), [*get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationGetCall.html), [*get sync authorization*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationGetSyncAuthorizationCall.html), [*keyvaluemaps create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationKeyvaluemapCreateCall.html), [*keyvaluemaps delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationKeyvaluemapDeleteCall.html), [*list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationListCall.html), [*operations get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationOperationGetCall.html), [*operations list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationOperationListCall.html), [*reports create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationReportCreateCall.html), [*reports delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationReportDeleteCall.html), [*reports get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationReportGetCall.html), [*reports list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationReportListCall.html), [*reports update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationReportUpdateCall.html), [*set sync authorization*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSetSyncAuthorizationCall.html), [*sharedflows create*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowCreateCall.html), [*sharedflows delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowDeleteCall.html), [*sharedflows deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowDeploymentListCall.html), [*sharedflows get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowGetCall.html), [*sharedflows list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowListCall.html), [*sharedflows revisions delete*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowRevisionDeleteCall.html), [*sharedflows revisions deployments list*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowRevisionDeploymentListCall.html), [*sharedflows revisions get*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowRevisionGetCall.html), [*sharedflows revisions update shared flow revision*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationSharedflowRevisionUpdateSharedFlowRevisionCall.html) and [*update*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.OrganizationUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/struct.Apigee.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.organizations().sharedflows_create(...).doit()
let r = hub.organizations().apis_revisions_update_api_proxy_revision(...).doit()
let r = hub.organizations().environments_queries_get_result(...).doit()
let r = hub.organizations().environments_resourcefiles_create(...).doit()
let r = hub.organizations().sharedflows_revisions_update_shared_flow_revision(...).doit()
let r = hub.organizations().apis_create(...).doit()
let r = hub.organizations().sharedflows_revisions_get(...).doit()
let r = hub.organizations().environments_resourcefiles_update(...).doit()
let r = hub.organizations().environments_keystores_aliases_update(...).doit()
let r = hub.organizations().environments_keystores_aliases_csr(...).doit()
let r = hub.organizations().environments_keystores_aliases_get_certificate(...).doit()
let r = hub.organizations().apis_revisions_get(...).doit()
let r = hub.organizations().environments_keystores_aliases_create(...).doit()
let r = hub.organizations().environments_resourcefiles_get(...).doit()
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
google-apigee1 = "*"
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
extern crate google_apigee1 as apigee1;
use apigee1::GoogleApiHttpBody;
use apigee1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use apigee1::Apigee;

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
let mut hub = Apigee::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleApiHttpBody::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.organizations().environments_keystores_aliases_create(req, "parent")
             .ignore_newline_validation(true)
             .ignore_expiry_validation(false)
             .format("sed")
             .alias("et")
             ._password("dolores")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-apigee1/1.0.14+20200625/google_apigee1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **apigee1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
