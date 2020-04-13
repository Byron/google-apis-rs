<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-reseller1_sandbox` library allows access to all features of the *Google reseller* service.

This documentation was generated from *reseller* crate version *1.0.13+20160329*, where *20160329* is the exact revision of the *reseller:v1sandbox* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *reseller* *v1_sandbox* API can be found at the
[official documentation site](https://developers.google.com/google-apps/reseller/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.Reseller.html) ... 

* [customers](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.Customer.html)
 * [*get*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.CustomerGetCall.html), [*insert*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.CustomerInsertCall.html), [*patch*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.CustomerPatchCall.html) and [*update*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.CustomerUpdateCall.html)
* [subscriptions](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.Subscription.html)
 * [*activate*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionActivateCall.html), [*change plan*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionChangePlanCall.html), [*change renewal settings*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionChangeRenewalSettingCall.html), [*change seats*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionChangeSeatCall.html), [*delete*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionDeleteCall.html), [*get*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionGetCall.html), [*insert*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionInsertCall.html), [*list*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionListCall.html), [*start paid service*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionStartPaidServiceCall.html) and [*suspend*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.SubscriptionSuspendCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/struct.Reseller.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.subscriptions().insert(...).doit()
let r = hub.subscriptions().delete(...).doit()
let r = hub.subscriptions().get(...).doit()
let r = hub.subscriptions().list(...).doit()
let r = hub.subscriptions().start_paid_service(...).doit()
let r = hub.subscriptions().change_seats(...).doit()
let r = hub.subscriptions().suspend(...).doit()
let r = hub.subscriptions().activate(...).doit()
let r = hub.subscriptions().change_renewal_settings(...).doit()
let r = hub.subscriptions().change_plan(...).doit()
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
google-reseller1_sandbox = "*"
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
extern crate google_reseller1_sandbox as reseller1_sandbox;
use reseller1_sandbox::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use reseller1_sandbox::Reseller;

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
let mut hub = Reseller::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.subscriptions().list()
             .page_token("eirmod")
             .max_results(53)
             .customer_name_prefix("Stet")
             .customer_id("sed")
             .customer_auth_token("et")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-reseller1_sandbox/1.0.13+20160329/google_reseller1_sandbox/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **reseller1_sandbox** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
