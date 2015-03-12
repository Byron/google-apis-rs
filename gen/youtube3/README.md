<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/README.md.mako'
DO NOT EDIT !
-->
The `youtube3` library allows access to all features of *YouTube*.

# Features

Handle the following *Resources* with ease from the central [hub](struct.YouTube.html) ... 

* activities ([*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.ActivityInsertMethodBuilder.html) and [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.ActivityListMethodBuilder.html))
* channel banners ([*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelBannerInsertMethodBuilder.html))
* channel sections ([*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelSectionDeleteMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelSectionInsertMethodBuilder.html), [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelSectionListMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelSectionUpdateMethodBuilder.html))
* channels ([*list*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelListMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.ChannelUpdateMethodBuilder.html))
* guide categories ([*list*](http://byron.github.io/google-apis-rs/youtube3/struct.GuideCategoryListMethodBuilder.html))
* i18n languages ([*list*](http://byron.github.io/google-apis-rs/youtube3/struct.I18nLanguageListMethodBuilder.html))
* i18n regions ([*list*](http://byron.github.io/google-apis-rs/youtube3/struct.I18nRegionListMethodBuilder.html))
* live broadcasts ([*bind*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastBindMethodBuilder.html), [*control*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastControlMethodBuilder.html), [*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastDeleteMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastInsertMethodBuilder.html), [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastListMethodBuilder.html), [*transition*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastTransitionMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveBroadcastUpdateMethodBuilder.html))
* live streams ([*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveStreamDeleteMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveStreamInsertMethodBuilder.html), [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveStreamListMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.LiveStreamUpdateMethodBuilder.html))
* playlist items ([*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistItemDeleteMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistItemInsertMethodBuilder.html), [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistItemListMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistItemUpdateMethodBuilder.html))
* playlists ([*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistDeleteMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistInsertMethodBuilder.html), [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistListMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.PlaylistUpdateMethodBuilder.html))
* search ([*list*](http://byron.github.io/google-apis-rs/youtube3/struct.SearchListMethodBuilder.html))
* subscriptions ([*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.SubscriptionDeleteMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.SubscriptionInsertMethodBuilder.html) and [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.SubscriptionListMethodBuilder.html))
* thumbnails ([*set*](http://byron.github.io/google-apis-rs/youtube3/struct.ThumbnailSetMethodBuilder.html))
* video categories ([*list*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoCategoryListMethodBuilder.html))
* videos ([*delete*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoDeleteMethodBuilder.html), [*getrating*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoGetRatingMethodBuilder.html), [*insert*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoInsertMethodBuilder.html), [*list*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoListMethodBuilder.html), [*rate*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoRateMethodBuilder.html) and [*update*](http://byron.github.io/google-apis-rs/youtube3/struct.VideoUpdateMethodBuilder.html))
* watermarks ([*set*](http://byron.github.io/google-apis-rs/youtube3/struct.WatermarkSetMethodBuilder.html) and [*unset*](http://byron.github.io/google-apis-rs/youtube3/struct.WatermarkUnsetMethodBuilder.html))

Everything else about the *YouTube* API can be found at the
[official documentation site](https://developers.google.com/youtube/v3).

# Structure of this Library

The API is structured into the following primary items:

* **[Hub](struct.YouTube.html)**
    * a central object to maintain state and allow accessing all *Activities*
* **[Resources](cmn/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](cmn/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](cmn/trait.MethodBuilder.html)**
    * operations to apply to *Resources*

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

# Usage (*TODO*)

## Instantiating the Hub

```Rust
extern crate hyper;
extern crate "yup-oauth2" as oauth2;
extern crate "rustc-serialize" as rustc_serialize;
extern crate youtube3;
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
# use youtube3::YouTube;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and `client_secret`, 
// among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about what's going on
// You probably want to bring in your own `TokenStorage` to persist tokens and retrieve them from storage.
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
// TODO: show how to handle the result !

```
**TODO** Example calls - there should soon be a generator able to do that with proper inputs

## Handling Errors

# Some details

## About Customization/Callbacks

## About parts

* Optionals needed for Json, otherwise I'd happily drop them
* explain that examples use all response parts, even though they are shown for request values

## About builder arguments

* pods are copy
* strings are &str
* request values are borrowed
* additional parameters using `param()`

[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **youtube3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsLICENSE.md
