<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-mybusiness4` library allows access to all features of the *Google My Business* service.

This documentation was generated from *My Business* crate version *1.0.13+0*, where *0* is the exact revision of the *mybusiness:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *My Business* *v4* API can be found at the
[official documentation site](https://developers.google.com/my-business/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.MyBusiness.html) ... 

* [accounts](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.Account.html)
 * [*admins create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountAdminCreateCall.html), [*admins delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountAdminDeleteCall.html), [*admins list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountAdminListCall.html), [*admins patch*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountAdminPatchCall.html), [*create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountCreateCall.html), [*delete notifications*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountDeleteNotificationCall.html), [*generate account number*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountGenerateAccountNumberCall.html), [*get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountGetCall.html), [*get notifications*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountGetNotificationCall.html), [*invitations accept*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountInvitationAcceptCall.html), [*invitations decline*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountInvitationDeclineCall.html), [*invitations list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountInvitationListCall.html), [*list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountListCall.html), [*list recommend google locations*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountListRecommendGoogleLocationCall.html), [*locations admins create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationAdminCreateCall.html), [*locations admins delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationAdminDeleteCall.html), [*locations admins list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationAdminListCall.html), [*locations admins patch*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationAdminPatchCall.html), [*locations associate*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationAssociateCall.html), [*locations batch get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationBatchGetCall.html), [*locations batch get reviews*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationBatchGetReviewCall.html), [*locations clear association*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationClearAssociationCall.html), [*locations create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationCreateCall.html), [*locations delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationDeleteCall.html), [*locations fetch verification options*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationFetchVerificationOptionCall.html), [*locations find matches*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationFindMatcheCall.html), [*locations followers get metadata*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationFollowerGetMetadataCall.html), [*locations get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationGetCall.html), [*locations get google updated*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationGetGoogleUpdatedCall.html), [*locations list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationListCall.html), [*locations local posts create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationLocalPostCreateCall.html), [*locations local posts delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationLocalPostDeleteCall.html), [*locations local posts get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationLocalPostGetCall.html), [*locations local posts list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationLocalPostListCall.html), [*locations local posts patch*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationLocalPostPatchCall.html), [*locations local posts report insights*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationLocalPostReportInsightCall.html), [*locations media create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaCreateCall.html), [*locations media customers get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaCustomerGetCall.html), [*locations media customers list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaCustomerListCall.html), [*locations media delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaDeleteCall.html), [*locations media get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaGetCall.html), [*locations media list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaListCall.html), [*locations media patch*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaPatchCall.html), [*locations media start upload*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationMediaStartUploadCall.html), [*locations patch*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationPatchCall.html), [*locations questions answers delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionAnswerDeleteCall.html), [*locations questions answers list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionAnswerListCall.html), [*locations questions answers upsert*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionAnswerUpsertCall.html), [*locations questions create*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionCreateCall.html), [*locations questions delete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionDeleteCall.html), [*locations questions list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionListCall.html), [*locations questions patch*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationQuestionPatchCall.html), [*locations report insights*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationReportInsightCall.html), [*locations reviews delete reply*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationReviewDeleteReplyCall.html), [*locations reviews get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationReviewGetCall.html), [*locations reviews list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationReviewListCall.html), [*locations reviews update reply*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationReviewUpdateReplyCall.html), [*locations transfer*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationTransferCall.html), [*locations verifications complete*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationVerificationCompleteCall.html), [*locations verifications list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationVerificationListCall.html), [*locations verify*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountLocationVerifyCall.html), [*update*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountUpdateCall.html) and [*update notifications*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AccountUpdateNotificationCall.html)
* [attributes](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.Attribute.html)
 * [*list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.AttributeListCall.html)
* [categories](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.Category.html)
 * [*list*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.CategoryListCall.html)
* [chains](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.Chain.html)
 * [*get*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.ChainGetCall.html) and [*search*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.ChainSearchCall.html)
* [google locations](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.GoogleLocation.html)
 * [*report*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.GoogleLocationReportCall.html) and [*search*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.GoogleLocationSearchCall.html)
* [verification tokens](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.VerificationToken.html)
 * [*generate*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.VerificationTokenGenerateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/struct.MyBusiness.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.accounts().locations_batch_get(...).doit()
let r = hub.accounts().locations_reviews_delete_reply(...).doit()
let r = hub.accounts().locations_patch(...).doit()
let r = hub.accounts().locations_questions_patch(...).doit()
let r = hub.accounts().locations_verifications_complete(...).doit()
let r = hub.accounts().locations_get_google_updated(...).doit()
let r = hub.accounts().locations_followers_get_metadata(...).doit()
let r = hub.accounts().generate_account_number(...).doit()
let r = hub.accounts().create(...).doit()
let r = hub.accounts().locations_questions_create(...).doit()
let r = hub.accounts().locations_media_create(...).doit()
let r = hub.accounts().locations_local_posts_get(...).doit()
let r = hub.accounts().locations_reviews_update_reply(...).doit()
let r = hub.accounts().list_recommend_google_locations(...).doit()
let r = hub.accounts().invitations_decline(...).doit()
let r = hub.accounts().locations_questions_list(...).doit()
let r = hub.accounts().locations_create(...).doit()
let r = hub.accounts().locations_media_get(...).doit()
let r = hub.accounts().locations_local_posts_report_insights(...).doit()
let r = hub.accounts().locations_questions_delete(...).doit()
let r = hub.accounts().admins_list(...).doit()
let r = hub.accounts().locations_questions_answers_delete(...).doit()
let r = hub.accounts().locations_media_customers_get(...).doit()
let r = hub.accounts().list(...).doit()
let r = hub.accounts().locations_local_posts_patch(...).doit()
let r = hub.accounts().locations_transfer(...).doit()
let r = hub.accounts().locations_reviews_get(...).doit()
let r = hub.accounts().locations_local_posts_create(...).doit()
let r = hub.accounts().locations_questions_answers_upsert(...).doit()
let r = hub.accounts().admins_patch(...).doit()
let r = hub.accounts().locations_find_matches(...).doit()
let r = hub.accounts().locations_verifications_list(...).doit()
let r = hub.accounts().locations_list(...).doit()
let r = hub.accounts().locations_admins_delete(...).doit()
let r = hub.accounts().locations_delete(...).doit()
let r = hub.accounts().delete_notifications(...).doit()
let r = hub.accounts().locations_media_customers_list(...).doit()
let r = hub.accounts().get(...).doit()
let r = hub.accounts().locations_media_delete(...).doit()
let r = hub.accounts().locations_clear_association(...).doit()
let r = hub.accounts().locations_verify(...).doit()
let r = hub.accounts().admins_delete(...).doit()
let r = hub.accounts().locations_local_posts_list(...).doit()
let r = hub.accounts().invitations_accept(...).doit()
let r = hub.accounts().locations_admins_list(...).doit()
let r = hub.accounts().locations_batch_get_reviews(...).doit()
let r = hub.accounts().admins_create(...).doit()
let r = hub.accounts().locations_admins_create(...).doit()
let r = hub.accounts().locations_media_start_upload(...).doit()
let r = hub.accounts().locations_report_insights(...).doit()
let r = hub.accounts().locations_associate(...).doit()
let r = hub.accounts().locations_reviews_list(...).doit()
let r = hub.accounts().locations_get(...).doit()
let r = hub.accounts().locations_questions_answers_list(...).doit()
let r = hub.accounts().locations_media_patch(...).doit()
let r = hub.accounts().locations_local_posts_delete(...).doit()
let r = hub.accounts().update(...).doit()
let r = hub.accounts().invitations_list(...).doit()
let r = hub.accounts().locations_admins_patch(...).doit()
let r = hub.accounts().get_notifications(...).doit()
let r = hub.accounts().locations_media_list(...).doit()
let r = hub.accounts().update_notifications(...).doit()
let r = hub.accounts().locations_fetch_verification_options(...).doit()
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
google-mybusiness4 = "*"
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
extern crate google_mybusiness4 as mybusiness4;
use mybusiness4::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use mybusiness4::MyBusiness;

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
let mut hub = MyBusiness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().locations_questions_list("parent")
             .page_token("sit")
             .page_size(-65)
             .order_by("sed")
             .filter("et")
             .answers_per_question(-18)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-mybusiness4/1.0.13+0/google_mybusiness4/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **mybusiness4** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
