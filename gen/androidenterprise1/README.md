<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-androidenterprise1` library allows access to all features of the *Google Android Enterprise* service.

This documentation was generated from *Android Enterprise* crate version *0.1.11+20160106*, where *20160106* is the exact revision of the *androidenterprise:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.11*.

Everything else about the *Android Enterprise* *v1* API can be found at the
[official documentation site](https://developers.google.com/android/work/play/emm-api).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.AndroidEnterprise.html) ... 

* [collections](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Collection.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionUpdateCall.html)
* collectionviewers
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionviewerDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionviewerGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionviewerListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionviewerPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.CollectionviewerUpdateCall.html)
* [devices](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Device.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.DeviceGetCall.html), [*get state*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.DeviceGetStateCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.DeviceListCall.html) and [*set state*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.DeviceSetStateCall.html)
* [enterprises](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Enterprise.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseDeleteCall.html), [*enroll*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseEnrollCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseGetCall.html), [*get store layout*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseGetStoreLayoutCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseListCall.html), [*send test push notification*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseSendTestPushNotificationCall.html), [*set account*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseSetAccountCall.html), [*set store layout*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseSetStoreLayoutCall.html) and [*unenroll*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EnterpriseUnenrollCall.html)
* [entitlements](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Entitlement.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EntitlementDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EntitlementGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EntitlementListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EntitlementPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.EntitlementUpdateCall.html)
* grouplicenses
 * [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.GrouplicenseGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.GrouplicenseListCall.html)
* grouplicenseusers
 * [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.GrouplicenseuserListCall.html)
* [installs](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Install.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.InstallDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.InstallGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.InstallListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.InstallPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.InstallUpdateCall.html)
* [permissions](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Permission.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.PermissionGetCall.html)
* [products](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.Product.html)
 * [*approve*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.ProductApproveCall.html), [*generate approval url*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.ProductGenerateApprovalUrlCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.ProductGetCall.html), [*get app restrictions schema*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.ProductGetAppRestrictionsSchemaCall.html), [*get permissions*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.ProductGetPermissionCall.html) and [*update permissions*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.ProductUpdatePermissionCall.html)
* storelayoutclusters
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutclusterDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutclusterGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutclusterInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutclusterListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutclusterPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutclusterUpdateCall.html)
* storelayoutpages
 * [*delete*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutpageDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutpageGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutpageInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutpageListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutpagePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.StorelayoutpageUpdateCall.html)
* [users](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.User.html)
 * [*generate token*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.UserGenerateTokenCall.html), [*get*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.UserGetCall.html), [*get available product set*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.UserGetAvailableProductSetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.UserListCall.html), [*revoke token*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.UserRevokeTokenCall.html) and [*set available product set*](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.UserSetAvailableProductSetCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_androidenterprise1/struct.AndroidEnterprise.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.enterprises().send_test_push_notification(...).doit()
let r = hub.enterprises().set_store_layout(...).doit()
let r = hub.enterprises().get_store_layout(...).doit()
let r = hub.enterprises().list(...).doit()
let r = hub.enterprises().unenroll(...).doit()
let r = hub.enterprises().set_account(...).doit()
let r = hub.enterprises().delete(...).doit()
let r = hub.enterprises().enroll(...).doit()
let r = hub.enterprises().insert(...).doit()
let r = hub.enterprises().get(...).doit()
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
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_androidenterprise1 as androidenterprise1;
use androidenterprise1::StoreLayout;
use androidenterprise1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use androidenterprise1::AndroidEnterprise;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::new(),
                              <MemoryStorage as Default>::default(), None);
let mut hub = AndroidEnterprise::new(hyper::Client::new(), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = StoreLayout::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.enterprises().set_store_layout(req, "enterpriseId")
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

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_androidenterprise1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_androidenterprise1/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_androidenterprise1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_androidenterprise1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **androidenterprise1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
