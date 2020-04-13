<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-vision1` library allows access to all features of the *Google Vision* service.

This documentation was generated from *Vision* crate version *1.0.13+20200329*, where *20200329* is the exact revision of the *vision:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *Vision* *v1* API can be found at the
[official documentation site](https://cloud.google.com/vision/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.Vision.html) ... 

* files
 * [*annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.FileAnnotateCall.html) and [*async batch annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.FileAsyncBatchAnnotateCall.html)
* [images](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.Image.html)
 * [*annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ImageAnnotateCall.html) and [*async batch annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ImageAsyncBatchAnnotateCall.html)
* locations
 * [*operations get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.LocationOperationGetCall.html)
* [operations](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.Operation.html)
 * [*cancel*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.OperationCancelCall.html), [*delete*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.OperationDeleteCall.html), [*get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.OperationGetCall.html) and [*list*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.OperationListCall.html)
* projects
 * [*files annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectFileAnnotateCall.html), [*files async batch annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectFileAsyncBatchAnnotateCall.html), [*images annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectImageAnnotateCall.html), [*images async batch annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectImageAsyncBatchAnnotateCall.html), [*locations files annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationFileAnnotateCall.html), [*locations files async batch annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationFileAsyncBatchAnnotateCall.html), [*locations images annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationImageAnnotateCall.html), [*locations images async batch annotate*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationImageAsyncBatchAnnotateCall.html), [*locations operations get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationOperationGetCall.html), [*locations product sets add product*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetAddProductCall.html), [*locations product sets create*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetCreateCall.html), [*locations product sets delete*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetDeleteCall.html), [*locations product sets get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetGetCall.html), [*locations product sets import*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetImportCall.html), [*locations product sets list*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetListCall.html), [*locations product sets patch*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetPatchCall.html), [*locations product sets products list*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetProductListCall.html), [*locations product sets remove product*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductSetRemoveProductCall.html), [*locations products create*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductCreateCall.html), [*locations products delete*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductDeleteCall.html), [*locations products get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductGetCall.html), [*locations products list*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductListCall.html), [*locations products patch*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductPatchCall.html), [*locations products purge*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductPurgeCall.html), [*locations products reference images create*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductReferenceImageCreateCall.html), [*locations products reference images delete*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductReferenceImageDeleteCall.html), [*locations products reference images get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductReferenceImageGetCall.html), [*locations products reference images list*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectLocationProductReferenceImageListCall.html) and [*operations get*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.ProjectOperationGetCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/struct.Vision.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().operations_get(...).doit()
let r = hub.operations().list(...).doit()
let r = hub.projects().files_async_batch_annotate(...).doit()
let r = hub.images().async_batch_annotate(...).doit()
let r = hub.projects().images_async_batch_annotate(...).doit()
let r = hub.projects().locations_files_async_batch_annotate(...).doit()
let r = hub.operations().cancel(...).doit()
let r = hub.locations().operations_get(...).doit()
let r = hub.files().async_batch_annotate(...).doit()
let r = hub.operations().delete(...).doit()
let r = hub.projects().locations_images_async_batch_annotate(...).doit()
let r = hub.projects().locations_products_purge(...).doit()
let r = hub.operations().get(...).doit()
let r = hub.projects().locations_operations_get(...).doit()
let r = hub.projects().locations_product_sets_import(...).doit()
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
google-vision1 = "*"
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
extern crate google_vision1 as vision1;
use vision1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use vision1::Vision;

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
let mut hub = Vision::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.operations().list("name")
             .page_token("sit")
             .page_size(-65)
             .filter("sed")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-vision1/1.0.13+20200329/google_vision1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **vision1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
