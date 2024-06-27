<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-firebasehosting1_beta1` library allows access to all features of the *Google Firebase Hosting* service.

This documentation was generated from *Firebase Hosting* crate version *5.0.5+20240625*, where *20240625* is the exact revision of the *firebasehosting:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Firebase Hosting* *v1_beta1* API can be found at the
[official documentation site](https://firebase.google.com/docs/hosting/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/FirebaseHosting) ... 

* projects
 * [*operations get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectOperationGetCall), [*sites channels create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelCreateCall), [*sites channels delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelDeleteCall), [*sites channels get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelGetCall), [*sites channels list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelListCall), [*sites channels patch*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelPatchCall), [*sites channels releases create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelReleaseCreateCall), [*sites channels releases get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelReleaseGetCall), [*sites channels releases list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteChannelReleaseListCall), [*sites create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCreateCall), [*sites custom domains create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainCreateCall), [*sites custom domains delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainDeleteCall), [*sites custom domains get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainGetCall), [*sites custom domains list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainListCall), [*sites custom domains operations get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainOperationGetCall), [*sites custom domains operations list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainOperationListCall), [*sites custom domains patch*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainPatchCall), [*sites custom domains undelete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteCustomDomainUndeleteCall), [*sites delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteDeleteCall), [*sites domains create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteDomainCreateCall), [*sites domains delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteDomainDeleteCall), [*sites domains get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteDomainGetCall), [*sites domains list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteDomainListCall), [*sites domains update*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteDomainUpdateCall), [*sites get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteGetCall), [*sites get config*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteGetConfigCall), [*sites list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteListCall), [*sites patch*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSitePatchCall), [*sites releases create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteReleaseCreateCall), [*sites releases get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteReleaseGetCall), [*sites releases list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteReleaseListCall), [*sites update config*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteUpdateConfigCall), [*sites versions clone*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionCloneCall), [*sites versions create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionCreateCall), [*sites versions delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionDeleteCall), [*sites versions files list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionFileListCall), [*sites versions get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionGetCall), [*sites versions list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionListCall), [*sites versions patch*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionPatchCall) and [*sites versions populate files*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::ProjectSiteVersionPopulateFileCall)
* [sites](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::Site)
 * [*channels create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelCreateCall), [*channels delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelDeleteCall), [*channels get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelGetCall), [*channels list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelListCall), [*channels patch*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelPatchCall), [*channels releases create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelReleaseCreateCall), [*channels releases get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelReleaseGetCall), [*channels releases list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteChannelReleaseListCall), [*domains create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteDomainCreateCall), [*domains delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteDomainDeleteCall), [*domains get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteDomainGetCall), [*domains list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteDomainListCall), [*domains update*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteDomainUpdateCall), [*get config*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteGetConfigCall), [*releases create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteReleaseCreateCall), [*releases get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteReleaseGetCall), [*releases list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteReleaseListCall), [*update config*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteUpdateConfigCall), [*versions clone*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionCloneCall), [*versions create*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionCreateCall), [*versions delete*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionDeleteCall), [*versions files list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionFileListCall), [*versions get*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionGetCall), [*versions list*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionListCall), [*versions patch*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionPatchCall) and [*versions populate files*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/api::SiteVersionPopulateFileCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/FirebaseHosting)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::CallBuilder)
* **[Resources](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().sites_create(...).doit().await
let r = hub.projects().sites_get(...).doit().await
let r = hub.projects().sites_patch(...).doit().await
let r = hub.sites().channels_releases_create(...).doit().await
let r = hub.sites().channels_releases_get(...).doit().await
let r = hub.sites().channels_releases_list(...).doit().await
let r = hub.sites().channels_create(...).doit().await
let r = hub.sites().channels_delete(...).doit().await
let r = hub.sites().channels_get(...).doit().await
let r = hub.sites().channels_list(...).doit().await
let r = hub.sites().channels_patch(...).doit().await
let r = hub.sites().domains_create(...).doit().await
let r = hub.sites().domains_delete(...).doit().await
let r = hub.sites().domains_get(...).doit().await
let r = hub.sites().domains_list(...).doit().await
let r = hub.sites().domains_update(...).doit().await
let r = hub.sites().releases_create(...).doit().await
let r = hub.sites().releases_get(...).doit().await
let r = hub.sites().releases_list(...).doit().await
let r = hub.sites().versions_files_list(...).doit().await
let r = hub.sites().versions_clone(...).doit().await
let r = hub.sites().versions_create(...).doit().await
let r = hub.sites().versions_delete(...).doit().await
let r = hub.sites().versions_get(...).doit().await
let r = hub.sites().versions_list(...).doit().await
let r = hub.sites().versions_patch(...).doit().await
let r = hub.sites().versions_populate_files(...).doit().await
let r = hub.sites().get_config(...).doit().await
let r = hub.sites().update_config(...).doit().await
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
google-firebasehosting1_beta1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_firebasehosting1_beta1 as firebasehosting1_beta1;
use firebasehosting1_beta1::api::Site;
use firebasehosting1_beta1::{Result, Error};
use std::default::Default;
use firebasehosting1_beta1::{FirebaseHosting, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = FirebaseHosting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Site::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().sites_create(req, "parent")
             .validate_only(true)
             .site_id("voluptua.")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::RequestValue) and 
[decodable](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-firebasehosting1_beta1/5.0.5+20240625/google_firebasehosting1_beta1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **firebasehosting1_beta1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

