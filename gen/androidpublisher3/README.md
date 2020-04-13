<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-androidpublisher3` library allows access to all features of the *Google Android Publisher* service.

This documentation was generated from *Android Publisher* crate version *1.0.13+20200331*, where *20200331* is the exact revision of the *androidpublisher:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *Android Publisher* *v3* API can be found at the
[official documentation site](https://developers.google.com/android-publisher).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.AndroidPublisher.html) ... 

* edits
 * [*apks addexternallyhosted*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditApkAddexternallyhostedCall.html), [*apks list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditApkListCall.html), [*apks upload*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditApkUploadCall.html), [*bundles list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditBundleListCall.html), [*bundles upload*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditBundleUploadCall.html), [*commit*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditCommitCall.html), [*delete*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditDeleteCall.html), [*deobfuscationfiles upload*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditDeobfuscationfileUploadCall.html), [*details get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditDetailGetCall.html), [*details patch*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditDetailPatchCall.html), [*details update*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditDetailUpdateCall.html), [*expansionfiles get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditExpansionfileGetCall.html), [*expansionfiles patch*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditExpansionfilePatchCall.html), [*expansionfiles update*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditExpansionfileUpdateCall.html), [*expansionfiles upload*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditExpansionfileUploadCall.html), [*get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditGetCall.html), [*images delete*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditImageDeleteCall.html), [*images deleteall*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditImageDeleteallCall.html), [*images list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditImageListCall.html), [*images upload*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditImageUploadCall.html), [*insert*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditInsertCall.html), [*listings delete*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditListingDeleteCall.html), [*listings deleteall*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditListingDeleteallCall.html), [*listings get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditListingGetCall.html), [*listings list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditListingListCall.html), [*listings patch*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditListingPatchCall.html), [*listings update*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditListingUpdateCall.html), [*testers get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTesterGetCall.html), [*testers patch*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTesterPatchCall.html), [*testers update*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTesterUpdateCall.html), [*tracks get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTrackGetCall.html), [*tracks list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTrackListCall.html), [*tracks patch*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTrackPatchCall.html), [*tracks update*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditTrackUpdateCall.html) and [*validate*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditValidateCall.html)
* inappproducts
 * [*delete*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InappproductDeleteCall.html), [*get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InappproductGetCall.html), [*insert*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InappproductInsertCall.html), [*list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InappproductListCall.html), [*patch*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InappproductPatchCall.html) and [*update*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InappproductUpdateCall.html)
* internalappsharingartifacts
 * [*uploadapk*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InternalappsharingartifactUploadapkCall.html) and [*uploadbundle*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InternalappsharingartifactUploadbundleCall.html)
* orders
 * [*refund*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.OrderRefundCall.html)
* purchases
 * [*products acknowledge*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseProductAcknowledgeCall.html), [*products get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseProductGetCall.html), [*subscriptions acknowledge*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseSubscriptionAcknowledgeCall.html), [*subscriptions cancel*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseSubscriptionCancelCall.html), [*subscriptions defer*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseSubscriptionDeferCall.html), [*subscriptions get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseSubscriptionGetCall.html), [*subscriptions refund*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseSubscriptionRefundCall.html), [*subscriptions revoke*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseSubscriptionRevokeCall.html) and [*voidedpurchases list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.PurchaseVoidedpurchaseListCall.html)
* [reviews](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.Review.html)
 * [*get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.ReviewGetCall.html), [*list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.ReviewListCall.html) and [*reply*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.ReviewReplyCall.html)
* systemapks
 * [*variants create*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.SystemapkVariantCreateCall.html), [*variants download*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.SystemapkVariantDownloadCall.html), [*variants get*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.SystemapkVariantGetCall.html) and [*variants list*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.SystemapkVariantListCall.html)


Upload supported by ...

* [*deobfuscationfiles upload edits*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditDeobfuscationfileUploadCall.html)
* [*images upload edits*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditImageUploadCall.html)
* [*apks upload edits*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditApkUploadCall.html)
* [*expansionfiles upload edits*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditExpansionfileUploadCall.html)
* [*bundles upload edits*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.EditBundleUploadCall.html)
* [*uploadapk internalappsharingartifacts*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InternalappsharingartifactUploadapkCall.html)
* [*uploadbundle internalappsharingartifacts*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.InternalappsharingartifactUploadbundleCall.html)

Download supported by ...

* [*variants download systemapks*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.SystemapkVariantDownloadCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/struct.AndroidPublisher.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.inappproducts().patch(...).doit()
let r = hub.inappproducts().insert(...).doit()
let r = hub.inappproducts().get(...).doit()
let r = hub.inappproducts().update(...).doit()
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
google-androidpublisher3 = "*"
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
extern crate google_androidpublisher3 as androidpublisher3;
use androidpublisher3::InAppProduct;
use androidpublisher3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use androidpublisher3::AndroidPublisher;

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
let mut hub = AndroidPublisher::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = InAppProduct::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.inappproducts().patch(req, "packageName", "sku")
             .auto_convert_missing_prices(false)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-androidpublisher3/1.0.13+20200331/google_androidpublisher3/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **androidpublisher3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
