<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-youtube3` library allows access to all features of the *Google YouTube* service.

This documentation was generated from *YouTube* crate version *5.0.5+20240626*, where *20240626* is the exact revision of the *youtube:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *YouTube* *v3* API can be found at the
[official documentation site](https://developers.google.com/youtube/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/YouTube) ... 

* [abuse reports](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::AbuseReport)
 * [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::AbuseReportInsertCall)
* [activities](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Activity)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ActivityListCall)
* [captions](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Caption)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionDeleteCall), [*download*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionDownloadCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionUpdateCall)
* channel banners
 * [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelBannerInsertCall)
* [channel sections](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelSection)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelSectionDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelSectionInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelSectionListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelSectionUpdateCall)
* [channels](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Channel)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelUpdateCall)
* [comment threads](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentThread)
 * [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentThreadInsertCall) and [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentThreadListCall)
* [comments](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Comment)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentListCall), [*mark as spam*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentMarkAsSpamCall), [*set moderation status*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentSetModerationStatuCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CommentUpdateCall)
* [i18n languages](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::I18nLanguage)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::I18nLanguageListCall)
* [i18n regions](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::I18nRegion)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::I18nRegionListCall)
* [live broadcasts](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcast)
 * [*bind*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastBindCall), [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastInsertCall), [*insert cuepoint*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastInsertCuepointCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastListCall), [*transition*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastTransitionCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveBroadcastUpdateCall)
* [live chat bans](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatBan)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatBanDeleteCall) and [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatBanInsertCall)
* [live chat messages](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatMessage)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatMessageDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatMessageInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatMessageListCall) and [*transition*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatMessageTransitionCall)
* [live chat moderators](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatModerator)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatModeratorDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatModeratorInsertCall) and [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveChatModeratorListCall)
* [live streams](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveStream)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveStreamDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveStreamInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveStreamListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::LiveStreamUpdateCall)
* [members](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Member)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::MemberListCall)
* [memberships levels](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::MembershipsLevel)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::MembershipsLevelListCall)
* [playlist images](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImage)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImageDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImageInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImageListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImageUpdateCall)
* [playlist items](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistItem)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistItemDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistItemInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistItemListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistItemUpdateCall)
* [playlists](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Playlist)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistUpdateCall)
* search
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::SearchListCall)
* [subscriptions](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Subscription)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::SubscriptionDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::SubscriptionInsertCall) and [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::SubscriptionListCall)
* [super chat events](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::SuperChatEvent)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::SuperChatEventListCall)
* tests
 * [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::TestInsertCall)
* [third party links](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThirdPartyLink)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThirdPartyLinkDeleteCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThirdPartyLinkInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThirdPartyLinkListCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThirdPartyLinkUpdateCall)
* [thumbnails](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Thumbnail)
 * [*set*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThumbnailSetCall)
* [video abuse report reasons](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoAbuseReportReason)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoAbuseReportReasonListCall)
* [video categories](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoCategory)
 * [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoCategoryListCall)
* [videos](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::Video)
 * [*delete*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoDeleteCall), [*get rating*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoGetRatingCall), [*insert*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoInsertCall), [*list*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoListCall), [*rate*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoRateCall), [*report abuse*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoReportAbuseCall) and [*update*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoUpdateCall)
* watermarks
 * [*set*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::WatermarkSetCall) and [*unset*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::WatermarkUnsetCall)
* youtube
 * [*v3 update comment threads*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::YoutubeV3UpdateCommentThreadCall)


Upload supported by ...

* [*insert captions*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionInsertCall)
* [*update captions*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionUpdateCall)
* [*insert channel banners*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ChannelBannerInsertCall)
* [*insert playlist images*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImageInsertCall)
* [*update playlist images*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::PlaylistImageUpdateCall)
* [*set thumbnails*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::ThumbnailSetCall)
* [*insert videos*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::VideoInsertCall)
* [*set watermarks*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::WatermarkSetCall)

Download supported by ...

* [*download captions*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/api::CaptionDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/YouTube)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::CallBuilder)
* **[Resources](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.videos().delete(...).doit().await
let r = hub.videos().get_rating(...).doit().await
let r = hub.videos().insert(...).doit().await
let r = hub.videos().list(...).doit().await
let r = hub.videos().rate(...).doit().await
let r = hub.videos().report_abuse(...).doit().await
let r = hub.videos().update(...).doit().await
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
google-youtube3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_youtube3 as youtube3;
use youtube3::{Result, Error};
use std::default::Default;
use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.videos().list(&vec!["et".into()])
             .video_category_id("magna")
             .region_code("no")
             .page_token("ipsum")
             .on_behalf_of_content_owner("voluptua.")
             .my_rating("At")
             .max_width(-8)
             .max_results(21)
             .max_height(-2)
             .locale("takimata")
             .add_id("amet.")
             .hl("duo")
             .chart("ipsum")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::RequestValue) and 
[decodable](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-youtube3/5.0.5+20240626/google_youtube3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **youtube3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

