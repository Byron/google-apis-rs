<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dlp2` library allows access to all features of the *Google DLP* service.

This documentation was generated from *DLP* crate version *5.0.5+20240616*, where *20240616* is the exact revision of the *dlp:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *DLP* *v2* API can be found at the
[official documentation site](https://cloud.google.com/sensitive-data-protection/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/DLP) ... 

* info types
 * [*list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::InfoTypeListCall)
* locations
 * [*info types list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::LocationInfoTypeListCall)
* organizations
 * [*deidentify templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationDeidentifyTemplateCreateCall), [*deidentify templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationDeidentifyTemplateDeleteCall), [*deidentify templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationDeidentifyTemplateGetCall), [*deidentify templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationDeidentifyTemplateListCall), [*deidentify templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationDeidentifyTemplatePatchCall), [*inspect templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationInspectTemplateCreateCall), [*inspect templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationInspectTemplateDeleteCall), [*inspect templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationInspectTemplateGetCall), [*inspect templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationInspectTemplateListCall), [*inspect templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationInspectTemplatePatchCall), [*locations column data profiles get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationColumnDataProfileGetCall), [*locations column data profiles list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationColumnDataProfileListCall), [*locations connections create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationConnectionCreateCall), [*locations connections delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationConnectionDeleteCall), [*locations connections get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationConnectionGetCall), [*locations connections patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationConnectionPatchCall), [*locations connections search*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationConnectionSearchCall), [*locations deidentify templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDeidentifyTemplateCreateCall), [*locations deidentify templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDeidentifyTemplateDeleteCall), [*locations deidentify templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDeidentifyTemplateGetCall), [*locations deidentify templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDeidentifyTemplateListCall), [*locations deidentify templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDeidentifyTemplatePatchCall), [*locations discovery configs create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDiscoveryConfigCreateCall), [*locations discovery configs delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDiscoveryConfigDeleteCall), [*locations discovery configs get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDiscoveryConfigGetCall), [*locations discovery configs list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDiscoveryConfigListCall), [*locations discovery configs patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDiscoveryConfigPatchCall), [*locations dlp jobs list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationDlpJobListCall), [*locations inspect templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationInspectTemplateCreateCall), [*locations inspect templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationInspectTemplateDeleteCall), [*locations inspect templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationInspectTemplateGetCall), [*locations inspect templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationInspectTemplateListCall), [*locations inspect templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationInspectTemplatePatchCall), [*locations job triggers create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationJobTriggerCreateCall), [*locations job triggers delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationJobTriggerDeleteCall), [*locations job triggers get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationJobTriggerGetCall), [*locations job triggers list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationJobTriggerListCall), [*locations job triggers patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationJobTriggerPatchCall), [*locations project data profiles get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationProjectDataProfileGetCall), [*locations project data profiles list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationProjectDataProfileListCall), [*locations stored info types create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationStoredInfoTypeCreateCall), [*locations stored info types delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationStoredInfoTypeDeleteCall), [*locations stored info types get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationStoredInfoTypeGetCall), [*locations stored info types list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationStoredInfoTypeListCall), [*locations stored info types patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationStoredInfoTypePatchCall), [*locations table data profiles delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationTableDataProfileDeleteCall), [*locations table data profiles get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationTableDataProfileGetCall), [*locations table data profiles list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationLocationTableDataProfileListCall), [*stored info types create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationStoredInfoTypeCreateCall), [*stored info types delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationStoredInfoTypeDeleteCall), [*stored info types get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationStoredInfoTypeGetCall), [*stored info types list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationStoredInfoTypeListCall) and [*stored info types patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::OrganizationStoredInfoTypePatchCall)
* projects
 * [*content deidentify*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectContentDeidentifyCall), [*content inspect*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectContentInspectCall), [*content reidentify*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectContentReidentifyCall), [*deidentify templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDeidentifyTemplateCreateCall), [*deidentify templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDeidentifyTemplateDeleteCall), [*deidentify templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDeidentifyTemplateGetCall), [*deidentify templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDeidentifyTemplateListCall), [*deidentify templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDeidentifyTemplatePatchCall), [*dlp jobs cancel*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDlpJobCancelCall), [*dlp jobs create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDlpJobCreateCall), [*dlp jobs delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDlpJobDeleteCall), [*dlp jobs get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDlpJobGetCall), [*dlp jobs list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectDlpJobListCall), [*image redact*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectImageRedactCall), [*inspect templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectInspectTemplateCreateCall), [*inspect templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectInspectTemplateDeleteCall), [*inspect templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectInspectTemplateGetCall), [*inspect templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectInspectTemplateListCall), [*inspect templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectInspectTemplatePatchCall), [*job triggers activate*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectJobTriggerActivateCall), [*job triggers create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectJobTriggerCreateCall), [*job triggers delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectJobTriggerDeleteCall), [*job triggers get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectJobTriggerGetCall), [*job triggers list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectJobTriggerListCall), [*job triggers patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectJobTriggerPatchCall), [*locations column data profiles get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationColumnDataProfileGetCall), [*locations column data profiles list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationColumnDataProfileListCall), [*locations connections create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationConnectionCreateCall), [*locations connections delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationConnectionDeleteCall), [*locations connections get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationConnectionGetCall), [*locations connections list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationConnectionListCall), [*locations connections patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationConnectionPatchCall), [*locations connections search*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationConnectionSearchCall), [*locations content deidentify*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationContentDeidentifyCall), [*locations content inspect*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationContentInspectCall), [*locations content reidentify*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationContentReidentifyCall), [*locations deidentify templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDeidentifyTemplateCreateCall), [*locations deidentify templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDeidentifyTemplateDeleteCall), [*locations deidentify templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDeidentifyTemplateGetCall), [*locations deidentify templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDeidentifyTemplateListCall), [*locations deidentify templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDeidentifyTemplatePatchCall), [*locations discovery configs create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDiscoveryConfigCreateCall), [*locations discovery configs delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDiscoveryConfigDeleteCall), [*locations discovery configs get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDiscoveryConfigGetCall), [*locations discovery configs list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDiscoveryConfigListCall), [*locations discovery configs patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDiscoveryConfigPatchCall), [*locations dlp jobs cancel*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobCancelCall), [*locations dlp jobs create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobCreateCall), [*locations dlp jobs delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobDeleteCall), [*locations dlp jobs finish*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobFinishCall), [*locations dlp jobs get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobGetCall), [*locations dlp jobs hybrid inspect*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobHybridInspectCall), [*locations dlp jobs list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationDlpJobListCall), [*locations image redact*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationImageRedactCall), [*locations inspect templates create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationInspectTemplateCreateCall), [*locations inspect templates delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationInspectTemplateDeleteCall), [*locations inspect templates get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationInspectTemplateGetCall), [*locations inspect templates list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationInspectTemplateListCall), [*locations inspect templates patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationInspectTemplatePatchCall), [*locations job triggers activate*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerActivateCall), [*locations job triggers create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerCreateCall), [*locations job triggers delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerDeleteCall), [*locations job triggers get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerGetCall), [*locations job triggers hybrid inspect*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerHybridInspectCall), [*locations job triggers list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerListCall), [*locations job triggers patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationJobTriggerPatchCall), [*locations project data profiles get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationProjectDataProfileGetCall), [*locations project data profiles list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationProjectDataProfileListCall), [*locations stored info types create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationStoredInfoTypeCreateCall), [*locations stored info types delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationStoredInfoTypeDeleteCall), [*locations stored info types get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationStoredInfoTypeGetCall), [*locations stored info types list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationStoredInfoTypeListCall), [*locations stored info types patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationStoredInfoTypePatchCall), [*locations table data profiles delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationTableDataProfileDeleteCall), [*locations table data profiles get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationTableDataProfileGetCall), [*locations table data profiles list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectLocationTableDataProfileListCall), [*stored info types create*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectStoredInfoTypeCreateCall), [*stored info types delete*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectStoredInfoTypeDeleteCall), [*stored info types get*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectStoredInfoTypeGetCall), [*stored info types list*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectStoredInfoTypeListCall) and [*stored info types patch*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/api::ProjectStoredInfoTypePatchCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/DLP)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::CallBuilder)
* **[Resources](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.organizations().deidentify_templates_delete(...).doit().await
let r = hub.organizations().inspect_templates_delete(...).doit().await
let r = hub.organizations().locations_connections_delete(...).doit().await
let r = hub.organizations().locations_deidentify_templates_delete(...).doit().await
let r = hub.organizations().locations_discovery_configs_delete(...).doit().await
let r = hub.organizations().locations_inspect_templates_delete(...).doit().await
let r = hub.organizations().locations_job_triggers_delete(...).doit().await
let r = hub.organizations().locations_stored_info_types_delete(...).doit().await
let r = hub.organizations().locations_table_data_profiles_delete(...).doit().await
let r = hub.organizations().stored_info_types_delete(...).doit().await
let r = hub.projects().deidentify_templates_delete(...).doit().await
let r = hub.projects().dlp_jobs_cancel(...).doit().await
let r = hub.projects().dlp_jobs_delete(...).doit().await
let r = hub.projects().inspect_templates_delete(...).doit().await
let r = hub.projects().job_triggers_delete(...).doit().await
let r = hub.projects().locations_connections_delete(...).doit().await
let r = hub.projects().locations_deidentify_templates_delete(...).doit().await
let r = hub.projects().locations_discovery_configs_delete(...).doit().await
let r = hub.projects().locations_dlp_jobs_cancel(...).doit().await
let r = hub.projects().locations_dlp_jobs_delete(...).doit().await
let r = hub.projects().locations_dlp_jobs_finish(...).doit().await
let r = hub.projects().locations_inspect_templates_delete(...).doit().await
let r = hub.projects().locations_job_triggers_delete(...).doit().await
let r = hub.projects().locations_stored_info_types_delete(...).doit().await
let r = hub.projects().locations_table_data_profiles_delete(...).doit().await
let r = hub.projects().stored_info_types_delete(...).doit().await
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
google-dlp2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_dlp2 as dlp2;
use dlp2::api::GooglePrivacyDlpV2CancelDlpJobRequest;
use dlp2::{Result, Error};
use std::default::Default;
use dlp2::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GooglePrivacyDlpV2CancelDlpJobRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().dlp_jobs_cancel(req, "name")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::RequestValue) and 
[decodable](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dlp2/5.0.5+20240616/google_dlp2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **dlp2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

