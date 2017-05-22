<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-partners2` library allows access to all features of the *Google Partners* service.

This documentation was generated from *Partners* crate version *1.0.4+20170503*, where *20170503* is the exact revision of the *partners:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.4*.

Everything else about the *Partners* *v2* API can be found at the
[official documentation site](https://developers.google.com/partners/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.Partners.html) ... 

* analytics
 * [*list*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.AnalyticListCall.html)
* client messages
 * [*log*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.ClientMessageLogCall.html)
* [companies](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.Company.html)
 * [*get*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.CompanyGetCall.html), [*leads create*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.CompanyLeadCreateCall.html) and [*list*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.CompanyListCall.html)
* exams
 * [*get token*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.ExamGetTokenCall.html)
* [leads](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.Lead.html)
 * [*list*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.LeadListCall.html)
* offers
 * [*history list*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.OfferHistoryListCall.html) and [*list*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.OfferListCall.html)
* user events
 * [*log*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.UserEventLogCall.html)
* user states
 * [*list*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.UserStateListCall.html)
* [users](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.User.html)
 * [*create company relation*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.UserCreateCompanyRelationCall.html), [*delete company relation*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.UserDeleteCompanyRelationCall.html), [*get*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.UserGetCall.html) and [*update profile*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.UserUpdateProfileCall.html)

Other activities are ...

* [get partnersstatus](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.MethodGetPartnersstatuCall.html)
* [update companies](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.MethodUpdateCompanyCall.html)
* [update leads](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.MethodUpdateLeadCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/struct.Partners.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.users().create_company_relation(...).doit()
let r = hub.users().get(...).doit()
let r = hub.users().update_profile(...).doit()
let r = hub.users().delete_company_relation(...).doit()
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
google-partners2 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_partners2 as partners2;
use partners2::CompanyRelation;
use partners2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use partners2::Partners;

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
let mut hub = Partners::new(hyper::Client::new(), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = CompanyRelation::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.users().create_company_relation(req, "userId")
             .request_metadata_user_overrides_user_id("sit")
             .request_metadata_user_overrides_ip_address("Stet")
             .request_metadata_traffic_source_traffic_sub_id("sed")
             .request_metadata_traffic_source_traffic_source_id("et")
             .request_metadata_partners_session_id("dolores")
             .request_metadata_locale("kasd")
             .add_request_metadata_experiment_ids("accusam")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-partners2/1.0.4+20170503/google_partners2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **partners2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
