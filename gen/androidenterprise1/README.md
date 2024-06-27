<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-androidenterprise1` library allows access to all features of the *Google Android Enterprise* service.

This documentation was generated from *Android Enterprise* crate version *5.0.5+20240625*, where *20240625* is the exact revision of the *androidenterprise:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Android Enterprise* *v1* API can be found at the
[official documentation site](https://developers.google.com/android/work/play/emm-api).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/AndroidEnterprise) ... 

* [devices](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::Device)
 * [*force report upload*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::DeviceForceReportUploadCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::DeviceGetCall), [*get state*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::DeviceGetStateCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::DeviceListCall), [*set state*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::DeviceSetStateCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::DeviceUpdateCall)
* [enterprises](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::Enterprise)
 * [*acknowledge notification set*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseAcknowledgeNotificationSetCall), [*complete signup*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseCompleteSignupCall), [*create enrollment token*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseCreateEnrollmentTokenCall), [*create web token*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseCreateWebTokenCall), [*enroll*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseEnrollCall), [*generate signup url*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseGenerateSignupUrlCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseGetCall), [*get service account*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseGetServiceAccountCall), [*get store layout*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseGetStoreLayoutCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseListCall), [*pull notification set*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterprisePullNotificationSetCall), [*send test push notification*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseSendTestPushNotificationCall), [*set account*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseSetAccountCall), [*set store layout*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseSetStoreLayoutCall) and [*unenroll*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EnterpriseUnenrollCall)
* [entitlements](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::Entitlement)
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EntitlementDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EntitlementGetCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EntitlementListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::EntitlementUpdateCall)
* grouplicenses
 * [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::GrouplicenseGetCall) and [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::GrouplicenseListCall)
* grouplicenseusers
 * [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::GrouplicenseuserListCall)
* [installs](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::Install)
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::InstallDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::InstallGetCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::InstallListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::InstallUpdateCall)
* managedconfigurationsfordevice
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsfordeviceDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsfordeviceGetCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsfordeviceListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsfordeviceUpdateCall)
* managedconfigurationsforuser
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsforuserDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsforuserGetCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsforuserListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationsforuserUpdateCall)
* managedconfigurationssettings
 * [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ManagedconfigurationssettingListCall)
* [permissions](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::Permission)
 * [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::PermissionGetCall)
* [products](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::Product)
 * [*approve*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductApproveCall), [*generate approval url*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductGenerateApprovalUrlCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductGetCall), [*get app restrictions schema*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductGetAppRestrictionsSchemaCall), [*get permissions*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductGetPermissionCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductListCall) and [*unapprove*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ProductUnapproveCall)
* serviceaccountkeys
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ServiceaccountkeyDeleteCall), [*insert*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ServiceaccountkeyInsertCall) and [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::ServiceaccountkeyListCall)
* storelayoutclusters
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutclusterDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutclusterGetCall), [*insert*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutclusterInsertCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutclusterListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutclusterUpdateCall)
* storelayoutpages
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutpageDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutpageGetCall), [*insert*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutpageInsertCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutpageListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::StorelayoutpageUpdateCall)
* [users](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::User)
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserDeleteCall), [*generate authentication token*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserGenerateAuthenticationTokenCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserGetCall), [*get available product set*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserGetAvailableProductSetCall), [*insert*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserInsertCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserListCall), [*revoke device access*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserRevokeDeviceAccesCall), [*set available product set*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserSetAvailableProductSetCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::UserUpdateCall)
* webapps
 * [*delete*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::WebappDeleteCall), [*get*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::WebappGetCall), [*insert*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::WebappInsertCall), [*list*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::WebappListCall) and [*update*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/api::WebappUpdateCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/AndroidEnterprise)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::CallBuilder)
* **[Resources](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.enterprises().acknowledge_notification_set(...).doit().await
let r = hub.enterprises().complete_signup(...).doit().await
let r = hub.enterprises().create_enrollment_token(...).doit().await
let r = hub.enterprises().create_web_token(...).doit().await
let r = hub.enterprises().enroll(...).doit().await
let r = hub.enterprises().generate_signup_url(...).doit().await
let r = hub.enterprises().get(...).doit().await
let r = hub.enterprises().get_service_account(...).doit().await
let r = hub.enterprises().get_store_layout(...).doit().await
let r = hub.enterprises().list(...).doit().await
let r = hub.enterprises().pull_notification_set(...).doit().await
let r = hub.enterprises().send_test_push_notification(...).doit().await
let r = hub.enterprises().set_account(...).doit().await
let r = hub.enterprises().set_store_layout(...).doit().await
let r = hub.enterprises().unenroll(...).doit().await
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
google-androidenterprise1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_androidenterprise1 as androidenterprise1;
use androidenterprise1::{Result, Error};
use std::default::Default;
use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.enterprises().complete_signup()
             .enterprise_token("et")
             .completion_token("magna")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::RequestValue) and 
[decodable](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-androidenterprise1/5.0.5+20240625/google_androidenterprise1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **androidenterprise1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

