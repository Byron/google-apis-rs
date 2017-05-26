<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-games1` library allows access to all features of the *Google Games* service.

This documentation was generated from *Games* crate version *1.0.5+20170511*, where *20170511* is the exact revision of the *games:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.5*.

Everything else about the *Games* *v1* API can be found at the
[official documentation site](https://developers.google.com/games/services/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Games.html) ... 

* [achievement definitions](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementDefinition.html)
 * [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementDefinitionListCall.html)
* achievements
 * [*increment*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementIncrementCall.html), [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementListCall.html), [*reveal*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementRevealCall.html), [*set steps at least*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementSetStepsAtLeastCall.html), [*unlock*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementUnlockCall.html) and [*update multiple*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.AchievementUpdateMultipleCall.html)
* [applications](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Application.html)
 * [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ApplicationGetCall.html), [*played*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ApplicationPlayedCall.html) and [*verify*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ApplicationVerifyCall.html)
* events
 * [*list by player*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.EventListByPlayerCall.html), [*list definitions*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.EventListDefinitionCall.html) and [*record*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.EventRecordCall.html)
* [leaderboards](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Leaderboard.html)
 * [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.LeaderboardGetCall.html) and [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.LeaderboardListCall.html)
* metagame
 * [*get metagame config*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.MetagameGetMetagameConfigCall.html) and [*list categories by player*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.MetagameListCategoriesByPlayerCall.html)
* [players](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Player.html)
 * [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.PlayerGetCall.html) and [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.PlayerListCall.html)
* pushtokens
 * [*remove*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.PushtokenRemoveCall.html) and [*update*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.PushtokenUpdateCall.html)
* [quest milestones](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.QuestMilestone.html)
 * [*claim*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.QuestMilestoneClaimCall.html)
* [quests](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Quest.html)
 * [*accept*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.QuestAcceptCall.html) and [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.QuestListCall.html)
* revisions
 * [*check*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RevisionCheckCall.html)
* [rooms](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Room.html)
 * [*create*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomCreateCall.html), [*decline*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomDeclineCall.html), [*dismiss*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomDismisCall.html), [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomGetCall.html), [*join*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomJoinCall.html), [*leave*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomLeaveCall.html), [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomListCall.html) and [*report status*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.RoomReportStatuCall.html)
* scores
 * [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ScoreGetCall.html), [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ScoreListCall.html), [*list window*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ScoreListWindowCall.html), [*submit*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ScoreSubmitCall.html) and [*submit multiple*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.ScoreSubmitMultipleCall.html)
* [snapshots](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Snapshot.html)
 * [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.SnapshotGetCall.html) and [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.SnapshotListCall.html)
* turn based matches
 * [*cancel*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheCancelCall.html), [*create*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheCreateCall.html), [*decline*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheDeclineCall.html), [*dismiss*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheDismisCall.html), [*finish*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheFinishCall.html), [*get*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheGetCall.html), [*join*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheJoinCall.html), [*leave*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheLeaveCall.html), [*leave turn*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheLeaveTurnCall.html), [*list*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheListCall.html), [*rematch*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheRematchCall.html), [*sync*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheSyncCall.html) and [*take turn*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.TurnBasedMatcheTakeTurnCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-games1/1.0.5+20170511/google_games1/struct.Games.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.turn_based_matches().decline(...).doit()
let r = hub.turn_based_matches().leave(...).doit()
let r = hub.turn_based_matches().finish(...).doit()
let r = hub.turn_based_matches().take_turn(...).doit()
let r = hub.turn_based_matches().create(...).doit()
let r = hub.turn_based_matches().join(...).doit()
let r = hub.turn_based_matches().leave_turn(...).doit()
let r = hub.turn_based_matches().get(...).doit()
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
google-games1 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_games1 as games1;
use games1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use games1::Games;

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
let mut hub = Games::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.turn_based_matches().leave_turn("matchId", -48)
             .pending_participant_id("Stet")
             .language("sed")
             .consistency_token("et")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-games1/1.0.5+20170511/google_games1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-games1/1.0.5+20170511/google_games1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-games1/1.0.5+20170511/google_games1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **games1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
