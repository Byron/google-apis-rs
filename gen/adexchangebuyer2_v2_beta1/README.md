<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-adexchangebuyer2_v2_beta1` library allows access to all features of the *Google AdExchangeBuyerII* service.

This documentation was generated from *AdExchangeBuyerII* crate version *5.0.5+20240625*, where *20240625* is the exact revision of the *adexchangebuyer2:v2beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *AdExchangeBuyerII* *v2_beta1* API can be found at the
[official documentation site](https://developers.google.com/authorized-buyers/apis/reference/rest/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/AdExchangeBuyerII) ... 

* accounts
 * [*clients create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientCreateCall), [*clients get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientGetCall), [*clients invitations create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientInvitationCreateCall), [*clients invitations get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientInvitationGetCall), [*clients invitations list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientInvitationListCall), [*clients list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientListCall), [*clients update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientUpdateCall), [*clients users get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientUserGetCall), [*clients users list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientUserListCall), [*clients users update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountClientUserUpdateCall), [*creatives create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeCreateCall), [*creatives deal associations add*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeDealAssociationAddCall), [*creatives deal associations list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeDealAssociationListCall), [*creatives deal associations remove*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeDealAssociationRemoveCall), [*creatives get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeGetCall), [*creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeListCall), [*creatives stop watching*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeStopWatchingCall), [*creatives update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeUpdateCall), [*creatives watch*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountCreativeWatchCall), [*finalized proposals list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountFinalizedProposalListCall), [*finalized proposals pause*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountFinalizedProposalPauseCall), [*finalized proposals resume*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountFinalizedProposalResumeCall), [*products get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProductGetCall), [*products list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProductListCall), [*proposals accept*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalAcceptCall), [*proposals add note*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalAddNoteCall), [*proposals cancel negotiation*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalCancelNegotiationCall), [*proposals complete setup*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalCompleteSetupCall), [*proposals create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalCreateCall), [*proposals get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalGetCall), [*proposals list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalListCall), [*proposals pause*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalPauseCall), [*proposals resume*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalResumeCall), [*proposals update*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountProposalUpdateCall), [*publisher profiles get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountPublisherProfileGetCall) and [*publisher profiles list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::AccountPublisherProfileListCall)
* bidders
 * [*accounts filter sets bid metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetBidMetricListCall), [*accounts filter sets bid response errors list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetBidResponseErrorListCall), [*accounts filter sets bid responses without bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetBidResponsesWithoutBidListCall), [*accounts filter sets create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetCreateCall), [*accounts filter sets delete*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetDeleteCall), [*accounts filter sets filtered bid requests list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetFilteredBidRequestListCall), [*accounts filter sets filtered bids creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetFilteredBidCreativeListCall), [*accounts filter sets filtered bids details list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetFilteredBidDetailListCall), [*accounts filter sets filtered bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetFilteredBidListCall), [*accounts filter sets get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetGetCall), [*accounts filter sets impression metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetImpressionMetricListCall), [*accounts filter sets list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetListCall), [*accounts filter sets losing bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetLosingBidListCall), [*accounts filter sets non billable winning bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderAccountFilterSetNonBillableWinningBidListCall), [*filter sets bid metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetBidMetricListCall), [*filter sets bid response errors list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetBidResponseErrorListCall), [*filter sets bid responses without bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetBidResponsesWithoutBidListCall), [*filter sets create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetCreateCall), [*filter sets delete*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetDeleteCall), [*filter sets filtered bid requests list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetFilteredBidRequestListCall), [*filter sets filtered bids creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetFilteredBidCreativeListCall), [*filter sets filtered bids details list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetFilteredBidDetailListCall), [*filter sets filtered bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetFilteredBidListCall), [*filter sets get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetGetCall), [*filter sets impression metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetImpressionMetricListCall), [*filter sets list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetListCall), [*filter sets losing bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetLosingBidListCall) and [*filter sets non billable winning bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BidderFilterSetNonBillableWinningBidListCall)
* [buyers](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::Buyer)
 * [*filter sets bid metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetBidMetricListCall), [*filter sets bid response errors list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetBidResponseErrorListCall), [*filter sets bid responses without bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetBidResponsesWithoutBidListCall), [*filter sets create*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetCreateCall), [*filter sets delete*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetDeleteCall), [*filter sets filtered bid requests list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetFilteredBidRequestListCall), [*filter sets filtered bids creatives list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetFilteredBidCreativeListCall), [*filter sets filtered bids details list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetFilteredBidDetailListCall), [*filter sets filtered bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetFilteredBidListCall), [*filter sets get*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetGetCall), [*filter sets impression metrics list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetImpressionMetricListCall), [*filter sets list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetListCall), [*filter sets losing bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetLosingBidListCall) and [*filter sets non billable winning bids list*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/api::BuyerFilterSetNonBillableWinningBidListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/AdExchangeBuyerII)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::CallBuilder)
* **[Resources](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.buyers().filter_sets_bid_metrics_list(...).doit().await
let r = hub.buyers().filter_sets_bid_response_errors_list(...).doit().await
let r = hub.buyers().filter_sets_bid_responses_without_bids_list(...).doit().await
let r = hub.buyers().filter_sets_filtered_bid_requests_list(...).doit().await
let r = hub.buyers().filter_sets_filtered_bids_creatives_list(...).doit().await
let r = hub.buyers().filter_sets_filtered_bids_details_list(...).doit().await
let r = hub.buyers().filter_sets_filtered_bids_list(...).doit().await
let r = hub.buyers().filter_sets_impression_metrics_list(...).doit().await
let r = hub.buyers().filter_sets_losing_bids_list(...).doit().await
let r = hub.buyers().filter_sets_non_billable_winning_bids_list(...).doit().await
let r = hub.buyers().filter_sets_create(...).doit().await
let r = hub.buyers().filter_sets_delete(...).doit().await
let r = hub.buyers().filter_sets_get(...).doit().await
let r = hub.buyers().filter_sets_list(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_adexchangebuyer2_v2_beta1 as adexchangebuyer2_v2_beta1;
use adexchangebuyer2_v2_beta1::{Result, Error};
use std::default::Default;
use adexchangebuyer2_v2_beta1::{AdExchangeBuyerII, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = AdExchangeBuyerII::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.buyers().filter_sets_filtered_bids_creatives_list("filterSetName", -33)
             .page_token("no")
             .page_size(-55)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::RequestValue) and 
[decodable](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-adexchangebuyer2_v2_beta1/5.0.5+20240625/google_adexchangebuyer2_v2_beta1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **adexchangebuyer2_v2_beta1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

