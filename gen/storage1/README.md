<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-storage1` library allows access to all features of the *Google storage* service.

This documentation was generated from *storage* crate version *5.0.5+20240621*, where *20240621* is the exact revision of the *storage:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *storage* *v1* API can be found at the
[official documentation site](https://developers.google.com/storage/docs/json_api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/Storage) ... 

* anywhere caches
 * [*disable*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachDisableCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachGetCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachListCall), [*pause*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachPauseCall), [*resume*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachResumeCall) and [*update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::AnywhereCachUpdateCall)
* [bucket access controls](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControl)
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControlDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControlGetCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControlInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControlListCall), [*patch*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControlPatchCall) and [*update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketAccessControlUpdateCall)
* [buckets](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::Bucket)
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketGetCall), [*get iam policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketGetIamPolicyCall), [*get storage layout*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketGetStorageLayoutCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketListCall), [*lock retention policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketLockRetentionPolicyCall), [*operations cancel*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketOperationCancelCall), [*operations get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketOperationGetCall), [*operations list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketOperationListCall), [*patch*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketPatchCall), [*set iam policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketSetIamPolicyCall), [*test iam permissions*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketTestIamPermissionCall) and [*update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::BucketUpdateCall)
* [channels](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::Channel)
 * [*stop*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ChannelStopCall)
* default object access controls
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::DefaultObjectAccessControlDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::DefaultObjectAccessControlGetCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::DefaultObjectAccessControlInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::DefaultObjectAccessControlListCall), [*patch*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::DefaultObjectAccessControlPatchCall) and [*update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::DefaultObjectAccessControlUpdateCall)
* [folders](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::Folder)
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::FolderDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::FolderGetCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::FolderInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::FolderListCall) and [*rename*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::FolderRenameCall)
* [managed folders](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolder)
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderGetCall), [*get iam policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderGetIamPolicyCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderListCall), [*set iam policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ManagedFolderTestIamPermissionCall)
* [notifications](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::Notification)
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::NotificationDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::NotificationGetCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::NotificationInsertCall) and [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::NotificationListCall)
* [object access controls](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControl)
 * [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControlDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControlGetCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControlInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControlListCall), [*patch*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControlPatchCall) and [*update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectAccessControlUpdateCall)
* [objects](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::Object)
 * [*bulk restore*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectBulkRestoreCall), [*compose*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectComposeCall), [*copy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectCopyCall), [*delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectDeleteCall), [*get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectGetCall), [*get iam policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectGetIamPolicyCall), [*insert*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectInsertCall), [*list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectListCall), [*patch*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectPatchCall), [*restore*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectRestoreCall), [*rewrite*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectRewriteCall), [*set iam policy*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectSetIamPolicyCall), [*test iam permissions*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectTestIamPermissionCall), [*update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectUpdateCall) and [*watch all*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectWatchAllCall)
* projects
 * [*hmac keys create*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ProjectHmacKeyCreateCall), [*hmac keys delete*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ProjectHmacKeyDeleteCall), [*hmac keys get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ProjectHmacKeyGetCall), [*hmac keys list*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ProjectHmacKeyListCall), [*hmac keys update*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ProjectHmacKeyUpdateCall) and [*service account get*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ProjectServiceAccountGetCall)


Upload supported by ...

* [*insert objects*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectInsertCall)

Download supported by ...

* [*get objects*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectGetCall)

Subscription supported by ...

* [*list objects*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectListCall)
* [*watch all objects*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/api::ObjectWatchAllCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/Storage)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::CallBuilder)
* **[Resources](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.objects().bulk_restore(...).doit().await
let r = hub.objects().compose(...).doit().await
let r = hub.objects().copy(...).doit().await
let r = hub.objects().delete(...).doit().await
let r = hub.objects().get(...).doit().await
let r = hub.objects().get_iam_policy(...).doit().await
let r = hub.objects().insert(...).doit().await
let r = hub.objects().list(...).doit().await
let r = hub.objects().patch(...).doit().await
let r = hub.objects().restore(...).doit().await
let r = hub.objects().rewrite(...).doit().await
let r = hub.objects().set_iam_policy(...).doit().await
let r = hub.objects().test_iam_permissions(...).doit().await
let r = hub.objects().update(...).doit().await
let r = hub.objects().watch_all(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_storage1 as storage1;
use storage1::api::Object;
use storage1::{Result, Error};
use std::default::Default;
use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Object::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
             .user_project("voluptua.")
             .source_generation(-27)
             .rewrite_token("sanctus")
             .projection("sed")
             .max_bytes_rewritten_per_call(-2)
             .if_source_metageneration_not_match(-59)
             .if_source_metageneration_match(-52)
             .if_source_generation_not_match(-20)
             .if_source_generation_match(-55)
             .if_metageneration_not_match(-62)
             .if_metageneration_match(-51)
             .if_generation_not_match(-12)
             .if_generation_match(-75)
             .destination_predefined_acl("dolor")
             .destination_kms_key_name("ea")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::RequestValue) and 
[decodable](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-storage1/5.0.5+20240621/google_storage1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **storage1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

