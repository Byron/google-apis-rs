<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-drive3` library allows access to all features of the *Google drive* service.

This documentation was generated from *drive* crate version *5.0.5+20240618*, where *20240618* is the exact revision of the *drive:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *drive* *v3* API can be found at the
[official documentation site](https://developers.google.com/drive/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/DriveHub) ... 

* [about](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::About)
 * [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::AboutGetCall)
* [apps](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::App)
 * [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::AppGetCall) and [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::AppListCall)
* [changes](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Change)
 * [*get start page token*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ChangeGetStartPageTokenCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ChangeListCall) and [*watch*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ChangeWatchCall)
* [channels](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Channel)
 * [*stop*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ChannelStopCall)
* [comments](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Comment)
 * [*create*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::CommentCreateCall), [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::CommentDeleteCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::CommentGetCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::CommentListCall) and [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::CommentUpdateCall)
* [drives](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Drive)
 * [*create*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveCreateCall), [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveDeleteCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveGetCall), [*hide*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveHideCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveListCall), [*unhide*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveUnhideCall) and [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::DriveUpdateCall)
* [files](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::File)
 * [*copy*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileCopyCall), [*create*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileCreateCall), [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileDeleteCall), [*empty trash*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileEmptyTrashCall), [*export*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileExportCall), [*generate ids*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileGenerateIdCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileGetCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileListCall), [*list labels*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileListLabelCall), [*modify labels*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileModifyLabelCall), [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileUpdateCall) and [*watch*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileWatchCall)
* [permissions](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Permission)
 * [*create*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::PermissionCreateCall), [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::PermissionDeleteCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::PermissionGetCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::PermissionListCall) and [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::PermissionUpdateCall)
* [replies](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Reply)
 * [*create*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ReplyCreateCall), [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ReplyDeleteCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ReplyGetCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ReplyListCall) and [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ReplyUpdateCall)
* [revisions](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::Revision)
 * [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::RevisionDeleteCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::RevisionGetCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::RevisionListCall) and [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::RevisionUpdateCall)
* teamdrives
 * [*create*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::TeamdriveCreateCall), [*delete*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::TeamdriveDeleteCall), [*get*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::TeamdriveGetCall), [*list*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::TeamdriveListCall) and [*update*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::TeamdriveUpdateCall)


Upload supported by ...

* [*create files*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileCreateCall)
* [*update files*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileUpdateCall)

Download supported by ...

* [*export files*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileExportCall)
* [*get files*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileGetCall)
* [*get revisions*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::RevisionGetCall)

Subscription supported by ...

* [*list changes*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ChangeListCall)
* [*watch changes*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::ChangeWatchCall)
* [*get files*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileGetCall)
* [*watch files*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/api::FileWatchCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/DriveHub)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::CallBuilder)
* **[Resources](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.files().copy(...).doit().await
let r = hub.files().create(...).doit().await
let r = hub.files().delete(...).doit().await
let r = hub.files().empty_trash(...).doit().await
let r = hub.files().export(...).doit().await
let r = hub.files().generate_ids(...).doit().await
let r = hub.files().get(...).doit().await
let r = hub.files().list(...).doit().await
let r = hub.files().list_labels(...).doit().await
let r = hub.files().modify_labels(...).doit().await
let r = hub.files().update(...).doit().await
let r = hub.files().watch(...).doit().await
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
google-drive3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_drive3 as drive3;
use drive3::{Result, Error};
use std::default::Default;
use drive3::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.files().list()
             .team_drive_id("et")
             .supports_team_drives(true)
             .supports_all_drives(false)
             .spaces("amet.")
             .q("takimata")
             .page_token("amet.")
             .page_size(-20)
             .order_by("ipsum")
             .include_team_drive_items(true)
             .include_permissions_for_view("Lorem")
             .include_labels("gubergren")
             .include_items_from_all_drives(false)
             .drive_id("dolor")
             .corpus("ea")
             .corpora("ipsum")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::RequestValue) and 
[decodable](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-drive3/5.0.5+20240618/google_drive3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **drive3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

