<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-mybusiness4` library allows access to all features of the *Google My Business* service.

This documentation was generated from *My Business* crate version *5.0.2-beta-1+0*, where *0* is the exact revision of the *mybusiness:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.2-beta-1*.

Everything else about the *My Business* *v4* API can be found at the
[official documentation site](https://developers.google.com/my-business/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/MyBusiness) ... 

* [accounts](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::Account)
 * [*admins create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountAdminCreateCall), [*admins delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountAdminDeleteCall), [*admins list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountAdminListCall), [*admins patch*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountAdminPatchCall), [*create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountCreateCall), [*delete notifications*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountDeleteNotificationCall), [*generate account number*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountGenerateAccountNumberCall), [*get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountGetCall), [*get notifications*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountGetNotificationCall), [*invitations accept*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountInvitationAcceptCall), [*invitations decline*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountInvitationDeclineCall), [*invitations list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountInvitationListCall), [*list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountListCall), [*list recommend google locations*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountListRecommendGoogleLocationCall), [*locations admins create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationAdminCreateCall), [*locations admins delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationAdminDeleteCall), [*locations admins list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationAdminListCall), [*locations admins patch*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationAdminPatchCall), [*locations associate*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationAssociateCall), [*locations batch get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationBatchGetCall), [*locations batch get reviews*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationBatchGetReviewCall), [*locations clear association*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationClearAssociationCall), [*locations create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationCreateCall), [*locations delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationDeleteCall), [*locations fetch verification options*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationFetchVerificationOptionCall), [*locations find matches*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationFindMatchCall), [*locations followers get metadata*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationFollowerGetMetadataCall), [*locations get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationGetCall), [*locations get google updated*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationGetGoogleUpdatedCall), [*locations list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationListCall), [*locations local posts create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationLocalPostCreateCall), [*locations local posts delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationLocalPostDeleteCall), [*locations local posts get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationLocalPostGetCall), [*locations local posts list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationLocalPostListCall), [*locations local posts patch*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationLocalPostPatchCall), [*locations local posts report insights*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationLocalPostReportInsightCall), [*locations media create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaCreateCall), [*locations media customers get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaCustomerGetCall), [*locations media customers list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaCustomerListCall), [*locations media delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaDeleteCall), [*locations media get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaGetCall), [*locations media list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaListCall), [*locations media patch*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaPatchCall), [*locations media start upload*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationMediaStartUploadCall), [*locations patch*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationPatchCall), [*locations questions answers delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionAnswerDeleteCall), [*locations questions answers list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionAnswerListCall), [*locations questions answers upsert*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionAnswerUpsertCall), [*locations questions create*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionCreateCall), [*locations questions delete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionDeleteCall), [*locations questions list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionListCall), [*locations questions patch*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationQuestionPatchCall), [*locations report insights*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationReportInsightCall), [*locations reviews delete reply*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationReviewDeleteReplyCall), [*locations reviews get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationReviewGetCall), [*locations reviews list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationReviewListCall), [*locations reviews update reply*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationReviewUpdateReplyCall), [*locations transfer*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationTransferCall), [*locations verifications complete*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationVerificationCompleteCall), [*locations verifications list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationVerificationListCall), [*locations verify*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountLocationVerifyCall), [*update*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountUpdateCall) and [*update notifications*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AccountUpdateNotificationCall)
* [attributes](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::Attribute)
 * [*list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::AttributeListCall)
* [categories](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::Category)
 * [*list*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::CategoryListCall)
* [chains](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::Chain)
 * [*get*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::ChainGetCall) and [*search*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::ChainSearchCall)
* [google locations](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::GoogleLocation)
 * [*report*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::GoogleLocationReportCall) and [*search*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::GoogleLocationSearchCall)
* [verification tokens](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::VerificationToken)
 * [*generate*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/api::VerificationTokenGenerateCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/MyBusiness)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::CallBuilder)
* **[Resources](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().admins_create(...).doit().await
let r = hub.accounts().admins_delete(...).doit().await
let r = hub.accounts().admins_list(...).doit().await
let r = hub.accounts().admins_patch(...).doit().await
let r = hub.accounts().invitations_accept(...).doit().await
let r = hub.accounts().invitations_decline(...).doit().await
let r = hub.accounts().invitations_list(...).doit().await
let r = hub.accounts().locations_admins_create(...).doit().await
let r = hub.accounts().locations_admins_delete(...).doit().await
let r = hub.accounts().locations_admins_list(...).doit().await
let r = hub.accounts().locations_admins_patch(...).doit().await
let r = hub.accounts().locations_followers_get_metadata(...).doit().await
let r = hub.accounts().locations_local_posts_create(...).doit().await
let r = hub.accounts().locations_local_posts_delete(...).doit().await
let r = hub.accounts().locations_local_posts_get(...).doit().await
let r = hub.accounts().locations_local_posts_list(...).doit().await
let r = hub.accounts().locations_local_posts_patch(...).doit().await
let r = hub.accounts().locations_local_posts_report_insights(...).doit().await
let r = hub.accounts().locations_media_customers_get(...).doit().await
let r = hub.accounts().locations_media_customers_list(...).doit().await
let r = hub.accounts().locations_media_create(...).doit().await
let r = hub.accounts().locations_media_delete(...).doit().await
let r = hub.accounts().locations_media_get(...).doit().await
let r = hub.accounts().locations_media_list(...).doit().await
let r = hub.accounts().locations_media_patch(...).doit().await
let r = hub.accounts().locations_media_start_upload(...).doit().await
let r = hub.accounts().locations_questions_answers_delete(...).doit().await
let r = hub.accounts().locations_questions_answers_list(...).doit().await
let r = hub.accounts().locations_questions_answers_upsert(...).doit().await
let r = hub.accounts().locations_questions_create(...).doit().await
let r = hub.accounts().locations_questions_delete(...).doit().await
let r = hub.accounts().locations_questions_list(...).doit().await
let r = hub.accounts().locations_questions_patch(...).doit().await
let r = hub.accounts().locations_reviews_delete_reply(...).doit().await
let r = hub.accounts().locations_reviews_get(...).doit().await
let r = hub.accounts().locations_reviews_list(...).doit().await
let r = hub.accounts().locations_reviews_update_reply(...).doit().await
let r = hub.accounts().locations_verifications_complete(...).doit().await
let r = hub.accounts().locations_verifications_list(...).doit().await
let r = hub.accounts().locations_associate(...).doit().await
let r = hub.accounts().locations_batch_get(...).doit().await
let r = hub.accounts().locations_batch_get_reviews(...).doit().await
let r = hub.accounts().locations_clear_association(...).doit().await
let r = hub.accounts().locations_create(...).doit().await
let r = hub.accounts().locations_delete(...).doit().await
let r = hub.accounts().locations_fetch_verification_options(...).doit().await
let r = hub.accounts().locations_find_matches(...).doit().await
let r = hub.accounts().locations_get(...).doit().await
let r = hub.accounts().locations_get_google_updated(...).doit().await
let r = hub.accounts().locations_list(...).doit().await
let r = hub.accounts().locations_patch(...).doit().await
let r = hub.accounts().locations_report_insights(...).doit().await
let r = hub.accounts().locations_transfer(...).doit().await
let r = hub.accounts().locations_verify(...).doit().await
let r = hub.accounts().create(...).doit().await
let r = hub.accounts().delete_notifications(...).doit().await
let r = hub.accounts().generate_account_number(...).doit().await
let r = hub.accounts().get(...).doit().await
let r = hub.accounts().get_notifications(...).doit().await
let r = hub.accounts().list(...).doit().await
let r = hub.accounts().list_recommend_google_locations(...).doit().await
let r = hub.accounts().update(...).doit().await
let r = hub.accounts().update_notifications(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_mybusiness4 as mybusiness4;
use mybusiness4::{Result, Error};
use std::default::Default;
use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().locations_questions_list("parent")
             .page_token("magna")
             .page_size(-11)
             .order_by("ipsum")
             .filter("voluptua.")
             .answers_per_question(-27)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Delegate) to the 
[Method Builder](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::RequestValue) and 
[decodable](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-mybusiness4/5.0.2-beta-1+0/google_mybusiness4/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **mybusiness4** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

