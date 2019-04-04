<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-storage1` library allows access to all features of the *Google storage* service.

This documentation was generated from *storage* crate version *1.0.8+20190226*, where *20190226* is the exact revision of the *storage:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.

Everything else about the *storage* *v1* API can be found at the
[official documentation site](https://developers.google.com/storage/docs/json_api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.Storage.html) ... 

* [bucket access controls](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControl.html)
 * [*delete*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControlDeleteCall.html), [*get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControlGetCall.html), [*insert*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControlInsertCall.html), [*list*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControlListCall.html), [*patch*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControlPatchCall.html) and [*update*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketAccessControlUpdateCall.html)
* [buckets](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.Bucket.html)
 * [*delete*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketDeleteCall.html), [*get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketGetCall.html), [*get iam policy*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketGetIamPolicyCall.html), [*insert*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketInsertCall.html), [*list*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketListCall.html), [*lock retention policy*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketLockRetentionPolicyCall.html), [*patch*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketPatchCall.html), [*set iam policy*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketSetIamPolicyCall.html), [*test iam permissions*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketTestIamPermissionCall.html) and [*update*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.BucketUpdateCall.html)
* [channels](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.Channel.html)
 * [*stop*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ChannelStopCall.html)
* default object access controls
 * [*delete*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.DefaultObjectAccessControlDeleteCall.html), [*get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.DefaultObjectAccessControlGetCall.html), [*insert*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.DefaultObjectAccessControlInsertCall.html), [*list*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.DefaultObjectAccessControlListCall.html), [*patch*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.DefaultObjectAccessControlPatchCall.html) and [*update*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.DefaultObjectAccessControlUpdateCall.html)
* [notifications](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.Notification.html)
 * [*delete*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.NotificationDeleteCall.html), [*get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.NotificationGetCall.html), [*insert*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.NotificationInsertCall.html) and [*list*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.NotificationListCall.html)
* [object access controls](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControl.html)
 * [*delete*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControlDeleteCall.html), [*get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControlGetCall.html), [*insert*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControlInsertCall.html), [*list*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControlListCall.html), [*patch*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControlPatchCall.html) and [*update*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectAccessControlUpdateCall.html)
* [objects](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.Object.html)
 * [*compose*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectComposeCall.html), [*copy*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectCopyCall.html), [*delete*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectDeleteCall.html), [*get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectGetCall.html), [*get iam policy*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectGetIamPolicyCall.html), [*insert*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectInsertCall.html), [*list*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectListCall.html), [*patch*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectPatchCall.html), [*rewrite*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectRewriteCall.html), [*set iam policy*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectSetIamPolicyCall.html), [*test iam permissions*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectTestIamPermissionCall.html), [*update*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectUpdateCall.html) and [*watch all*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectWatchAllCall.html)
* projects
 * [*service account get*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ProjectServiceAccountGetCall.html)


Upload supported by ...

* [*insert objects*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectInsertCall.html)

Download supported by ...

* [*get objects*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectGetCall.html)

Subscription supported by ...

* [*watch all objects*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectWatchAllCall.html)
* [*list objects*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.ObjectListCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/struct.Storage.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.objects().get_iam_policy(...).doit()
let r = hub.objects().list(...).doit()
let r = hub.objects().rewrite(...).doit()
let r = hub.objects().test_iam_permissions(...).doit()
let r = hub.objects().copy(...).doit()
let r = hub.objects().watch_all(...).doit()
let r = hub.objects().get(...).doit()
let r = hub.objects().set_iam_policy(...).doit()
let r = hub.objects().insert(...).doit()
let r = hub.objects().compose(...).doit()
let r = hub.objects().update(...).doit()
let r = hub.objects().delete(...).doit()
let r = hub.objects().patch(...).doit()
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
google-storage1 = "*"
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
extern crate google_storage1 as storage1;
use storage1::Object;
use storage1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use storage1::Storage;

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
let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Object::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
             .user_project("et")
             .source_generation("dolores")
             .rewrite_token("kasd")
             .projection("accusam")
             .max_bytes_rewritten_per_call("takimata")
             .if_source_metageneration_not_match("justo")
             .if_source_metageneration_match("amet.")
             .if_source_generation_not_match("erat")
             .if_source_generation_match("labore")
             .if_metageneration_not_match("sea")
             .if_metageneration_match("nonumy")
             .if_generation_not_match("dolores")
             .if_generation_match("gubergren")
             .destination_predefined_acl("sadipscing")
             .destination_kms_key_name("aliquyam")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-storage1/1.0.8+20190226/google_storage1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **storage1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
