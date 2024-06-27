<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-tagmanager2` library allows access to all features of the *Google Tag Manager* service.

This documentation was generated from *Tag Manager* crate version *5.0.5+20240619*, where *20240619* is the exact revision of the *tagmanager:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Tag Manager* *v2* API can be found at the
[official documentation site](https://developers.google.com/tag-manager).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/TagManager) ... 

* [accounts](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::Account)
 * [*containers combine*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerCombineCall), [*containers create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerCreateCall), [*containers delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerDeleteCall), [*containers destinations get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerDestinationGetCall), [*containers destinations link*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerDestinationLinkCall), [*containers destinations list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerDestinationListCall), [*containers environments create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerEnvironmentCreateCall), [*containers environments delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerEnvironmentDeleteCall), [*containers environments get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerEnvironmentGetCall), [*containers environments list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerEnvironmentListCall), [*containers environments reauthorize*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerEnvironmentReauthorizeCall), [*containers environments update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerEnvironmentUpdateCall), [*containers get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerGetCall), [*containers list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerListCall), [*containers lookup*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerLookupCall), [*containers move_tag_id*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerMoveTagIdCall), [*containers snippet*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerSnippetCall), [*containers update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerUpdateCall), [*containers version_headers latest*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionHeaderLatestCall), [*containers version_headers list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionHeaderListCall), [*containers versions delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionDeleteCall), [*containers versions get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionGetCall), [*containers versions live*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionLiveCall), [*containers versions publish*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionPublishCall), [*containers versions set_latest*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionSetLatestCall), [*containers versions undelete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionUndeleteCall), [*containers versions update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerVersionUpdateCall), [*containers workspaces built_in_variables create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceBuiltInVariableCreateCall), [*containers workspaces built_in_variables delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceBuiltInVariableDeleteCall), [*containers workspaces built_in_variables list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceBuiltInVariableListCall), [*containers workspaces built_in_variables revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceBuiltInVariableRevertCall), [*containers workspaces clients create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceClientCreateCall), [*containers workspaces clients delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceClientDeleteCall), [*containers workspaces clients get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceClientGetCall), [*containers workspaces clients list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceClientListCall), [*containers workspaces clients revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceClientRevertCall), [*containers workspaces clients update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceClientUpdateCall), [*containers workspaces create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceCreateCall), [*containers workspaces create_version*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceCreateVersionCall), [*containers workspaces delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceDeleteCall), [*containers workspaces folders create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderCreateCall), [*containers workspaces folders delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderDeleteCall), [*containers workspaces folders entities*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderEntityCall), [*containers workspaces folders get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderGetCall), [*containers workspaces folders list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderListCall), [*containers workspaces folders move_entities_to_folder*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderMoveEntitiesToFolderCall), [*containers workspaces folders revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderRevertCall), [*containers workspaces folders update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceFolderUpdateCall), [*containers workspaces get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGetCall), [*containers workspaces get status*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGetStatuCall), [*containers workspaces gtag_config create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGtagConfigCreateCall), [*containers workspaces gtag_config delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGtagConfigDeleteCall), [*containers workspaces gtag_config get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGtagConfigGetCall), [*containers workspaces gtag_config list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGtagConfigListCall), [*containers workspaces gtag_config update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceGtagConfigUpdateCall), [*containers workspaces list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceListCall), [*containers workspaces quick_preview*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceQuickPreviewCall), [*containers workspaces resolve_conflict*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceResolveConflictCall), [*containers workspaces sync*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceSyncCall), [*containers workspaces tags create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTagCreateCall), [*containers workspaces tags delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTagDeleteCall), [*containers workspaces tags get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTagGetCall), [*containers workspaces tags list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTagListCall), [*containers workspaces tags revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTagRevertCall), [*containers workspaces tags update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTagUpdateCall), [*containers workspaces templates create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTemplateCreateCall), [*containers workspaces templates delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTemplateDeleteCall), [*containers workspaces templates get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTemplateGetCall), [*containers workspaces templates list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTemplateListCall), [*containers workspaces templates revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTemplateRevertCall), [*containers workspaces templates update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTemplateUpdateCall), [*containers workspaces transformations create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTransformationCreateCall), [*containers workspaces transformations delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTransformationDeleteCall), [*containers workspaces transformations get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTransformationGetCall), [*containers workspaces transformations list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTransformationListCall), [*containers workspaces transformations revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTransformationRevertCall), [*containers workspaces transformations update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTransformationUpdateCall), [*containers workspaces triggers create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTriggerCreateCall), [*containers workspaces triggers delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTriggerDeleteCall), [*containers workspaces triggers get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTriggerGetCall), [*containers workspaces triggers list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTriggerListCall), [*containers workspaces triggers revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTriggerRevertCall), [*containers workspaces triggers update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceTriggerUpdateCall), [*containers workspaces update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceUpdateCall), [*containers workspaces variables create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceVariableCreateCall), [*containers workspaces variables delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceVariableDeleteCall), [*containers workspaces variables get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceVariableGetCall), [*containers workspaces variables list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceVariableListCall), [*containers workspaces variables revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceVariableRevertCall), [*containers workspaces variables update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceVariableUpdateCall), [*containers workspaces zones create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceZoneCreateCall), [*containers workspaces zones delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceZoneDeleteCall), [*containers workspaces zones get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceZoneGetCall), [*containers workspaces zones list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceZoneListCall), [*containers workspaces zones revert*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceZoneRevertCall), [*containers workspaces zones update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountContainerWorkspaceZoneUpdateCall), [*get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountGetCall), [*list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountListCall), [*update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountUpdateCall), [*user_permissions create*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountUserPermissionCreateCall), [*user_permissions delete*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountUserPermissionDeleteCall), [*user_permissions get*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountUserPermissionGetCall), [*user_permissions list*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountUserPermissionListCall) and [*user_permissions update*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/api::AccountUserPermissionUpdateCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/TagManager)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::CallBuilder)
* **[Resources](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().containers_destinations_get(...).doit().await
let r = hub.accounts().containers_destinations_link(...).doit().await
let r = hub.accounts().containers_destinations_list(...).doit().await
let r = hub.accounts().containers_environments_create(...).doit().await
let r = hub.accounts().containers_environments_delete(...).doit().await
let r = hub.accounts().containers_environments_get(...).doit().await
let r = hub.accounts().containers_environments_list(...).doit().await
let r = hub.accounts().containers_environments_reauthorize(...).doit().await
let r = hub.accounts().containers_environments_update(...).doit().await
let r = hub.accounts().containers_version_headers_latest(...).doit().await
let r = hub.accounts().containers_version_headers_list(...).doit().await
let r = hub.accounts().containers_versions_delete(...).doit().await
let r = hub.accounts().containers_versions_get(...).doit().await
let r = hub.accounts().containers_versions_live(...).doit().await
let r = hub.accounts().containers_versions_publish(...).doit().await
let r = hub.accounts().containers_versions_set_latest(...).doit().await
let r = hub.accounts().containers_versions_undelete(...).doit().await
let r = hub.accounts().containers_versions_update(...).doit().await
let r = hub.accounts().containers_workspaces_built_in_variables_create(...).doit().await
let r = hub.accounts().containers_workspaces_built_in_variables_delete(...).doit().await
let r = hub.accounts().containers_workspaces_built_in_variables_list(...).doit().await
let r = hub.accounts().containers_workspaces_built_in_variables_revert(...).doit().await
let r = hub.accounts().containers_workspaces_clients_create(...).doit().await
let r = hub.accounts().containers_workspaces_clients_delete(...).doit().await
let r = hub.accounts().containers_workspaces_clients_get(...).doit().await
let r = hub.accounts().containers_workspaces_clients_list(...).doit().await
let r = hub.accounts().containers_workspaces_clients_revert(...).doit().await
let r = hub.accounts().containers_workspaces_clients_update(...).doit().await
let r = hub.accounts().containers_workspaces_folders_create(...).doit().await
let r = hub.accounts().containers_workspaces_folders_delete(...).doit().await
let r = hub.accounts().containers_workspaces_folders_entities(...).doit().await
let r = hub.accounts().containers_workspaces_folders_get(...).doit().await
let r = hub.accounts().containers_workspaces_folders_list(...).doit().await
let r = hub.accounts().containers_workspaces_folders_move_entities_to_folder(...).doit().await
let r = hub.accounts().containers_workspaces_folders_revert(...).doit().await
let r = hub.accounts().containers_workspaces_folders_update(...).doit().await
let r = hub.accounts().containers_workspaces_gtag_config_create(...).doit().await
let r = hub.accounts().containers_workspaces_gtag_config_delete(...).doit().await
let r = hub.accounts().containers_workspaces_gtag_config_get(...).doit().await
let r = hub.accounts().containers_workspaces_gtag_config_list(...).doit().await
let r = hub.accounts().containers_workspaces_gtag_config_update(...).doit().await
let r = hub.accounts().containers_workspaces_tags_create(...).doit().await
let r = hub.accounts().containers_workspaces_tags_delete(...).doit().await
let r = hub.accounts().containers_workspaces_tags_get(...).doit().await
let r = hub.accounts().containers_workspaces_tags_list(...).doit().await
let r = hub.accounts().containers_workspaces_tags_revert(...).doit().await
let r = hub.accounts().containers_workspaces_tags_update(...).doit().await
let r = hub.accounts().containers_workspaces_templates_create(...).doit().await
let r = hub.accounts().containers_workspaces_templates_delete(...).doit().await
let r = hub.accounts().containers_workspaces_templates_get(...).doit().await
let r = hub.accounts().containers_workspaces_templates_list(...).doit().await
let r = hub.accounts().containers_workspaces_templates_revert(...).doit().await
let r = hub.accounts().containers_workspaces_templates_update(...).doit().await
let r = hub.accounts().containers_workspaces_transformations_create(...).doit().await
let r = hub.accounts().containers_workspaces_transformations_delete(...).doit().await
let r = hub.accounts().containers_workspaces_transformations_get(...).doit().await
let r = hub.accounts().containers_workspaces_transformations_list(...).doit().await
let r = hub.accounts().containers_workspaces_transformations_revert(...).doit().await
let r = hub.accounts().containers_workspaces_transformations_update(...).doit().await
let r = hub.accounts().containers_workspaces_triggers_create(...).doit().await
let r = hub.accounts().containers_workspaces_triggers_delete(...).doit().await
let r = hub.accounts().containers_workspaces_triggers_get(...).doit().await
let r = hub.accounts().containers_workspaces_triggers_list(...).doit().await
let r = hub.accounts().containers_workspaces_triggers_revert(...).doit().await
let r = hub.accounts().containers_workspaces_triggers_update(...).doit().await
let r = hub.accounts().containers_workspaces_variables_create(...).doit().await
let r = hub.accounts().containers_workspaces_variables_delete(...).doit().await
let r = hub.accounts().containers_workspaces_variables_get(...).doit().await
let r = hub.accounts().containers_workspaces_variables_list(...).doit().await
let r = hub.accounts().containers_workspaces_variables_revert(...).doit().await
let r = hub.accounts().containers_workspaces_variables_update(...).doit().await
let r = hub.accounts().containers_workspaces_zones_create(...).doit().await
let r = hub.accounts().containers_workspaces_zones_delete(...).doit().await
let r = hub.accounts().containers_workspaces_zones_get(...).doit().await
let r = hub.accounts().containers_workspaces_zones_list(...).doit().await
let r = hub.accounts().containers_workspaces_zones_revert(...).doit().await
let r = hub.accounts().containers_workspaces_zones_update(...).doit().await
let r = hub.accounts().containers_workspaces_create(...).doit().await
let r = hub.accounts().containers_workspaces_create_version(...).doit().await
let r = hub.accounts().containers_workspaces_delete(...).doit().await
let r = hub.accounts().containers_workspaces_get(...).doit().await
let r = hub.accounts().containers_workspaces_get_status(...).doit().await
let r = hub.accounts().containers_workspaces_list(...).doit().await
let r = hub.accounts().containers_workspaces_quick_preview(...).doit().await
let r = hub.accounts().containers_workspaces_resolve_conflict(...).doit().await
let r = hub.accounts().containers_workspaces_sync(...).doit().await
let r = hub.accounts().containers_workspaces_update(...).doit().await
let r = hub.accounts().containers_combine(...).doit().await
let r = hub.accounts().containers_create(...).doit().await
let r = hub.accounts().containers_delete(...).doit().await
let r = hub.accounts().containers_get(...).doit().await
let r = hub.accounts().containers_list(...).doit().await
let r = hub.accounts().containers_lookup(...).doit().await
let r = hub.accounts().containers_move_tag_id(...).doit().await
let r = hub.accounts().containers_snippet(...).doit().await
let r = hub.accounts().containers_update(...).doit().await
let r = hub.accounts().user_permissions_create(...).doit().await
let r = hub.accounts().user_permissions_delete(...).doit().await
let r = hub.accounts().user_permissions_get(...).doit().await
let r = hub.accounts().user_permissions_list(...).doit().await
let r = hub.accounts().user_permissions_update(...).doit().await
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
google-tagmanager2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_tagmanager2 as tagmanager2;
use tagmanager2::{Result, Error};
use std::default::Default;
use tagmanager2::{TagManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().containers_move_tag_id("path")
             .tag_name("magna")
             .tag_id("no")
             .copy_users(true)
             .copy_terms_of_service(false)
             .copy_settings(true)
             .allow_user_permission_feature_update(true)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::RequestValue) and 
[decodable](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-tagmanager2/5.0.5+20240619/google_tagmanager2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **tagmanager2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

