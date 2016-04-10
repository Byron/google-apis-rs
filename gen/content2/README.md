<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-content2` library allows access to all features of the *Google Shopping Content* service.

This documentation was generated from *Shopping Content* crate version *0.1.13+20160331*, where *20160331* is the exact revision of the *content:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.13*.

Everything else about the *Shopping Content* *v2* API can be found at the
[official documentation site](https://developers.google.com/shopping-content).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_content2/struct.ShoppingContent.html) ... 

* [accounts](http://byron.github.io/google-apis-rs/google_content2/struct.Account.html)
 * [*authinfo*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountAuthinfoCall.html), [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountCustombatchCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountUpdateCall.html)
* accountshipping
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountshippingCustombatchCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountshippingGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountshippingListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountshippingPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountshippingUpdateCall.html)
* accountstatuses
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountstatuseCustombatchCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountstatuseGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.AccountstatuseListCall.html)
* accounttax
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccounttaxCustombatchCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.AccounttaxGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.AccounttaxListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_content2/struct.AccounttaxPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_content2/struct.AccounttaxUpdateCall.html)
* [datafeeds](http://byron.github.io/google-apis-rs/google_content2/struct.Datafeed.html)
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedCustombatchCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedUpdateCall.html)
* datafeedstatuses
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedstatuseCustombatchCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedstatuseGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.DatafeedstatuseListCall.html)
* [inventory](http://byron.github.io/google-apis-rs/google_content2/struct.Inventory.html)
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.InventoryCustombatchCall.html) and [*set*](http://byron.github.io/google-apis-rs/google_content2/struct.InventorySetCall.html)
* [orders](http://byron.github.io/google-apis-rs/google_content2/struct.Order.html)
 * [*acknowledge*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderAcknowledgeCall.html), [*advancetestorder*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderAdvancetestorderCall.html), [*cancel*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderCancelCall.html), [*cancellineitem*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderCancellineitemCall.html), [*createtestorder*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderCreatetestorderCall.html), [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderCustombatchCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderGetCall.html), [*getbymerchantorderid*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderGetbymerchantorderidCall.html), [*gettestordertemplate*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderGettestordertemplateCall.html), [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderListCall.html), [*refund*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderRefundCall.html), [*returnlineitem*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderReturnlineitemCall.html), [*shiplineitems*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderShiplineitemCall.html), [*updatemerchantorderid*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderUpdatemerchantorderidCall.html) and [*updateshipment*](http://byron.github.io/google-apis-rs/google_content2/struct.OrderUpdateshipmentCall.html)
* [products](http://byron.github.io/google-apis-rs/google_content2/struct.Product.html)
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductCustombatchCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductListCall.html)
* productstatuses
 * [*custombatch*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductstatuseCustombatchCall.html), [*get*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductstatuseGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_content2/struct.ProductstatuseListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_content2/struct.ShoppingContent.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_content2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_content2/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_content2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_content2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_content2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.orders().get(...).doit()
let r = hub.orders().list(...).doit()
let r = hub.orders().updateshipment(...).doit()
let r = hub.orders().advancetestorder(...).doit()
let r = hub.orders().updatemerchantorderid(...).doit()
let r = hub.orders().returnlineitem(...).doit()
let r = hub.orders().gettestordertemplate(...).doit()
let r = hub.orders().createtestorder(...).doit()
let r = hub.orders().refund(...).doit()
let r = hub.orders().custombatch(...).doit()
let r = hub.orders().cancellineitem(...).doit()
let r = hub.orders().getbymerchantorderid(...).doit()
let r = hub.orders().acknowledge(...).doit()
let r = hub.orders().cancel(...).doit()
let r = hub.orders().shiplineitems(...).doit()
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
google-content2 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_content2 as content2;
use content2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use content2::ShoppingContent;

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
let mut hub = ShoppingContent::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.orders().list("merchantId")
             .add_statuses("sit")
             .placed_date_start("Stet")
             .placed_date_end("sed")
             .page_token("et")
             .order_by("dolores")
             .max_results(38)
             .acknowledged(true)
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

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_content2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_content2/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_content2/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_content2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_content2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_content2/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_content2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_content2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_content2/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_content2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_content2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_content2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_content2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **content2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
