<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-monitoring3` library allows access to all features of the *Google Monitoring* service.

This documentation was generated from *Monitoring* crate version *1.0.13+20200329*, where *20200329* is the exact revision of the *monitoring:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *Monitoring* *v3* API can be found at the
[official documentation site](https://cloud.google.com/monitoring/api/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.Monitoring.html) ... 

* projects
 * [*alert policies create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectAlertPolicyCreateCall.html), [*alert policies delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectAlertPolicyDeleteCall.html), [*alert policies get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectAlertPolicyGetCall.html), [*alert policies list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectAlertPolicyListCall.html), [*alert policies patch*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectAlertPolicyPatchCall.html), [*collectd time series create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectCollectdTimeSeryCreateCall.html), [*groups create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectGroupCreateCall.html), [*groups delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectGroupDeleteCall.html), [*groups get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectGroupGetCall.html), [*groups list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectGroupListCall.html), [*groups members list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectGroupMemberListCall.html), [*groups update*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectGroupUpdateCall.html), [*metric descriptors create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectMetricDescriptorCreateCall.html), [*metric descriptors delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectMetricDescriptorDeleteCall.html), [*metric descriptors get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectMetricDescriptorGetCall.html), [*metric descriptors list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectMetricDescriptorListCall.html), [*monitored resource descriptors get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectMonitoredResourceDescriptorGetCall.html), [*monitored resource descriptors list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectMonitoredResourceDescriptorListCall.html), [*notification channel descriptors get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelDescriptorGetCall.html), [*notification channel descriptors list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelDescriptorListCall.html), [*notification channels create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelCreateCall.html), [*notification channels delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelDeleteCall.html), [*notification channels get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelGetCall.html), [*notification channels get verification code*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelGetVerificationCodeCall.html), [*notification channels list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelListCall.html), [*notification channels patch*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelPatchCall.html), [*notification channels send verification code*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelSendVerificationCodeCall.html), [*notification channels verify*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectNotificationChannelVerifyCall.html), [*time series create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectTimeSeryCreateCall.html), [*time series list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectTimeSeryListCall.html), [*time series query*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectTimeSeryQueryCall.html), [*uptime check configs create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectUptimeCheckConfigCreateCall.html), [*uptime check configs delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectUptimeCheckConfigDeleteCall.html), [*uptime check configs get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectUptimeCheckConfigGetCall.html), [*uptime check configs list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectUptimeCheckConfigListCall.html) and [*uptime check configs patch*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ProjectUptimeCheckConfigPatchCall.html)
* [services](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.Service.html)
 * [*create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceCreateCall.html), [*delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceDeleteCall.html), [*get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceGetCall.html), [*list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceListCall.html), [*patch*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServicePatchCall.html), [*service level objectives create*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceServiceLevelObjectiveCreateCall.html), [*service level objectives delete*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceServiceLevelObjectiveDeleteCall.html), [*service level objectives get*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceServiceLevelObjectiveGetCall.html), [*service level objectives list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceServiceLevelObjectiveListCall.html) and [*service level objectives patch*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.ServiceServiceLevelObjectivePatchCall.html)
* [uptime check ips](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.UptimeCheckIp.html)
 * [*list*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.UptimeCheckIpListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/struct.Monitoring.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.services().service_level_objectives_patch(...).doit()
let r = hub.services().service_level_objectives_create(...).doit()
let r = hub.services().service_level_objectives_delete(...).doit()
let r = hub.services().service_level_objectives_get(...).doit()
let r = hub.services().service_level_objectives_list(...).doit()
let r = hub.services().create(...).doit()
let r = hub.services().list(...).doit()
let r = hub.services().get(...).doit()
let r = hub.services().delete(...).doit()
let r = hub.services().patch(...).doit()
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
google-monitoring3 = "*"
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
extern crate google_monitoring3 as monitoring3;
use monitoring3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use monitoring3::Monitoring;

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
let mut hub = Monitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.services().service_level_objectives_list("parent")
             .view("sit")
             .page_token("Stet")
             .page_size(-42)
             .filter("et")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-monitoring3/1.0.13+20200329/google_monitoring3/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **monitoring3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
