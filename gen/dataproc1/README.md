<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dataproc1` library allows access to all features of the *Google Dataproc* service.

This documentation was generated from *Dataproc* crate version *5.0.5+20240617*, where *20240617* is the exact revision of the *dataproc:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Dataproc* *v1* API can be found at the
[official documentation site](https://cloud.google.com/dataproc/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/Dataproc) ... 

* projects
 * [*locations autoscaling policies create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyCreateCall), [*locations autoscaling policies delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyDeleteCall), [*locations autoscaling policies get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyGetCall), [*locations autoscaling policies get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyGetIamPolicyCall), [*locations autoscaling policies list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyListCall), [*locations autoscaling policies set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicySetIamPolicyCall), [*locations autoscaling policies test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyTestIamPermissionCall), [*locations autoscaling policies update*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationAutoscalingPolicyUpdateCall), [*locations batches analyze*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationBatchAnalyzeCall), [*locations batches create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationBatchCreateCall), [*locations batches delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationBatchDeleteCall), [*locations batches get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationBatchGetCall), [*locations batches list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationBatchListCall), [*locations operations cancel*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationOperationListCall), [*locations session templates create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionTemplateCreateCall), [*locations session templates delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionTemplateDeleteCall), [*locations session templates get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionTemplateGetCall), [*locations session templates list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionTemplateListCall), [*locations session templates patch*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionTemplatePatchCall), [*locations sessions create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionCreateCall), [*locations sessions delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionDeleteCall), [*locations sessions get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionGetCall), [*locations sessions list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionListCall), [*locations sessions terminate*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationSessionTerminateCall), [*locations workflow templates create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateCreateCall), [*locations workflow templates delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateDeleteCall), [*locations workflow templates get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateGetCall), [*locations workflow templates get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateGetIamPolicyCall), [*locations workflow templates instantiate*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateInstantiateCall), [*locations workflow templates instantiate inline*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateInstantiateInlineCall), [*locations workflow templates list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateListCall), [*locations workflow templates set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateSetIamPolicyCall), [*locations workflow templates test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateTestIamPermissionCall), [*locations workflow templates update*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectLocationWorkflowTemplateUpdateCall), [*regions autoscaling policies create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyCreateCall), [*regions autoscaling policies delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyDeleteCall), [*regions autoscaling policies get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyGetCall), [*regions autoscaling policies get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyGetIamPolicyCall), [*regions autoscaling policies list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyListCall), [*regions autoscaling policies set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicySetIamPolicyCall), [*regions autoscaling policies test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyTestIamPermissionCall), [*regions autoscaling policies update*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionAutoscalingPolicyUpdateCall), [*regions clusters create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterCreateCall), [*regions clusters delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterDeleteCall), [*regions clusters diagnose*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterDiagnoseCall), [*regions clusters get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterGetCall), [*regions clusters get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterGetIamPolicyCall), [*regions clusters inject credentials*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterInjectCredentialCall), [*regions clusters list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterListCall), [*regions clusters node groups create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterNodeGroupCreateCall), [*regions clusters node groups get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterNodeGroupGetCall), [*regions clusters node groups repair*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterNodeGroupRepairCall), [*regions clusters node groups resize*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterNodeGroupResizeCall), [*regions clusters patch*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterPatchCall), [*regions clusters repair*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterRepairCall), [*regions clusters set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterSetIamPolicyCall), [*regions clusters start*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterStartCall), [*regions clusters stop*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterStopCall), [*regions clusters test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionClusterTestIamPermissionCall), [*regions jobs cancel*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobCancelCall), [*regions jobs delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobDeleteCall), [*regions jobs get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobGetCall), [*regions jobs get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobGetIamPolicyCall), [*regions jobs list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobListCall), [*regions jobs patch*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobPatchCall), [*regions jobs set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobSetIamPolicyCall), [*regions jobs submit*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobSubmitCall), [*regions jobs submit as operation*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobSubmitAsOperationCall), [*regions jobs test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionJobTestIamPermissionCall), [*regions operations cancel*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationCancelCall), [*regions operations delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationDeleteCall), [*regions operations get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationGetCall), [*regions operations get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationGetIamPolicyCall), [*regions operations list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationListCall), [*regions operations set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationSetIamPolicyCall), [*regions operations test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionOperationTestIamPermissionCall), [*regions workflow templates create*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateCreateCall), [*regions workflow templates delete*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateDeleteCall), [*regions workflow templates get*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateGetCall), [*regions workflow templates get iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateGetIamPolicyCall), [*regions workflow templates instantiate*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateInstantiateCall), [*regions workflow templates instantiate inline*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateInstantiateInlineCall), [*regions workflow templates list*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateListCall), [*regions workflow templates set iam policy*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateSetIamPolicyCall), [*regions workflow templates test iam permissions*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateTestIamPermissionCall) and [*regions workflow templates update*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/api::ProjectRegionWorkflowTemplateUpdateCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/Dataproc)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::CallBuilder)
* **[Resources](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_batches_analyze(...).doit().await
let r = hub.projects().locations_batches_create(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_sessions_create(...).doit().await
let r = hub.projects().locations_sessions_delete(...).doit().await
let r = hub.projects().locations_sessions_terminate(...).doit().await
let r = hub.projects().locations_workflow_templates_instantiate(...).doit().await
let r = hub.projects().locations_workflow_templates_instantiate_inline(...).doit().await
let r = hub.projects().regions_clusters_node_groups_create(...).doit().await
let r = hub.projects().regions_clusters_node_groups_repair(...).doit().await
let r = hub.projects().regions_clusters_node_groups_resize(...).doit().await
let r = hub.projects().regions_clusters_create(...).doit().await
let r = hub.projects().regions_clusters_delete(...).doit().await
let r = hub.projects().regions_clusters_diagnose(...).doit().await
let r = hub.projects().regions_clusters_inject_credentials(...).doit().await
let r = hub.projects().regions_clusters_patch(...).doit().await
let r = hub.projects().regions_clusters_repair(...).doit().await
let r = hub.projects().regions_clusters_start(...).doit().await
let r = hub.projects().regions_clusters_stop(...).doit().await
let r = hub.projects().regions_jobs_submit_as_operation(...).doit().await
let r = hub.projects().regions_operations_get(...).doit().await
let r = hub.projects().regions_workflow_templates_instantiate(...).doit().await
let r = hub.projects().regions_workflow_templates_instantiate_inline(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_dataproc1 as dataproc1;
use dataproc1::api::Cluster;
use dataproc1::{Result, Error};
use std::default::Default;
use dataproc1::{Dataproc, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Dataproc::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Cluster::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().regions_clusters_patch(req, "projectId", "region", "clusterName")
             .update_mask(FieldMask::new::<&str>(&[]))
             .request_id("ipsum")
             .graceful_decommission_timeout(chrono::Duration::seconds(9579437))
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::RequestValue) and 
[decodable](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dataproc1/5.0.5+20240617/google_dataproc1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **dataproc1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

