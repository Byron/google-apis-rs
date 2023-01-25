<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-analyticsadmin1_alpha` library allows access to all features of the *Google Google Analytics Admin* service.

This documentation was generated from *Google Analytics Admin* crate version *5.0.2-beta-1+20220307*, where *20220307* is the exact revision of the *analyticsadmin:v1alpha* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.2-beta-1*.

Everything else about the *Google Analytics Admin* *v1_alpha* API can be found at the
[official documentation site](http://code.google.com/apis/analytics/docs/mgmt/home.html).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/GoogleAnalyticsAdmin) ... 

* account summaries
 * [*list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountSummaryListCall)
* accounts
 * [*delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountDeleteCall), [*get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountGetCall), [*get data sharing settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountGetDataSharingSettingCall), [*list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountListCall), [*patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountPatchCall), [*provision account ticket*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountProvisionAccountTicketCall), [*search change history events*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountSearchChangeHistoryEventCall), [*user links audit*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkAuditCall), [*user links batch create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkBatchCreateCall), [*user links batch delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkBatchDeleteCall), [*user links batch get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkBatchGetCall), [*user links batch update*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkBatchUpdateCall), [*user links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkCreateCall), [*user links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkDeleteCall), [*user links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkGetCall), [*user links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkListCall) and [*user links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::AccountUserLinkPatchCall)
* properties
 * [*acknowledge user data collection*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyAcknowledgeUserDataCollectionCall), [*conversion events create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyConversionEventCreateCall), [*conversion events delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyConversionEventDeleteCall), [*conversion events get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyConversionEventGetCall), [*conversion events list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyConversionEventListCall), [*create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCreateCall), [*custom dimensions archive*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomDimensionArchiveCall), [*custom dimensions create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomDimensionCreateCall), [*custom dimensions get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomDimensionGetCall), [*custom dimensions list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomDimensionListCall), [*custom dimensions patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomDimensionPatchCall), [*custom metrics archive*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomMetricArchiveCall), [*custom metrics create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomMetricCreateCall), [*custom metrics get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomMetricGetCall), [*custom metrics list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomMetricListCall), [*custom metrics patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyCustomMetricPatchCall), [*data streams create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamCreateCall), [*data streams delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamDeleteCall), [*data streams get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamGetCall), [*data streams get global site tag*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamGetGlobalSiteTagCall), [*data streams list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamListCall), [*data streams measurement protocol secrets create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretCreateCall), [*data streams measurement protocol secrets delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretDeleteCall), [*data streams measurement protocol secrets get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretGetCall), [*data streams measurement protocol secrets list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretListCall), [*data streams measurement protocol secrets patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamMeasurementProtocolSecretPatchCall), [*data streams patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDataStreamPatchCall), [*delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDeleteCall), [*display video360 advertiser link proposals approve*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalApproveCall), [*display video360 advertiser link proposals cancel*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalCancelCall), [*display video360 advertiser link proposals create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalCreateCall), [*display video360 advertiser link proposals delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalDeleteCall), [*display video360 advertiser link proposals get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalGetCall), [*display video360 advertiser link proposals list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkProposalListCall), [*display video360 advertiser links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkCreateCall), [*display video360 advertiser links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkDeleteCall), [*display video360 advertiser links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkGetCall), [*display video360 advertiser links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkListCall), [*display video360 advertiser links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyDisplayVideo360AdvertiserLinkPatchCall), [*firebase links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyFirebaseLinkCreateCall), [*firebase links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyFirebaseLinkDeleteCall), [*firebase links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyFirebaseLinkListCall), [*get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGetCall), [*get data retention settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGetDataRetentionSettingCall), [*get google signals settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGetGoogleSignalsSettingCall), [*google ads links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkCreateCall), [*google ads links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkDeleteCall), [*google ads links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkListCall), [*google ads links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyGoogleAdsLinkPatchCall), [*list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyListCall), [*patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyPatchCall), [*update data retention settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUpdateDataRetentionSettingCall), [*update google signals settings*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUpdateGoogleSignalsSettingCall), [*user links audit*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkAuditCall), [*user links batch create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkBatchCreateCall), [*user links batch delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkBatchDeleteCall), [*user links batch get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkBatchGetCall), [*user links batch update*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkBatchUpdateCall), [*user links create*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkCreateCall), [*user links delete*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkDeleteCall), [*user links get*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkGetCall), [*user links list*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkListCall) and [*user links patch*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/api::PropertyUserLinkPatchCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/GoogleAnalyticsAdmin)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::CallBuilder)
* **[Resources](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().user_links_batch_delete(...).doit().await
let r = hub.accounts().user_links_delete(...).doit().await
let r = hub.accounts().delete(...).doit().await
let r = hub.properties().conversion_events_delete(...).doit().await
let r = hub.properties().custom_dimensions_archive(...).doit().await
let r = hub.properties().custom_metrics_archive(...).doit().await
let r = hub.properties().data_streams_measurement_protocol_secrets_delete(...).doit().await
let r = hub.properties().data_streams_delete(...).doit().await
let r = hub.properties().display_video360_advertiser_link_proposals_delete(...).doit().await
let r = hub.properties().display_video360_advertiser_links_delete(...).doit().await
let r = hub.properties().firebase_links_delete(...).doit().await
let r = hub.properties().google_ads_links_delete(...).doit().await
let r = hub.properties().user_links_batch_delete(...).doit().await
let r = hub.properties().user_links_delete(...).doit().await
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
google-analyticsadmin1_alpha = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_analyticsadmin1_alpha as analyticsadmin1_alpha;
use analyticsadmin1_alpha::api::GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest;
use analyticsadmin1_alpha::{Result, Error};
use std::default::Default;
use analyticsadmin1_alpha::{GoogleAnalyticsAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = GoogleAnalyticsAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().user_links_batch_delete(req, "parent")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Delegate) to the 
[Method Builder](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::RequestValue) and 
[decodable](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-analyticsadmin1_alpha/5.0.2-beta-1+20220307/google_analyticsadmin1_alpha/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **analyticsadmin1_alpha** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

