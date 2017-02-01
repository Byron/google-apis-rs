// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *YouTube* crate version *1.0.3+20161202*, where *20161202* is the exact revision of the *youtube:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.3*.
//! 
//! Everything else about the *YouTube* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/youtube/v3).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/youtube3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.YouTube.html) ... 
//! 
//! * [activities](struct.Activity.html)
//!  * [*insert*](struct.ActivityInsertCall.html) and [*list*](struct.ActivityListCall.html)
//! * [captions](struct.Caption.html)
//!  * [*delete*](struct.CaptionDeleteCall.html), [*download*](struct.CaptionDownloadCall.html), [*insert*](struct.CaptionInsertCall.html), [*list*](struct.CaptionListCall.html) and [*update*](struct.CaptionUpdateCall.html)
//! * channel banners
//!  * [*insert*](struct.ChannelBannerInsertCall.html)
//! * [channel sections](struct.ChannelSection.html)
//!  * [*delete*](struct.ChannelSectionDeleteCall.html), [*insert*](struct.ChannelSectionInsertCall.html), [*list*](struct.ChannelSectionListCall.html) and [*update*](struct.ChannelSectionUpdateCall.html)
//! * [channels](struct.Channel.html)
//!  * [*list*](struct.ChannelListCall.html) and [*update*](struct.ChannelUpdateCall.html)
//! * [comment threads](struct.CommentThread.html)
//!  * [*insert*](struct.CommentThreadInsertCall.html), [*list*](struct.CommentThreadListCall.html) and [*update*](struct.CommentThreadUpdateCall.html)
//! * [comments](struct.Comment.html)
//!  * [*delete*](struct.CommentDeleteCall.html), [*insert*](struct.CommentInsertCall.html), [*list*](struct.CommentListCall.html), [*mark as spam*](struct.CommentMarkAsSpamCall.html), [*set moderation status*](struct.CommentSetModerationStatuCall.html) and [*update*](struct.CommentUpdateCall.html)
//! * [fan funding events](struct.FanFundingEvent.html)
//!  * [*list*](struct.FanFundingEventListCall.html)
//! * [guide categories](struct.GuideCategory.html)
//!  * [*list*](struct.GuideCategoryListCall.html)
//! * [i18n languages](struct.I18nLanguage.html)
//!  * [*list*](struct.I18nLanguageListCall.html)
//! * [i18n regions](struct.I18nRegion.html)
//!  * [*list*](struct.I18nRegionListCall.html)
//! * [live broadcasts](struct.LiveBroadcast.html)
//!  * [*bind*](struct.LiveBroadcastBindCall.html), [*control*](struct.LiveBroadcastControlCall.html), [*delete*](struct.LiveBroadcastDeleteCall.html), [*insert*](struct.LiveBroadcastInsertCall.html), [*list*](struct.LiveBroadcastListCall.html), [*transition*](struct.LiveBroadcastTransitionCall.html) and [*update*](struct.LiveBroadcastUpdateCall.html)
//! * [live chat bans](struct.LiveChatBan.html)
//!  * [*delete*](struct.LiveChatBanDeleteCall.html) and [*insert*](struct.LiveChatBanInsertCall.html)
//! * [live chat messages](struct.LiveChatMessage.html)
//!  * [*delete*](struct.LiveChatMessageDeleteCall.html), [*insert*](struct.LiveChatMessageInsertCall.html) and [*list*](struct.LiveChatMessageListCall.html)
//! * [live chat moderators](struct.LiveChatModerator.html)
//!  * [*delete*](struct.LiveChatModeratorDeleteCall.html), [*insert*](struct.LiveChatModeratorInsertCall.html) and [*list*](struct.LiveChatModeratorListCall.html)
//! * [live streams](struct.LiveStream.html)
//!  * [*delete*](struct.LiveStreamDeleteCall.html), [*insert*](struct.LiveStreamInsertCall.html), [*list*](struct.LiveStreamListCall.html) and [*update*](struct.LiveStreamUpdateCall.html)
//! * [playlist items](struct.PlaylistItem.html)
//!  * [*delete*](struct.PlaylistItemDeleteCall.html), [*insert*](struct.PlaylistItemInsertCall.html), [*list*](struct.PlaylistItemListCall.html) and [*update*](struct.PlaylistItemUpdateCall.html)
//! * [playlists](struct.Playlist.html)
//!  * [*delete*](struct.PlaylistDeleteCall.html), [*insert*](struct.PlaylistInsertCall.html), [*list*](struct.PlaylistListCall.html) and [*update*](struct.PlaylistUpdateCall.html)
//! * search
//!  * [*list*](struct.SearchListCall.html)
//! * [sponsors](struct.Sponsor.html)
//!  * [*list*](struct.SponsorListCall.html)
//! * [subscriptions](struct.Subscription.html)
//!  * [*delete*](struct.SubscriptionDeleteCall.html), [*insert*](struct.SubscriptionInsertCall.html) and [*list*](struct.SubscriptionListCall.html)
//! * [thumbnails](struct.Thumbnail.html)
//!  * [*set*](struct.ThumbnailSetCall.html)
//! * [video abuse report reasons](struct.VideoAbuseReportReason.html)
//!  * [*list*](struct.VideoAbuseReportReasonListCall.html)
//! * [video categories](struct.VideoCategory.html)
//!  * [*list*](struct.VideoCategoryListCall.html)
//! * [videos](struct.Video.html)
//!  * [*delete*](struct.VideoDeleteCall.html), [*get rating*](struct.VideoGetRatingCall.html), [*insert*](struct.VideoInsertCall.html), [*list*](struct.VideoListCall.html), [*rate*](struct.VideoRateCall.html), [*report abuse*](struct.VideoReportAbuseCall.html) and [*update*](struct.VideoUpdateCall.html)
//! * watermarks
//!  * [*set*](struct.WatermarkSetCall.html) and [*unset*](struct.WatermarkUnsetCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*set thumbnails*](struct.ThumbnailSetCall.html)
//! * [*insert captions*](struct.CaptionInsertCall.html)
//! * [*insert videos*](struct.VideoInsertCall.html)
//! * [*set watermarks*](struct.WatermarkSetCall.html)
//! * [*insert channel banners*](struct.ChannelBannerInsertCall.html)
//! * [*update captions*](struct.CaptionUpdateCall.html)
//! 
//! Download supported by ...
//! 
//! * [*download captions*](struct.CaptionDownloadCall.html)
//! 
//! Subscription supported by ...
//! 
//! * [*list playlist items*](struct.PlaylistItemListCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.YouTube.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.videos().rate(...).doit()
//! let r = hub.videos().report_abuse(...).doit()
//! let r = hub.videos().get_rating(...).doit()
//! let r = hub.videos().list(...).doit()
//! let r = hub.videos().insert(...).doit()
//! let r = hub.videos().update(...).doit()
//! let r = hub.videos().delete(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-youtube3 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_youtube3 as youtube3;
//! use youtube3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use youtube3::YouTube;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = YouTube::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.videos().list("part")
//!              .video_category_id("Lorem")
//!              .region_code("sed")
//!              .page_token("sit")
//!              .on_behalf_of_content_owner("dolore")
//!              .my_rating("et")
//!              .max_width(75)
//!              .max_results(3)
//!              .max_height(14)
//!              .locale("diam")
//!              .id("tempor")
//!              .hl("et")
//!              .chart("erat")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![cfg_attr(feature = "nightly", feature(proc_macro))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));