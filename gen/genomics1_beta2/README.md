<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-genomics1_beta2` library allows access to all features of the *Google genomics* service.

This documentation was generated from *genomics* crate version *0.1.5+20150326*, where *20150326* is the exact revision of the *genomics:v1beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.5*.

Everything else about the *genomics* *v1_beta2* API can be found at the
[official documentation site](https://developers.google.com/genomics/v1beta2/reference).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Genomics.html) ... 

* [annotation sets](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSet.html)
 * [*create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSetCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSetDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSetGetCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSetPatchCall.html), [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSetSearchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSetUpdateCall.html)
* [annotations](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Annotation.html)
 * [*batch create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationBatchCreateCall.html), [*create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationGetCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationPatchCall.html), [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationSearchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.AnnotationUpdateCall.html)
* callsets
 * [*create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.CallsetCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.CallsetDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.CallsetGetCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.CallsetPatchCall.html), [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.CallsetSearchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.CallsetUpdateCall.html)
* [datasets](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Dataset.html)
 * [*create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetPatchCall.html), [*undelete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetUndeleteCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.DatasetUpdateCall.html)
* experimental
 * [*jobs create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ExperimentalJobCreateCall.html)
* [jobs](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Job.html)
 * [*cancel*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.JobCancelCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.JobGetCall.html) and [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.JobSearchCall.html)
* readgroupsets
 * [*align*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetAlignCall.html), [*call*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetCallCall.html), [*coveragebuckets list*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetCoveragebucketListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetDeleteCall.html), [*export*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetExportCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetGetCall.html), [*import*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetImportCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetPatchCall.html), [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetSearchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadgroupsetUpdateCall.html)
* [reads](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Read.html)
 * [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReadSearchCall.html)
* [references](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Reference.html)
 * [*bases list*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReferenceBaseListCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReferenceGetCall.html) and [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReferenceSearchCall.html)
* referencesets
 * [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReferencesetGetCall.html) and [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.ReferencesetSearchCall.html)
* streaming variant store
 * [*streamvariants*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.StreamingVariantStoreStreamvariantCall.html)
* [variants](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Variant.html)
 * [*create*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantGetCall.html), [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantSearchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantUpdateCall.html)
* variantsets
 * [*delete*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetDeleteCall.html), [*export*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetExportCall.html), [*get*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetGetCall.html), [*import variants*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetImportVariantCall.html), [*merge variants*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetMergeVariantCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetPatchCall.html), [*search*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetSearchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.VariantsetUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_genomics1_beta2/struct.Genomics.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.annotations().search(...).doit()
let r = hub.annotations().patch(...).doit()
let r = hub.annotations().create(...).doit()
let r = hub.annotations().delete(...).doit()
let r = hub.annotations().update(...).doit()
let r = hub.annotations().get(...).doit()
let r = hub.annotations().batch_create(...).doit()
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
google-genomics1_beta2 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_genomics1_beta2 as genomics1_beta2;
use genomics1_beta2::Annotation;
use genomics1_beta2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use genomics1_beta2::Genomics;

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
let mut hub = Genomics::new(hyper::Client::new(), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req: Annotation = Default::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.annotations().patch(&req, "annotationId")
             .doit();

match result {
    Err(e) => match e {
        Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
        Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
        Error::MissingToken => println!("OAuth2: Missing Token"),
        Error::Cancelled => println!("Operation canceled by user"),
        Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
        Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
        Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
        Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
    },
    Ok(_) => println!("Success (value doesn't print)"),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_genomics1_beta2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_genomics1_beta2/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_genomics1_beta2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_genomics1_beta2/trait.RequestValue.html) are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **genomics1_beta2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
