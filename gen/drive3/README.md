<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-drive3` library allows access to all features of the *Google drive* service.

This documentation was generated from *drive* crate version *1.0.13+20200326*, where *20200326* is the exact revision of the *drive:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *drive* *v3* API can be found at the
[official documentation site](https://developers.google.com/drive/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveHub.html) ... 

* [about](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.About.html)
 * [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.AboutGetCall.html)
* [changes](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Change.html)
 * [*get start page token*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ChangeGetStartPageTokenCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ChangeListCall.html) and [*watch*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ChangeWatchCall.html)
* [channels](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Channel.html)
 * [*stop*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ChannelStopCall.html)
* [comments](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Comment.html)
 * [*create*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.CommentCreateCall.html), [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.CommentDeleteCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.CommentGetCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.CommentListCall.html) and [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.CommentUpdateCall.html)
* [drives](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Drive.html)
 * [*create*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveCreateCall.html), [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveDeleteCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveGetCall.html), [*hide*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveHideCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveListCall.html), [*unhide*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveUnhideCall.html) and [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveUpdateCall.html)
* [files](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.File.html)
 * [*copy*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileCopyCall.html), [*create*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileCreateCall.html), [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileDeleteCall.html), [*empty trash*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileEmptyTrashCall.html), [*export*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileExportCall.html), [*generate ids*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileGenerateIdCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileGetCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileListCall.html), [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileUpdateCall.html) and [*watch*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileWatchCall.html)
* [permissions](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Permission.html)
 * [*create*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.PermissionCreateCall.html), [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.PermissionDeleteCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.PermissionGetCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.PermissionListCall.html) and [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.PermissionUpdateCall.html)
* [replies](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Reply.html)
 * [*create*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ReplyCreateCall.html), [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ReplyDeleteCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ReplyGetCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ReplyListCall.html) and [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ReplyUpdateCall.html)
* [revisions](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.Revision.html)
 * [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.RevisionDeleteCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.RevisionGetCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.RevisionListCall.html) and [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.RevisionUpdateCall.html)
* teamdrives
 * [*create*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.TeamdriveCreateCall.html), [*delete*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.TeamdriveDeleteCall.html), [*get*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.TeamdriveGetCall.html), [*list*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.TeamdriveListCall.html) and [*update*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.TeamdriveUpdateCall.html)


Upload supported by ...

* [*update files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileUpdateCall.html)
* [*create files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileCreateCall.html)

Download supported by ...

* [*watch files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileWatchCall.html)
* [*export files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileExportCall.html)
* [*get revisions*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.RevisionGetCall.html)
* [*get files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileGetCall.html)

Subscription supported by ...

* [*watch files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileWatchCall.html)
* [*list changes*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ChangeListCall.html)
* [*create files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileCreateCall.html)
* [*get files*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.FileGetCall.html)
* [*watch changes*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.ChangeWatchCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/struct.DriveHub.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.CallBuilder.html)**
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
let r = hub.files().create(...).doit()
let r = hub.files().list(...).doit()
let r = hub.files().delete(...).doit()
let r = hub.files().update(...).doit()
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
google-drive3 = "*"
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
extern crate google_drive3 as drive3;
use drive3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use drive3::DriveHub;

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
let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.files().list()
             .team_drive_id("eirmod")
             .supports_team_drives(true)
             .supports_all_drives(false)
             .spaces("sed")
             .q("et")
             .page_token("dolores")
             .page_size(-63)
             .order_by("accusam")
             .include_team_drive_items(true)
             .include_items_from_all_drives(false)
             .drive_id("amet.")
             .corpus("erat")
             .corpora("labore")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-drive3/1.0.13+20200326/google_drive3/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **drive3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
