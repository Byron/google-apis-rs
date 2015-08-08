<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-adexchangeseller2` library allows access to all features of the *Google Ad Exchange Seller* service.

This documentation was generated from *Ad Exchange Seller* crate version *0.1.9+20150719*, where *20150719* is the exact revision of the *adexchangeseller:v2.0* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.9*.

Everything else about the *Ad Exchange Seller* *v2* API can be found at the
[official documentation site](https://developers.google.com/ad-exchange/seller-rest/).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AdExchangeSeller.html) ... 

* [accounts](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.Account.html)
 * [*adclients list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountAdclientListCall.html), [*alerts list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountAlertListCall.html), [*customchannels get*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountCustomchannelGetCall.html), [*customchannels list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountCustomchannelListCall.html), [*get*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountListCall.html), [*metadata dimensions list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountMetadataDimensionListCall.html), [*metadata metrics list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountMetadataMetricListCall.html), [*preferreddeals get*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountPreferreddealGetCall.html), [*preferreddeals list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountPreferreddealListCall.html), [*reports generate*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountReportGenerateCall.html), [*reports saved generate*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountReportSavedGenerateCall.html), [*reports saved list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountReportSavedListCall.html) and [*urlchannels list*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountUrlchannelListCall.html)


Download supported by ...

* [*reports generate accounts*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AccountReportGenerateCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_adexchangeseller2/struct.AdExchangeSeller.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.accounts().reports_saved_generate(...).doit()
let r = hub.accounts().metadata_metrics_list(...).doit()
let r = hub.accounts().reports_generate(...).doit()
let r = hub.accounts().urlchannels_list(...).doit()
let r = hub.accounts().reports_saved_list(...).doit()
let r = hub.accounts().preferreddeals_get(...).doit()
let r = hub.accounts().adclients_list(...).doit()
let r = hub.accounts().metadata_dimensions_list(...).doit()
let r = hub.accounts().customchannels_get(...).doit()
let r = hub.accounts().customchannels_list(...).doit()
let r = hub.accounts().list(...).doit()
let r = hub.accounts().alerts_list(...).doit()
let r = hub.accounts().preferreddeals_list(...).doit()
let r = hub.accounts().get(...).doit()
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
google-adexchangeseller2 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_adexchangeseller2 as adexchangeseller2;
use adexchangeseller2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use adexchangeseller2::AdExchangeSeller;

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
let mut hub = AdExchangeSeller::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().reports_generate("accountId", "startDate", "endDate")
             .start_index(59)
             .add_sort("et")
             .add_metric("dolores")
             .max_results(38)
             .locale("accusam")
             .add_filter("takimata")
             .add_dimension("justo")
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

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_adexchangeseller2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_adexchangeseller2/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_adexchangeseller2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_adexchangeseller2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **adexchangeseller2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
