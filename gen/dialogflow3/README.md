<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dialogflow3` library allows access to all features of the *Google Dialogflow* service.

This documentation was generated from *Dialogflow* crate version *5.0.5+20240614*, where *20240614* is the exact revision of the *dialogflow:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Dialogflow* *v3* API can be found at the
[official documentation site](https://cloud.google.com/dialogflow/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/Dialogflow) ... 

* projects
 * [*locations agents changelogs get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentChangelogGetCall), [*locations agents changelogs list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentChangelogListCall), [*locations agents create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentCreateCall), [*locations agents delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentDeleteCall), [*locations agents entity types create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypeCreateCall), [*locations agents entity types delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypeDeleteCall), [*locations agents entity types export*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypeExportCall), [*locations agents entity types get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypeGetCall), [*locations agents entity types import*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypeImportCall), [*locations agents entity types list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypeListCall), [*locations agents entity types patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEntityTypePatchCall), [*locations agents environments continuous test results list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentContinuousTestResultListCall), [*locations agents environments create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentCreateCall), [*locations agents environments delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentDeleteCall), [*locations agents environments deploy flow*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentDeployFlowCall), [*locations agents environments deployments get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentDeploymentGetCall), [*locations agents environments deployments list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentDeploymentListCall), [*locations agents environments experiments create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentCreateCall), [*locations agents environments experiments delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentDeleteCall), [*locations agents environments experiments get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentGetCall), [*locations agents environments experiments list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentListCall), [*locations agents environments experiments patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentPatchCall), [*locations agents environments experiments start*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentStartCall), [*locations agents environments experiments stop*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentExperimentStopCall), [*locations agents environments get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentGetCall), [*locations agents environments list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentListCall), [*locations agents environments lookup environment history*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentLookupEnvironmentHistoryCall), [*locations agents environments patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentPatchCall), [*locations agents environments run continuous test*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentRunContinuousTestCall), [*locations agents environments sessions detect intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionDetectIntentCall), [*locations agents environments sessions entity types create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionEntityTypeCreateCall), [*locations agents environments sessions entity types delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionEntityTypeDeleteCall), [*locations agents environments sessions entity types get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionEntityTypeGetCall), [*locations agents environments sessions entity types list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionEntityTypeListCall), [*locations agents environments sessions entity types patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionEntityTypePatchCall), [*locations agents environments sessions fulfill intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionFulfillIntentCall), [*locations agents environments sessions match intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionMatchIntentCall), [*locations agents environments sessions server streaming detect intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentEnvironmentSessionServerStreamingDetectIntentCall), [*locations agents export*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentExportCall), [*locations agents flows create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowCreateCall), [*locations agents flows delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowDeleteCall), [*locations agents flows export*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowExportCall), [*locations agents flows get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowGetCall), [*locations agents flows get validation result*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowGetValidationResultCall), [*locations agents flows import*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowImportCall), [*locations agents flows list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowListCall), [*locations agents flows pages create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowPageCreateCall), [*locations agents flows pages delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowPageDeleteCall), [*locations agents flows pages get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowPageGetCall), [*locations agents flows pages list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowPageListCall), [*locations agents flows pages patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowPagePatchCall), [*locations agents flows patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowPatchCall), [*locations agents flows train*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowTrainCall), [*locations agents flows transition route groups create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowTransitionRouteGroupCreateCall), [*locations agents flows transition route groups delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowTransitionRouteGroupDeleteCall), [*locations agents flows transition route groups get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowTransitionRouteGroupGetCall), [*locations agents flows transition route groups list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowTransitionRouteGroupListCall), [*locations agents flows transition route groups patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowTransitionRouteGroupPatchCall), [*locations agents flows validate*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowValidateCall), [*locations agents flows versions compare versions*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionCompareVersionCall), [*locations agents flows versions create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionCreateCall), [*locations agents flows versions delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionDeleteCall), [*locations agents flows versions get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionGetCall), [*locations agents flows versions list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionListCall), [*locations agents flows versions load*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionLoadCall), [*locations agents flows versions patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentFlowVersionPatchCall), [*locations agents generators create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGeneratorCreateCall), [*locations agents generators delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGeneratorDeleteCall), [*locations agents generators get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGeneratorGetCall), [*locations agents generators list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGeneratorListCall), [*locations agents generators patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGeneratorPatchCall), [*locations agents get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGetCall), [*locations agents get generative settings*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGetGenerativeSettingCall), [*locations agents get validation result*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentGetValidationResultCall), [*locations agents intents create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentCreateCall), [*locations agents intents delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentDeleteCall), [*locations agents intents export*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentExportCall), [*locations agents intents get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentGetCall), [*locations agents intents import*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentImportCall), [*locations agents intents list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentListCall), [*locations agents intents patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentIntentPatchCall), [*locations agents list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentListCall), [*locations agents patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentPatchCall), [*locations agents restore*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentRestoreCall), [*locations agents sessions detect intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionDetectIntentCall), [*locations agents sessions entity types create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionEntityTypeCreateCall), [*locations agents sessions entity types delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionEntityTypeDeleteCall), [*locations agents sessions entity types get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionEntityTypeGetCall), [*locations agents sessions entity types list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionEntityTypeListCall), [*locations agents sessions entity types patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionEntityTypePatchCall), [*locations agents sessions fulfill intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionFulfillIntentCall), [*locations agents sessions match intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionMatchIntentCall), [*locations agents sessions server streaming detect intent*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionServerStreamingDetectIntentCall), [*locations agents sessions submit answer feedback*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentSessionSubmitAnswerFeedbackCall), [*locations agents test cases batch delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseBatchDeleteCall), [*locations agents test cases batch run*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseBatchRunCall), [*locations agents test cases calculate coverage*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseCalculateCoverageCall), [*locations agents test cases create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseCreateCall), [*locations agents test cases export*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseExportCall), [*locations agents test cases get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseGetCall), [*locations agents test cases import*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseImportCall), [*locations agents test cases list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseListCall), [*locations agents test cases patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCasePatchCall), [*locations agents test cases results get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseResultGetCall), [*locations agents test cases results list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseResultListCall), [*locations agents test cases run*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTestCaseRunCall), [*locations agents transition route groups create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTransitionRouteGroupCreateCall), [*locations agents transition route groups delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTransitionRouteGroupDeleteCall), [*locations agents transition route groups get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTransitionRouteGroupGetCall), [*locations agents transition route groups list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTransitionRouteGroupListCall), [*locations agents transition route groups patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentTransitionRouteGroupPatchCall), [*locations agents update generative settings*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentUpdateGenerativeSettingCall), [*locations agents validate*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentValidateCall), [*locations agents webhooks create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentWebhookCreateCall), [*locations agents webhooks delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentWebhookDeleteCall), [*locations agents webhooks get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentWebhookGetCall), [*locations agents webhooks list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentWebhookListCall), [*locations agents webhooks patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationAgentWebhookPatchCall), [*locations get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationGetCall), [*locations list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationOperationListCall), [*locations security settings create*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationSecuritySettingCreateCall), [*locations security settings delete*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationSecuritySettingDeleteCall), [*locations security settings get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationSecuritySettingGetCall), [*locations security settings list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationSecuritySettingListCall), [*locations security settings patch*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectLocationSecuritySettingPatchCall), [*operations cancel*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectOperationCancelCall), [*operations get*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectOperationGetCall) and [*operations list*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/api::ProjectOperationListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/Dialogflow)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::CallBuilder)
* **[Resources](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_agents_entity_types_export(...).doit().await
let r = hub.projects().locations_agents_entity_types_import(...).doit().await
let r = hub.projects().locations_agents_environments_create(...).doit().await
let r = hub.projects().locations_agents_environments_deploy_flow(...).doit().await
let r = hub.projects().locations_agents_environments_patch(...).doit().await
let r = hub.projects().locations_agents_environments_run_continuous_test(...).doit().await
let r = hub.projects().locations_agents_flows_versions_create(...).doit().await
let r = hub.projects().locations_agents_flows_versions_load(...).doit().await
let r = hub.projects().locations_agents_flows_export(...).doit().await
let r = hub.projects().locations_agents_flows_import(...).doit().await
let r = hub.projects().locations_agents_flows_train(...).doit().await
let r = hub.projects().locations_agents_intents_export(...).doit().await
let r = hub.projects().locations_agents_intents_import(...).doit().await
let r = hub.projects().locations_agents_test_cases_batch_run(...).doit().await
let r = hub.projects().locations_agents_test_cases_export(...).doit().await
let r = hub.projects().locations_agents_test_cases_import(...).doit().await
let r = hub.projects().locations_agents_test_cases_run(...).doit().await
let r = hub.projects().locations_agents_export(...).doit().await
let r = hub.projects().locations_agents_restore(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().operations_get(...).doit().await
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
google-dialogflow3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_dialogflow3 as dialogflow3;
use dialogflow3::api::GoogleCloudDialogflowCxV3Environment;
use dialogflow3::{Result, Error};
use std::default::Default;
use dialogflow3::{Dialogflow, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Dialogflow::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleCloudDialogflowCxV3Environment::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_agents_environments_patch(req, "name")
             .update_mask(FieldMask::new::<&str>(&[]))
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::RequestValue) and 
[decodable](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dialogflow3/5.0.5+20240614/google_dialogflow3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **dialogflow3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

