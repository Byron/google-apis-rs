<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-tagmanager1` library allows access to all features of the *Google Tag Manager* service.

This documentation was generated from *Tag Manager* crate version *5.0.5+20240619*, where *20240619* is the exact revision of the *tagmanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Tag Manager* *v1* API can be found at the
[official documentation site](https://developers.google.com/tag-manager).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/TagManager) ... 

* [accounts](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::Account)
 * [*containers create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerCreateCall), [*containers delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerDeleteCall), [*containers environments create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerEnvironmentCreateCall), [*containers environments delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerEnvironmentDeleteCall), [*containers environments get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerEnvironmentGetCall), [*containers environments list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerEnvironmentListCall), [*containers environments update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerEnvironmentUpdateCall), [*containers folders create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerFolderCreateCall), [*containers folders delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerFolderDeleteCall), [*containers folders entities list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerFolderEntityListCall), [*containers folders get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerFolderGetCall), [*containers folders list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerFolderListCall), [*containers folders update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerFolderUpdateCall), [*containers get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerGetCall), [*containers list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerListCall), [*containers move_folders update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerMoveFolderUpdateCall), [*containers reauthorize_environments update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerReauthorizeEnvironmentUpdateCall), [*containers tags create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTagCreateCall), [*containers tags delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTagDeleteCall), [*containers tags get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTagGetCall), [*containers tags list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTagListCall), [*containers tags update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTagUpdateCall), [*containers triggers create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTriggerCreateCall), [*containers triggers delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTriggerDeleteCall), [*containers triggers get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTriggerGetCall), [*containers triggers list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTriggerListCall), [*containers triggers update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerTriggerUpdateCall), [*containers update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerUpdateCall), [*containers variables create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVariableCreateCall), [*containers variables delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVariableDeleteCall), [*containers variables get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVariableGetCall), [*containers variables list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVariableListCall), [*containers variables update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVariableUpdateCall), [*containers versions create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionCreateCall), [*containers versions delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionDeleteCall), [*containers versions get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionGetCall), [*containers versions list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionListCall), [*containers versions publish*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionPublishCall), [*containers versions restore*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionRestoreCall), [*containers versions undelete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionUndeleteCall), [*containers versions update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountContainerVersionUpdateCall), [*get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountGetCall), [*list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountListCall), [*permissions create*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountPermissionCreateCall), [*permissions delete*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountPermissionDeleteCall), [*permissions get*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountPermissionGetCall), [*permissions list*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountPermissionListCall), [*permissions update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountPermissionUpdateCall) and [*update*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/api::AccountUpdateCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/TagManager)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::CallBuilder)
* **[Resources](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().containers_environments_create(...).doit().await
let r = hub.accounts().containers_environments_delete(...).doit().await
let r = hub.accounts().containers_environments_get(...).doit().await
let r = hub.accounts().containers_environments_list(...).doit().await
let r = hub.accounts().containers_environments_update(...).doit().await
let r = hub.accounts().containers_folders_entities_list(...).doit().await
let r = hub.accounts().containers_folders_create(...).doit().await
let r = hub.accounts().containers_folders_delete(...).doit().await
let r = hub.accounts().containers_folders_get(...).doit().await
let r = hub.accounts().containers_folders_list(...).doit().await
let r = hub.accounts().containers_folders_update(...).doit().await
let r = hub.accounts().containers_move_folders_update(...).doit().await
let r = hub.accounts().containers_reauthorize_environments_update(...).doit().await
let r = hub.accounts().containers_tags_create(...).doit().await
let r = hub.accounts().containers_tags_delete(...).doit().await
let r = hub.accounts().containers_tags_get(...).doit().await
let r = hub.accounts().containers_tags_list(...).doit().await
let r = hub.accounts().containers_tags_update(...).doit().await
let r = hub.accounts().containers_triggers_create(...).doit().await
let r = hub.accounts().containers_triggers_delete(...).doit().await
let r = hub.accounts().containers_triggers_get(...).doit().await
let r = hub.accounts().containers_triggers_list(...).doit().await
let r = hub.accounts().containers_triggers_update(...).doit().await
let r = hub.accounts().containers_variables_create(...).doit().await
let r = hub.accounts().containers_variables_delete(...).doit().await
let r = hub.accounts().containers_variables_get(...).doit().await
let r = hub.accounts().containers_variables_list(...).doit().await
let r = hub.accounts().containers_variables_update(...).doit().await
let r = hub.accounts().containers_versions_create(...).doit().await
let r = hub.accounts().containers_versions_delete(...).doit().await
let r = hub.accounts().containers_versions_get(...).doit().await
let r = hub.accounts().containers_versions_list(...).doit().await
let r = hub.accounts().containers_versions_publish(...).doit().await
let r = hub.accounts().containers_versions_restore(...).doit().await
let r = hub.accounts().containers_versions_undelete(...).doit().await
let r = hub.accounts().containers_versions_update(...).doit().await
let r = hub.accounts().containers_create(...).doit().await
let r = hub.accounts().containers_delete(...).doit().await
let r = hub.accounts().containers_get(...).doit().await
let r = hub.accounts().containers_list(...).doit().await
let r = hub.accounts().containers_update(...).doit().await
let r = hub.accounts().permissions_create(...).doit().await
let r = hub.accounts().permissions_delete(...).doit().await
let r = hub.accounts().permissions_get(...).doit().await
let r = hub.accounts().permissions_list(...).doit().await
let r = hub.accounts().permissions_update(...).doit().await
let r = hub.accounts().get(...).doit().await
let r = hub.accounts().list(...).doit().await
let r = hub.accounts().update(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_tagmanager1 as tagmanager1;
use tagmanager1::api::Folder;
use tagmanager1::{Result, Error};
use std::default::Default;
use tagmanager1::{TagManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = TagManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Folder::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().containers_move_folders_update(req, "accountId", "containerId", "folderId")
             .add_variable_id("ipsum")
             .add_trigger_id("voluptua.")
             .add_tag_id("At")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::RequestValue) and 
[decodable](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-tagmanager1/5.0.5+20240619/google_tagmanager1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **tagmanager1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

