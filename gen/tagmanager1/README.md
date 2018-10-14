<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-tagmanager1` library allows access to all features of the *Google Tag Manager* service.

This documentation was generated from *Tag Manager* crate version *1.0.8+20171108*, where *20171108* is the exact revision of the *tagmanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.

Everything else about the *Tag Manager* *v1* API can be found at the
[official documentation site](https://developers.google.com/tag-manager/api/v1/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.TagManager.html) ... 

* [accounts](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.Account.html)
 * [*containers create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerCreateCall.html), [*containers delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerDeleteCall.html), [*containers environments create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerEnvironmentCreateCall.html), [*containers environments delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerEnvironmentDeleteCall.html), [*containers environments get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerEnvironmentGetCall.html), [*containers environments list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerEnvironmentListCall.html), [*containers environments patch*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerEnvironmentPatchCall.html), [*containers environments update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerEnvironmentUpdateCall.html), [*containers folders create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerFolderCreateCall.html), [*containers folders delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerFolderDeleteCall.html), [*containers folders entities list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerFolderEntityListCall.html), [*containers folders get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerFolderGetCall.html), [*containers folders list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerFolderListCall.html), [*containers folders update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerFolderUpdateCall.html), [*containers get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerGetCall.html), [*containers list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerListCall.html), [*containers move_folders update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerMoveFolderUpdateCall.html), [*containers reauthorize_environments update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerReauthorizeEnvironmentUpdateCall.html), [*containers tags create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTagCreateCall.html), [*containers tags delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTagDeleteCall.html), [*containers tags get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTagGetCall.html), [*containers tags list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTagListCall.html), [*containers tags update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTagUpdateCall.html), [*containers triggers create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTriggerCreateCall.html), [*containers triggers delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTriggerDeleteCall.html), [*containers triggers get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTriggerGetCall.html), [*containers triggers list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTriggerListCall.html), [*containers triggers update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerTriggerUpdateCall.html), [*containers update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerUpdateCall.html), [*containers variables create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVariableCreateCall.html), [*containers variables delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVariableDeleteCall.html), [*containers variables get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVariableGetCall.html), [*containers variables list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVariableListCall.html), [*containers variables update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVariableUpdateCall.html), [*containers versions create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionCreateCall.html), [*containers versions delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionDeleteCall.html), [*containers versions get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionGetCall.html), [*containers versions list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionListCall.html), [*containers versions publish*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionPublishCall.html), [*containers versions restore*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionRestoreCall.html), [*containers versions undelete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionUndeleteCall.html), [*containers versions update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountContainerVersionUpdateCall.html), [*get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountGetCall.html), [*list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountListCall.html), [*permissions create*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountPermissionCreateCall.html), [*permissions delete*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountPermissionDeleteCall.html), [*permissions get*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountPermissionGetCall.html), [*permissions list*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountPermissionListCall.html), [*permissions update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountPermissionUpdateCall.html) and [*update*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.AccountUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/struct.TagManager.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.CallBuilder.html)**
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
let r = hub.accounts().containers_environments_create(...).doit()
let r = hub.accounts().permissions_create(...).doit()
let r = hub.accounts().containers_environments_delete(...).doit()
let r = hub.accounts().permissions_delete(...).doit()
let r = hub.accounts().containers_get(...).doit()
let r = hub.accounts().containers_environments_patch(...).doit()
let r = hub.accounts().containers_versions_list(...).doit()
let r = hub.accounts().containers_triggers_update(...).doit()
let r = hub.accounts().containers_triggers_get(...).doit()
let r = hub.accounts().containers_delete(...).doit()
let r = hub.accounts().containers_folders_delete(...).doit()
let r = hub.accounts().containers_create(...).doit()
let r = hub.accounts().containers_tags_delete(...).doit()
let r = hub.accounts().containers_folders_entities_list(...).doit()
let r = hub.accounts().containers_versions_undelete(...).doit()
let r = hub.accounts().containers_environments_list(...).doit()
let r = hub.accounts().containers_tags_list(...).doit()
let r = hub.accounts().containers_versions_publish(...).doit()
let r = hub.accounts().containers_folders_get(...).doit()
let r = hub.accounts().containers_environments_get(...).doit()
let r = hub.accounts().containers_tags_create(...).doit()
let r = hub.accounts().containers_triggers_list(...).doit()
let r = hub.accounts().containers_versions_delete(...).doit()
let r = hub.accounts().update(...).doit()
let r = hub.accounts().containers_versions_create(...).doit()
let r = hub.accounts().permissions_get(...).doit()
let r = hub.accounts().containers_move_folders_update(...).doit()
let r = hub.accounts().containers_versions_restore(...).doit()
let r = hub.accounts().containers_variables_create(...).doit()
let r = hub.accounts().containers_variables_list(...).doit()
let r = hub.accounts().containers_tags_get(...).doit()
let r = hub.accounts().containers_variables_get(...).doit()
let r = hub.accounts().containers_triggers_delete(...).doit()
let r = hub.accounts().containers_triggers_create(...).doit()
let r = hub.accounts().containers_folders_update(...).doit()
let r = hub.accounts().list(...).doit()
let r = hub.accounts().permissions_update(...).doit()
let r = hub.accounts().containers_variables_delete(...).doit()
let r = hub.accounts().containers_reauthorize_environments_update(...).doit()
let r = hub.accounts().get(...).doit()
let r = hub.accounts().containers_update(...).doit()
let r = hub.accounts().containers_tags_update(...).doit()
let r = hub.accounts().containers_environments_update(...).doit()
let r = hub.accounts().containers_folders_create(...).doit()
let r = hub.accounts().containers_folders_list(...).doit()
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
extern crate google_tagmanager1 as tagmanager1;
use tagmanager1::Folder;
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
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
let mut hub = TagManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Folder::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().containers_move_folders_update(req, "accountId", "containerId", "folderId")
             .add_variable_id("sed")
             .add_trigger_id("et")
             .add_tag_id("dolores")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-tagmanager1/1.0.8+20171108/google_tagmanager1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **tagmanager1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
