<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-gmail1` library allows access to all features of the *Google gmail* service.

This documentation was generated from *gmail* crate version *1.0.7+20180904*, where *20180904* is the exact revision of the *gmail:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.7*.

Everything else about the *gmail* *v1* API can be found at the
[official documentation site](https://developers.google.com/gmail/api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.Gmail.html) ... 

* users
 * [*drafts create*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftCreateCall.html), [*drafts delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftDeleteCall.html), [*drafts get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftGetCall.html), [*drafts list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftListCall.html), [*drafts send*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftSendCall.html), [*drafts update*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftUpdateCall.html), [*get profile*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserGetProfileCall.html), [*history list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserHistoryListCall.html), [*labels create*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserLabelCreateCall.html), [*labels delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserLabelDeleteCall.html), [*labels get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserLabelGetCall.html), [*labels list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserLabelListCall.html), [*labels patch*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserLabelPatchCall.html), [*labels update*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserLabelUpdateCall.html), [*messages attachments get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageAttachmentGetCall.html), [*messages batch delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageBatchDeleteCall.html), [*messages batch modify*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageBatchModifyCall.html), [*messages delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageDeleteCall.html), [*messages get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageGetCall.html), [*messages import*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageImportCall.html), [*messages insert*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageInsertCall.html), [*messages list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageListCall.html), [*messages modify*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageModifyCall.html), [*messages send*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageSendCall.html), [*messages trash*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageTrashCall.html), [*messages untrash*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageUntrashCall.html), [*settings delegates create*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingDelegateCreateCall.html), [*settings delegates delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingDelegateDeleteCall.html), [*settings delegates get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingDelegateGetCall.html), [*settings delegates list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingDelegateListCall.html), [*settings filters create*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingFilterCreateCall.html), [*settings filters delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingFilterDeleteCall.html), [*settings filters get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingFilterGetCall.html), [*settings filters list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingFilterListCall.html), [*settings forwarding addresses create*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingForwardingAddresseCreateCall.html), [*settings forwarding addresses delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingForwardingAddresseDeleteCall.html), [*settings forwarding addresses get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingForwardingAddresseGetCall.html), [*settings forwarding addresses list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingForwardingAddresseListCall.html), [*settings get auto forwarding*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingGetAutoForwardingCall.html), [*settings get imap*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingGetImapCall.html), [*settings get pop*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingGetPopCall.html), [*settings get vacation*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingGetVacationCall.html), [*settings send as create*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendACreateCall.html), [*settings send as delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendADeleteCall.html), [*settings send as get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendAGetCall.html), [*settings send as list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendAListCall.html), [*settings send as patch*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendAPatchCall.html), [*settings send as smime info delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendASmimeInfoDeleteCall.html), [*settings send as smime info get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendASmimeInfoGetCall.html), [*settings send as smime info insert*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendASmimeInfoInsertCall.html), [*settings send as smime info list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendASmimeInfoListCall.html), [*settings send as smime info set default*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendASmimeInfoSetDefaultCall.html), [*settings send as update*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendAUpdateCall.html), [*settings send as verify*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingSendAVerifyCall.html), [*settings update auto forwarding*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingUpdateAutoForwardingCall.html), [*settings update imap*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingUpdateImapCall.html), [*settings update pop*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingUpdatePopCall.html), [*settings update vacation*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserSettingUpdateVacationCall.html), [*stop*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserStopCall.html), [*threads delete*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserThreadDeleteCall.html), [*threads get*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserThreadGetCall.html), [*threads list*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserThreadListCall.html), [*threads modify*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserThreadModifyCall.html), [*threads trash*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserThreadTrashCall.html), [*threads untrash*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserThreadUntrashCall.html) and [*watch*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserWatchCall.html)


Upload supported by ...

* [*messages import users*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageImportCall.html)
* [*drafts create users*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftCreateCall.html)
* [*drafts send users*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftSendCall.html)
* [*messages send users*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageSendCall.html)
* [*drafts update users*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserDraftUpdateCall.html)
* [*messages insert users*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.UserMessageInsertCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/struct.Gmail.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.users().messages_untrash(...).doit()
let r = hub.users().messages_get(...).doit()
let r = hub.users().messages_modify(...).doit()
let r = hub.users().messages_import(...).doit()
let r = hub.users().messages_insert(...).doit()
let r = hub.users().messages_send(...).doit()
let r = hub.users().messages_trash(...).doit()
let r = hub.users().drafts_send(...).doit()
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
google-gmail1 = "*"
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
extern crate google_gmail1 as gmail1;
use gmail1::Message;
use gmail1::{Result, Error};
use std::fs;
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use gmail1::Gmail;

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
let mut hub = Gmail::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Message::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `upload(...)`.
// Values shown here are possibly random and not representative !
let result = hub.users().messages_import(req, "userId")
             .process_for_calendar(true)
             .never_mark_spam(false)
             .internal_date_source("sed")
             .deleted(false)
             .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());

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

All errors produced by the system are provided either as [Result](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-gmail1/1.0.7+20180904/google_gmail1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **gmail1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
