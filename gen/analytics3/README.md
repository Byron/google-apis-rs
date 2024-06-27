<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-analytics3` library allows access to all features of the *Google analytics* service.

This documentation was generated from *analytics* crate version *5.0.5+20190807*, where *20190807* is the exact revision of the *analytics:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *analytics* *v3* API can be found at the
[official documentation site](https://developers.google.com/analytics/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/Analytics) ... 

* data
 * [*ga get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::DataGaGetCall), [*mcf get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::DataMcfGetCall) and [*realtime get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::DataRealtimeGetCall)
* management
 * [*account summaries list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementAccountSummaryListCall), [*account user links delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementAccountUserLinkDeleteCall), [*account user links insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementAccountUserLinkInsertCall), [*account user links list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementAccountUserLinkListCall), [*account user links update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementAccountUserLinkUpdateCall), [*accounts list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementAccountListCall), [*client id hash client id*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementClientIdHashClientIdCall), [*custom data sources list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomDataSourceListCall), [*custom dimensions get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomDimensionGetCall), [*custom dimensions insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomDimensionInsertCall), [*custom dimensions list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomDimensionListCall), [*custom dimensions patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomDimensionPatchCall), [*custom dimensions update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomDimensionUpdateCall), [*custom metrics get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomMetricGetCall), [*custom metrics insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomMetricInsertCall), [*custom metrics list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomMetricListCall), [*custom metrics patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomMetricPatchCall), [*custom metrics update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementCustomMetricUpdateCall), [*experiments delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementExperimentDeleteCall), [*experiments get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementExperimentGetCall), [*experiments insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementExperimentInsertCall), [*experiments list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementExperimentListCall), [*experiments patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementExperimentPatchCall), [*experiments update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementExperimentUpdateCall), [*filters delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementFilterDeleteCall), [*filters get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementFilterGetCall), [*filters insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementFilterInsertCall), [*filters list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementFilterListCall), [*filters patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementFilterPatchCall), [*filters update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementFilterUpdateCall), [*goals get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementGoalGetCall), [*goals insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementGoalInsertCall), [*goals list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementGoalListCall), [*goals patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementGoalPatchCall), [*goals update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementGoalUpdateCall), [*profile filter links delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileFilterLinkDeleteCall), [*profile filter links get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileFilterLinkGetCall), [*profile filter links insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileFilterLinkInsertCall), [*profile filter links list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileFilterLinkListCall), [*profile filter links patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileFilterLinkPatchCall), [*profile filter links update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileFilterLinkUpdateCall), [*profile user links delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileUserLinkDeleteCall), [*profile user links insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileUserLinkInsertCall), [*profile user links list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileUserLinkListCall), [*profile user links update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileUserLinkUpdateCall), [*profiles delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileDeleteCall), [*profiles get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileGetCall), [*profiles insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileInsertCall), [*profiles list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileListCall), [*profiles patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfilePatchCall), [*profiles update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementProfileUpdateCall), [*remarketing audience delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementRemarketingAudienceDeleteCall), [*remarketing audience get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementRemarketingAudienceGetCall), [*remarketing audience insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementRemarketingAudienceInsertCall), [*remarketing audience list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementRemarketingAudienceListCall), [*remarketing audience patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementRemarketingAudiencePatchCall), [*remarketing audience update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementRemarketingAudienceUpdateCall), [*segments list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementSegmentListCall), [*unsampled reports delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUnsampledReportDeleteCall), [*unsampled reports get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUnsampledReportGetCall), [*unsampled reports insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUnsampledReportInsertCall), [*unsampled reports list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUnsampledReportListCall), [*uploads delete upload data*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUploadDeleteUploadDataCall), [*uploads get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUploadGetCall), [*uploads list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUploadListCall), [*uploads upload data*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUploadUploadDataCall), [*web property ad words links delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebPropertyAdWordsLinkDeleteCall), [*web property ad words links get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebPropertyAdWordsLinkGetCall), [*web property ad words links insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebPropertyAdWordsLinkInsertCall), [*web property ad words links list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebPropertyAdWordsLinkListCall), [*web property ad words links patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebPropertyAdWordsLinkPatchCall), [*web property ad words links update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebPropertyAdWordsLinkUpdateCall), [*webproperties get*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyGetCall), [*webproperties insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyInsertCall), [*webproperties list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyListCall), [*webproperties patch*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyPatchCall), [*webproperties update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyUpdateCall), [*webproperty user links delete*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyUserLinkDeleteCall), [*webproperty user links insert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyUserLinkInsertCall), [*webproperty user links list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyUserLinkListCall) and [*webproperty user links update*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementWebpropertyUserLinkUpdateCall)
* metadata
 * [*columns list*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::MetadataColumnListCall)
* provisioning
 * [*create account ticket*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ProvisioningCreateAccountTicketCall) and [*create account tree*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ProvisioningCreateAccountTreeCall)
* user deletion
 * [*user deletion request upsert*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::UserDeletionUserDeletionRequestUpsertCall)


Upload supported by ...

* [*uploads upload data management*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/api::ManagementUploadUploadDataCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/Analytics)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::CallBuilder)
* **[Resources](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.management().account_user_links_insert(...).doit().await
let r = hub.management().account_user_links_update(...).doit().await
let r = hub.management().profile_user_links_insert(...).doit().await
let r = hub.management().profile_user_links_update(...).doit().await
let r = hub.management().webproperty_user_links_insert(...).doit().await
let r = hub.management().webproperty_user_links_update(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_analytics3 as analytics3;
use analytics3::api::EntityUserLink;
use analytics3::{Result, Error};
use std::default::Default;
use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = EntityUserLink::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.management().profile_user_links_update(req, "accountId", "webPropertyId", "profileId", "linkId")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::RequestValue) and 
[decodable](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-analytics3/5.0.5+20190807/google_analytics3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **analytics3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

