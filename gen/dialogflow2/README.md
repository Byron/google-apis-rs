<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-dialogflow2` library allows access to all features of the *Google Dialogflow* service.

This documentation was generated from *Dialogflow* crate version *1.0.13+20200408*, where *20200408* is the exact revision of the *dialogflow:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.

Everything else about the *Dialogflow* *v2* API can be found at the
[official documentation site](https://cloud.google.com/dialogflow/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.Dialogflow.html) ... 

* projects
 * [*agent entity types batch delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeBatchDeleteCall.html), [*agent entity types batch update*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeBatchUpdateCall.html), [*agent entity types create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeCreateCall.html), [*agent entity types delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeDeleteCall.html), [*agent entity types entities batch create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeEntityBatchCreateCall.html), [*agent entity types entities batch delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeEntityBatchDeleteCall.html), [*agent entity types entities batch update*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeEntityBatchUpdateCall.html), [*agent entity types get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeGetCall.html), [*agent entity types list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypeListCall.html), [*agent entity types patch*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEntityTypePatchCall.html), [*agent environments users sessions contexts create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionContextCreateCall.html), [*agent environments users sessions contexts delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionContextDeleteCall.html), [*agent environments users sessions contexts get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionContextGetCall.html), [*agent environments users sessions contexts list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionContextListCall.html), [*agent environments users sessions contexts patch*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionContextPatchCall.html), [*agent environments users sessions delete contexts*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionDeleteContextCall.html), [*agent environments users sessions detect intent*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionDetectIntentCall.html), [*agent environments users sessions entity types create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionEntityTypeCreateCall.html), [*agent environments users sessions entity types delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionEntityTypeDeleteCall.html), [*agent environments users sessions entity types get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionEntityTypeGetCall.html), [*agent environments users sessions entity types list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionEntityTypeListCall.html), [*agent environments users sessions entity types patch*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentEnvironmentUserSessionEntityTypePatchCall.html), [*agent export*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentExportCall.html), [*agent get fulfillment*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentGetFulfillmentCall.html), [*agent get validation result*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentGetValidationResultCall.html), [*agent import*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentImportCall.html), [*agent intents batch delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentBatchDeleteCall.html), [*agent intents batch update*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentBatchUpdateCall.html), [*agent intents create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentCreateCall.html), [*agent intents delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentDeleteCall.html), [*agent intents get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentGetCall.html), [*agent intents list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentListCall.html), [*agent intents patch*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentIntentPatchCall.html), [*agent restore*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentRestoreCall.html), [*agent search*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSearchCall.html), [*agent sessions contexts create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionContextCreateCall.html), [*agent sessions contexts delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionContextDeleteCall.html), [*agent sessions contexts get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionContextGetCall.html), [*agent sessions contexts list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionContextListCall.html), [*agent sessions contexts patch*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionContextPatchCall.html), [*agent sessions delete contexts*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionDeleteContextCall.html), [*agent sessions detect intent*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionDetectIntentCall.html), [*agent sessions entity types create*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionEntityTypeCreateCall.html), [*agent sessions entity types delete*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionEntityTypeDeleteCall.html), [*agent sessions entity types get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionEntityTypeGetCall.html), [*agent sessions entity types list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionEntityTypeListCall.html), [*agent sessions entity types patch*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentSessionEntityTypePatchCall.html), [*agent train*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentTrainCall.html), [*agent update fulfillment*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectAgentUpdateFulfillmentCall.html), [*delete agent*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectDeleteAgentCall.html), [*get agent*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectGetAgentCall.html), [*locations operations cancel*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectLocationOperationCancelCall.html), [*locations operations get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectLocationOperationGetCall.html), [*locations operations list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectLocationOperationListCall.html), [*operations cancel*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectOperationCancelCall.html), [*operations get*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectOperationGetCall.html), [*operations list*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectOperationListCall.html) and [*set agent*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.ProjectSetAgentCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/struct.Dialogflow.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().agent_restore(...).doit()
let r = hub.projects().agent_intents_batch_delete(...).doit()
let r = hub.projects().agent_entity_types_batch_delete(...).doit()
let r = hub.projects().agent_train(...).doit()
let r = hub.projects().agent_export(...).doit()
let r = hub.projects().agent_entity_types_batch_update(...).doit()
let r = hub.projects().agent_import(...).doit()
let r = hub.projects().agent_intents_batch_update(...).doit()
let r = hub.projects().locations_operations_get(...).doit()
let r = hub.projects().agent_entity_types_entities_batch_update(...).doit()
let r = hub.projects().agent_entity_types_entities_batch_delete(...).doit()
let r = hub.projects().operations_get(...).doit()
let r = hub.projects().agent_entity_types_entities_batch_create(...).doit()
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
google-dialogflow2 = "*"
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
extern crate google_dialogflow2 as dialogflow2;
use dialogflow2::GoogleCloudDialogflowV2RestoreAgentRequest;
use dialogflow2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use dialogflow2::Dialogflow;

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
let mut hub = Dialogflow::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleCloudDialogflowV2RestoreAgentRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().agent_restore(req, "parent")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-dialogflow2/1.0.13+20200408/google_dialogflow2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **dialogflow2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
