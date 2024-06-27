<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-drive2` library allows access to all features of the *Google drive* service.

This documentation was generated from *drive* crate version *5.0.5+20240618*, where *20240618* is the exact revision of the *drive:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *drive* *v2* API can be found at the
[official documentation site](https://developers.google.com/drive/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/DriveHub) ... 

* [about](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::About)
 * [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::AboutGetCall)
* [apps](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::App)
 * [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::AppGetCall) and [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::AppListCall)
* [changes](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Change)
 * [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChangeGetCall), [*get start page token*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChangeGetStartPageTokenCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChangeListCall) and [*watch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChangeWatchCall)
* [channels](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Channel)
 * [*stop*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChannelStopCall)
* children
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChildDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChildGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChildInsertCall) and [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChildListCall)
* [comments](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Comment)
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::CommentDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::CommentGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::CommentInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::CommentListCall), [*patch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::CommentPatchCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::CommentUpdateCall)
* [drives](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Drive)
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveGetCall), [*hide*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveHideCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveListCall), [*unhide*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveUnhideCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::DriveUpdateCall)
* [files](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::File)
 * [*copy*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileCopyCall), [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileDeleteCall), [*empty trash*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileEmptyTrashCall), [*export*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileExportCall), [*generate ids*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileGenerateIdCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileListCall), [*list labels*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileListLabelCall), [*modify labels*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileModifyLabelCall), [*patch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FilePatchCall), [*touch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileTouchCall), [*trash*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileTrashCall), [*untrash*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileUntrashCall), [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileUpdateCall) and [*watch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileWatchCall)
* parents
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ParentDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ParentGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ParentInsertCall) and [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ParentListCall)
* [permissions](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Permission)
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionGetCall), [*get id for email*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionGetIdForEmailCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionListCall), [*patch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionPatchCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PermissionUpdateCall)
* [properties](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Property)
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PropertyDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PropertyGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PropertyInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PropertyListCall), [*patch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PropertyPatchCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::PropertyUpdateCall)
* replies
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ReplyDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ReplyGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ReplyInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ReplyListCall), [*patch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ReplyPatchCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ReplyUpdateCall)
* [revisions](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::Revision)
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::RevisionDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::RevisionGetCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::RevisionListCall), [*patch*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::RevisionPatchCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::RevisionUpdateCall)
* teamdrives
 * [*delete*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::TeamdriveDeleteCall), [*get*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::TeamdriveGetCall), [*insert*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::TeamdriveInsertCall), [*list*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::TeamdriveListCall) and [*update*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::TeamdriveUpdateCall)


Upload supported by ...

* [*insert files*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileInsertCall)
* [*update files*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileUpdateCall)

Download supported by ...

* [*export files*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileExportCall)
* [*get files*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileGetCall)

Subscription supported by ...

* [*list changes*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChangeListCall)
* [*watch changes*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::ChangeWatchCall)
* [*get files*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileGetCall)
* [*watch files*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/api::FileWatchCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/DriveHub)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::CallBuilder)
* **[Resources](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.files().copy(...).doit().await
let r = hub.files().delete(...).doit().await
let r = hub.files().empty_trash(...).doit().await
let r = hub.files().export(...).doit().await
let r = hub.files().generate_ids(...).doit().await
let r = hub.files().get(...).doit().await
let r = hub.files().insert(...).doit().await
let r = hub.files().list(...).doit().await
let r = hub.files().list_labels(...).doit().await
let r = hub.files().modify_labels(...).doit().await
let r = hub.files().patch(...).doit().await
let r = hub.files().touch(...).doit().await
let r = hub.files().trash(...).doit().await
let r = hub.files().untrash(...).doit().await
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
google-drive2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_drive2 as drive2;
use drive2::api::File;
use drive2::{Result, Error};
use std::default::Default;
use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = File::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.files().patch(req, "fileId")
             .use_content_as_indexable_text(true)
             .update_viewed_date(false)
             .timed_text_track_name("amet.")
             .timed_text_language("takimata")
             .supports_team_drives(true)
             .supports_all_drives(true)
             .set_modified_date(true)
             .remove_parents("Lorem")
             .pinned(false)
             .ocr_language("dolor")
             .ocr(true)
             .new_revision(false)
             .modified_date_behavior("amet")
             .include_permissions_for_view("duo")
             .include_labels("ipsum")
             .enforce_single_parent(false)
             .convert(true)
             .add_parents("ipsum")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::RequestValue) and 
[decodable](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-drive2/5.0.5+20240618/google_drive2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **drive2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

