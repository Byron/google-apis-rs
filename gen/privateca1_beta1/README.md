<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-privateca1_beta1` library allows access to all features of the *Google Certificate Authority Service* service.

This documentation was generated from *Certificate Authority Service* crate version *6.0.0+20240619*, where *20240619* is the exact revision of the *privateca:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.

Everything else about the *Certificate Authority Service* *v1_beta1* API can be found at the
[official documentation site](https://cloud.google.com/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/CertificateAuthorityService) ...

* projects
 * [*locations certificate authorities certificate revocation lists get iam policy*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListGetIamPolicyCall), [*locations certificate authorities certificate revocation lists set iam policy*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListSetIamPolicyCall), [*locations certificate authorities certificate revocation lists test iam permissions*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListTestIamPermissionCall), [*locations certificate authorities get iam policy*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityGetIamPolicyCall), [*locations certificate authorities set iam policy*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationCertificateAuthoritySetIamPolicyCall), [*locations certificate authorities test iam permissions*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityTestIamPermissionCall), [*locations get*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationGetCall), [*locations list*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationOperationListCall), [*locations reusable configs get iam policy*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationReusableConfigGetIamPolicyCall), [*locations reusable configs set iam policy*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationReusableConfigSetIamPolicyCall) and [*locations reusable configs test iam permissions*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/api::ProjectLocationReusableConfigTestIamPermissionCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/CertificateAuthorityService)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::CallBuilder)
* **[Resources](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_certificate_authorities_certificate_revocation_lists_get_iam_policy(...).doit().await
let r = hub.projects().locations_certificate_authorities_certificate_revocation_lists_set_iam_policy(...).doit().await
let r = hub.projects().locations_certificate_authorities_get_iam_policy(...).doit().await
let r = hub.projects().locations_certificate_authorities_set_iam_policy(...).doit().await
let r = hub.projects().locations_reusable_configs_get_iam_policy(...).doit().await
let r = hub.projects().locations_reusable_configs_set_iam_policy(...).doit().await
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
google-privateca1_beta1 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_privateca1_beta1 as privateca1_beta1;
use privateca1_beta1::{Result, Error};
use privateca1_beta1::{CertificateAuthorityService, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

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
let mut hub = CertificateAuthorityService::new(client, auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_certificate_authorities_certificate_revocation_lists_get_iam_policy("resource")
             .options_requested_policy_version(-33)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Delegate) to the
[Method Builder](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::RequestValue) and
[decodable](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-privateca1_beta1/6.0.0+20240619/google_privateca1_beta1/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **privateca1_beta1** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

