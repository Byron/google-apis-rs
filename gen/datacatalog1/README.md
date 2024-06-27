<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-datacatalog1` library allows access to all features of the *Google Data Catalog* service.

This documentation was generated from *Data Catalog* crate version *5.0.5+20240624*, where *20240624* is the exact revision of the *datacatalog:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Data Catalog* *v1* API can be found at the
[official documentation site](https://cloud.google.com/data-catalog/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/DataCatalog) ... 

* catalog
 * [*search*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::CatalogSearchCall)
* entries
 * [*lookup*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::EntryLookupCall)
* projects
 * [*locations entry groups create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupCreateCall), [*locations entry groups delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupDeleteCall), [*locations entry groups entries create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryCreateCall), [*locations entry groups entries delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryDeleteCall), [*locations entry groups entries get*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryGetCall), [*locations entry groups entries get iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryGetIamPolicyCall), [*locations entry groups entries import*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryImportCall), [*locations entry groups entries list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryListCall), [*locations entry groups entries modify entry contacts*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryModifyEntryContactCall), [*locations entry groups entries modify entry overview*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryModifyEntryOverviewCall), [*locations entry groups entries patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryPatchCall), [*locations entry groups entries star*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryStarCall), [*locations entry groups entries tags create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryTagCreateCall), [*locations entry groups entries tags delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryTagDeleteCall), [*locations entry groups entries tags list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryTagListCall), [*locations entry groups entries tags patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryTagPatchCall), [*locations entry groups entries tags reconcile*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryTagReconcileCall), [*locations entry groups entries test iam permissions*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryTestIamPermissionCall), [*locations entry groups entries unstar*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupEntryUnstarCall), [*locations entry groups get*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupGetCall), [*locations entry groups get iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupGetIamPolicyCall), [*locations entry groups list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupListCall), [*locations entry groups patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupPatchCall), [*locations entry groups set iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupSetIamPolicyCall), [*locations entry groups tags create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupTagCreateCall), [*locations entry groups tags delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupTagDeleteCall), [*locations entry groups tags list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupTagListCall), [*locations entry groups tags patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupTagPatchCall), [*locations entry groups test iam permissions*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationEntryGroupTestIamPermissionCall), [*locations operations cancel*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationOperationListCall), [*locations tag templates create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateCreateCall), [*locations tag templates delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateDeleteCall), [*locations tag templates fields create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateFieldCreateCall), [*locations tag templates fields delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateFieldDeleteCall), [*locations tag templates fields enum values rename*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateFieldEnumValueRenameCall), [*locations tag templates fields patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateFieldPatchCall), [*locations tag templates fields rename*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateFieldRenameCall), [*locations tag templates get*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateGetCall), [*locations tag templates get iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateGetIamPolicyCall), [*locations tag templates patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplatePatchCall), [*locations tag templates set iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateSetIamPolicyCall), [*locations tag templates test iam permissions*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTagTemplateTestIamPermissionCall), [*locations taxonomies create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyCreateCall), [*locations taxonomies delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyDeleteCall), [*locations taxonomies export*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyExportCall), [*locations taxonomies get*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyGetCall), [*locations taxonomies get iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyGetIamPolicyCall), [*locations taxonomies import*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyImportCall), [*locations taxonomies list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyListCall), [*locations taxonomies patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPatchCall), [*locations taxonomies policy tags create*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagCreateCall), [*locations taxonomies policy tags delete*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagDeleteCall), [*locations taxonomies policy tags get*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagGetCall), [*locations taxonomies policy tags get iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagGetIamPolicyCall), [*locations taxonomies policy tags list*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagListCall), [*locations taxonomies policy tags patch*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagPatchCall), [*locations taxonomies policy tags set iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagSetIamPolicyCall), [*locations taxonomies policy tags test iam permissions*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyPolicyTagTestIamPermissionCall), [*locations taxonomies replace*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyReplaceCall), [*locations taxonomies set iam policy*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomySetIamPolicyCall) and [*locations taxonomies test iam permissions*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/api::ProjectLocationTaxonomyTestIamPermissionCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/DataCatalog)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::CallBuilder)
* **[Resources](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_entry_groups_entries_tags_delete(...).doit().await
let r = hub.projects().locations_entry_groups_entries_delete(...).doit().await
let r = hub.projects().locations_entry_groups_tags_delete(...).doit().await
let r = hub.projects().locations_entry_groups_delete(...).doit().await
let r = hub.projects().locations_operations_cancel(...).doit().await
let r = hub.projects().locations_operations_delete(...).doit().await
let r = hub.projects().locations_tag_templates_fields_delete(...).doit().await
let r = hub.projects().locations_tag_templates_delete(...).doit().await
let r = hub.projects().locations_taxonomies_policy_tags_delete(...).doit().await
let r = hub.projects().locations_taxonomies_delete(...).doit().await
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
google-datacatalog1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_datacatalog1 as datacatalog1;
use datacatalog1::{Result, Error};
use std::default::Default;
use datacatalog1::{DataCatalog, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = DataCatalog::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_entry_groups_delete("name")
             .force(true)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::RequestValue) and 
[decodable](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-datacatalog1/5.0.5+20240624/google_datacatalog1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **datacatalog1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

