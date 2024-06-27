<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-cloudbilling1` library allows access to all features of the *Google Cloudbilling* service.

This documentation was generated from *Cloudbilling* crate version *5.0.5+20240614*, where *20240614* is the exact revision of the *cloudbilling:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Cloudbilling* *v1* API can be found at the
[official documentation site](https://cloud.google.com/billing/docs/apis).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/Cloudbilling) ... 

* [billing accounts](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccount)
 * [*create*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountCreateCall), [*get*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountGetCall), [*get iam policy*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountGetIamPolicyCall), [*list*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountListCall), [*move*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountMoveCall), [*patch*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountPatchCall), [*projects list*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountProjectListCall), [*set iam policy*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountSetIamPolicyCall), [*sub accounts create*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountSubAccountCreateCall), [*sub accounts list*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountSubAccountListCall) and [*test iam permissions*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::BillingAccountTestIamPermissionCall)
* organizations
 * [*billing accounts create*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::OrganizationBillingAccountCreateCall), [*billing accounts list*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::OrganizationBillingAccountListCall) and [*billing accounts move*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::OrganizationBillingAccountMoveCall)
* projects
 * [*get billing info*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::ProjectGetBillingInfoCall) and [*update billing info*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::ProjectUpdateBillingInfoCall)
* [services](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::Service)
 * [*list*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::ServiceListCall) and [*skus list*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/api::ServiceSkuListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/Cloudbilling)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::CallBuilder)
* **[Resources](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.billing_accounts().projects_list(...).doit().await
let r = hub.billing_accounts().sub_accounts_create(...).doit().await
let r = hub.billing_accounts().sub_accounts_list(...).doit().await
let r = hub.billing_accounts().create(...).doit().await
let r = hub.billing_accounts().get(...).doit().await
let r = hub.billing_accounts().get_iam_policy(...).doit().await
let r = hub.billing_accounts().list(...).doit().await
let r = hub.billing_accounts().move_(...).doit().await
let r = hub.billing_accounts().patch(...).doit().await
let r = hub.billing_accounts().set_iam_policy(...).doit().await
let r = hub.billing_accounts().test_iam_permissions(...).doit().await
let r = hub.organizations().billing_accounts_create(...).doit().await
let r = hub.organizations().billing_accounts_move(...).doit().await
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
google-cloudbilling1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_cloudbilling1 as cloudbilling1;
use cloudbilling1::{Result, Error};
use std::default::Default;
use cloudbilling1::{Cloudbilling, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Cloudbilling::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.billing_accounts().sub_accounts_list("parent")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::RequestValue) and 
[decodable](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-cloudbilling1/5.0.5+20240614/google_cloudbilling1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **cloudbilling1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

