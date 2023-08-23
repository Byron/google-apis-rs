<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-bigquery2` library allows access to all features of the *Google bigquery* service.

This documentation was generated from *bigquery* crate version *5.0.3+20230114*, where *20230114* is the exact revision of the *bigquery:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.3*.

Everything else about the *bigquery* *v2* API can be found at the
[official documentation site](https://cloud.google.com/bigquery/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/Bigquery) ... 

* [datasets](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::Dataset)
 * [*delete*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::DatasetDeleteCall), [*get*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::DatasetGetCall), [*insert*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::DatasetInsertCall), [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::DatasetListCall), [*patch*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::DatasetPatchCall) and [*update*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::DatasetUpdateCall)
* [jobs](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::Job)
 * [*cancel*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobCancelCall), [*delete*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobDeleteCall), [*get*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobGetCall), [*get query results*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobGetQueryResultCall), [*insert*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobInsertCall), [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobListCall) and [*query*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobQueryCall)
* [models](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::Model)
 * [*delete*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::ModelDeleteCall), [*get*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::ModelGetCall), [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::ModelListCall) and [*patch*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::ModelPatchCall)
* projects
 * [*get service account*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::ProjectGetServiceAccountCall) and [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::ProjectListCall)
* [routines](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::Routine)
 * [*delete*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RoutineDeleteCall), [*get*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RoutineGetCall), [*insert*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RoutineInsertCall), [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RoutineListCall) and [*update*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RoutineUpdateCall)
* [row access policies](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RowAccessPolicy)
 * [*get iam policy*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RowAccessPolicyGetIamPolicyCall), [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RowAccessPolicyListCall), [*set iam policy*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RowAccessPolicySetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::RowAccessPolicyTestIamPermissionCall)
* tabledata
 * [*insert all*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TabledataInsertAllCall) and [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TabledataListCall)
* [tables](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::Table)
 * [*delete*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableDeleteCall), [*get*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableGetCall), [*get iam policy*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableGetIamPolicyCall), [*insert*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableInsertCall), [*list*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableListCall), [*patch*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TablePatchCall), [*set iam policy*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableSetIamPolicyCall), [*test iam permissions*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableTestIamPermissionCall) and [*update*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::TableUpdateCall)


Upload supported by ...

* [*insert jobs*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/api::JobInsertCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/Bigquery)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::CallBuilder)
* **[Resources](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.tables().delete(...).doit().await
let r = hub.tables().get(...).doit().await
let r = hub.tables().get_iam_policy(...).doit().await
let r = hub.tables().insert(...).doit().await
let r = hub.tables().list(...).doit().await
let r = hub.tables().patch(...).doit().await
let r = hub.tables().set_iam_policy(...).doit().await
let r = hub.tables().test_iam_permissions(...).doit().await
let r = hub.tables().update(...).doit().await
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
google-bigquery2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_bigquery2 as bigquery2;
use bigquery2::{Result, Error};
use std::default::Default;
use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.tables().get("projectId", "datasetId", "tableId")
             .view("ipsum")
             .selected_fields("voluptua.")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::RequestValue) and 
[decodable](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-bigquery2/5.0.3+20230114/google_bigquery2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **bigquery2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

