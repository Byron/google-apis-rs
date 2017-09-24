<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-analytics3` library allows access to all features of the *Google analytics* service.

This documentation was generated from *analytics* crate version *1.0.6+20170321*, where *20170321* is the exact revision of the *analytics:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.6*.

Everything else about the *analytics* *v3* API can be found at the
[official documentation site](https://developers.google.com/analytics/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.Analytics.html) ... 

* data
 * [*ga get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.DataGaGetCall.html), [*mcf get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.DataMcfGetCall.html) and [*realtime get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.DataRealtimeGetCall.html)
* management
 * [*account summaries list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementAccountSummaryListCall.html), [*account user links delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementAccountUserLinkDeleteCall.html), [*account user links insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementAccountUserLinkInsertCall.html), [*account user links list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementAccountUserLinkListCall.html), [*account user links update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementAccountUserLinkUpdateCall.html), [*accounts list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementAccountListCall.html), [*custom data sources list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomDataSourceListCall.html), [*custom dimensions get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomDimensionGetCall.html), [*custom dimensions insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomDimensionInsertCall.html), [*custom dimensions list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomDimensionListCall.html), [*custom dimensions patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomDimensionPatchCall.html), [*custom dimensions update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomDimensionUpdateCall.html), [*custom metrics get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomMetricGetCall.html), [*custom metrics insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomMetricInsertCall.html), [*custom metrics list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomMetricListCall.html), [*custom metrics patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomMetricPatchCall.html), [*custom metrics update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementCustomMetricUpdateCall.html), [*experiments delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementExperimentDeleteCall.html), [*experiments get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementExperimentGetCall.html), [*experiments insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementExperimentInsertCall.html), [*experiments list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementExperimentListCall.html), [*experiments patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementExperimentPatchCall.html), [*experiments update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementExperimentUpdateCall.html), [*filters delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementFilterDeleteCall.html), [*filters get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementFilterGetCall.html), [*filters insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementFilterInsertCall.html), [*filters list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementFilterListCall.html), [*filters patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementFilterPatchCall.html), [*filters update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementFilterUpdateCall.html), [*goals get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementGoalGetCall.html), [*goals insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementGoalInsertCall.html), [*goals list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementGoalListCall.html), [*goals patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementGoalPatchCall.html), [*goals update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementGoalUpdateCall.html), [*profile filter links delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileFilterLinkDeleteCall.html), [*profile filter links get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileFilterLinkGetCall.html), [*profile filter links insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileFilterLinkInsertCall.html), [*profile filter links list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileFilterLinkListCall.html), [*profile filter links patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileFilterLinkPatchCall.html), [*profile filter links update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileFilterLinkUpdateCall.html), [*profile user links delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileUserLinkDeleteCall.html), [*profile user links insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileUserLinkInsertCall.html), [*profile user links list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileUserLinkListCall.html), [*profile user links update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileUserLinkUpdateCall.html), [*profiles delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileDeleteCall.html), [*profiles get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileGetCall.html), [*profiles insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileInsertCall.html), [*profiles list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileListCall.html), [*profiles patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfilePatchCall.html), [*profiles update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementProfileUpdateCall.html), [*remarketing audience delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementRemarketingAudienceDeleteCall.html), [*remarketing audience get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementRemarketingAudienceGetCall.html), [*remarketing audience insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementRemarketingAudienceInsertCall.html), [*remarketing audience list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementRemarketingAudienceListCall.html), [*remarketing audience patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementRemarketingAudiencePatchCall.html), [*remarketing audience update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementRemarketingAudienceUpdateCall.html), [*segments list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementSegmentListCall.html), [*unsampled reports delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUnsampledReportDeleteCall.html), [*unsampled reports get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUnsampledReportGetCall.html), [*unsampled reports insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUnsampledReportInsertCall.html), [*unsampled reports list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUnsampledReportListCall.html), [*uploads delete upload data*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUploadDeleteUploadDataCall.html), [*uploads get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUploadGetCall.html), [*uploads list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUploadListCall.html), [*uploads upload data*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUploadUploadDataCall.html), [*web property ad words links delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebPropertyAdWordsLinkDeleteCall.html), [*web property ad words links get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebPropertyAdWordsLinkGetCall.html), [*web property ad words links insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebPropertyAdWordsLinkInsertCall.html), [*web property ad words links list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebPropertyAdWordsLinkListCall.html), [*web property ad words links patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebPropertyAdWordsLinkPatchCall.html), [*web property ad words links update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebPropertyAdWordsLinkUpdateCall.html), [*webproperties get*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyGetCall.html), [*webproperties insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyInsertCall.html), [*webproperties list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyListCall.html), [*webproperties patch*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyPatchCall.html), [*webproperties update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyUpdateCall.html), [*webproperty user links delete*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyUserLinkDeleteCall.html), [*webproperty user links insert*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyUserLinkInsertCall.html), [*webproperty user links list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyUserLinkListCall.html) and [*webproperty user links update*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementWebpropertyUserLinkUpdateCall.html)
* metadata
 * [*columns list*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.MetadataColumnListCall.html)
* provisioning
 * [*create account ticket*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ProvisioningCreateAccountTicketCall.html)


Upload supported by ...

* [*uploads upload data management*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.ManagementUploadUploadDataCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/struct.Analytics.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.management().webproperty_user_links_insert(...).doit()
let r = hub.management().profile_user_links_insert(...).doit()
let r = hub.management().profile_user_links_update(...).doit()
let r = hub.management().account_user_links_update(...).doit()
let r = hub.management().webproperty_user_links_update(...).doit()
let r = hub.management().account_user_links_insert(...).doit()
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
google-analytics3 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_analytics3 as analytics3;
use analytics3::EntityUserLink;
use analytics3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use analytics3::Analytics;

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
let mut hub = Analytics::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = EntityUserLink::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.management().profile_user_links_update(req, "accountId", "webPropertyId", "profileId", "linkId")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-analytics3/1.0.6+20170321/google_analytics3/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **analytics3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
