<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-pubsub1` library allows access to all features of the *Google pubsub* service.

This documentation was generated from *pubsub* crate version *1.0.0+20160317*, where *20160317* is the exact revision of the *pubsub:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.0*.

Everything else about the *pubsub* *v1* API can be found at the
[official documentation site](https://cloud.google.com/pubsub/docs).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_pubsub1/struct.Pubsub.html) ... 

* projects
 * [*subscriptions acknowledge*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionAcknowledgeCall.html), [*subscriptions create*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionCreateCall.html), [*subscriptions delete*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionDeleteCall.html), [*subscriptions get*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionGetCall.html), [*subscriptions get iam policy*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionGetIamPolicyCall.html), [*subscriptions list*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionListCall.html), [*subscriptions modify ack deadline*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionModifyAckDeadlineCall.html), [*subscriptions modify push config*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionModifyPushConfigCall.html), [*subscriptions pull*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionPullCall.html), [*subscriptions set iam policy*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionSetIamPolicyCall.html), [*subscriptions test iam permissions*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectSubscriptionTestIamPermissionCall.html), [*topics create*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicCreateCall.html), [*topics delete*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicDeleteCall.html), [*topics get*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicGetCall.html), [*topics get iam policy*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicGetIamPolicyCall.html), [*topics list*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicListCall.html), [*topics publish*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicPublishCall.html), [*topics set iam policy*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicSetIamPolicyCall.html), [*topics subscriptions list*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicSubscriptionListCall.html) and [*topics test iam permissions*](http://byron.github.io/google-apis-rs/google_pubsub1/struct.ProjectTopicTestIamPermissionCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_pubsub1/struct.Pubsub.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_pubsub1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_pubsub1/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_pubsub1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_pubsub1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_pubsub1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().subscriptions_acknowledge(...).doit()
let r = hub.projects().subscriptions_modify_push_config(...).doit()
let r = hub.projects().subscriptions_delete(...).doit()
let r = hub.projects().topics_delete(...).doit()
let r = hub.projects().subscriptions_modify_ack_deadline(...).doit()
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
google-pubsub1 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_pubsub1 as pubsub1;
use pubsub1::AcknowledgeRequest;
use pubsub1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use pubsub1::Pubsub;

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
let mut hub = Pubsub::new(hyper::Client::new(), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = AcknowledgeRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().subscriptions_acknowledge(req, "subscription")
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

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_pubsub1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_pubsub1/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_pubsub1/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_pubsub1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_pubsub1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_pubsub1/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_pubsub1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_pubsub1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_pubsub1/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_pubsub1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_pubsub1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_pubsub1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_pubsub1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **pubsub1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
