<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-mapsengine1` library allows access to all features of the *Google Maps Engine* service.

This documentation was generated from *Maps Engine* crate version *0.1.8+20150414*, where *20150414* is the exact revision of the *mapsengine:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.8*.

Everything else about the *Maps Engine* *v1* API can be found at the
[official documentation site](https://developers.google.com/maps-engine/).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapsEngine.html) ... 

* [assets](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.Asset.html)
 * [*get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.AssetGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.AssetListCall.html), [*parents list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.AssetParentListCall.html) and [*permissions list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.AssetPermissionListCall.html)
* [layers](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.Layer.html)
 * [*cancel processing*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerCancelProcessingCall.html), [*create*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerGetCall.html), [*get published*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerGetPublishedCall.html), [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerListCall.html), [*list published*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerListPublishedCall.html), [*parents list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerParentListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerPatchCall.html), [*permissions batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerPermissionBatchDeleteCall.html), [*permissions batch update*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerPermissionBatchUpdateCall.html), [*permissions list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerPermissionListCall.html), [*process*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerProcesCall.html), [*publish*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerPublishCall.html) and [*unpublish*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.LayerUnpublishCall.html)
* [maps](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.Map.html)
 * [*create*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapGetCall.html), [*get published*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapGetPublishedCall.html), [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapListCall.html), [*list published*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapListPublishedCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapPatchCall.html), [*permissions batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapPermissionBatchDeleteCall.html), [*permissions batch update*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapPermissionBatchUpdateCall.html), [*permissions list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapPermissionListCall.html), [*publish*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapPublishCall.html) and [*unpublish*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapUnpublishCall.html)
* [projects](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.Project.html)
 * [*icons create*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.ProjectIconCreateCall.html), [*icons get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.ProjectIconGetCall.html), [*icons list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.ProjectIconListCall.html) and [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.ProjectListCall.html)
* [raster collections](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollection.html)
 * [*cancel processing*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionCancelProcessingCall.html), [*create*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionListCall.html), [*parents list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionParentListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionPatchCall.html), [*permissions batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionPermissionBatchDeleteCall.html), [*permissions batch update*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionPermissionBatchUpdateCall.html), [*permissions list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionPermissionListCall.html), [*process*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionProcesCall.html), [*rasters batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionRasterBatchDeleteCall.html), [*rasters batch insert*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionRasterBatchInsertCall.html) and [*rasters list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterCollectionRasterListCall.html)
* [rasters](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.Raster.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterDeleteCall.html), [*files insert*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterFileInsertCall.html), [*get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterListCall.html), [*parents list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterParentListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterPatchCall.html), [*permissions batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterPermissionBatchDeleteCall.html), [*permissions batch update*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterPermissionBatchUpdateCall.html), [*permissions list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterPermissionListCall.html), [*process*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterProcesCall.html) and [*upload*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterUploadCall.html)
* [tables](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.Table.html)
 * [*create*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableCreateCall.html), [*delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableDeleteCall.html), [*features batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFeatureBatchDeleteCall.html), [*features batch insert*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFeatureBatchInsertCall.html), [*features batch patch*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFeatureBatchPatchCall.html), [*features get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFeatureGetCall.html), [*features list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFeatureListCall.html), [*files insert*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFileInsertCall.html), [*get*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableListCall.html), [*parents list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableParentListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TablePatchCall.html), [*permissions batch delete*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TablePermissionBatchDeleteCall.html), [*permissions batch update*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TablePermissionBatchUpdateCall.html), [*permissions list*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TablePermissionListCall.html), [*process*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableProcesCall.html) and [*upload*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableUploadCall.html)


Upload supported by ...

* [*icons create projects*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.ProjectIconCreateCall.html)
* [*files insert tables*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.TableFileInsertCall.html)
* [*files insert rasters*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.RasterFileInsertCall.html)

Download supported by ...

* [*icons get projects*](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.ProjectIconGetCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_mapsengine1/struct.MapsEngine.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.tables().features_batch_insert(...).doit()
let r = hub.tables().list(...).doit()
let r = hub.tables().patch(...).doit()
let r = hub.tables().get(...).doit()
let r = hub.tables().files_insert(...).doit()
let r = hub.tables().features_batch_delete(...).doit()
let r = hub.tables().permissions_batch_delete(...).doit()
let r = hub.tables().permissions_batch_update(...).doit()
let r = hub.tables().features_list(...).doit()
let r = hub.tables().process(...).doit()
let r = hub.tables().upload(...).doit()
let r = hub.tables().delete(...).doit()
let r = hub.tables().parents_list(...).doit()
let r = hub.tables().features_batch_patch(...).doit()
let r = hub.tables().permissions_list(...).doit()
let r = hub.tables().features_get(...).doit()
let r = hub.tables().create(...).doit()
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
google-mapsengine1 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_mapsengine1 as mapsengine1;
use mapsengine1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use mapsengine1::MapsEngine;

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
let mut hub = MapsEngine::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.tables().list()
             .tags("eirmod")
             .search("sit")
             .role("Stet")
             .project_id("sed")
             .processing_status("et")
             .page_token("dolores")
             .modified_before("kasd")
             .modified_after("accusam")
             .max_results(93)
             .creator_email("justo")
             .created_before("amet.")
             .created_after("erat")
             .bbox("labore")
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

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_mapsengine1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_mapsengine1/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_mapsengine1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_mapsengine1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **mapsengine1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
