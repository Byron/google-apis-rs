<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-ml1` library allows access to all features of the *Google Cloud Machine Learning Engine* service.

This documentation was generated from *Cloud Machine Learning Engine* crate version *1.0.13+20200328*, where *20200328* is the exact revision of the *ml:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *Cloud Machine Learning Engine* *v1* API can be found at the
[official documentation site](https://cloud.google.com/ml/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.CloudMachineLearningEngine.html) ... 

* projects
 * [*explain*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectExplainCall.html), [*get config*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectGetConfigCall.html), [*jobs cancel*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobCancelCall.html), [*jobs create*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobCreateCall.html), [*jobs get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobGetCall.html), [*jobs get iam policy*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobGetIamPolicyCall.html), [*jobs list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobListCall.html), [*jobs patch*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobPatchCall.html), [*jobs set iam policy*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobSetIamPolicyCall.html), [*jobs test iam permissions*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectJobTestIamPermissionCall.html), [*locations get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationGetCall.html), [*locations list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationListCall.html), [*locations operations cancel*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationOperationCancelCall.html), [*locations operations get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationOperationGetCall.html), [*locations studies create*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyCreateCall.html), [*locations studies delete*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyDeleteCall.html), [*locations studies get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyGetCall.html), [*locations studies list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyListCall.html), [*locations studies trials add measurement*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialAddMeasurementCall.html), [*locations studies trials check early stopping state*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialCheckEarlyStoppingStateCall.html), [*locations studies trials complete*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialCompleteCall.html), [*locations studies trials create*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialCreateCall.html), [*locations studies trials delete*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialDeleteCall.html), [*locations studies trials get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialGetCall.html), [*locations studies trials list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialListCall.html), [*locations studies trials stop*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialStopCall.html), [*locations studies trials suggest*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectLocationStudyTrialSuggestCall.html), [*models create*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelCreateCall.html), [*models delete*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelDeleteCall.html), [*models get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelGetCall.html), [*models get iam policy*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelGetIamPolicyCall.html), [*models list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelListCall.html), [*models patch*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelPatchCall.html), [*models set iam policy*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelSetIamPolicyCall.html), [*models test iam permissions*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelTestIamPermissionCall.html), [*models versions create*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelVersionCreateCall.html), [*models versions delete*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelVersionDeleteCall.html), [*models versions get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelVersionGetCall.html), [*models versions list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelVersionListCall.html), [*models versions patch*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelVersionPatchCall.html), [*models versions set default*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectModelVersionSetDefaultCall.html), [*operations cancel*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectOperationCancelCall.html), [*operations get*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectOperationGetCall.html), [*operations list*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectOperationListCall.html) and [*predict*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.ProjectPredictCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/struct.CloudMachineLearningEngine.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().locations_studies_trials_suggest(...).doit()
let r = hub.projects().models_versions_create(...).doit()
let r = hub.projects().models_versions_patch(...).doit()
let r = hub.projects().operations_get(...).doit()
let r = hub.projects().models_versions_delete(...).doit()
let r = hub.projects().locations_operations_get(...).doit()
let r = hub.projects().locations_studies_trials_check_early_stopping_state(...).doit()
let r = hub.projects().models_delete(...).doit()
let r = hub.projects().models_patch(...).doit()
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
google-ml1 = "*"
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
extern crate google_ml1 as ml1;
use ml1::GoogleCloudMlV1__Version;
use ml1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use ml1::CloudMachineLearningEngine;

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
let mut hub = CloudMachineLearningEngine::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleCloudMlV1__Version::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().models_versions_patch(req, "name")
             .update_mask("sit")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-ml1/1.0.13+20200328/google_ml1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **ml1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
