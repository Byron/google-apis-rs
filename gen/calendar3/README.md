<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-calendar3` library allows access to all features of the *Google calendar* service.

This documentation was generated from *calendar* crate version *5.0.5+20240523*, where *20240523* is the exact revision of the *calendar:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *calendar* *v3* API can be found at the
[official documentation site](https://developers.google.com/google-apps/calendar/firstapp).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/CalendarHub) ... 

* [acl](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::Acl)
 * [*delete*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclDeleteCall), [*get*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclGetCall), [*insert*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclInsertCall), [*list*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclListCall), [*patch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclPatchCall), [*update*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclUpdateCall) and [*watch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclWatchCall)
* [calendar list](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarList)
 * [*delete*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListDeleteCall), [*get*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListGetCall), [*insert*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListInsertCall), [*list*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListListCall), [*patch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListPatchCall), [*update*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListUpdateCall) and [*watch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListWatchCall)
* [calendars](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::Calendar)
 * [*clear*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarClearCall), [*delete*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarDeleteCall), [*get*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarGetCall), [*insert*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarInsertCall), [*patch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarPatchCall) and [*update*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarUpdateCall)
* [channels](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::Channel)
 * [*stop*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::ChannelStopCall)
* colors
 * [*get*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::ColorGetCall)
* [events](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::Event)
 * [*delete*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventDeleteCall), [*get*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventGetCall), [*import*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventImportCall), [*insert*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventInsertCall), [*instances*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventInstanceCall), [*list*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventListCall), [*move*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventMoveCall), [*patch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventPatchCall), [*quick add*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventQuickAddCall), [*update*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventUpdateCall) and [*watch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventWatchCall)
* freebusy
 * [*query*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::FreebusyQueryCall)
* [settings](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::Setting)
 * [*get*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::SettingGetCall), [*list*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::SettingListCall) and [*watch*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::SettingWatchCall)


Subscription supported by ...

* [*list acl*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclListCall)
* [*watch acl*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::AclWatchCall)
* [*list calendar list*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListListCall)
* [*watch calendar list*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::CalendarListWatchCall)
* [*instances events*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventInstanceCall)
* [*list events*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventListCall)
* [*watch events*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::EventWatchCall)
* [*list settings*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::SettingListCall)
* [*watch settings*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/api::SettingWatchCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/CalendarHub)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::CallBuilder)
* **[Resources](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.events().delete(...).doit().await
let r = hub.events().get(...).doit().await
let r = hub.events().import(...).doit().await
let r = hub.events().insert(...).doit().await
let r = hub.events().instances(...).doit().await
let r = hub.events().list(...).doit().await
let r = hub.events().move_(...).doit().await
let r = hub.events().patch(...).doit().await
let r = hub.events().quick_add(...).doit().await
let r = hub.events().update(...).doit().await
let r = hub.events().watch(...).doit().await
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
google-calendar3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_calendar3 as calendar3;
use calendar3::api::Channel;
use calendar3::{Result, Error};
use std::default::Default;
use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Channel::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.events().watch(req, "calendarId")
             .updated_min(chrono::Utc::now())
             .time_zone("magna")
             .time_min(chrono::Utc::now())
             .time_max(chrono::Utc::now())
             .sync_token("no")
             .single_events(true)
             .show_hidden_invitations(false)
             .show_deleted(true)
             .add_shared_extended_property("amet.")
             .q("duo")
             .add_private_extended_property("ipsum")
             .page_token("gubergren")
             .order_by("Lorem")
             .max_results(-12)
             .max_attendees(-75)
             .i_cal_uid("dolor")
             .add_event_types("ea")
             .always_include_email(true)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::RequestValue) and 
[decodable](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-calendar3/5.0.5+20240523/google_calendar3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **calendar3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

