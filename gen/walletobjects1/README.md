<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-walletobjects1` library allows access to all features of the *Google Walletobjects* service.

This documentation was generated from *Walletobjects* crate version *5.0.5+20240626*, where *20240626* is the exact revision of the *walletobjects:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Walletobjects* *v1* API can be found at the
[official documentation site](https://developers.google.com/pay/passes).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/Walletobjects) ... 

* eventticketclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketclasUpdateCall)
* eventticketobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectListCall), [*modifylinkedofferobjects*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectModifylinkedofferobjectCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::EventticketobjectUpdateCall)
* flightclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightclasUpdateCall)
* flightobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightobjectListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::FlightobjectUpdateCall)
* genericclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericclasUpdateCall)
* genericobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericobjectListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GenericobjectUpdateCall)
* giftcardclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardclasUpdateCall)
* giftcardobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardobjectListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::GiftcardobjectUpdateCall)
* [issuer](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::Issuer)
 * [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::IssuerGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::IssuerInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::IssuerListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::IssuerPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::IssuerUpdateCall)
* jwt
 * [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::JwtInsertCall)
* loyaltyclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyclasUpdateCall)
* loyaltyobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectListCall), [*modifylinkedofferobjects*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectModifylinkedofferobjectCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::LoyaltyobjectUpdateCall)
* [media](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::Media)
 * [*download*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::MediaDownloadCall) and [*upload*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::MediaUploadCall)
* offerclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferclasUpdateCall)
* offerobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferobjectListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::OfferobjectUpdateCall)
* [permissions](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::Permission)
 * [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::PermissionGetCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::PermissionUpdateCall)
* smarttap
 * [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::SmarttapInsertCall)
* transitclass
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitclasAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitclasGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitclasInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitclasListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitclasPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitclasUpdateCall)
* transitobject
 * [*addmessage*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitobjectAddmessageCall), [*get*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitobjectGetCall), [*insert*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitobjectInsertCall), [*list*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitobjectListCall), [*patch*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitobjectPatchCall) and [*update*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::TransitobjectUpdateCall)


Upload supported by ...

* [*upload media*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::MediaUploadCall)

Download supported by ...

* [*download media*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/api::MediaDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/Walletobjects)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::CallBuilder)
* **[Resources](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.eventticketclass().addmessage(...).doit().await
let r = hub.eventticketobject().addmessage(...).doit().await
let r = hub.flightclass().addmessage(...).doit().await
let r = hub.flightobject().addmessage(...).doit().await
let r = hub.genericclass().addmessage(...).doit().await
let r = hub.genericobject().addmessage(...).doit().await
let r = hub.giftcardclass().addmessage(...).doit().await
let r = hub.giftcardobject().addmessage(...).doit().await
let r = hub.loyaltyclass().addmessage(...).doit().await
let r = hub.loyaltyobject().addmessage(...).doit().await
let r = hub.offerclass().addmessage(...).doit().await
let r = hub.offerobject().addmessage(...).doit().await
let r = hub.transitclass().addmessage(...).doit().await
let r = hub.transitobject().addmessage(...).doit().await
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
google-walletobjects1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_walletobjects1 as walletobjects1;
use walletobjects1::api::AddMessageRequest;
use walletobjects1::{Result, Error};
use std::default::Default;
use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = AddMessageRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.eventticketclass().addmessage(req, "resourceId")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::RequestValue) and 
[decodable](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-walletobjects1/5.0.5+20240626/google_walletobjects1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **walletobjects1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

