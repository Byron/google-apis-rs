<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-tagmanager2` library allows access to all features of the *Google Tag Manager* service.

This documentation was generated from *Tag Manager* crate version *1.0.8+20171108*, where *20171108* is the exact revision of the *tagmanager:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.

Everything else about the *Tag Manager* *v2* API can be found at the
[official documentation site](https://developers.google.com/tag-manager/api/v2/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.TagManager.html) ... 

* [accounts](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.Account.html)
 * [*containers create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerCreateCall.html), [*containers delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerDeleteCall.html), [*containers environments create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentCreateCall.html), [*containers environments delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentDeleteCall.html), [*containers environments get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentGetCall.html), [*containers environments list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentListCall.html), [*containers environments patch*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentPatchCall.html), [*containers environments reauthorize*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentReauthorizeCall.html), [*containers environments update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerEnvironmentUpdateCall.html), [*containers get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerGetCall.html), [*containers list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerListCall.html), [*containers update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerUpdateCall.html), [*containers version_headers latest*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionHeaderLatestCall.html), [*containers version_headers list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionHeaderListCall.html), [*containers versions delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionDeleteCall.html), [*containers versions get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionGetCall.html), [*containers versions live*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionLiveCall.html), [*containers versions publish*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionPublishCall.html), [*containers versions set_latest*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionSetLatestCall.html), [*containers versions undelete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionUndeleteCall.html), [*containers versions update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerVersionUpdateCall.html), [*containers workspaces built_in_variables create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceBuiltInVariableCreateCall.html), [*containers workspaces built_in_variables delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceBuiltInVariableDeleteCall.html), [*containers workspaces built_in_variables list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceBuiltInVariableListCall.html), [*containers workspaces built_in_variables revert*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceBuiltInVariableRevertCall.html), [*containers workspaces create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceCreateCall.html), [*containers workspaces create_version*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceCreateVersionCall.html), [*containers workspaces delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceDeleteCall.html), [*containers workspaces folders create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderCreateCall.html), [*containers workspaces folders delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderDeleteCall.html), [*containers workspaces folders entities*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderEntityCall.html), [*containers workspaces folders get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderGetCall.html), [*containers workspaces folders list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderListCall.html), [*containers workspaces folders move_entities_to_folder*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderMoveEntitiesToFolderCall.html), [*containers workspaces folders revert*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderRevertCall.html), [*containers workspaces folders update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceFolderUpdateCall.html), [*containers workspaces get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceGetCall.html), [*containers workspaces get proposal*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceGetProposalCall.html), [*containers workspaces get status*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceGetStatuCall.html), [*containers workspaces list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceListCall.html), [*containers workspaces proposal create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceProposalCreateCall.html), [*containers workspaces proposal delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceProposalDeleteCall.html), [*containers workspaces quick_preview*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceQuickPreviewCall.html), [*containers workspaces resolve_conflict*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceResolveConflictCall.html), [*containers workspaces sync*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceSyncCall.html), [*containers workspaces tags create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTagCreateCall.html), [*containers workspaces tags delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTagDeleteCall.html), [*containers workspaces tags get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTagGetCall.html), [*containers workspaces tags list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTagListCall.html), [*containers workspaces tags revert*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTagRevertCall.html), [*containers workspaces tags update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTagUpdateCall.html), [*containers workspaces triggers create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTriggerCreateCall.html), [*containers workspaces triggers delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTriggerDeleteCall.html), [*containers workspaces triggers get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTriggerGetCall.html), [*containers workspaces triggers list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTriggerListCall.html), [*containers workspaces triggers revert*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTriggerRevertCall.html), [*containers workspaces triggers update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceTriggerUpdateCall.html), [*containers workspaces update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceUpdateCall.html), [*containers workspaces update proposal*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceUpdateProposalCall.html), [*containers workspaces variables create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceVariableCreateCall.html), [*containers workspaces variables delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceVariableDeleteCall.html), [*containers workspaces variables get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceVariableGetCall.html), [*containers workspaces variables list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceVariableListCall.html), [*containers workspaces variables revert*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceVariableRevertCall.html), [*containers workspaces variables update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountContainerWorkspaceVariableUpdateCall.html), [*get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountGetCall.html), [*list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountListCall.html), [*update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountUpdateCall.html), [*user_permissions create*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountUserPermissionCreateCall.html), [*user_permissions delete*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountUserPermissionDeleteCall.html), [*user_permissions get*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountUserPermissionGetCall.html), [*user_permissions list*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountUserPermissionListCall.html) and [*user_permissions update*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.AccountUserPermissionUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/struct.TagManager.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.accounts().containers_workspaces_get(...).doit()
let r = hub.accounts().containers_workspaces_folders_move_entities_to_folder(...).doit()
let r = hub.accounts().containers_version_headers_latest(...).doit()
let r = hub.accounts().containers_workspaces_triggers_create(...).doit()
let r = hub.accounts().containers_workspaces_list(...).doit()
let r = hub.accounts().containers_workspaces_built_in_variables_revert(...).doit()
let r = hub.accounts().containers_workspaces_built_in_variables_delete(...).doit()
let r = hub.accounts().containers_environments_create(...).doit()
let r = hub.accounts().containers_versions_publish(...).doit()
let r = hub.accounts().containers_workspaces_folders_get(...).doit()
let r = hub.accounts().containers_workspaces_tags_create(...).doit()
let r = hub.accounts().containers_environments_delete(...).doit()
let r = hub.accounts().containers_get(...).doit()
let r = hub.accounts().user_permissions_create(...).doit()
let r = hub.accounts().containers_workspaces_folders_list(...).doit()
let r = hub.accounts().containers_environments_reauthorize(...).doit()
let r = hub.accounts().containers_workspaces_delete(...).doit()
let r = hub.accounts().containers_workspaces_get_proposal(...).doit()
let r = hub.accounts().user_permissions_list(...).doit()
let r = hub.accounts().containers_workspaces_folders_delete(...).doit()
let r = hub.accounts().containers_workspaces_quick_preview(...).doit()
let r = hub.accounts().containers_workspaces_variables_get(...).doit()
let r = hub.accounts().containers_versions_set_latest(...).doit()
let r = hub.accounts().containers_workspaces_variables_update(...).doit()
let r = hub.accounts().containers_environments_list(...).doit()
let r = hub.accounts().containers_workspaces_tags_list(...).doit()
let r = hub.accounts().containers_versions_undelete(...).doit()
let r = hub.accounts().containers_workspaces_triggers_list(...).doit()
let r = hub.accounts().containers_workspaces_create_version(...).doit()
let r = hub.accounts().containers_delete(...).doit()
let r = hub.accounts().containers_workspaces_create(...).doit()
let r = hub.accounts().update(...).doit()
let r = hub.accounts().containers_version_headers_list(...).doit()
let r = hub.accounts().containers_workspaces_built_in_variables_list(...).doit()
let r = hub.accounts().user_permissions_update(...).doit()
let r = hub.accounts().containers_environments_get(...).doit()
let r = hub.accounts().get(...).doit()
let r = hub.accounts().containers_versions_live(...).doit()
let r = hub.accounts().list(...).doit()
let r = hub.accounts().containers_workspaces_variables_list(...).doit()
let r = hub.accounts().containers_workspaces_variables_revert(...).doit()
let r = hub.accounts().containers_workspaces_folders_entities(...).doit()
let r = hub.accounts().containers_workspaces_folders_create(...).doit()
let r = hub.accounts().containers_workspaces_get_status(...).doit()
let r = hub.accounts().containers_workspaces_tags_delete(...).doit()
let r = hub.accounts().containers_update(...).doit()
let r = hub.accounts().containers_environments_patch(...).doit()
let r = hub.accounts().containers_workspaces_folders_update(...).doit()
let r = hub.accounts().containers_workspaces_built_in_variables_create(...).doit()
let r = hub.accounts().containers_workspaces_proposal_delete(...).doit()
let r = hub.accounts().containers_list(...).doit()
let r = hub.accounts().containers_workspaces_triggers_delete(...).doit()
let r = hub.accounts().containers_workspaces_sync(...).doit()
let r = hub.accounts().containers_workspaces_tags_revert(...).doit()
let r = hub.accounts().containers_versions_delete(...).doit()
let r = hub.accounts().containers_workspaces_tags_update(...).doit()
let r = hub.accounts().containers_create(...).doit()
let r = hub.accounts().containers_workspaces_triggers_update(...).doit()
let r = hub.accounts().containers_workspaces_variables_create(...).doit()
let r = hub.accounts().user_permissions_get(...).doit()
let r = hub.accounts().containers_workspaces_triggers_revert(...).doit()
let r = hub.accounts().containers_workspaces_variables_delete(...).doit()
let r = hub.accounts().containers_workspaces_tags_get(...).doit()
let r = hub.accounts().containers_workspaces_resolve_conflict(...).doit()
let r = hub.accounts().containers_environments_update(...).doit()
let r = hub.accounts().containers_workspaces_triggers_get(...).doit()
let r = hub.accounts().containers_workspaces_folders_revert(...).doit()
let r = hub.accounts().containers_workspaces_proposal_create(...).doit()
let r = hub.accounts().containers_versions_update(...).doit()
let r = hub.accounts().containers_versions_get(...).doit()
let r = hub.accounts().containers_workspaces_update(...).doit()
let r = hub.accounts().containers_workspaces_update_proposal(...).doit()
let r = hub.accounts().user_permissions_delete(...).doit()
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
google-tagmanager2 = "*"
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
extern crate google_tagmanager2 as tagmanager2;
use tagmanager2::Folder;
use tagmanager2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use tagmanager2::TagManager;

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
let result = hub.accounts().containers_workspaces_folders_move_entities_to_folder(req, "path")
             .add_variable_id("sit")
             .add_trigger_id("Stet")
             .add_tag_id("sed")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-tagmanager2/1.0.8+20171108/google_tagmanager2/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **tagmanager2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
