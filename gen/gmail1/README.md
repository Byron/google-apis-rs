<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-gmail1` library allows access to all features of the *Google Gmail* service.

This documentation was generated from *Gmail* crate version *5.0.5+20240624*, where *20240624* is the exact revision of the *gmail:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Gmail* *v1* API can be found at the
[official documentation site](https://developers.google.com/gmail/api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/Gmail) ... 

* users
 * [*drafts create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftCreateCall), [*drafts delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftDeleteCall), [*drafts get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftGetCall), [*drafts list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftListCall), [*drafts send*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftSendCall), [*drafts update*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftUpdateCall), [*get profile*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserGetProfileCall), [*history list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserHistoryListCall), [*labels create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserLabelCreateCall), [*labels delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserLabelDeleteCall), [*labels get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserLabelGetCall), [*labels list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserLabelListCall), [*labels patch*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserLabelPatchCall), [*labels update*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserLabelUpdateCall), [*messages attachments get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageAttachmentGetCall), [*messages batch delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageBatchDeleteCall), [*messages batch modify*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageBatchModifyCall), [*messages delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageDeleteCall), [*messages get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageGetCall), [*messages import*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageImportCall), [*messages insert*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageInsertCall), [*messages list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageListCall), [*messages modify*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageModifyCall), [*messages send*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageSendCall), [*messages trash*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageTrashCall), [*messages untrash*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageUntrashCall), [*settings cse identities create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseIdentityCreateCall), [*settings cse identities delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseIdentityDeleteCall), [*settings cse identities get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseIdentityGetCall), [*settings cse identities list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseIdentityListCall), [*settings cse identities patch*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseIdentityPatchCall), [*settings cse keypairs create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseKeypairCreateCall), [*settings cse keypairs disable*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseKeypairDisableCall), [*settings cse keypairs enable*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseKeypairEnableCall), [*settings cse keypairs get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseKeypairGetCall), [*settings cse keypairs list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseKeypairListCall), [*settings cse keypairs obliterate*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingCseKeypairObliterateCall), [*settings delegates create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingDelegateCreateCall), [*settings delegates delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingDelegateDeleteCall), [*settings delegates get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingDelegateGetCall), [*settings delegates list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingDelegateListCall), [*settings filters create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingFilterCreateCall), [*settings filters delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingFilterDeleteCall), [*settings filters get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingFilterGetCall), [*settings filters list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingFilterListCall), [*settings forwarding addresses create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingForwardingAddressCreateCall), [*settings forwarding addresses delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingForwardingAddressDeleteCall), [*settings forwarding addresses get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingForwardingAddressGetCall), [*settings forwarding addresses list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingForwardingAddressListCall), [*settings get auto forwarding*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingGetAutoForwardingCall), [*settings get imap*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingGetImapCall), [*settings get language*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingGetLanguageCall), [*settings get pop*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingGetPopCall), [*settings get vacation*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingGetVacationCall), [*settings send as create*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendACreateCall), [*settings send as delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendADeleteCall), [*settings send as get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendAGetCall), [*settings send as list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendAListCall), [*settings send as patch*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendAPatchCall), [*settings send as smime info delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendASmimeInfoDeleteCall), [*settings send as smime info get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendASmimeInfoGetCall), [*settings send as smime info insert*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendASmimeInfoInsertCall), [*settings send as smime info list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendASmimeInfoListCall), [*settings send as smime info set default*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendASmimeInfoSetDefaultCall), [*settings send as update*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendAUpdateCall), [*settings send as verify*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingSendAVerifyCall), [*settings update auto forwarding*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingUpdateAutoForwardingCall), [*settings update imap*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingUpdateImapCall), [*settings update language*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingUpdateLanguageCall), [*settings update pop*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingUpdatePopCall), [*settings update vacation*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserSettingUpdateVacationCall), [*stop*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserStopCall), [*threads delete*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserThreadDeleteCall), [*threads get*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserThreadGetCall), [*threads list*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserThreadListCall), [*threads modify*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserThreadModifyCall), [*threads trash*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserThreadTrashCall), [*threads untrash*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserThreadUntrashCall) and [*watch*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserWatchCall)


Upload supported by ...

* [*drafts create users*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftCreateCall)
* [*drafts send users*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftSendCall)
* [*drafts update users*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserDraftUpdateCall)
* [*messages import users*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageImportCall)
* [*messages insert users*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageInsertCall)
* [*messages send users*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/api::UserMessageSendCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/Gmail)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::CallBuilder)
* **[Resources](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.users().drafts_send(...).doit().await
let r = hub.users().messages_get(...).doit().await
let r = hub.users().messages_import(...).doit().await
let r = hub.users().messages_insert(...).doit().await
let r = hub.users().messages_modify(...).doit().await
let r = hub.users().messages_send(...).doit().await
let r = hub.users().messages_trash(...).doit().await
let r = hub.users().messages_untrash(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_gmail1 as gmail1;
use gmail1::api::Message;
use gmail1::{Result, Error};
use std::fs;
use std::default::Default;
use gmail1::{Gmail, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Message::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `upload_resumable(...)`.
// Values shown here are possibly random and not representative !
let result = hub.users().messages_import(req, "userId")
             .process_for_calendar(true)
             .never_mark_spam(false)
             .internal_date_source("amet.")
             .deleted(true)
             .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;

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

All errors produced by the system are provided either as [Result](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::RequestValue) and 
[decodable](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-gmail1/5.0.5+20240624/google_gmail1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **gmail1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

