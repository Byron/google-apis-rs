<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-tagmanager1` library allows access to all features of the *Google Tag Manager* service.

This documentation was generated from *Tag Manager* crate version *0.1.2+20150121*, where *20150121* is the exact revision of the *tagmanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.2*.

Everything else about the *Tag Manager* *v1* API can be found at the
[official documentation site](https://developers.google.com/tag-manager/api/v1/).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.TagManager.html) ... 

* [accounts](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.Account.html)
 * [*containers create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerCreateCall.html), [*containers delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerDeleteCall.html), [*containers get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerGetCall.html), [*containers list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerListCall.html), [*containers macros create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerMacroCreateCall.html), [*containers macros delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerMacroDeleteCall.html), [*containers macros get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerMacroGetCall.html), [*containers macros list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerMacroListCall.html), [*containers macros update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerMacroUpdateCall.html), [*containers rules create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerRuleCreateCall.html), [*containers rules delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerRuleDeleteCall.html), [*containers rules get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerRuleGetCall.html), [*containers rules list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerRuleListCall.html), [*containers rules update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerRuleUpdateCall.html), [*containers tags create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTagCreateCall.html), [*containers tags delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTagDeleteCall.html), [*containers tags get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTagGetCall.html), [*containers tags list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTagListCall.html), [*containers tags update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTagUpdateCall.html), [*containers triggers create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTriggerCreateCall.html), [*containers triggers delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTriggerDeleteCall.html), [*containers triggers get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTriggerGetCall.html), [*containers triggers list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTriggerListCall.html), [*containers triggers update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerTriggerUpdateCall.html), [*containers update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerUpdateCall.html), [*containers variables create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVariableCreateCall.html), [*containers variables delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVariableDeleteCall.html), [*containers variables get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVariableGetCall.html), [*containers variables list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVariableListCall.html), [*containers variables update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVariableUpdateCall.html), [*containers versions create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionCreateCall.html), [*containers versions delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionDeleteCall.html), [*containers versions get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionGetCall.html), [*containers versions list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionListCall.html), [*containers versions publish*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionPublishCall.html), [*containers versions restore*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionRestoreCall.html), [*containers versions undelete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionUndeleteCall.html), [*containers versions update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountContainerVersionUpdateCall.html), [*get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountListCall.html), [*permissions create*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountPermissionCreateCall.html), [*permissions delete*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountPermissionDeleteCall.html), [*permissions get*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountPermissionGetCall.html), [*permissions list*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountPermissionListCall.html), [*permissions update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountPermissionUpdateCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.AccountUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google-tagmanager1/struct.TagManager.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.accounts().containers_list(...).doit()
let r = hub.accounts().permissions_list(...).doit()
let r = hub.accounts().containers_versions_undelete(...).doit()
let r = hub.accounts().permissions_create(...).doit()
let r = hub.accounts().permissions_delete(...).doit()
let r = hub.accounts().containers_get(...).doit()
let r = hub.accounts().containers_versions_list(...).doit()
let r = hub.accounts().containers_triggers_update(...).doit()
let r = hub.accounts().containers_triggers_get(...).doit()
let r = hub.accounts().containers_delete(...).doit()
let r = hub.accounts().containers_create(...).doit()
let r = hub.accounts().containers_tags_delete(...).doit()
let r = hub.accounts().containers_rules_update(...).doit()
let r = hub.accounts().containers_rules_delete(...).doit()
let r = hub.accounts().containers_tags_list(...).doit()
let r = hub.accounts().containers_versions_publish(...).doit()
let r = hub.accounts().containers_tags_create(...).doit()
let r = hub.accounts().containers_triggers_list(...).doit()
let r = hub.accounts().containers_versions_delete(...).doit()
let r = hub.accounts().update(...).doit()
let r = hub.accounts().containers_macros_delete(...).doit()
let r = hub.accounts().containers_versions_create(...).doit()
let r = hub.accounts().permissions_get(...).doit()
let r = hub.accounts().containers_rules_create(...).doit()
let r = hub.accounts().containers_versions_restore(...).doit()
let r = hub.accounts().containers_rules_get(...).doit()
let r = hub.accounts().containers_variables_create(...).doit()
let r = hub.accounts().containers_variables_list(...).doit()
let r = hub.accounts().containers_macros_create(...).doit()
let r = hub.accounts().containers_tags_get(...).doit()
let r = hub.accounts().containers_variables_get(...).doit()
let r = hub.accounts().containers_triggers_delete(...).doit()
let r = hub.accounts().containers_macros_list(...).doit()
let r = hub.accounts().containers_triggers_create(...).doit()
let r = hub.accounts().containers_macros_update(...).doit()
let r = hub.accounts().list(...).doit()
let r = hub.accounts().permissions_update(...).doit()
let r = hub.accounts().containers_variables_delete(...).doit()
let r = hub.accounts().get(...).doit()
let r = hub.accounts().containers_update(...).doit()
let r = hub.accounts().containers_rules_list(...).doit()
let r = hub.accounts().containers_tags_update(...).doit()
let r = hub.accounts().containers_macros_get(...).doit()
let r = hub.accounts().containers_versions_update(...).doit()
let r = hub.accounts().containers_variables_update(...).doit()
let r = hub.accounts().containers_versions_get(...).doit()
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
google-tagmanager1 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate "yup-oauth2" as oauth2;
extern crate "google-tagmanager1" as tagmanager1;
use tagmanager1::Trigger;
use tagmanager1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use tagmanager1::TagManager;

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
let mut hub = TagManager::new(hyper::Client::new(), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req: Trigger = Default::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().containers_triggers_update(&req, "accountId", "containerId", "triggerId")
             .fingerprint("sed")
             .doit();

match result {
    Err(e) => match e {
        Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
        Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
        Error::MissingToken => println!("OAuth2: Missing Token"),
        Error::Cancelled => println!("Operation canceled by user"),
        Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
        Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
        Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
        Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
    },
    Ok(_) => println!("Success (value doesn't print)"),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google-tagmanager1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google-tagmanager1/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google-tagmanager1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google-tagmanager1/trait.RequestValue.html) are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **tagmanager1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
