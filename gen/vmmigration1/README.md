<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-vmmigration1` library allows access to all features of the *Google VM Migration Service* service.

This documentation was generated from *VM Migration Service* crate version *5.0.5+20240613*, where *20240613* is the exact revision of the *vmmigration:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *VM Migration Service* *v1* API can be found at the
[official documentation site](https://cloud.google.com/migrate/virtual-machines).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/VMMigrationService) ... 

* projects
 * [*locations get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGetCall), [*locations groups add group migration*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupAddGroupMigrationCall), [*locations groups create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupCreateCall), [*locations groups delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupDeleteCall), [*locations groups get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupGetCall), [*locations groups list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupListCall), [*locations groups patch*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupPatchCall), [*locations groups remove group migration*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationGroupRemoveGroupMigrationCall), [*locations image imports create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportCreateCall), [*locations image imports delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportDeleteCall), [*locations image imports get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportGetCall), [*locations image imports image import jobs cancel*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportImageImportJobCancelCall), [*locations image imports image import jobs get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportImageImportJobGetCall), [*locations image imports image import jobs list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportImageImportJobListCall), [*locations image imports list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationImageImportListCall), [*locations list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationOperationListCall), [*locations sources create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceCreateCall), [*locations sources datacenter connectors create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceDatacenterConnectorCreateCall), [*locations sources datacenter connectors delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceDatacenterConnectorDeleteCall), [*locations sources datacenter connectors get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceDatacenterConnectorGetCall), [*locations sources datacenter connectors list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceDatacenterConnectorListCall), [*locations sources datacenter connectors upgrade appliance*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceDatacenterConnectorUpgradeApplianceCall), [*locations sources delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceDeleteCall), [*locations sources fetch inventory*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceFetchInventoryCall), [*locations sources get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceGetCall), [*locations sources list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceListCall), [*locations sources migrating vms clone jobs cancel*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCloneJobCancelCall), [*locations sources migrating vms clone jobs create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCloneJobCreateCall), [*locations sources migrating vms clone jobs get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCloneJobGetCall), [*locations sources migrating vms clone jobs list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCloneJobListCall), [*locations sources migrating vms create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCreateCall), [*locations sources migrating vms cutover jobs cancel*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCutoverJobCancelCall), [*locations sources migrating vms cutover jobs create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCutoverJobCreateCall), [*locations sources migrating vms cutover jobs get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCutoverJobGetCall), [*locations sources migrating vms cutover jobs list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmCutoverJobListCall), [*locations sources migrating vms delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmDeleteCall), [*locations sources migrating vms finalize migration*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmFinalizeMigrationCall), [*locations sources migrating vms get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmGetCall), [*locations sources migrating vms list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmListCall), [*locations sources migrating vms patch*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmPatchCall), [*locations sources migrating vms pause migration*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmPauseMigrationCall), [*locations sources migrating vms replication cycles get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmReplicationCycleGetCall), [*locations sources migrating vms replication cycles list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmReplicationCycleListCall), [*locations sources migrating vms resume migration*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmResumeMigrationCall), [*locations sources migrating vms start migration*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceMigratingVmStartMigrationCall), [*locations sources patch*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourcePatchCall), [*locations sources utilization reports create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceUtilizationReportCreateCall), [*locations sources utilization reports delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceUtilizationReportDeleteCall), [*locations sources utilization reports get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceUtilizationReportGetCall), [*locations sources utilization reports list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationSourceUtilizationReportListCall), [*locations target projects create*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationTargetProjectCreateCall), [*locations target projects delete*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationTargetProjectDeleteCall), [*locations target projects get*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationTargetProjectGetCall), [*locations target projects list*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationTargetProjectListCall) and [*locations target projects patch*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/api::ProjectLocationTargetProjectPatchCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/VMMigrationService)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::CallBuilder)
* **[Resources](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_groups_add_group_migration(...).doit().await
let r = hub.projects().locations_groups_create(...).doit().await
let r = hub.projects().locations_groups_delete(...).doit().await
let r = hub.projects().locations_groups_patch(...).doit().await
let r = hub.projects().locations_groups_remove_group_migration(...).doit().await
let r = hub.projects().locations_image_imports_image_import_jobs_cancel(...).doit().await
let r = hub.projects().locations_image_imports_create(...).doit().await
let r = hub.projects().locations_image_imports_delete(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_sources_datacenter_connectors_create(...).doit().await
let r = hub.projects().locations_sources_datacenter_connectors_delete(...).doit().await
let r = hub.projects().locations_sources_datacenter_connectors_upgrade_appliance(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_clone_jobs_cancel(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_clone_jobs_create(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_cutover_jobs_cancel(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_cutover_jobs_create(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_create(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_delete(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_finalize_migration(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_patch(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_pause_migration(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_resume_migration(...).doit().await
let r = hub.projects().locations_sources_migrating_vms_start_migration(...).doit().await
let r = hub.projects().locations_sources_utilization_reports_create(...).doit().await
let r = hub.projects().locations_sources_utilization_reports_delete(...).doit().await
let r = hub.projects().locations_sources_create(...).doit().await
let r = hub.projects().locations_sources_delete(...).doit().await
let r = hub.projects().locations_sources_patch(...).doit().await
let r = hub.projects().locations_target_projects_create(...).doit().await
let r = hub.projects().locations_target_projects_delete(...).doit().await
let r = hub.projects().locations_target_projects_patch(...).doit().await
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
google-vmmigration1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_vmmigration1 as vmmigration1;
use vmmigration1::api::Group;
use vmmigration1::{Result, Error};
use std::default::Default;
use vmmigration1::{VMMigrationService, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = VMMigrationService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Group::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_groups_create(req, "parent")
             .request_id("magna")
             .group_id("no")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::RequestValue) and 
[decodable](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-vmmigration1/5.0.5+20240613/google_vmmigration1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **vmmigration1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

