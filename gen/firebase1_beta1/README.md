<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-firebase1_beta1` library allows access to all features of the *Google Firebase Management* service.

This documentation was generated from *Firebase Management* crate version *5.0.5+20240624*, where *20240624* is the exact revision of the *firebase:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Firebase Management* *v1_beta1* API can be found at the
[official documentation site](https://firebase.google.com).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/FirebaseManagement) ... 

* available projects
 * [*list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::AvailableProjectListCall)
* [operations](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::Operation)
 * [*get*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::OperationGetCall)
* projects
 * [*add firebase*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAddFirebaseCall), [*add google analytics*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAddGoogleAnalyticCall), [*android apps create*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppCreateCall), [*android apps get*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppGetCall), [*android apps get config*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppGetConfigCall), [*android apps list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppListCall), [*android apps patch*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppPatchCall), [*android apps remove*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppRemoveCall), [*android apps sha create*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppShaCreateCall), [*android apps sha delete*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppShaDeleteCall), [*android apps sha list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppShaListCall), [*android apps undelete*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAndroidAppUndeleteCall), [*available locations list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectAvailableLocationListCall), [*default location finalize*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectDefaultLocationFinalizeCall), [*get*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectGetCall), [*get admin sdk config*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectGetAdminSdkConfigCall), [*get analytics details*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectGetAnalyticsDetailCall), [*ios apps create*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppCreateCall), [*ios apps get*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppGetCall), [*ios apps get config*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppGetConfigCall), [*ios apps list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppListCall), [*ios apps patch*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppPatchCall), [*ios apps remove*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppRemoveCall), [*ios apps undelete*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectIosAppUndeleteCall), [*list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectListCall), [*patch*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectPatchCall), [*remove analytics*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectRemoveAnalyticCall), [*search apps*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectSearchAppCall), [*web apps create*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppCreateCall), [*web apps get*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppGetCall), [*web apps get config*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppGetConfigCall), [*web apps list*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppListCall), [*web apps patch*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppPatchCall), [*web apps remove*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppRemoveCall) and [*web apps undelete*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/api::ProjectWebAppUndeleteCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/FirebaseManagement)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::CallBuilder)
* **[Resources](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.operations().get(...).doit().await
let r = hub.projects().android_apps_create(...).doit().await
let r = hub.projects().android_apps_remove(...).doit().await
let r = hub.projects().android_apps_undelete(...).doit().await
let r = hub.projects().default_location_finalize(...).doit().await
let r = hub.projects().ios_apps_create(...).doit().await
let r = hub.projects().ios_apps_remove(...).doit().await
let r = hub.projects().ios_apps_undelete(...).doit().await
let r = hub.projects().web_apps_create(...).doit().await
let r = hub.projects().web_apps_remove(...).doit().await
let r = hub.projects().web_apps_undelete(...).doit().await
let r = hub.projects().add_firebase(...).doit().await
let r = hub.projects().add_google_analytics(...).doit().await
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
google-firebase1_beta1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_firebase1_beta1 as firebase1_beta1;
use firebase1_beta1::api::AndroidApp;
use firebase1_beta1::{Result, Error};
use std::default::Default;
use firebase1_beta1::{FirebaseManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = FirebaseManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = AndroidApp::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().android_apps_create(req, "parent")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::RequestValue) and 
[decodable](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-firebase1_beta1/5.0.5+20240624/google_firebase1_beta1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **firebase1_beta1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

