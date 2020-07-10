<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-appengine1` library allows access to all features of the *Google appengine* service.

This documentation was generated from *appengine* crate version *1.0.14+20200701*, where *20200701* is the exact revision of the *appengine:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.

Everything else about the *appengine* *v1* API can be found at the
[official documentation site](https://cloud.google.com/appengine/docs/admin-api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.Appengine.html) ... 

* apps
 * [*authorized certificates create*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppAuthorizedCertificateCreateCall.html), [*authorized certificates delete*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppAuthorizedCertificateDeleteCall.html), [*authorized certificates get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppAuthorizedCertificateGetCall.html), [*authorized certificates list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppAuthorizedCertificateListCall.html), [*authorized certificates patch*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppAuthorizedCertificatePatchCall.html), [*authorized domains list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppAuthorizedDomainListCall.html), [*create*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppCreateCall.html), [*domain mappings create*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppDomainMappingCreateCall.html), [*domain mappings delete*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppDomainMappingDeleteCall.html), [*domain mappings get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppDomainMappingGetCall.html), [*domain mappings list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppDomainMappingListCall.html), [*domain mappings patch*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppDomainMappingPatchCall.html), [*firewall ingress rules batch update*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppFirewallIngressRuleBatchUpdateCall.html), [*firewall ingress rules create*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppFirewallIngressRuleCreateCall.html), [*firewall ingress rules delete*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppFirewallIngressRuleDeleteCall.html), [*firewall ingress rules get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppFirewallIngressRuleGetCall.html), [*firewall ingress rules list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppFirewallIngressRuleListCall.html), [*firewall ingress rules patch*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppFirewallIngressRulePatchCall.html), [*get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppGetCall.html), [*locations get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppLocationGetCall.html), [*locations list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppLocationListCall.html), [*operations get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppOperationGetCall.html), [*operations list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppOperationListCall.html), [*patch*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppPatchCall.html), [*repair*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppRepairCall.html), [*services delete*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceDeleteCall.html), [*services get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceGetCall.html), [*services list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceListCall.html), [*services patch*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServicePatchCall.html), [*services versions create*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionCreateCall.html), [*services versions delete*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionDeleteCall.html), [*services versions get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionGetCall.html), [*services versions instances debug*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionInstanceDebugCall.html), [*services versions instances delete*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionInstanceDeleteCall.html), [*services versions instances get*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionInstanceGetCall.html), [*services versions instances list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionInstanceListCall.html), [*services versions list*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionListCall.html) and [*services versions patch*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.AppServiceVersionPatchCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/struct.Appengine.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.apps().services_versions_create(...).doit()
let r = hub.apps().patch(...).doit()
let r = hub.apps().services_versions_instances_debug(...).doit()
let r = hub.apps().create(...).doit()
let r = hub.apps().services_versions_instances_delete(...).doit()
let r = hub.apps().services_versions_patch(...).doit()
let r = hub.apps().domain_mappings_create(...).doit()
let r = hub.apps().domain_mappings_patch(...).doit()
let r = hub.apps().operations_get(...).doit()
let r = hub.apps().domain_mappings_delete(...).doit()
let r = hub.apps().repair(...).doit()
let r = hub.apps().services_versions_delete(...).doit()
let r = hub.apps().services_patch(...).doit()
let r = hub.apps().services_delete(...).doit()
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
# This project intentionally uses an old version of Hyper. See
# https://github.com/Byron/google-apis-rs/issues/173 for more
# information.
hyper = "^0.10"
hyper-rustls = "^0.6"
serde = "^1.0"
serde_json = "^1.0"
yup-oauth2 = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_appengine1 as appengine1;
use appengine1::DebugInstanceRequest;
use appengine1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use appengine1::Appengine;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Appengine::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = DebugInstanceRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.apps().services_versions_instances_debug(req, "appsId", "servicesId", "versionsId", "instancesId")
             .doit();

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-appengine1/1.0.14+20200701/google_appengine1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **appengine1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
