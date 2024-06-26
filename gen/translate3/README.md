<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-translate3` library allows access to all features of the *Google Translate* service.

This documentation was generated from *Translate* crate version *5.0.5+20240301*, where *20240301* is the exact revision of the *translate:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Translate* *v3* API can be found at the
[official documentation site](https://cloud.google.com/translate/docs/quickstarts).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/Translate) ... 

* projects
 * [*detect language*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectDetectLanguageCall), [*get supported languages*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectGetSupportedLanguageCall), [*locations adaptive mt datasets adaptive mt files adaptive mt sentences list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileAdaptiveMtSentenceListCall), [*locations adaptive mt datasets adaptive mt files delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileDeleteCall), [*locations adaptive mt datasets adaptive mt files get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileGetCall), [*locations adaptive mt datasets adaptive mt files list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileListCall), [*locations adaptive mt datasets adaptive mt sentences list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetAdaptiveMtSentenceListCall), [*locations adaptive mt datasets create*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetCreateCall), [*locations adaptive mt datasets delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetDeleteCall), [*locations adaptive mt datasets get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetGetCall), [*locations adaptive mt datasets import adaptive mt file*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetImportAdaptiveMtFileCall), [*locations adaptive mt datasets list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtDatasetListCall), [*locations adaptive mt translate*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationAdaptiveMtTranslateCall), [*locations batch translate document*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationBatchTranslateDocumentCall), [*locations batch translate text*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationBatchTranslateTextCall), [*locations datasets create*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetCreateCall), [*locations datasets delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetDeleteCall), [*locations datasets examples list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetExampleListCall), [*locations datasets export data*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetExportDataCall), [*locations datasets get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetGetCall), [*locations datasets import data*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetImportDataCall), [*locations datasets list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDatasetListCall), [*locations detect language*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationDetectLanguageCall), [*locations get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGetCall), [*locations get supported languages*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGetSupportedLanguageCall), [*locations glossaries create*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryCreateCall), [*locations glossaries delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryDeleteCall), [*locations glossaries get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryGetCall), [*locations glossaries glossary entries create*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryGlossaryEntryCreateCall), [*locations glossaries glossary entries delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryGlossaryEntryDeleteCall), [*locations glossaries glossary entries get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryGlossaryEntryGetCall), [*locations glossaries glossary entries list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryGlossaryEntryListCall), [*locations glossaries glossary entries patch*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryGlossaryEntryPatchCall), [*locations glossaries list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryListCall), [*locations glossaries patch*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationGlossaryPatchCall), [*locations list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationListCall), [*locations models create*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationModelCreateCall), [*locations models delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationModelDeleteCall), [*locations models get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationModelGetCall), [*locations models list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationModelListCall), [*locations operations cancel*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationOperationListCall), [*locations operations wait*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationOperationWaitCall), [*locations romanize text*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationRomanizeTextCall), [*locations translate document*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationTranslateDocumentCall), [*locations translate text*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectLocationTranslateTextCall), [*romanize text*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectRomanizeTextCall) and [*translate text*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/api::ProjectTranslateTextCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/Translate)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::CallBuilder)
* **[Resources](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_datasets_create(...).doit().await
let r = hub.projects().locations_datasets_delete(...).doit().await
let r = hub.projects().locations_datasets_export_data(...).doit().await
let r = hub.projects().locations_datasets_import_data(...).doit().await
let r = hub.projects().locations_glossaries_create(...).doit().await
let r = hub.projects().locations_glossaries_delete(...).doit().await
let r = hub.projects().locations_glossaries_patch(...).doit().await
let r = hub.projects().locations_models_create(...).doit().await
let r = hub.projects().locations_models_delete(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_operations_wait(...).doit().await
let r = hub.projects().locations_batch_translate_document(...).doit().await
let r = hub.projects().locations_batch_translate_text(...).doit().await
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
google-translate3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_translate3 as translate3;
use translate3::api::Glossary;
use translate3::{Result, Error};
use std::default::Default;
use translate3::{Translate, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Translate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Glossary::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_glossaries_patch(req, "name")
             .update_mask(FieldMask::new::<&str>(&[]))
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::RequestValue) and 
[decodable](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-translate3/5.0.5+20240301/google_translate3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **translate3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

