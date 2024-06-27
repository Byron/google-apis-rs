<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-appengine1` library allows access to all features of the *Google appengine* service.

This documentation was generated from *appengine* crate version *5.0.5+20240624*, where *20240624* is the exact revision of the *appengine:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *appengine* *v1* API can be found at the
[official documentation site](https://cloud.google.com/appengine/docs/admin-api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/Appengine) ... 

* apps
 * [*authorized certificates create*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppAuthorizedCertificateCreateCall), [*authorized certificates delete*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppAuthorizedCertificateDeleteCall), [*authorized certificates get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppAuthorizedCertificateGetCall), [*authorized certificates list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppAuthorizedCertificateListCall), [*authorized certificates patch*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppAuthorizedCertificatePatchCall), [*authorized domains list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppAuthorizedDomainListCall), [*create*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppCreateCall), [*domain mappings create*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppDomainMappingCreateCall), [*domain mappings delete*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppDomainMappingDeleteCall), [*domain mappings get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppDomainMappingGetCall), [*domain mappings list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppDomainMappingListCall), [*domain mappings patch*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppDomainMappingPatchCall), [*firewall ingress rules batch update*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppFirewallIngressRuleBatchUpdateCall), [*firewall ingress rules create*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppFirewallIngressRuleCreateCall), [*firewall ingress rules delete*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppFirewallIngressRuleDeleteCall), [*firewall ingress rules get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppFirewallIngressRuleGetCall), [*firewall ingress rules list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppFirewallIngressRuleListCall), [*firewall ingress rules patch*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppFirewallIngressRulePatchCall), [*get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppGetCall), [*list runtimes*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppListRuntimeCall), [*locations get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppLocationGetCall), [*locations list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppLocationListCall), [*operations get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppOperationGetCall), [*operations list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppOperationListCall), [*patch*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppPatchCall), [*repair*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppRepairCall), [*services delete*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceDeleteCall), [*services get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceGetCall), [*services list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceListCall), [*services patch*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServicePatchCall), [*services versions create*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionCreateCall), [*services versions delete*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionDeleteCall), [*services versions get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionGetCall), [*services versions instances debug*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionInstanceDebugCall), [*services versions instances delete*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionInstanceDeleteCall), [*services versions instances get*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionInstanceGetCall), [*services versions instances list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionInstanceListCall), [*services versions list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionListCall) and [*services versions patch*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::AppServiceVersionPatchCall)
* projects
 * [*locations applications authorized domains list*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/api::ProjectLocationApplicationAuthorizedDomainListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/Appengine)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::CallBuilder)
* **[Resources](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.apps().domain_mappings_create(...).doit().await
let r = hub.apps().domain_mappings_delete(...).doit().await
let r = hub.apps().domain_mappings_patch(...).doit().await
let r = hub.apps().operations_get(...).doit().await
let r = hub.apps().services_versions_instances_debug(...).doit().await
let r = hub.apps().services_versions_instances_delete(...).doit().await
let r = hub.apps().services_versions_create(...).doit().await
let r = hub.apps().services_versions_delete(...).doit().await
let r = hub.apps().services_versions_patch(...).doit().await
let r = hub.apps().services_delete(...).doit().await
let r = hub.apps().services_patch(...).doit().await
let r = hub.apps().create(...).doit().await
let r = hub.apps().patch(...).doit().await
let r = hub.apps().repair(...).doit().await
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
google-appengine1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_appengine1 as appengine1;
use appengine1::api::DebugInstanceRequest;
use appengine1::{Result, Error};
use std::default::Default;
use appengine1::{Appengine, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Appengine::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = DebugInstanceRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.apps().services_versions_instances_debug(req, "appsId", "servicesId", "versionsId", "instancesId")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::RequestValue) and 
[decodable](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-appengine1/5.0.5+20240624/google_appengine1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **appengine1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

