<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-chat1` library allows access to all features of the *Google Hangouts Chat* service.

This documentation was generated from *Hangouts Chat* crate version *3.0.0+20220302*, where *20220302* is the exact revision of the *chat:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v3.0.0*.

Everything else about the *Hangouts Chat* *v1* API can be found at the
[official documentation site](https://developers.google.com/hangouts/chat).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/HangoutsChat) ... 

* dms
 * [*conversations messages*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::DmConversationMessageCall), [*messages*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::DmMessageCall) and [*webhooks*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::DmWebhookCall)
* [media](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::Media)
 * [*download*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::MediaDownloadCall)
* rooms
 * [*conversations messages*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::RoomConversationMessageCall), [*messages*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::RoomMessageCall) and [*webhooks*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::RoomWebhookCall)
* [spaces](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::Space)
 * [*get*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceGetCall), [*list*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceListCall), [*members get*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMemberGetCall), [*members list*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMemberListCall), [*messages attachments get*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMessageAttachmentGetCall), [*messages create*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMessageCreateCall), [*messages delete*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMessageDeleteCall), [*messages get*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMessageGetCall), [*messages update*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceMessageUpdateCall) and [*webhooks*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::SpaceWebhookCall)


Download supported by ...

* [*download media*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/api::MediaDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/HangoutsChat)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::CallBuilder)
* **[Resources](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.dms().conversations_messages(...).doit().await
let r = hub.dms().messages(...).doit().await
let r = hub.dms().webhooks(...).doit().await
let r = hub.rooms().conversations_messages(...).doit().await
let r = hub.rooms().messages(...).doit().await
let r = hub.rooms().webhooks(...).doit().await
let r = hub.spaces().messages_create(...).doit().await
let r = hub.spaces().messages_get(...).doit().await
let r = hub.spaces().messages_update(...).doit().await
let r = hub.spaces().webhooks(...).doit().await
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
google-chat1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_chat1 as chat1;
use chat1::api::Message;
use chat1::{Result, Error};
use std::default::Default;
use chat1::{HangoutsChat, oauth2, hyper, hyper_rustls};

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
let mut hub = HangoutsChat::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Message::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.dms().conversations_messages(req, "parent")
             .thread_key("magna")
             .request_id("no")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::RequestValue) and 
[decodable](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-chat1/3.0.0+20220302/google_chat1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **chat1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md
