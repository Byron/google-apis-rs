<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-youtube3` library allows access to all features of the *Google YouTube* service.

This documentation was generated from *YouTube* crate version *0.1.2+20150309*, where *20150309* is the exact revision of the *youtube:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.2*.

Everything else about the *YouTube* *v3* API can be found at the
[official documentation site](https://developers.google.com/youtube/v3).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google-youtube3/struct.YouTube.html) ... 

* [activities](http://byron.github.io/google-apis-rs/google-youtube3/struct.Activity.html)
 * [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ActivityInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ActivityListCall.html)
* channel banners
 * [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelBannerInsertCall.html)
* [channel sections](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelSection.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelSectionDeleteCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelSectionInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelSectionListCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelSectionUpdateCall.html)
* [channels](http://byron.github.io/google-apis-rs/google-youtube3/struct.Channel.html)
 * [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelListCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelUpdateCall.html)
* [guide categories](http://byron.github.io/google-apis-rs/google-youtube3/struct.GuideCategory.html)
 * [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.GuideCategoryListCall.html)
* [i18n languages](http://byron.github.io/google-apis-rs/google-youtube3/struct.I18nLanguage.html)
 * [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.I18nLanguageListCall.html)
* [i18n regions](http://byron.github.io/google-apis-rs/google-youtube3/struct.I18nRegion.html)
 * [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.I18nRegionListCall.html)
* [live broadcasts](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcast.html)
 * [*bind*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastBindCall.html), [*control*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastControlCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastDeleteCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastListCall.html), [*transition*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastTransitionCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveBroadcastUpdateCall.html)
* [live streams](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveStream.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveStreamDeleteCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveStreamInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveStreamListCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.LiveStreamUpdateCall.html)
* [playlist items](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistItem.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistItemDeleteCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistItemInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistItemListCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistItemUpdateCall.html)
* [playlists](http://byron.github.io/google-apis-rs/google-youtube3/struct.Playlist.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistDeleteCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistListCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistUpdateCall.html)
* search
 * [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.SearchListCall.html)
* [subscriptions](http://byron.github.io/google-apis-rs/google-youtube3/struct.Subscription.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.SubscriptionDeleteCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.SubscriptionInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.SubscriptionListCall.html)
* [thumbnails](http://byron.github.io/google-apis-rs/google-youtube3/struct.Thumbnail.html)
 * [*set*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ThumbnailSetCall.html)
* [video categories](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoCategory.html)
 * [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoCategoryListCall.html)
* [videos](http://byron.github.io/google-apis-rs/google-youtube3/struct.Video.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoDeleteCall.html), [*get rating*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoGetRatingCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoListCall.html), [*rate*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoRateCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoUpdateCall.html)
* watermarks
 * [*set*](http://byron.github.io/google-apis-rs/google-youtube3/struct.WatermarkSetCall.html) and [*unset*](http://byron.github.io/google-apis-rs/google-youtube3/struct.WatermarkUnsetCall.html)


Upload supported by ...

* [*set watermarks*](http://byron.github.io/google-apis-rs/google-youtube3/struct.WatermarkSetCall.html)
* [*insert channel banners*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ChannelBannerInsertCall.html)
* [*set thumbnails*](http://byron.github.io/google-apis-rs/google-youtube3/struct.ThumbnailSetCall.html)
* [*insert videos*](http://byron.github.io/google-apis-rs/google-youtube3/struct.VideoInsertCall.html)

Subscription supported by ...

* [*list playlist items*](http://byron.github.io/google-apis-rs/google-youtube3/struct.PlaylistItemListCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google-youtube3/struct.YouTube.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google-youtube3/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google-youtube3/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google-youtube3/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google-youtube3/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google-youtube3/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.live_broadcasts().control(...).doit()
let r = hub.live_broadcasts().insert(...).doit()
let r = hub.live_broadcasts().list(...).doit()
let r = hub.live_broadcasts().transition(...).doit()
let r = hub.live_broadcasts().update(...).doit()
let r = hub.live_broadcasts().delete(...).doit()
let r = hub.live_broadcasts().bind(...).doit()
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
```

## A complete example

```Rust
extern crate hyper;
extern crate "yup-oauth2" as oauth2;
extern crate "google-youtube3" as youtube3;
use youtube3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use youtube3::YouTube;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::new(),
                              <MemoryStorage as Default>::default(), None);
let mut hub = YouTube::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.live_broadcasts().list("part")
             .page_token("sit")
             .on_behalf_of_content_owner_channel("Stet")
             .on_behalf_of_content_owner("sed")
             .mine(false)
             .max_results(83)
             .id("kasd")
             .broadcast_status("accusam")
             .doit();

match result {
    Err(e) => match e {
        Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
        Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
        Error::MissingToken => println!("OAuth2: Missing Token"),
        Error::Cancelled => println!("Operation canceled by user"),
        Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
        Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
        Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
        Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
    },
    Ok(_) => println!("Success (value doesn't print)"),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google-youtube3/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google-youtube3/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google-youtube3/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google-youtube3/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google-youtube3/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google-youtube3/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google-youtube3/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google-youtube3/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google-youtube3/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google-youtube3/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google-youtube3/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google-youtube3/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google-youtube3/trait.RequestValue.html) are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **youtube3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
