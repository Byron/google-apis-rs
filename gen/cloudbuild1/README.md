<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-cloudbuild1` library allows access to all features of the *Google Cloud Build* service.

This documentation was generated from *Cloud Build* crate version *5.0.5+20240618*, where *20240618* is the exact revision of the *cloudbuild:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Cloud Build* *v1* API can be found at the
[official documentation site](https://cloud.google.com/cloud-build/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/CloudBuild) ... 

* github dot com webhook
 * [*receive*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::GithubDotComWebhookReceiveCall)
* locations
 * [*regional webhook*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::LocationRegionalWebhookCall)
* [operations](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::Operation)
 * [*cancel*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::OperationCancelCall) and [*get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::OperationGetCall)
* projects
 * [*builds approve*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectBuildApproveCall), [*builds cancel*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectBuildCancelCall), [*builds create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectBuildCreateCall), [*builds get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectBuildGetCall), [*builds list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectBuildListCall), [*builds retry*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectBuildRetryCall), [*github enterprise configs create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectGithubEnterpriseConfigCreateCall), [*github enterprise configs delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectGithubEnterpriseConfigDeleteCall), [*github enterprise configs get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectGithubEnterpriseConfigGetCall), [*github enterprise configs list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectGithubEnterpriseConfigListCall), [*github enterprise configs patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectGithubEnterpriseConfigPatchCall), [*locations bitbucket server configs connected repositories batch create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigConnectedRepositoryBatchCreateCall), [*locations bitbucket server configs create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigCreateCall), [*locations bitbucket server configs delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigDeleteCall), [*locations bitbucket server configs get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigGetCall), [*locations bitbucket server configs list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigListCall), [*locations bitbucket server configs patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigPatchCall), [*locations bitbucket server configs remove bitbucket server connected repository*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigRemoveBitbucketServerConnectedRepositoryCall), [*locations bitbucket server configs repos list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBitbucketServerConfigRepoListCall), [*locations builds approve*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBuildApproveCall), [*locations builds cancel*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBuildCancelCall), [*locations builds create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBuildCreateCall), [*locations builds get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBuildGetCall), [*locations builds list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBuildListCall), [*locations builds retry*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationBuildRetryCall), [*locations get default service account*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGetDefaultServiceAccountCall), [*locations git lab configs connected repositories batch create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigConnectedRepositoryBatchCreateCall), [*locations git lab configs create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigCreateCall), [*locations git lab configs delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigDeleteCall), [*locations git lab configs get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigGetCall), [*locations git lab configs list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigListCall), [*locations git lab configs patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigPatchCall), [*locations git lab configs remove git lab connected repository*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigRemoveGitLabConnectedRepositoryCall), [*locations git lab configs repos list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGitLabConfigRepoListCall), [*locations github enterprise configs create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGithubEnterpriseConfigCreateCall), [*locations github enterprise configs delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGithubEnterpriseConfigDeleteCall), [*locations github enterprise configs get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGithubEnterpriseConfigGetCall), [*locations github enterprise configs list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGithubEnterpriseConfigListCall), [*locations github enterprise configs patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationGithubEnterpriseConfigPatchCall), [*locations operations cancel*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationOperationCancelCall), [*locations operations get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationOperationGetCall), [*locations triggers create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerCreateCall), [*locations triggers delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerDeleteCall), [*locations triggers get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerGetCall), [*locations triggers list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerListCall), [*locations triggers patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerPatchCall), [*locations triggers run*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerRunCall), [*locations triggers webhook*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationTriggerWebhookCall), [*locations worker pools create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationWorkerPoolCreateCall), [*locations worker pools delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationWorkerPoolDeleteCall), [*locations worker pools get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationWorkerPoolGetCall), [*locations worker pools list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationWorkerPoolListCall), [*locations worker pools patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectLocationWorkerPoolPatchCall), [*triggers create*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerCreateCall), [*triggers delete*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerDeleteCall), [*triggers get*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerGetCall), [*triggers list*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerListCall), [*triggers patch*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerPatchCall), [*triggers run*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerRunCall) and [*triggers webhook*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::ProjectTriggerWebhookCall)

Other activities are ...

* [webhook](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/api::MethodWebhookCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/CloudBuild)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::CallBuilder)
* **[Resources](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.operations().cancel(...).doit().await
let r = hub.operations().get(...).doit().await
let r = hub.projects().builds_approve(...).doit().await
let r = hub.projects().builds_create(...).doit().await
let r = hub.projects().builds_retry(...).doit().await
let r = hub.projects().github_enterprise_configs_create(...).doit().await
let r = hub.projects().github_enterprise_configs_delete(...).doit().await
let r = hub.projects().github_enterprise_configs_patch(...).doit().await
let r = hub.projects().locations_bitbucket_server_configs_connected_repositories_batch_create(...).doit().await
let r = hub.projects().locations_bitbucket_server_configs_create(...).doit().await
let r = hub.projects().locations_bitbucket_server_configs_delete(...).doit().await
let r = hub.projects().locations_bitbucket_server_configs_patch(...).doit().await
let r = hub.projects().locations_builds_approve(...).doit().await
let r = hub.projects().locations_builds_create(...).doit().await
let r = hub.projects().locations_builds_retry(...).doit().await
let r = hub.projects().locations_git_lab_configs_connected_repositories_batch_create(...).doit().await
let r = hub.projects().locations_git_lab_configs_create(...).doit().await
let r = hub.projects().locations_git_lab_configs_delete(...).doit().await
let r = hub.projects().locations_git_lab_configs_patch(...).doit().await
let r = hub.projects().locations_github_enterprise_configs_create(...).doit().await
let r = hub.projects().locations_github_enterprise_configs_delete(...).doit().await
let r = hub.projects().locations_github_enterprise_configs_patch(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_triggers_run(...).doit().await
let r = hub.projects().locations_worker_pools_create(...).doit().await
let r = hub.projects().locations_worker_pools_delete(...).doit().await
let r = hub.projects().locations_worker_pools_patch(...).doit().await
let r = hub.projects().triggers_run(...).doit().await
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
google-cloudbuild1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_cloudbuild1 as cloudbuild1;
use cloudbuild1::api::GitHubEnterpriseConfig;
use cloudbuild1::{Result, Error};
use std::default::Default;
use cloudbuild1::{CloudBuild, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = CloudBuild::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GitHubEnterpriseConfig::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().github_enterprise_configs_create(req, "parent")
             .project_id("magna")
             .ghe_config_id("no")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::RequestValue) and 
[decodable](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-cloudbuild1/5.0.5+20240618/google_cloudbuild1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **cloudbuild1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

