<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-drive2` library allows access to all features of the *Google drive* service.

This documentation was generated from *drive* crate version *1.0.8+20190328*, where *20190328* is the exact revision of the *drive:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.

Everything else about the *drive* *v2* API can be found at the
[official documentation site](https://developers.google.com/drive/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Drive.html) ... 

* [about](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.About.html)
 * [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.AboutGetCall.html)
* [apps](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.App.html)
 * [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.AppGetCall.html) and [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.AppListCall.html)
* [changes](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Change.html)
 * [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChangeGetCall.html), [*get start page token*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChangeGetStartPageTokenCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChangeListCall.html) and [*watch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChangeWatchCall.html)
* [channels](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Channel.html)
 * [*stop*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChannelStopCall.html)
* children
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChildrenDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChildrenGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChildrenInsertCall.html) and [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChildrenListCall.html)
* [comments](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Comment.html)
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.CommentDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.CommentGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.CommentInsertCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.CommentListCall.html), [*patch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.CommentPatchCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.CommentUpdateCall.html)
* [files](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.File.html)
 * [*copy*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileCopyCall.html), [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileDeleteCall.html), [*empty trash*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileEmptyTrashCall.html), [*export*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileExportCall.html), [*generate ids*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileGenerateIdCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileInsertCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileListCall.html), [*patch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FilePatchCall.html), [*touch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileTouchCall.html), [*trash*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileTrashCall.html), [*untrash*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileUntrashCall.html), [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileUpdateCall.html) and [*watch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileWatchCall.html)
* parents
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ParentDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ParentGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ParentInsertCall.html) and [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ParentListCall.html)
* [permissions](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Permission.html)
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionGetCall.html), [*get id for email*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionGetIdForEmailCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionInsertCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionListCall.html), [*patch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionPatchCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PermissionUpdateCall.html)
* [properties](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Property.html)
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PropertyDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PropertyGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PropertyInsertCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PropertyListCall.html), [*patch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PropertyPatchCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.PropertyUpdateCall.html)
* realtime
 * [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RealtimeGetCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RealtimeUpdateCall.html)
* replies
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ReplyDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ReplyGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ReplyInsertCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ReplyListCall.html), [*patch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ReplyPatchCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ReplyUpdateCall.html)
* [revisions](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Revision.html)
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RevisionDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RevisionGetCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RevisionListCall.html), [*patch*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RevisionPatchCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RevisionUpdateCall.html)
* teamdrives
 * [*delete*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.TeamdriveDeleteCall.html), [*get*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.TeamdriveGetCall.html), [*insert*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.TeamdriveInsertCall.html), [*list*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.TeamdriveListCall.html) and [*update*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.TeamdriveUpdateCall.html)


Upload supported by ...

* [*update files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileUpdateCall.html)
* [*insert files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileInsertCall.html)
* [*update realtime*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RealtimeUpdateCall.html)

Download supported by ...

* [*watch files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileWatchCall.html)
* [*get realtime*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.RealtimeGetCall.html)
* [*get files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileGetCall.html)
* [*export files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileExportCall.html)

Subscription supported by ...

* [*watch files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileWatchCall.html)
* [*insert files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileInsertCall.html)
* [*get files*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.FileGetCall.html)
* [*watch changes*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChangeWatchCall.html)
* [*list changes*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.ChangeListCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/struct.Drive.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.files().watch(...).doit()
let r = hub.files().empty_trash(...).doit()
let r = hub.files().generate_ids(...).doit()
let r = hub.files().copy(...).doit()
let r = hub.files().list(...).doit()
let r = hub.files().delete(...).doit()
let r = hub.files().patch(...).doit()
let r = hub.files().update(...).doit()
let r = hub.files().insert(...).doit()
let r = hub.files().untrash(...).doit()
let r = hub.files().trash(...).doit()
let r = hub.files().touch(...).doit()
let r = hub.files().get(...).doit()
let r = hub.files().export(...).doit()
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
extern crate google_drive2 as drive2;
use drive2::File;
use drive2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use drive2::Drive;

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
let mut hub = Drive::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
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
             .timed_text_track_name("sed")
             .timed_text_language("et")
             .supports_team_drives(true)
             .set_modified_date(false)
             .remove_parents("accusam")
             .pinned(true)
             .ocr_language("justo")
             .ocr(true)
             .new_revision(false)
             .modified_date_behavior("labore")
             .convert(true)
             .add_parents("nonumy")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-drive2/1.0.8+20190328/google_drive2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **drive2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
