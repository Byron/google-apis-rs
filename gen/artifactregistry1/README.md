<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-artifactregistry1` library allows access to all features of the *Google Artifact Registry* service.

This documentation was generated from *Artifact Registry* crate version *6.0.0+20240608*, where *20240608* is the exact revision of the *artifactregistry:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.

Everything else about the *Artifact Registry* *v1* API can be found at the
[official documentation site](https://cloud.google.com/artifacts/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/ArtifactRegistry) ...

* projects
 * [*get project settings*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectGetProjectSettingCall), [*locations get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationGetCall), [*locations get vpcsc config*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationGetVpcscConfigCall), [*locations list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationListCall), [*locations operations get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationOperationGetCall), [*locations repositories apt artifacts import*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryAptArtifactImportCall), [*locations repositories apt artifacts upload*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryAptArtifactUploadCall), [*locations repositories create*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryCreateCall), [*locations repositories delete*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryDeleteCall), [*locations repositories docker images get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryDockerImageGetCall), [*locations repositories docker images list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryDockerImageListCall), [*locations repositories files delete*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryFileDeleteCall), [*locations repositories files download*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryFileDownloadCall), [*locations repositories files get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryFileGetCall), [*locations repositories files list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryFileListCall), [*locations repositories generic artifacts upload*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGenericArtifactUploadCall), [*locations repositories get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGetCall), [*locations repositories get iam policy*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGetIamPolicyCall), [*locations repositories go modules upload*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGoModuleUploadCall), [*locations repositories googet artifacts import*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGoogetArtifactImportCall), [*locations repositories googet artifacts upload*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGoogetArtifactUploadCall), [*locations repositories kfp artifacts upload*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryKfpArtifactUploadCall), [*locations repositories list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryListCall), [*locations repositories maven artifacts get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryMavenArtifactGetCall), [*locations repositories maven artifacts list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryMavenArtifactListCall), [*locations repositories npm packages get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryNpmPackageGetCall), [*locations repositories npm packages list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryNpmPackageListCall), [*locations repositories packages delete*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageDeleteCall), [*locations repositories packages get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageGetCall), [*locations repositories packages list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageListCall), [*locations repositories packages patch*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackagePatchCall), [*locations repositories packages tags create*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageTagCreateCall), [*locations repositories packages tags delete*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageTagDeleteCall), [*locations repositories packages tags get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageTagGetCall), [*locations repositories packages tags list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageTagListCall), [*locations repositories packages tags patch*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageTagPatchCall), [*locations repositories packages versions batch delete*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageVersionBatchDeleteCall), [*locations repositories packages versions delete*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageVersionDeleteCall), [*locations repositories packages versions get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageVersionGetCall), [*locations repositories packages versions list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPackageVersionListCall), [*locations repositories patch*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPatchCall), [*locations repositories python packages get*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPythonPackageGetCall), [*locations repositories python packages list*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryPythonPackageListCall), [*locations repositories set iam policy*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositorySetIamPolicyCall), [*locations repositories test iam permissions*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryTestIamPermissionCall), [*locations repositories yum artifacts import*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryYumArtifactImportCall), [*locations repositories yum artifacts upload*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryYumArtifactUploadCall), [*locations update vpcsc config*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationUpdateVpcscConfigCall) and [*update project settings*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectUpdateProjectSettingCall)


Upload supported by ...

* [*locations repositories apt artifacts upload projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryAptArtifactUploadCall)
* [*locations repositories generic artifacts upload projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGenericArtifactUploadCall)
* [*locations repositories go modules upload projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGoModuleUploadCall)
* [*locations repositories googet artifacts upload projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryGoogetArtifactUploadCall)
* [*locations repositories kfp artifacts upload projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryKfpArtifactUploadCall)
* [*locations repositories yum artifacts upload projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryYumArtifactUploadCall)

Download supported by ...

* [*locations repositories files download projects*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/api::ProjectLocationRepositoryFileDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/ArtifactRegistry)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::CallBuilder)
* **[Resources](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_repositories_apt_artifacts_import(...).doit().await
let r = hub.projects().locations_repositories_files_delete(...).doit().await
let r = hub.projects().locations_repositories_googet_artifacts_import(...).doit().await
let r = hub.projects().locations_repositories_packages_versions_batch_delete(...).doit().await
let r = hub.projects().locations_repositories_packages_versions_delete(...).doit().await
let r = hub.projects().locations_repositories_packages_delete(...).doit().await
let r = hub.projects().locations_repositories_yum_artifacts_import(...).doit().await
let r = hub.projects().locations_repositories_create(...).doit().await
let r = hub.projects().locations_repositories_delete(...).doit().await
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
google-artifactregistry1 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_artifactregistry1 as artifactregistry1;
use artifactregistry1::api::Repository;
use artifactregistry1::{Result, Error};
use artifactregistry1::{ArtifactRegistry, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build()
);
let mut hub = ArtifactRegistry::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Repository::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_repositories_create(req, "parent")
             .repository_id("magna")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Delegate) to the
[Method Builder](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::RequestValue) and
[decodable](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-artifactregistry1/6.0.0+20240608/google_artifactregistry1/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **artifactregistry1** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

