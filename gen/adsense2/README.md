<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-adsense2` library allows access to all features of the *Google adsense* service.

This documentation was generated from *adsense* crate version *6.0.0+20240626*, where *20240626* is the exact revision of the *adsense:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.

Everything else about the *adsense* *v2* API can be found at the
[official documentation site](https://developers.google.com/adsense/management/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/Adsense) ...

* [accounts](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::Account)
 * [*adclients adunits create*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientAdunitCreateCall), [*adclients adunits get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientAdunitGetCall), [*adclients adunits get adcode*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientAdunitGetAdcodeCall), [*adclients adunits list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientAdunitListCall), [*adclients adunits list linked custom channels*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientAdunitListLinkedCustomChannelCall), [*adclients adunits patch*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientAdunitPatchCall), [*adclients customchannels create*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientCustomchannelCreateCall), [*adclients customchannels delete*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientCustomchannelDeleteCall), [*adclients customchannels get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientCustomchannelGetCall), [*adclients customchannels list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientCustomchannelListCall), [*adclients customchannels list linked ad units*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientCustomchannelListLinkedAdUnitCall), [*adclients customchannels patch*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientCustomchannelPatchCall), [*adclients get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientGetCall), [*adclients get adcode*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientGetAdcodeCall), [*adclients list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientListCall), [*adclients urlchannels get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientUrlchannelGetCall), [*adclients urlchannels list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAdclientUrlchannelListCall), [*alerts list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountAlertListCall), [*get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountGetCall), [*get ad blocking recovery tag*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountGetAdBlockingRecoveryTagCall), [*list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountListCall), [*list child accounts*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountListChildAccountCall), [*payments list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountPaymentListCall), [*policy issues get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountPolicyIssueGetCall), [*policy issues list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountPolicyIssueListCall), [*reports generate*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountReportGenerateCall), [*reports generate csv*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountReportGenerateCsvCall), [*reports get saved*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountReportGetSavedCall), [*reports saved generate*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountReportSavedGenerateCall), [*reports saved generate csv*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountReportSavedGenerateCsvCall), [*reports saved list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountReportSavedListCall), [*sites get*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountSiteGetCall) and [*sites list*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/api::AccountSiteListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/Adsense)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::CallBuilder)
* **[Resources](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().adclients_adunits_create(...).doit().await
let r = hub.accounts().adclients_adunits_get(...).doit().await
let r = hub.accounts().adclients_adunits_get_adcode(...).doit().await
let r = hub.accounts().adclients_adunits_list(...).doit().await
let r = hub.accounts().adclients_adunits_list_linked_custom_channels(...).doit().await
let r = hub.accounts().adclients_adunits_patch(...).doit().await
let r = hub.accounts().adclients_customchannels_create(...).doit().await
let r = hub.accounts().adclients_customchannels_delete(...).doit().await
let r = hub.accounts().adclients_customchannels_get(...).doit().await
let r = hub.accounts().adclients_customchannels_list(...).doit().await
let r = hub.accounts().adclients_customchannels_list_linked_ad_units(...).doit().await
let r = hub.accounts().adclients_customchannels_patch(...).doit().await
let r = hub.accounts().adclients_urlchannels_get(...).doit().await
let r = hub.accounts().adclients_urlchannels_list(...).doit().await
let r = hub.accounts().adclients_get(...).doit().await
let r = hub.accounts().adclients_get_adcode(...).doit().await
let r = hub.accounts().adclients_list(...).doit().await
let r = hub.accounts().alerts_list(...).doit().await
let r = hub.accounts().payments_list(...).doit().await
let r = hub.accounts().policy_issues_get(...).doit().await
let r = hub.accounts().policy_issues_list(...).doit().await
let r = hub.accounts().reports_saved_generate(...).doit().await
let r = hub.accounts().reports_saved_generate_csv(...).doit().await
let r = hub.accounts().reports_saved_list(...).doit().await
let r = hub.accounts().reports_generate(...).doit().await
let r = hub.accounts().reports_generate_csv(...).doit().await
let r = hub.accounts().reports_get_saved(...).doit().await
let r = hub.accounts().sites_get(...).doit().await
let r = hub.accounts().sites_list(...).doit().await
let r = hub.accounts().get(...).doit().await
let r = hub.accounts().get_ad_blocking_recovery_tag(...).doit().await
let r = hub.accounts().list(...).doit().await
let r = hub.accounts().list_child_accounts(...).doit().await
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
google-adsense2 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_adsense2 as adsense2;
use adsense2::{Result, Error};
use adsense2::{Adsense, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build()
);
let mut hub = Adsense::new(client, auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().reports_generate("account")
             .start_date_year(-33)
             .start_date_month(-11)
             .start_date_day(-55)
             .reporting_time_zone("voluptua.")
             .add_order_by("At")
             .add_metrics("sanctus")
             .limit(-80)
             .language_code("amet.")
             .add_filters("takimata")
             .end_date_year(-52)
             .end_date_month(-20)
             .end_date_day(-55)
             .add_dimensions("gubergren")
             .date_range("Lorem")
             .currency_code("gubergren")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Delegate) to the
[Method Builder](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::RequestValue) and
[decodable](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-adsense2/6.0.0+20240626/google_adsense2/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **adsense2** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

