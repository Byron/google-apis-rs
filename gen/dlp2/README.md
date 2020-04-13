<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dlp2` library allows access to all features of the *Google DLP* service.

This documentation was generated from *DLP* crate version *1.0.13+20200405*, where *20200405* is the exact revision of the *dlp:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *DLP* *v2* API can be found at the
[official documentation site](https://cloud.google.com/dlp/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.DLP.html) ... 

* info types
 * [*list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.InfoTypeListCall.html)
* locations
 * [*info types list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.LocationInfoTypeListCall.html)
* organizations
 * [*deidentify templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationDeidentifyTemplateCreateCall.html), [*deidentify templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationDeidentifyTemplateDeleteCall.html), [*deidentify templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationDeidentifyTemplateGetCall.html), [*deidentify templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationDeidentifyTemplateListCall.html), [*deidentify templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationDeidentifyTemplatePatchCall.html), [*inspect templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationInspectTemplateCreateCall.html), [*inspect templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationInspectTemplateDeleteCall.html), [*inspect templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationInspectTemplateGetCall.html), [*inspect templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationInspectTemplateListCall.html), [*inspect templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationInspectTemplatePatchCall.html), [*locations deidentify templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationDeidentifyTemplateCreateCall.html), [*locations deidentify templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationDeidentifyTemplateDeleteCall.html), [*locations deidentify templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationDeidentifyTemplateGetCall.html), [*locations deidentify templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationDeidentifyTemplateListCall.html), [*locations deidentify templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationDeidentifyTemplatePatchCall.html), [*locations inspect templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationInspectTemplateCreateCall.html), [*locations inspect templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationInspectTemplateDeleteCall.html), [*locations inspect templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationInspectTemplateGetCall.html), [*locations inspect templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationInspectTemplateListCall.html), [*locations inspect templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationInspectTemplatePatchCall.html), [*locations stored info types create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationStoredInfoTypeCreateCall.html), [*locations stored info types delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationStoredInfoTypeDeleteCall.html), [*locations stored info types get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationStoredInfoTypeGetCall.html), [*locations stored info types list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationStoredInfoTypeListCall.html), [*locations stored info types patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationLocationStoredInfoTypePatchCall.html), [*stored info types create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationStoredInfoTypeCreateCall.html), [*stored info types delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationStoredInfoTypeDeleteCall.html), [*stored info types get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationStoredInfoTypeGetCall.html), [*stored info types list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationStoredInfoTypeListCall.html) and [*stored info types patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.OrganizationStoredInfoTypePatchCall.html)
* projects
 * [*content deidentify*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectContentDeidentifyCall.html), [*content inspect*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectContentInspectCall.html), [*content reidentify*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectContentReidentifyCall.html), [*deidentify templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDeidentifyTemplateCreateCall.html), [*deidentify templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDeidentifyTemplateDeleteCall.html), [*deidentify templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDeidentifyTemplateGetCall.html), [*deidentify templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDeidentifyTemplateListCall.html), [*deidentify templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDeidentifyTemplatePatchCall.html), [*dlp jobs cancel*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDlpJobCancelCall.html), [*dlp jobs create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDlpJobCreateCall.html), [*dlp jobs delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDlpJobDeleteCall.html), [*dlp jobs get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDlpJobGetCall.html), [*dlp jobs list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectDlpJobListCall.html), [*image redact*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectImageRedactCall.html), [*inspect templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectInspectTemplateCreateCall.html), [*inspect templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectInspectTemplateDeleteCall.html), [*inspect templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectInspectTemplateGetCall.html), [*inspect templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectInspectTemplateListCall.html), [*inspect templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectInspectTemplatePatchCall.html), [*job triggers activate*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectJobTriggerActivateCall.html), [*job triggers create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectJobTriggerCreateCall.html), [*job triggers delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectJobTriggerDeleteCall.html), [*job triggers get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectJobTriggerGetCall.html), [*job triggers list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectJobTriggerListCall.html), [*job triggers patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectJobTriggerPatchCall.html), [*locations content deidentify*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationContentDeidentifyCall.html), [*locations content inspect*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationContentInspectCall.html), [*locations content reidentify*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationContentReidentifyCall.html), [*locations deidentify templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDeidentifyTemplateCreateCall.html), [*locations deidentify templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDeidentifyTemplateDeleteCall.html), [*locations deidentify templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDeidentifyTemplateGetCall.html), [*locations deidentify templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDeidentifyTemplateListCall.html), [*locations deidentify templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDeidentifyTemplatePatchCall.html), [*locations dlp jobs cancel*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobCancelCall.html), [*locations dlp jobs create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobCreateCall.html), [*locations dlp jobs delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobDeleteCall.html), [*locations dlp jobs finish*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobFinishCall.html), [*locations dlp jobs get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobGetCall.html), [*locations dlp jobs hybrid inspect*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobHybridInspectCall.html), [*locations dlp jobs list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationDlpJobListCall.html), [*locations image redact*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationImageRedactCall.html), [*locations inspect templates create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationInspectTemplateCreateCall.html), [*locations inspect templates delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationInspectTemplateDeleteCall.html), [*locations inspect templates get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationInspectTemplateGetCall.html), [*locations inspect templates list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationInspectTemplateListCall.html), [*locations inspect templates patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationInspectTemplatePatchCall.html), [*locations job triggers activate*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerActivateCall.html), [*locations job triggers create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerCreateCall.html), [*locations job triggers delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerDeleteCall.html), [*locations job triggers get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerGetCall.html), [*locations job triggers hybrid inspect*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerHybridInspectCall.html), [*locations job triggers list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerListCall.html), [*locations job triggers patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationJobTriggerPatchCall.html), [*locations stored info types create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationStoredInfoTypeCreateCall.html), [*locations stored info types delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationStoredInfoTypeDeleteCall.html), [*locations stored info types get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationStoredInfoTypeGetCall.html), [*locations stored info types list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationStoredInfoTypeListCall.html), [*locations stored info types patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectLocationStoredInfoTypePatchCall.html), [*stored info types create*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectStoredInfoTypeCreateCall.html), [*stored info types delete*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectStoredInfoTypeDeleteCall.html), [*stored info types get*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectStoredInfoTypeGetCall.html), [*stored info types list*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectStoredInfoTypeListCall.html) and [*stored info types patch*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.ProjectStoredInfoTypePatchCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/struct.DLP.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().deidentify_templates_delete(...).doit()
let r = hub.organizations().locations_deidentify_templates_delete(...).doit()
let r = hub.projects().locations_stored_info_types_delete(...).doit()
let r = hub.organizations().locations_inspect_templates_delete(...).doit()
let r = hub.projects().locations_deidentify_templates_delete(...).doit()
let r = hub.projects().locations_dlp_jobs_finish(...).doit()
let r = hub.projects().dlp_jobs_cancel(...).doit()
let r = hub.projects().stored_info_types_delete(...).doit()
let r = hub.organizations().locations_stored_info_types_delete(...).doit()
let r = hub.projects().job_triggers_delete(...).doit()
let r = hub.organizations().stored_info_types_delete(...).doit()
let r = hub.projects().locations_dlp_jobs_delete(...).doit()
let r = hub.projects().locations_dlp_jobs_cancel(...).doit()
let r = hub.organizations().deidentify_templates_delete(...).doit()
let r = hub.projects().dlp_jobs_delete(...).doit()
let r = hub.projects().inspect_templates_delete(...).doit()
let r = hub.projects().locations_job_triggers_delete(...).doit()
let r = hub.organizations().inspect_templates_delete(...).doit()
let r = hub.projects().locations_inspect_templates_delete(...).doit()
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
extern crate google_dlp2 as dlp2;
use dlp2::GooglePrivacyDlpV2FinishDlpJobRequest;
use dlp2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use dlp2::DLP;

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
let mut hub = DLP::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GooglePrivacyDlpV2FinishDlpJobRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_dlp_jobs_finish(req, "name")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dlp2/1.0.13+20200405/google_dlp2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **dlp2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
