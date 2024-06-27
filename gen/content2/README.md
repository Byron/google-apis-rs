<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-content2` library allows access to all features of the *Google Shopping Content* service.

This documentation was generated from *Shopping Content* crate version *5.0.5+20220303*, where *20220303* is the exact revision of the *content:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Shopping Content* *v2* API can be found at the
[official documentation site](https://developers.google.com/shopping-content/v2/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-content2/5.0.5+20220303/google_content2/ShoppingContent) ... 

* [accounts](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::Account)
 * [*authinfo*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountAuthinfoCall), [*claimwebsite*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountClaimwebsiteCall), [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountCustombatchCall), [*delete*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountDeleteCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountGetCall), [*insert*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountInsertCall), [*link*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountLinkCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountListCall) and [*update*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountUpdateCall)
* accountstatuses
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountstatusCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountstatusGetCall) and [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccountstatusListCall)
* accounttax
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccounttaxCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccounttaxGetCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccounttaxListCall) and [*update*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::AccounttaxUpdateCall)
* [datafeeds](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::Datafeed)
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedCustombatchCall), [*delete*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedDeleteCall), [*fetchnow*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedFetchnowCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedGetCall), [*insert*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedInsertCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedListCall) and [*update*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedUpdateCall)
* datafeedstatuses
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedstatusCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedstatusGetCall) and [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::DatafeedstatusListCall)
* [inventory](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::Inventory)
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::InventoryCustombatchCall) and [*set*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::InventorySetCall)
* liasettings
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingGetCall), [*getaccessiblegmbaccounts*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingGetaccessiblegmbaccountCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingListCall), [*listposdataproviders*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingListposdataproviderCall), [*requestgmbaccess*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingRequestgmbaccesCall), [*requestinventoryverification*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingRequestinventoryverificationCall), [*setinventoryverificationcontact*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingSetinventoryverificationcontactCall), [*setposdataprovider*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingSetposdataproviderCall) and [*update*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::LiasettingUpdateCall)
* orderinvoices
 * [*createchargeinvoice*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderinvoiceCreatechargeinvoiceCall) and [*createrefundinvoice*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderinvoiceCreaterefundinvoiceCall)
* orderreports
 * [*listdisbursements*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderreportListdisbursementCall) and [*listtransactions*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderreportListtransactionCall)
* orderreturns
 * [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderreturnGetCall) and [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderreturnListCall)
* [orders](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::Order)
 * [*acknowledge*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderAcknowledgeCall), [*advancetestorder*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderAdvancetestorderCall), [*cancel*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderCancelCall), [*cancellineitem*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderCancellineitemCall), [*canceltestorderbycustomer*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderCanceltestorderbycustomerCall), [*createtestorder*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderCreatetestorderCall), [*createtestreturn*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderCreatetestreturnCall), [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderGetCall), [*getbymerchantorderid*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderGetbymerchantorderidCall), [*gettestordertemplate*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderGettestordertemplateCall), [*instorerefundlineitem*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderInstorerefundlineitemCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderListCall), [*refund*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderRefundCall), [*rejectreturnlineitem*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderRejectreturnlineitemCall), [*returnlineitem*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderReturnlineitemCall), [*returnrefundlineitem*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderReturnrefundlineitemCall), [*setlineitemmetadata*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderSetlineitemmetadataCall), [*shiplineitems*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderShiplineitemCall), [*updatelineitemshippingdetails*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderUpdatelineitemshippingdetailCall), [*updatemerchantorderid*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderUpdatemerchantorderidCall) and [*updateshipment*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::OrderUpdateshipmentCall)
* pos
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoCustombatchCall), [*delete*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoDeleteCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoGetCall), [*insert*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoInsertCall), [*inventory*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoInventoryCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoListCall) and [*sale*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::PoSaleCall)
* [products](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::Product)
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductCustombatchCall), [*delete*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductDeleteCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductGetCall), [*insert*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductInsertCall) and [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductListCall)
* productstatuses
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductstatusCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductstatusGetCall) and [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ProductstatusListCall)
* shippingsettings
 * [*custombatch*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingCustombatchCall), [*get*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingGetCall), [*getsupportedcarriers*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingGetsupportedcarrierCall), [*getsupportedholidays*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingGetsupportedholidayCall), [*getsupportedpickupservices*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingGetsupportedpickupserviceCall), [*list*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingListCall) and [*update*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/api::ShippingsettingUpdateCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-content2/5.0.5+20220303/google_content2/ShoppingContent)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::CallBuilder)
* **[Resources](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.orders().acknowledge(...).doit().await
let r = hub.orders().advancetestorder(...).doit().await
let r = hub.orders().cancel(...).doit().await
let r = hub.orders().cancellineitem(...).doit().await
let r = hub.orders().canceltestorderbycustomer(...).doit().await
let r = hub.orders().createtestorder(...).doit().await
let r = hub.orders().createtestreturn(...).doit().await
let r = hub.orders().custombatch(...).doit().await
let r = hub.orders().get(...).doit().await
let r = hub.orders().getbymerchantorderid(...).doit().await
let r = hub.orders().gettestordertemplate(...).doit().await
let r = hub.orders().instorerefundlineitem(...).doit().await
let r = hub.orders().list(...).doit().await
let r = hub.orders().refund(...).doit().await
let r = hub.orders().rejectreturnlineitem(...).doit().await
let r = hub.orders().returnlineitem(...).doit().await
let r = hub.orders().returnrefundlineitem(...).doit().await
let r = hub.orders().setlineitemmetadata(...).doit().await
let r = hub.orders().shiplineitems(...).doit().await
let r = hub.orders().updatelineitemshippingdetails(...).doit().await
let r = hub.orders().updatemerchantorderid(...).doit().await
let r = hub.orders().updateshipment(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_content2 as content2;
use content2::{Result, Error};
use std::default::Default;
use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.orders().list(79)
             .add_statuses("magna")
             .placed_date_start("no")
             .placed_date_end("ipsum")
             .page_token("voluptua.")
             .order_by("At")
             .max_results(93)
             .acknowledged(false)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::RequestValue) and 
[decodable](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-content2/5.0.5+20220303/google_content2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **content2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

