<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-metastore1_beta` library allows access to all features of the *Google Dataproc Metastore* service.

This documentation was generated from *Dataproc Metastore* crate version *5.0.5+20240613*, where *20240613* is the exact revision of the *metastore:v1beta* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Dataproc Metastore* *v1_beta* API can be found at the
[official documentation site](https://cloud.google.com/dataproc-metastore/docs).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/DataprocMetastore) ... 

* projects
 * [*locations federations create*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationCreateCall), [*locations federations delete*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationDeleteCall), [*locations federations get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationGetCall), [*locations federations get iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationGetIamPolicyCall), [*locations federations list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationListCall), [*locations federations patch*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationPatchCall), [*locations federations set iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationSetIamPolicyCall), [*locations federations test iam permissions*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationFederationTestIamPermissionCall), [*locations get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationGetCall), [*locations list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationOperationListCall), [*locations services alter location*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceAlterLocationCall), [*locations services alter table properties*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceAlterTablePropertyCall), [*locations services backups create*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupCreateCall), [*locations services backups delete*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupDeleteCall), [*locations services backups get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupGetCall), [*locations services backups get iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupGetIamPolicyCall), [*locations services backups list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupListCall), [*locations services backups set iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupSetIamPolicyCall), [*locations services backups test iam permissions*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceBackupTestIamPermissionCall), [*locations services cancel migration*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceCancelMigrationCall), [*locations services complete migration*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceCompleteMigrationCall), [*locations services create*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceCreateCall), [*locations services databases get iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDatabaseGetIamPolicyCall), [*locations services databases set iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDatabaseSetIamPolicyCall), [*locations services databases tables get iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDatabaseTableGetIamPolicyCall), [*locations services databases tables set iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDatabaseTableSetIamPolicyCall), [*locations services databases tables test iam permissions*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDatabaseTableTestIamPermissionCall), [*locations services databases test iam permissions*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDatabaseTestIamPermissionCall), [*locations services delete*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceDeleteCall), [*locations services export metadata*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceExportMetadataCall), [*locations services get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceGetCall), [*locations services get iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceGetIamPolicyCall), [*locations services list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceListCall), [*locations services metadata imports create*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMetadataImportCreateCall), [*locations services metadata imports get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMetadataImportGetCall), [*locations services metadata imports list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMetadataImportListCall), [*locations services metadata imports patch*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMetadataImportPatchCall), [*locations services migration executions delete*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMigrationExecutionDeleteCall), [*locations services migration executions get*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMigrationExecutionGetCall), [*locations services migration executions list*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMigrationExecutionListCall), [*locations services move table to database*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceMoveTableToDatabaseCall), [*locations services patch*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServicePatchCall), [*locations services query metadata*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceQueryMetadataCall), [*locations services remove iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceRemoveIamPolicyCall), [*locations services restore*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceRestoreCall), [*locations services set iam policy*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceSetIamPolicyCall), [*locations services start migration*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceStartMigrationCall) and [*locations services test iam permissions*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/api::ProjectLocationServiceTestIamPermissionCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/DataprocMetastore)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::CallBuilder)
* **[Resources](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_federations_create(...).doit().await
let r = hub.projects().locations_federations_delete(...).doit().await
let r = hub.projects().locations_federations_patch(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_services_backups_create(...).doit().await
let r = hub.projects().locations_services_backups_delete(...).doit().await
let r = hub.projects().locations_services_metadata_imports_create(...).doit().await
let r = hub.projects().locations_services_metadata_imports_patch(...).doit().await
let r = hub.projects().locations_services_migration_executions_delete(...).doit().await
let r = hub.projects().locations_services_alter_location(...).doit().await
let r = hub.projects().locations_services_alter_table_properties(...).doit().await
let r = hub.projects().locations_services_cancel_migration(...).doit().await
let r = hub.projects().locations_services_complete_migration(...).doit().await
let r = hub.projects().locations_services_create(...).doit().await
let r = hub.projects().locations_services_delete(...).doit().await
let r = hub.projects().locations_services_export_metadata(...).doit().await
let r = hub.projects().locations_services_move_table_to_database(...).doit().await
let r = hub.projects().locations_services_patch(...).doit().await
let r = hub.projects().locations_services_query_metadata(...).doit().await
let r = hub.projects().locations_services_restore(...).doit().await
let r = hub.projects().locations_services_start_migration(...).doit().await
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
google-metastore1_beta = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_metastore1_beta as metastore1_beta;
use metastore1_beta::api::Federation;
use metastore1_beta::{Result, Error};
use std::default::Default;
use metastore1_beta::{DataprocMetastore, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = DataprocMetastore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Federation::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_federations_create(req, "parent")
             .request_id("magna")
             .federation_id("no")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Delegate) to the 
[Method Builder](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::RequestValue) and 
[decodable](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-metastore1_beta/5.0.5+20240613/google_metastore1_beta/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **metastore1_beta** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

