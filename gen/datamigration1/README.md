<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-datamigration1` library allows access to all features of the *Google Database Migration Service* service.

This documentation was generated from *Database Migration Service* crate version *5.0.5+20240621*, where *20240621* is the exact revision of the *datamigration:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Database Migration Service* *v1* API can be found at the
[official documentation site](https://cloud.google.com/database-migration/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/DatabaseMigrationService) ... 

* projects
 * [*locations connection profiles create*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileCreateCall), [*locations connection profiles delete*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileDeleteCall), [*locations connection profiles get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileGetCall), [*locations connection profiles get iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileGetIamPolicyCall), [*locations connection profiles list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileListCall), [*locations connection profiles patch*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfilePatchCall), [*locations connection profiles set iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileSetIamPolicyCall), [*locations connection profiles test iam permissions*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConnectionProfileTestIamPermissionCall), [*locations conversion workspaces apply*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceApplyCall), [*locations conversion workspaces commit*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceCommitCall), [*locations conversion workspaces convert*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceConvertCall), [*locations conversion workspaces create*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceCreateCall), [*locations conversion workspaces delete*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceDeleteCall), [*locations conversion workspaces describe conversion workspace revisions*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceDescribeConversionWorkspaceRevisionCall), [*locations conversion workspaces describe database entities*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceDescribeDatabaseEntityCall), [*locations conversion workspaces get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceGetCall), [*locations conversion workspaces get iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceGetIamPolicyCall), [*locations conversion workspaces list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceListCall), [*locations conversion workspaces mapping rules create*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceMappingRuleCreateCall), [*locations conversion workspaces mapping rules delete*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceMappingRuleDeleteCall), [*locations conversion workspaces mapping rules get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceMappingRuleGetCall), [*locations conversion workspaces mapping rules import*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceMappingRuleImportCall), [*locations conversion workspaces mapping rules list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceMappingRuleListCall), [*locations conversion workspaces patch*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspacePatchCall), [*locations conversion workspaces rollback*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceRollbackCall), [*locations conversion workspaces search background jobs*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceSearchBackgroundJobCall), [*locations conversion workspaces seed*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceSeedCall), [*locations conversion workspaces set iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceSetIamPolicyCall), [*locations conversion workspaces test iam permissions*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationConversionWorkspaceTestIamPermissionCall), [*locations fetch static ips*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationFetchStaticIpCall), [*locations get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationGetCall), [*locations list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationListCall), [*locations migration jobs create*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobCreateCall), [*locations migration jobs delete*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobDeleteCall), [*locations migration jobs demote destination*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobDemoteDestinationCall), [*locations migration jobs generate ssh script*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobGenerateSshScriptCall), [*locations migration jobs generate tcp proxy script*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobGenerateTcpProxyScriptCall), [*locations migration jobs get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobGetCall), [*locations migration jobs get iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobGetIamPolicyCall), [*locations migration jobs list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobListCall), [*locations migration jobs patch*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobPatchCall), [*locations migration jobs promote*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobPromoteCall), [*locations migration jobs restart*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobRestartCall), [*locations migration jobs resume*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobResumeCall), [*locations migration jobs set iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobSetIamPolicyCall), [*locations migration jobs start*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobStartCall), [*locations migration jobs stop*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobStopCall), [*locations migration jobs test iam permissions*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobTestIamPermissionCall), [*locations migration jobs verify*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationMigrationJobVerifyCall), [*locations operations cancel*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationOperationListCall), [*locations private connections create*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionCreateCall), [*locations private connections delete*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionDeleteCall), [*locations private connections get*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionGetCall), [*locations private connections get iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionGetIamPolicyCall), [*locations private connections list*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionListCall), [*locations private connections set iam policy*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionSetIamPolicyCall) and [*locations private connections test iam permissions*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/api::ProjectLocationPrivateConnectionTestIamPermissionCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/DatabaseMigrationService)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::CallBuilder)
* **[Resources](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_connection_profiles_create(...).doit().await
let r = hub.projects().locations_connection_profiles_delete(...).doit().await
let r = hub.projects().locations_connection_profiles_patch(...).doit().await
let r = hub.projects().locations_conversion_workspaces_mapping_rules_import(...).doit().await
let r = hub.projects().locations_conversion_workspaces_apply(...).doit().await
let r = hub.projects().locations_conversion_workspaces_commit(...).doit().await
let r = hub.projects().locations_conversion_workspaces_convert(...).doit().await
let r = hub.projects().locations_conversion_workspaces_create(...).doit().await
let r = hub.projects().locations_conversion_workspaces_delete(...).doit().await
let r = hub.projects().locations_conversion_workspaces_patch(...).doit().await
let r = hub.projects().locations_conversion_workspaces_rollback(...).doit().await
let r = hub.projects().locations_conversion_workspaces_seed(...).doit().await
let r = hub.projects().locations_migration_jobs_create(...).doit().await
let r = hub.projects().locations_migration_jobs_delete(...).doit().await
let r = hub.projects().locations_migration_jobs_demote_destination(...).doit().await
let r = hub.projects().locations_migration_jobs_patch(...).doit().await
let r = hub.projects().locations_migration_jobs_promote(...).doit().await
let r = hub.projects().locations_migration_jobs_restart(...).doit().await
let r = hub.projects().locations_migration_jobs_resume(...).doit().await
let r = hub.projects().locations_migration_jobs_start(...).doit().await
let r = hub.projects().locations_migration_jobs_stop(...).doit().await
let r = hub.projects().locations_migration_jobs_verify(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_private_connections_create(...).doit().await
let r = hub.projects().locations_private_connections_delete(...).doit().await
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
google-datamigration1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_datamigration1 as datamigration1;
use datamigration1::api::ConnectionProfile;
use datamigration1::{Result, Error};
use std::default::Default;
use datamigration1::{DatabaseMigrationService, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = DatabaseMigrationService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = ConnectionProfile::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_connection_profiles_create(req, "parent")
             .validate_only(true)
             .skip_validation(false)
             .request_id("amet.")
             .connection_profile_id("takimata")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::RequestValue) and 
[decodable](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-datamigration1/5.0.5+20240621/google_datamigration1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **datamigration1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

