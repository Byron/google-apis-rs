<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-adsense1d4` library allows access to all features of the *Google AdSense* service.

This documentation was generated from *AdSense* crate version *5.0.5+20201002*, where *20201002* is the exact revision of the *adsense:v1.4* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *AdSense* *v1d4* API can be found at the
[official documentation site](https://developers.google.com/adsense/management/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/AdSense) ... 

* [accounts](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::Account)
 * [*adclients get ad code*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAdclientGetAdCodeCall), [*adclients list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAdclientListCall), [*adunits customchannels list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAdunitCustomchannelListCall), [*adunits get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAdunitGetCall), [*adunits get ad code*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAdunitGetAdCodeCall), [*adunits list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAdunitListCall), [*alerts delete*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAlertDeleteCall), [*alerts list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountAlertListCall), [*customchannels adunits list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountCustomchannelAdunitListCall), [*customchannels get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountCustomchannelGetCall), [*customchannels list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountCustomchannelListCall), [*get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountGetCall), [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountListCall), [*payments list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountPaymentListCall), [*reports generate*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountReportGenerateCall), [*reports saved generate*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountReportSavedGenerateCall), [*reports saved list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountReportSavedListCall), [*savedadstyles get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountSavedadstyleGetCall), [*savedadstyles list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountSavedadstyleListCall) and [*urlchannels list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountUrlchannelListCall)
* adclients
 * [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AdclientListCall)
* adunits
 * [*customchannels list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AdunitCustomchannelListCall), [*get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AdunitGetCall), [*get ad code*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AdunitGetAdCodeCall) and [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AdunitListCall)
* [alerts](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::Alert)
 * [*delete*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AlertDeleteCall) and [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AlertListCall)
* customchannels
 * [*adunits list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::CustomchannelAdunitListCall), [*get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::CustomchannelGetCall) and [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::CustomchannelListCall)
* [metadata](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::Metadata)
 * [*dimensions list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::MetadataDimensionListCall) and [*metrics list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::MetadataMetricListCall)
* [payments](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::Payment)
 * [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::PaymentListCall)
* reports
 * [*generate*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::ReportGenerateCall), [*saved generate*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::ReportSavedGenerateCall) and [*saved list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::ReportSavedListCall)
* savedadstyles
 * [*get*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::SavedadstyleGetCall) and [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::SavedadstyleListCall)
* urlchannels
 * [*list*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::UrlchannelListCall)


Download supported by ...

* [*reports generate accounts*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::AccountReportGenerateCall)
* [*generate reports*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/api::ReportGenerateCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/AdSense)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::CallBuilder)
* **[Resources](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().adclients_get_ad_code(...).doit().await
let r = hub.accounts().adclients_list(...).doit().await
let r = hub.accounts().adunits_customchannels_list(...).doit().await
let r = hub.accounts().adunits_get(...).doit().await
let r = hub.accounts().adunits_get_ad_code(...).doit().await
let r = hub.accounts().adunits_list(...).doit().await
let r = hub.accounts().alerts_delete(...).doit().await
let r = hub.accounts().alerts_list(...).doit().await
let r = hub.accounts().customchannels_adunits_list(...).doit().await
let r = hub.accounts().customchannels_get(...).doit().await
let r = hub.accounts().customchannels_list(...).doit().await
let r = hub.accounts().payments_list(...).doit().await
let r = hub.accounts().reports_saved_generate(...).doit().await
let r = hub.accounts().reports_saved_list(...).doit().await
let r = hub.accounts().reports_generate(...).doit().await
let r = hub.accounts().savedadstyles_get(...).doit().await
let r = hub.accounts().savedadstyles_list(...).doit().await
let r = hub.accounts().urlchannels_list(...).doit().await
let r = hub.accounts().get(...).doit().await
let r = hub.accounts().list(...).doit().await
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
google-adsense1d4 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_adsense1d4 as adsense1d4;
use adsense1d4::{Result, Error};
use std::default::Default;
use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().reports_generate("accountId", "startDate", "endDate")
             .use_timezone_reporting(true)
             .start_index(-28)
             .add_sort("At")
             .add_metric("sanctus")
             .max_results(-80)
             .locale("amet.")
             .add_filter("takimata")
             .add_dimension("amet.")
             .currency("duo")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Delegate) to the 
[Method Builder](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::RequestValue) and 
[decodable](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-adsense1d4/5.0.5+20201002/google_adsense1d4/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **adsense1d4** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

