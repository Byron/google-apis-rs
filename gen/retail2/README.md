<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-retail2` library allows access to all features of the *Google Cloud Retail* service.

This documentation was generated from *Cloud Retail* crate version *5.0.5+20240614*, where *20240614* is the exact revision of the *retail:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Cloud Retail* *v2* API can be found at the
[official documentation site](https://cloud.google.com/recommendations).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/CloudRetail) ... 

* projects
 * [*locations catalogs attributes config add catalog attribute*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogAttributesConfigAddCatalogAttributeCall), [*locations catalogs attributes config remove catalog attribute*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogAttributesConfigRemoveCatalogAttributeCall), [*locations catalogs attributes config replace catalog attribute*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogAttributesConfigReplaceCatalogAttributeCall), [*locations catalogs branches operations get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchOperationGetCall), [*locations catalogs branches products add fulfillment places*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductAddFulfillmentPlaceCall), [*locations catalogs branches products add local inventories*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductAddLocalInventoryCall), [*locations catalogs branches products create*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductCreateCall), [*locations catalogs branches products delete*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductDeleteCall), [*locations catalogs branches products get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductGetCall), [*locations catalogs branches products import*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductImportCall), [*locations catalogs branches products list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductListCall), [*locations catalogs branches products patch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductPatchCall), [*locations catalogs branches products purge*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductPurgeCall), [*locations catalogs branches products remove fulfillment places*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductRemoveFulfillmentPlaceCall), [*locations catalogs branches products remove local inventories*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductRemoveLocalInventoryCall), [*locations catalogs branches products set inventory*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogBranchProductSetInventoryCall), [*locations catalogs complete query*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogCompleteQueryCall), [*locations catalogs completion data import*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogCompletionDataImportCall), [*locations catalogs controls create*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogControlCreateCall), [*locations catalogs controls delete*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogControlDeleteCall), [*locations catalogs controls get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogControlGetCall), [*locations catalogs controls list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogControlListCall), [*locations catalogs controls patch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogControlPatchCall), [*locations catalogs export analytics metrics*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogExportAnalyticsMetricCall), [*locations catalogs get attributes config*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogGetAttributesConfigCall), [*locations catalogs get completion config*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogGetCompletionConfigCall), [*locations catalogs get default branch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogGetDefaultBranchCall), [*locations catalogs list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogListCall), [*locations catalogs models create*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelCreateCall), [*locations catalogs models delete*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelDeleteCall), [*locations catalogs models get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelGetCall), [*locations catalogs models list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelListCall), [*locations catalogs models patch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelPatchCall), [*locations catalogs models pause*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelPauseCall), [*locations catalogs models resume*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelResumeCall), [*locations catalogs models tune*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogModelTuneCall), [*locations catalogs operations get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogOperationGetCall), [*locations catalogs operations list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogOperationListCall), [*locations catalogs patch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogPatchCall), [*locations catalogs placements predict*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogPlacementPredictCall), [*locations catalogs placements search*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogPlacementSearchCall), [*locations catalogs serving configs add control*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigAddControlCall), [*locations catalogs serving configs create*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigCreateCall), [*locations catalogs serving configs delete*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigDeleteCall), [*locations catalogs serving configs get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigGetCall), [*locations catalogs serving configs list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigListCall), [*locations catalogs serving configs patch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigPatchCall), [*locations catalogs serving configs predict*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigPredictCall), [*locations catalogs serving configs remove control*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigRemoveControlCall), [*locations catalogs serving configs search*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogServingConfigSearchCall), [*locations catalogs set default branch*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogSetDefaultBranchCall), [*locations catalogs update attributes config*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUpdateAttributesConfigCall), [*locations catalogs update completion config*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUpdateCompletionConfigCall), [*locations catalogs user events collect*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUserEventCollectCall), [*locations catalogs user events import*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUserEventImportCall), [*locations catalogs user events purge*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUserEventPurgeCall), [*locations catalogs user events rejoin*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUserEventRejoinCall), [*locations catalogs user events write*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationCatalogUserEventWriteCall), [*locations operations get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectLocationOperationListCall), [*operations get*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectOperationGetCall) and [*operations list*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/api::ProjectOperationListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/CloudRetail)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::CallBuilder)
* **[Resources](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_catalogs_branches_operations_get(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_add_fulfillment_places(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_add_local_inventories(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_import(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_purge(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_remove_fulfillment_places(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_remove_local_inventories(...).doit().await
let r = hub.projects().locations_catalogs_branches_products_set_inventory(...).doit().await
let r = hub.projects().locations_catalogs_completion_data_import(...).doit().await
let r = hub.projects().locations_catalogs_models_create(...).doit().await
let r = hub.projects().locations_catalogs_models_tune(...).doit().await
let r = hub.projects().locations_catalogs_operations_get(...).doit().await
let r = hub.projects().locations_catalogs_user_events_import(...).doit().await
let r = hub.projects().locations_catalogs_user_events_purge(...).doit().await
let r = hub.projects().locations_catalogs_user_events_rejoin(...).doit().await
let r = hub.projects().locations_catalogs_export_analytics_metrics(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().operations_get(...).doit().await
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
google-retail2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_retail2 as retail2;
use retail2::api::GoogleCloudRetailV2Model;
use retail2::{Result, Error};
use std::default::Default;
use retail2::{CloudRetail, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = CloudRetail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleCloudRetailV2Model::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_catalogs_models_create(req, "parent")
             .dry_run(true)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::RequestValue) and 
[decodable](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-retail2/5.0.5+20240614/google_retail2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **retail2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

