<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-firebaseappcheck1_beta` library allows access to all features of the *Google Firebaseappcheck* service.

This documentation was generated from *Firebaseappcheck* crate version *5.0.5+20240617*, where *20240617* is the exact revision of the *firebaseappcheck:v1beta* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Firebaseappcheck* *v1_beta* API can be found at the
[official documentation site](https://firebase.google.com/docs/app-check).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/Firebaseappcheck) ... 

* jwks
 * [*get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::JwkGetCall)
* oauth clients
 * [*exchange app attest assertion*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::OauthClientExchangeAppAttestAssertionCall), [*exchange app attest attestation*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::OauthClientExchangeAppAttestAttestationCall), [*exchange debug token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::OauthClientExchangeDebugTokenCall) and [*generate app attest challenge*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::OauthClientGenerateAppAttestChallengeCall)
* projects
 * [*apps app attest config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppAppAttestConfigBatchGetCall), [*apps app attest config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppAppAttestConfigGetCall), [*apps app attest config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppAppAttestConfigPatchCall), [*apps debug tokens create*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDebugTokenCreateCall), [*apps debug tokens delete*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDebugTokenDeleteCall), [*apps debug tokens get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDebugTokenGetCall), [*apps debug tokens list*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDebugTokenListCall), [*apps debug tokens patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDebugTokenPatchCall), [*apps device check config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDeviceCheckConfigBatchGetCall), [*apps device check config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDeviceCheckConfigGetCall), [*apps device check config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppDeviceCheckConfigPatchCall), [*apps exchange app attest assertion*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeAppAttestAssertionCall), [*apps exchange app attest attestation*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeAppAttestAttestationCall), [*apps exchange custom token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeCustomTokenCall), [*apps exchange debug token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeDebugTokenCall), [*apps exchange device check token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeDeviceCheckTokenCall), [*apps exchange play integrity token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangePlayIntegrityTokenCall), [*apps exchange recaptcha enterprise token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeRecaptchaEnterpriseTokenCall), [*apps exchange recaptcha token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeRecaptchaTokenCall), [*apps exchange recaptcha v3 token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeRecaptchaV3TokenCall), [*apps exchange safety net token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppExchangeSafetyNetTokenCall), [*apps generate app attest challenge*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppGenerateAppAttestChallengeCall), [*apps generate play integrity challenge*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppGeneratePlayIntegrityChallengeCall), [*apps play integrity config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppPlayIntegrityConfigBatchGetCall), [*apps play integrity config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppPlayIntegrityConfigGetCall), [*apps play integrity config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppPlayIntegrityConfigPatchCall), [*apps recaptcha config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaConfigBatchGetCall), [*apps recaptcha config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaConfigGetCall), [*apps recaptcha config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaConfigPatchCall), [*apps recaptcha enterprise config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaEnterpriseConfigBatchGetCall), [*apps recaptcha enterprise config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaEnterpriseConfigGetCall), [*apps recaptcha enterprise config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaEnterpriseConfigPatchCall), [*apps recaptcha v3 config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaV3ConfigBatchGetCall), [*apps recaptcha v3 config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaV3ConfigGetCall), [*apps recaptcha v3 config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppRecaptchaV3ConfigPatchCall), [*apps safety net config batch get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppSafetyNetConfigBatchGetCall), [*apps safety net config get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppSafetyNetConfigGetCall), [*apps safety net config patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectAppSafetyNetConfigPatchCall), [*services batch update*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceBatchUpdateCall), [*services get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceGetCall), [*services list*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceListCall), [*services patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServicePatchCall), [*services resource policies batch update*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceResourcePolicyBatchUpdateCall), [*services resource policies create*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceResourcePolicyCreateCall), [*services resource policies delete*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceResourcePolicyDeleteCall), [*services resource policies get*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceResourcePolicyGetCall), [*services resource policies list*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceResourcePolicyListCall), [*services resource policies patch*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectServiceResourcePolicyPatchCall) and [*verify app check token*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/api::ProjectVerifyAppCheckTokenCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/Firebaseappcheck)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::CallBuilder)
* **[Resources](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.oauth_clients().exchange_app_attest_assertion(...).doit().await
let r = hub.oauth_clients().exchange_debug_token(...).doit().await
let r = hub.projects().apps_exchange_app_attest_assertion(...).doit().await
let r = hub.projects().apps_exchange_custom_token(...).doit().await
let r = hub.projects().apps_exchange_debug_token(...).doit().await
let r = hub.projects().apps_exchange_device_check_token(...).doit().await
let r = hub.projects().apps_exchange_play_integrity_token(...).doit().await
let r = hub.projects().apps_exchange_recaptcha_enterprise_token(...).doit().await
let r = hub.projects().apps_exchange_recaptcha_token(...).doit().await
let r = hub.projects().apps_exchange_recaptcha_v3_token(...).doit().await
let r = hub.projects().apps_exchange_safety_net_token(...).doit().await
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
google-firebaseappcheck1_beta = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_firebaseappcheck1_beta as firebaseappcheck1_beta;
use firebaseappcheck1_beta::api::GoogleFirebaseAppcheckV1betaExchangeAppAttestAssertionRequest;
use firebaseappcheck1_beta::{Result, Error};
use std::default::Default;
use firebaseappcheck1_beta::{Firebaseappcheck, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Firebaseappcheck::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleFirebaseAppcheckV1betaExchangeAppAttestAssertionRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.oauth_clients().exchange_app_attest_assertion(req, "app")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Delegate) to the 
[Method Builder](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::RequestValue) and 
[decodable](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-firebaseappcheck1_beta/5.0.5+20240617/google_firebaseappcheck1_beta/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **firebaseappcheck1_beta** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

