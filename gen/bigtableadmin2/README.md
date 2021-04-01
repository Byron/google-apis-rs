<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-bigtableadmin2` library allows access to all features of the *Google Bigtable Admin* service.

This documentation was generated from *Bigtable Admin* crate version *2.0.0+20210323*, where *20210323* is the exact revision of the *bigtableadmin:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v2.0.0*.

Everything else about the *Bigtable Admin* *v2* API can be found at the
[official documentation site](https://cloud.google.com/bigtable/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/BigtableAdmin) ... 

* [operations](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::Operation)
 * [*cancel*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::OperationCancelCall), [*delete*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::OperationDeleteCall), [*get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::OperationGetCall) and [*projects operations list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::OperationProjectOperationListCall)
* projects
 * [*instances app profiles create*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceAppProfileCreateCall), [*instances app profiles delete*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceAppProfileDeleteCall), [*instances app profiles get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceAppProfileGetCall), [*instances app profiles list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceAppProfileListCall), [*instances app profiles patch*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceAppProfilePatchCall), [*instances clusters backups create*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupCreateCall), [*instances clusters backups delete*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupDeleteCall), [*instances clusters backups get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupGetCall), [*instances clusters backups get iam policy*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupGetIamPolicyCall), [*instances clusters backups list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupListCall), [*instances clusters backups patch*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupPatchCall), [*instances clusters backups set iam policy*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupSetIamPolicyCall), [*instances clusters backups test iam permissions*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterBackupTestIamPermissionCall), [*instances clusters create*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterCreateCall), [*instances clusters delete*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterDeleteCall), [*instances clusters get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterGetCall), [*instances clusters list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterListCall), [*instances clusters update*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceClusterUpdateCall), [*instances create*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceCreateCall), [*instances delete*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceDeleteCall), [*instances get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceGetCall), [*instances get iam policy*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceGetIamPolicyCall), [*instances list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceListCall), [*instances partial update instance*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstancePartialUpdateInstanceCall), [*instances set iam policy*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceSetIamPolicyCall), [*instances tables check consistency*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableCheckConsistencyCall), [*instances tables create*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableCreateCall), [*instances tables delete*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableDeleteCall), [*instances tables drop row range*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableDropRowRangeCall), [*instances tables generate consistency token*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableGenerateConsistencyTokenCall), [*instances tables get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableGetCall), [*instances tables get iam policy*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableGetIamPolicyCall), [*instances tables list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableListCall), [*instances tables modify column families*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableModifyColumnFamilyCall), [*instances tables restore*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableRestoreCall), [*instances tables set iam policy*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableSetIamPolicyCall), [*instances tables test iam permissions*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTableTestIamPermissionCall), [*instances test iam permissions*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceTestIamPermissionCall), [*instances update*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectInstanceUpdateCall), [*locations get*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectLocationGetCall) and [*locations list*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/api::ProjectLocationListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/BigtableAdmin)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::CallBuilder)
* **[Resources](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.operations().projects_operations_list(...).doit().await
let r = hub.operations().cancel(...).doit().await
let r = hub.operations().delete(...).doit().await
let r = hub.operations().get(...).doit().await
let r = hub.projects().instances_app_profiles_patch(...).doit().await
let r = hub.projects().instances_clusters_backups_create(...).doit().await
let r = hub.projects().instances_clusters_create(...).doit().await
let r = hub.projects().instances_clusters_update(...).doit().await
let r = hub.projects().instances_tables_restore(...).doit().await
let r = hub.projects().instances_create(...).doit().await
let r = hub.projects().instances_partial_update_instance(...).doit().await
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
google-bigtableadmin2 = "*"
# This project intentionally uses an old version of Hyper. See
# https://github.com/Byron/google-apis-rs/issues/173 for more
# information.
hyper = "^0.14"
hyper-rustls = "^0.22"
serde = "^1.0"
serde_json = "^1.0"
yup-oauth2 = "^5.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_bigtableadmin2 as bigtableadmin2;
use bigtableadmin2::{Result, Error};
use std::default::Default;
use oauth2;
use bigtableadmin2::BigtableAdmin;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = BigtableAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.operations().projects_operations_list("name")
             .page_token("magna")
             .page_size(-11)
             .filter("ipsum")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::RequestValue) and 
[decodable](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-bigtableadmin2/2.0.0+20210323/google_bigtableadmin2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **bigtableadmin2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
