<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-adexchangebuyer2_v2_beta1` library allows access to all features of the *Google AdExchangeBuyerII* service.

This documentation was generated from *AdExchangeBuyerII* crate version *1.0.13+20200409*, where *20200409* is the exact revision of the *adexchangebuyer2:v2beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *AdExchangeBuyerII* *v2_beta1* API can be found at the
[official documentation site](https://developers.google.com/authorized-buyers/apis/reference/rest/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AdExchangeBuyerII.html) ... 

* accounts
 * [*clients create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientCreateCall.html), [*clients get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientGetCall.html), [*clients invitations create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientInvitationCreateCall.html), [*clients invitations get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientInvitationGetCall.html), [*clients invitations list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientInvitationListCall.html), [*clients list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientListCall.html), [*clients update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientUpdateCall.html), [*clients users get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientUserGetCall.html), [*clients users list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientUserListCall.html), [*clients users update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountClientUserUpdateCall.html), [*creatives create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeCreateCall.html), [*creatives deal associations add*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeDealAssociationAddCall.html), [*creatives deal associations list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeDealAssociationListCall.html), [*creatives deal associations remove*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeDealAssociationRemoveCall.html), [*creatives get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeGetCall.html), [*creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeListCall.html), [*creatives stop watching*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeStopWatchingCall.html), [*creatives update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeUpdateCall.html), [*creatives watch*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountCreativeWatchCall.html), [*finalized proposals list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountFinalizedProposalListCall.html), [*products get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProductGetCall.html), [*products list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProductListCall.html), [*proposals accept*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalAcceptCall.html), [*proposals add note*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalAddNoteCall.html), [*proposals cancel negotiation*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalCancelNegotiationCall.html), [*proposals complete setup*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalCompleteSetupCall.html), [*proposals create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalCreateCall.html), [*proposals get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalGetCall.html), [*proposals list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalListCall.html), [*proposals pause*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalPauseCall.html), [*proposals resume*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalResumeCall.html), [*proposals update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountProposalUpdateCall.html), [*publisher profiles get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountPublisherProfileGetCall.html) and [*publisher profiles list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AccountPublisherProfileListCall.html)
* bidders
 * [*accounts filter sets bid metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetBidMetricListCall.html), [*accounts filter sets bid response errors list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetBidResponseErrorListCall.html), [*accounts filter sets bid responses without bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetBidResponsesWithoutBidListCall.html), [*accounts filter sets create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetCreateCall.html), [*accounts filter sets delete*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetDeleteCall.html), [*accounts filter sets filtered bid requests list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetFilteredBidRequestListCall.html), [*accounts filter sets filtered bids creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetFilteredBidCreativeListCall.html), [*accounts filter sets filtered bids details list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetFilteredBidDetailListCall.html), [*accounts filter sets filtered bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetFilteredBidListCall.html), [*accounts filter sets get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetGetCall.html), [*accounts filter sets impression metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetImpressionMetricListCall.html), [*accounts filter sets list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetListCall.html), [*accounts filter sets losing bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetLosingBidListCall.html), [*accounts filter sets non billable winning bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderAccountFilterSetNonBillableWinningBidListCall.html), [*filter sets bid metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetBidMetricListCall.html), [*filter sets bid response errors list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetBidResponseErrorListCall.html), [*filter sets bid responses without bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetBidResponsesWithoutBidListCall.html), [*filter sets create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetCreateCall.html), [*filter sets delete*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetDeleteCall.html), [*filter sets filtered bid requests list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetFilteredBidRequestListCall.html), [*filter sets filtered bids creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetFilteredBidCreativeListCall.html), [*filter sets filtered bids details list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetFilteredBidDetailListCall.html), [*filter sets filtered bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetFilteredBidListCall.html), [*filter sets get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetGetCall.html), [*filter sets impression metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetImpressionMetricListCall.html), [*filter sets list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetListCall.html), [*filter sets losing bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetLosingBidListCall.html) and [*filter sets non billable winning bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.BidderFilterSetNonBillableWinningBidListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/struct.AdExchangeBuyerII.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.accounts().proposals_get(...).doit()
let r = hub.accounts().proposals_resume(...).doit()
let r = hub.accounts().proposals_cancel_negotiation(...).doit()
let r = hub.accounts().proposals_create(...).doit()
let r = hub.accounts().proposals_update(...).doit()
let r = hub.accounts().proposals_accept(...).doit()
let r = hub.accounts().proposals_pause(...).doit()
let r = hub.accounts().proposals_complete_setup(...).doit()
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
google-adexchangebuyer2_v2_beta1 = "*"
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
extern crate google_adexchangebuyer2_v2_beta1 as adexchangebuyer2_v2_beta1;
use adexchangebuyer2_v2_beta1::ResumeProposalRequest;
use adexchangebuyer2_v2_beta1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use adexchangebuyer2_v2_beta1::AdExchangeBuyerII;

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
let mut hub = AdExchangeBuyerII::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = ResumeProposalRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().proposals_resume(req, "accountId", "proposalId")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-adexchangebuyer2_v2_beta1/1.0.13+20200409/google_adexchangebuyer2_v2_beta1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **adexchangebuyer2_v2_beta1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
