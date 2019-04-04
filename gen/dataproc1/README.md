<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dataproc1` library allows access to all features of the *Google Dataproc* service.

This documentation was generated from *Dataproc* crate version *1.0.8+20190313*, where *20190313* is the exact revision of the *dataproc:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.

Everything else about the *Dataproc* *v1* API can be found at the
[official documentation site](https://cloud.google.com/dataproc/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.Dataproc.html) ... 

* projects
 * [*locations workflow templates create*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateCreateCall.html), [*locations workflow templates delete*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateDeleteCall.html), [*locations workflow templates get*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateGetCall.html), [*locations workflow templates get iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateGetIamPolicyCall.html), [*locations workflow templates instantiate*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateInstantiateCall.html), [*locations workflow templates instantiate inline*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateInstantiateInlineCall.html), [*locations workflow templates list*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateListCall.html), [*locations workflow templates set iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateSetIamPolicyCall.html), [*locations workflow templates test iam permissions*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateTestIamPermissionCall.html), [*locations workflow templates update*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectLocationWorkflowTemplateUpdateCall.html), [*regions clusters create*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterCreateCall.html), [*regions clusters delete*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterDeleteCall.html), [*regions clusters diagnose*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterDiagnoseCall.html), [*regions clusters get*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterGetCall.html), [*regions clusters get iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterGetIamPolicyCall.html), [*regions clusters list*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterListCall.html), [*regions clusters patch*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterPatchCall.html), [*regions clusters set iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterSetIamPolicyCall.html), [*regions clusters test iam permissions*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionClusterTestIamPermissionCall.html), [*regions jobs cancel*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobCancelCall.html), [*regions jobs delete*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobDeleteCall.html), [*regions jobs get*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobGetCall.html), [*regions jobs get iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobGetIamPolicyCall.html), [*regions jobs list*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobListCall.html), [*regions jobs patch*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobPatchCall.html), [*regions jobs set iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobSetIamPolicyCall.html), [*regions jobs submit*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobSubmitCall.html), [*regions jobs test iam permissions*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionJobTestIamPermissionCall.html), [*regions operations cancel*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationCancelCall.html), [*regions operations delete*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationDeleteCall.html), [*regions operations get*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationGetCall.html), [*regions operations get iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationGetIamPolicyCall.html), [*regions operations list*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationListCall.html), [*regions operations set iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationSetIamPolicyCall.html), [*regions operations test iam permissions*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionOperationTestIamPermissionCall.html), [*regions workflow templates create*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateCreateCall.html), [*regions workflow templates delete*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateDeleteCall.html), [*regions workflow templates get*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateGetCall.html), [*regions workflow templates get iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateGetIamPolicyCall.html), [*regions workflow templates instantiate*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateInstantiateCall.html), [*regions workflow templates instantiate inline*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateInstantiateInlineCall.html), [*regions workflow templates list*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateListCall.html), [*regions workflow templates set iam policy*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateSetIamPolicyCall.html), [*regions workflow templates test iam permissions*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateTestIamPermissionCall.html) and [*regions workflow templates update*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.ProjectRegionWorkflowTemplateUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/struct.Dataproc.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().regions_clusters_get_iam_policy(...).doit()
let r = hub.projects().regions_workflow_templates_set_iam_policy(...).doit()
let r = hub.projects().regions_clusters_set_iam_policy(...).doit()
let r = hub.projects().regions_workflow_templates_get_iam_policy(...).doit()
let r = hub.projects().regions_jobs_get_iam_policy(...).doit()
let r = hub.projects().regions_operations_get_iam_policy(...).doit()
let r = hub.projects().regions_operations_set_iam_policy(...).doit()
let r = hub.projects().locations_workflow_templates_set_iam_policy(...).doit()
let r = hub.projects().regions_jobs_set_iam_policy(...).doit()
let r = hub.projects().locations_workflow_templates_get_iam_policy(...).doit()
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
google-dataproc1 = "*"
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
extern crate google_dataproc1 as dataproc1;
use dataproc1::GetIamPolicyRequest;
use dataproc1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use dataproc1::Dataproc;

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
let mut hub = Dataproc::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GetIamPolicyRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().regions_clusters_get_iam_policy(req, "resource")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dataproc1/1.0.8+20190313/google_dataproc1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **dataproc1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
