<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-adsense1d4` library allows access to all features of the *Google AdSense* service.

This documentation was generated from *AdSense* crate version *1.0.14+20200708*, where *20200708* is the exact revision of the *adsense:v1.4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.

Everything else about the *AdSense* *v1d4* API can be found at the
[official documentation site](https://developers.google.com/adsense/management/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdSense.html) ... 

* [accounts](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.Account.html)
 * [*adclients get ad code*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAdclientGetAdCodeCall.html), [*adclients list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAdclientListCall.html), [*adunits customchannels list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAdunitCustomchannelListCall.html), [*adunits get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAdunitGetCall.html), [*adunits get ad code*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAdunitGetAdCodeCall.html), [*adunits list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAdunitListCall.html), [*alerts delete*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAlertDeleteCall.html), [*alerts list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountAlertListCall.html), [*customchannels adunits list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountCustomchannelAdunitListCall.html), [*customchannels get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountCustomchannelGetCall.html), [*customchannels list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountCustomchannelListCall.html), [*get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountGetCall.html), [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountListCall.html), [*payments list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountPaymentListCall.html), [*reports generate*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountReportGenerateCall.html), [*reports saved generate*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountReportSavedGenerateCall.html), [*reports saved list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountReportSavedListCall.html), [*savedadstyles get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountSavedadstyleGetCall.html), [*savedadstyles list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountSavedadstyleListCall.html) and [*urlchannels list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountUrlchannelListCall.html)
* adclients
 * [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdclientListCall.html)
* adunits
 * [*customchannels list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdunitCustomchannelListCall.html), [*get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdunitGetCall.html), [*get ad code*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdunitGetAdCodeCall.html) and [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdunitListCall.html)
* [alerts](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.Alert.html)
 * [*delete*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AlertDeleteCall.html) and [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AlertListCall.html)
* customchannels
 * [*adunits list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.CustomchannelAdunitListCall.html), [*get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.CustomchannelGetCall.html) and [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.CustomchannelListCall.html)
* [metadata](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.Metadata.html)
 * [*dimensions list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.MetadataDimensionListCall.html) and [*metrics list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.MetadataMetricListCall.html)
* [payments](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.Payment.html)
 * [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.PaymentListCall.html)
* reports
 * [*generate*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.ReportGenerateCall.html), [*saved generate*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.ReportSavedGenerateCall.html) and [*saved list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.ReportSavedListCall.html)
* savedadstyles
 * [*get*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.SavedadstyleGetCall.html) and [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.SavedadstyleListCall.html)
* urlchannels
 * [*list*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.UrlchannelListCall.html)


Download supported by ...

* [*generate reports*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.ReportGenerateCall.html)
* [*reports generate accounts*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AccountReportGenerateCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/struct.AdSense.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.accounts().adunits_list(...).doit()
let r = hub.accounts().customchannels_list(...).doit()
let r = hub.accounts().adunits_customchannels_list(...).doit()
let r = hub.accounts().alerts_delete(...).doit()
let r = hub.accounts().savedadstyles_get(...).doit()
let r = hub.accounts().reports_generate(...).doit()
let r = hub.accounts().list(...).doit()
let r = hub.accounts().adclients_list(...).doit()
let r = hub.accounts().reports_saved_generate(...).doit()
let r = hub.accounts().adunits_get(...).doit()
let r = hub.accounts().alerts_list(...).doit()
let r = hub.accounts().savedadstyles_list(...).doit()
let r = hub.accounts().customchannels_get(...).doit()
let r = hub.accounts().get(...).doit()
let r = hub.accounts().customchannels_adunits_list(...).doit()
let r = hub.accounts().payments_list(...).doit()
let r = hub.accounts().adunits_get_ad_code(...).doit()
let r = hub.accounts().adclients_get_ad_code(...).doit()
let r = hub.accounts().reports_saved_list(...).doit()
let r = hub.accounts().urlchannels_list(...).doit()
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
extern crate google_adsense1d4 as adsense1d4;
use adsense1d4::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use adsense1d4::AdSense;

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
let mut hub = AdSense::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().reports_generate("accountId", "startDate", "endDate")
             .use_timezone_reporting(true)
             .start_index(-85)
             .add_sort("dolores")
             .add_metric("kasd")
             .max_results(-22)
             .locale("takimata")
             .add_filter("justo")
             .add_dimension("amet.")
             .currency("erat")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-adsense1d4/1.0.14+20200708/google_adsense1d4/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **adsense1d4** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
