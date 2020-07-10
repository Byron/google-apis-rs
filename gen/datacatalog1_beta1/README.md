<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-datacatalog1_beta1` library allows access to all features of the *Google Data Catalog* service.

This documentation was generated from *Data Catalog* crate version *1.0.14+20200528*, where *20200528* is the exact revision of the *datacatalog:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.

Everything else about the *Data Catalog* *v1_beta1* API can be found at the
[official documentation site](https://cloud.google.com/data-catalog/docs/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.DataCatalog.html) ... 

* catalog
 * [*search*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.CatalogSearchCall.html)
* entries
 * [*lookup*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.EntryLookupCall.html)
* projects
 * [*locations entry groups create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupCreateCall.html), [*locations entry groups delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupDeleteCall.html), [*locations entry groups entries create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryCreateCall.html), [*locations entry groups entries delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryDeleteCall.html), [*locations entry groups entries get*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryGetCall.html), [*locations entry groups entries get iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryGetIamPolicyCall.html), [*locations entry groups entries list*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryListCall.html), [*locations entry groups entries patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryPatchCall.html), [*locations entry groups entries tags create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryTagCreateCall.html), [*locations entry groups entries tags delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryTagDeleteCall.html), [*locations entry groups entries tags list*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryTagListCall.html), [*locations entry groups entries tags patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryTagPatchCall.html), [*locations entry groups entries test iam permissions*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupEntryTestIamPermissionCall.html), [*locations entry groups get*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupGetCall.html), [*locations entry groups get iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupGetIamPolicyCall.html), [*locations entry groups list*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupListCall.html), [*locations entry groups patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupPatchCall.html), [*locations entry groups set iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupSetIamPolicyCall.html), [*locations entry groups tags create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupTagCreateCall.html), [*locations entry groups tags delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupTagDeleteCall.html), [*locations entry groups tags list*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupTagListCall.html), [*locations entry groups tags patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupTagPatchCall.html), [*locations entry groups test iam permissions*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationEntryGroupTestIamPermissionCall.html), [*locations tag templates create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateCreateCall.html), [*locations tag templates delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateDeleteCall.html), [*locations tag templates fields create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateFieldCreateCall.html), [*locations tag templates fields delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateFieldDeleteCall.html), [*locations tag templates fields patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateFieldPatchCall.html), [*locations tag templates fields rename*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateFieldRenameCall.html), [*locations tag templates get*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateGetCall.html), [*locations tag templates get iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateGetIamPolicyCall.html), [*locations tag templates patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplatePatchCall.html), [*locations tag templates set iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateSetIamPolicyCall.html), [*locations tag templates test iam permissions*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTagTemplateTestIamPermissionCall.html), [*locations taxonomies create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyCreateCall.html), [*locations taxonomies delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyDeleteCall.html), [*locations taxonomies export*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyExportCall.html), [*locations taxonomies get*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyGetCall.html), [*locations taxonomies get iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyGetIamPolicyCall.html), [*locations taxonomies import*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyImportCall.html), [*locations taxonomies list*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyListCall.html), [*locations taxonomies patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPatchCall.html), [*locations taxonomies policy tags create*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagCreateCall.html), [*locations taxonomies policy tags delete*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagDeleteCall.html), [*locations taxonomies policy tags get*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagGetCall.html), [*locations taxonomies policy tags get iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagGetIamPolicyCall.html), [*locations taxonomies policy tags list*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagListCall.html), [*locations taxonomies policy tags patch*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagPatchCall.html), [*locations taxonomies policy tags set iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagSetIamPolicyCall.html), [*locations taxonomies policy tags test iam permissions*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyPolicyTagTestIamPermissionCall.html), [*locations taxonomies set iam policy*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomySetIamPolicyCall.html) and [*locations taxonomies test iam permissions*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.ProjectLocationTaxonomyTestIamPermissionCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/struct.DataCatalog.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().locations_tag_templates_get_iam_policy(...).doit()
let r = hub.projects().locations_tag_templates_set_iam_policy(...).doit()
let r = hub.projects().locations_entry_groups_entries_get_iam_policy(...).doit()
let r = hub.projects().locations_taxonomies_set_iam_policy(...).doit()
let r = hub.projects().locations_taxonomies_policy_tags_get_iam_policy(...).doit()
let r = hub.projects().locations_taxonomies_get_iam_policy(...).doit()
let r = hub.projects().locations_taxonomies_policy_tags_set_iam_policy(...).doit()
let r = hub.projects().locations_entry_groups_get_iam_policy(...).doit()
let r = hub.projects().locations_entry_groups_set_iam_policy(...).doit()
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
google-datacatalog1_beta1 = "*"
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
extern crate google_datacatalog1_beta1 as datacatalog1_beta1;
use datacatalog1_beta1::GetIamPolicyRequest;
use datacatalog1_beta1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use datacatalog1_beta1::DataCatalog;

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
let mut hub = DataCatalog::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GetIamPolicyRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_tag_templates_get_iam_policy(req, "resource")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-datacatalog1_beta1/1.0.14+20200528/google_datacatalog1_beta1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **datacatalog1_beta1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
