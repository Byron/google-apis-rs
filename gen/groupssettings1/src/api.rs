use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage the settings of a G Suite group
    AppGroupSetting,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppGroupSetting => "https://www.googleapis.com/auth/apps.groups.settings",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppGroupSetting
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Groupssettings related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_groupssettings1 as groupssettings1;
/// use groupssettings1::api::Groups;
/// use groupssettings1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use groupssettings1::{Groupssettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Groupssettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Groups::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().patch(req, "groupUniqueId")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Groupssettings<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Groupssettings<S> {}

impl<'a, S> Groupssettings<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Groupssettings<S> {
        Groupssettings {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2-beta-1".to_string(),
            _base_url: "https://www.googleapis.com/groups/v1/groups/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn groups(&'a self) -> GroupMethods<'a, S> {
        GroupMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2-beta-1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/groups/v1/groups/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// JSON template for Group resource
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get groups](GroupGetCall) (response)
/// * [patch groups](GroupPatchCall) (request|response)
/// * [update groups](GroupUpdateCall) (request|response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Groups {
    /// Identifies whether members external to your organization can join the group. Possible values are:  
    /// - true: G Suite users external to your organization can become members of this group. 
    /// - false: Users not belonging to the organization are not allowed to become members of this group.
    #[serde(rename="allowExternalMembers")]
    
    pub allow_external_members: Option<String>,
    /// Deprecated. Allows Google to contact administrator of the group.  
    /// - true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. 
    /// - false: Google can not contact managers of this group.
    #[serde(rename="allowGoogleCommunication")]
    
    pub allow_google_communication: Option<String>,
    /// Allows posting from web. Possible values are:  
    /// - true: Allows any member to post to the group forum. 
    /// - false: Members only use Gmail to communicate with the group.
    #[serde(rename="allowWebPosting")]
    
    pub allow_web_posting: Option<String>,
    /// Allows the group to be archived only. Possible values are:  
    /// - true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.  
    /// - If true, the whoCanPostMessage property is set to NONE_CAN_POST.  
    /// - If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
    /// - false: The group is active and can receive messages.  
    /// - When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error.
    #[serde(rename="archiveOnly")]
    
    pub archive_only: Option<String>,
    /// Set the content of custom footer text. The maximum number of characters is 1,000.
    #[serde(rename="customFooterText")]
    
    pub custom_footer_text: Option<String>,
    /// An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.  
    /// - When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. 
    /// - If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned.
    #[serde(rename="customReplyTo")]
    
    pub custom_reply_to: Option<String>,
    /// Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:  
    /// - true 
    /// - false
    #[serde(rename="customRolesEnabledForSettingsToBeMerged")]
    
    pub custom_roles_enabled_for_settings_to_be_merged: Option<String>,
    /// When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true.
    #[serde(rename="defaultMessageDenyNotificationText")]
    
    pub default_message_deny_notification_text: Option<String>,
    /// Default sender for members who can post messages as the group. Possible values are: - `DEFAULT_SELF`: By default messages will be sent from the user - `GROUP`: By default messages will be sent from the group
    
    pub default_sender: Option<String>,
    /// Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters.
    
    pub description: Option<String>,
    /// The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.
    /// When you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address.
    
    pub email: Option<String>,
    /// Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:  
    /// - true 
    /// - false
    #[serde(rename="enableCollaborativeInbox")]
    
    pub enable_collaborative_inbox: Option<String>,
    /// Indicates if favorite replies should be displayed above other replies.  
    /// - true: Favorite replies will be displayed above other replies. 
    /// - false: Favorite replies will not be displayed above other replies.
    #[serde(rename="favoriteRepliesOnTop")]
    
    pub favorite_replies_on_top: Option<String>,
    /// Whether to include custom footer. Possible values are:  
    /// - true 
    /// - false
    #[serde(rename="includeCustomFooter")]
    
    pub include_custom_footer: Option<String>,
    /// Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:  
    /// - true: Group is included in the Global Address List. 
    /// - false: Group is not included in the Global Address List.
    #[serde(rename="includeInGlobalAddressList")]
    
    pub include_in_global_address_list: Option<String>,
    /// Allows the Group contents to be archived. Possible values are:  
    /// - true: Archive messages sent to the group. 
    /// - false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive.
    #[serde(rename="isArchived")]
    
    pub is_archived: Option<String>,
    /// The type of the resource. It is always groupsSettings#groups.
    
    pub kind: Option<String>,
    /// Deprecated. The maximum size of a message is 25Mb.
    #[serde(rename="maxMessageBytes")]
    
    pub max_message_bytes: Option<i32>,
    /// Enables members to post messages as the group. Possible values are:  
    /// - true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. 
    /// - false: Members can not post in behalf of the group's email address.
    #[serde(rename="membersCanPostAsTheGroup")]
    
    pub members_can_post_as_the_group: Option<String>,
    /// Deprecated. The default message display font always has a value of "DEFAULT_FONT".
    #[serde(rename="messageDisplayFont")]
    
    pub message_display_font: Option<String>,
    /// Moderation level of incoming messages. Possible values are:  
    /// - MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
    /// - MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
    /// - MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
    /// - MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.
    /// When memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group.
    #[serde(rename="messageModerationLevel")]
    
    pub message_moderation_level: Option<String>,
    /// Name of the group, which has a maximum size of 75 characters.
    
    pub name: Option<String>,
    /// The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags.
    #[serde(rename="primaryLanguage")]
    
    pub primary_language: Option<String>,
    /// Specifies who receives the default reply. Possible values are:  
    /// - REPLY_TO_CUSTOM: For replies to messages, use the group's custom email address.
    /// When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds the custom email address used when replying to a message. If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a value. Otherwise an error is returned.
    ///  
    /// - REPLY_TO_SENDER: The reply sent to author of message. 
    /// - REPLY_TO_LIST: This reply message is sent to the group. 
    /// - REPLY_TO_OWNER: The reply is sent to the owner(s) of the group. This does not include the group's managers. 
    /// - REPLY_TO_IGNORE: Group users individually decide where the message reply is sent. 
    /// - REPLY_TO_MANAGERS: This reply message is sent to the group's managers, which includes all managers and the group owner.
    #[serde(rename="replyTo")]
    
    pub reply_to: Option<String>,
    /// Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:  
    /// - true: When a message is rejected, send the deny message notification to the message author.
    /// The defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.
    ///  
    /// - false: When a message is rejected, no notification is sent.
    #[serde(rename="sendMessageDenyNotification")]
    
    pub send_message_deny_notification: Option<String>,
    /// Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:  
    /// - true: All groups in the account are listed in the Groups directory. 
    /// - false: All groups in the account are not listed in the directory.
    #[serde(rename="showInGroupDirectory")]
    
    pub show_in_group_directory: Option<String>,
    /// Specifies moderation levels for messages detected as spam. Possible values are:  
    /// - ALLOW: Post the message to the group. 
    /// - MODERATE: Send the message to the moderation queue. This is the default. 
    /// - SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. 
    /// - REJECT: Immediately reject the message.
    #[serde(rename="spamModerationLevel")]
    
    pub spam_moderation_level: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:  
    /// - ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. 
    /// - ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. 
    /// - ALL_OWNERS_CAN_ADD: Only owners can directly add new members. 
    /// - NONE_CAN_ADD: No one can directly add new members.
    #[serde(rename="whoCanAdd")]
    
    pub who_can_add: Option<String>,
    /// Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always "NONE".
    #[serde(rename="whoCanAddReferences")]
    
    pub who_can_add_references: Option<String>,
    /// Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
    /// - ALL_MEMBERS_CAN_APPROVE 
    /// - ALL_MANAGERS_CAN_APPROVE 
    /// - ALL_OWNERS_CAN_APPROVE 
    /// - NONE_CAN_APPROVE
    #[serde(rename="whoCanApproveMembers")]
    
    pub who_can_approve_members: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanApproveMessages")]
    
    pub who_can_approve_messages: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanAssignTopics")]
    
    pub who_can_assign_topics: Option<String>,
    /// Specifies who can moderate metadata. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanAssistContent")]
    
    pub who_can_assist_content: Option<String>,
    /// Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanBanUsers")]
    
    pub who_can_ban_users: Option<String>,
    /// Permission to contact owner of the group via web UI. Possible values are:  
    /// - ALL_IN_DOMAIN_CAN_CONTACT 
    /// - ALL_MANAGERS_CAN_CONTACT 
    /// - ALL_MEMBERS_CAN_CONTACT 
    /// - ANYONE_CAN_CONTACT 
    /// - ALL_OWNERS_CAN_CONTACT
    #[serde(rename="whoCanContactOwner")]
    
    pub who_can_contact_owner: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanDeleteAnyPost")]
    
    pub who_can_delete_any_post: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanDeleteTopics")]
    
    pub who_can_delete_topics: Option<String>,
    /// Specifies the set of users for whom this group is discoverable. Possible values are:  
    /// - ANYONE_CAN_DISCOVER 
    /// - ALL_IN_DOMAIN_CAN_DISCOVER 
    /// - ALL_MEMBERS_CAN_DISCOVER
    #[serde(rename="whoCanDiscoverGroup")]
    
    pub who_can_discover_group: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanEnterFreeFormTags")]
    
    pub who_can_enter_free_form_tags: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanHideAbuse")]
    
    pub who_can_hide_abuse: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:  
    /// - ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. 
    /// - ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. 
    /// - ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. 
    /// - NONE_CAN_INVITE: No one can invite a new member candidate.
    #[serde(rename="whoCanInvite")]
    
    pub who_can_invite: Option<String>,
    /// Permission to join group. Possible values are:  
    /// - ANYONE_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. 
    /// - ALL_IN_DOMAIN_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. 
    /// - INVITED_CAN_JOIN: Candidates for membership can be invited to join.  
    /// - CAN_REQUEST_TO_JOIN: Non members can request an invitation to join.
    #[serde(rename="whoCanJoin")]
    
    pub who_can_join: Option<String>,
    /// Permission to leave the group. Possible values are:  
    /// - ALL_MANAGERS_CAN_LEAVE 
    /// - ALL_MEMBERS_CAN_LEAVE 
    /// - NONE_CAN_LEAVE
    #[serde(rename="whoCanLeaveGroup")]
    
    pub who_can_leave_group: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanLockTopics")]
    
    pub who_can_lock_topics: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMakeTopicsSticky")]
    
    pub who_can_make_topics_sticky: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkDuplicate")]
    
    pub who_can_mark_duplicate: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkFavoriteReplyOnAnyTopic")]
    
    pub who_can_mark_favorite_reply_on_any_topic: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkFavoriteReplyOnOwnTopic")]
    
    pub who_can_mark_favorite_reply_on_own_topic: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkNoResponseNeeded")]
    
    pub who_can_mark_no_response_needed: Option<String>,
    /// Specifies who can moderate content. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModerateContent")]
    
    pub who_can_moderate_content: Option<String>,
    /// Specifies who can manage members. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModerateMembers")]
    
    pub who_can_moderate_members: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModifyMembers")]
    
    pub who_can_modify_members: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModifyTagsAndCategories")]
    
    pub who_can_modify_tags_and_categories: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMoveTopicsIn")]
    
    pub who_can_move_topics_in: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMoveTopicsOut")]
    
    pub who_can_move_topics_out: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanPostAnnouncements")]
    
    pub who_can_post_announcements: Option<String>,
    /// Permissions to post messages. Possible values are:  
    /// - NONE_CAN_POST: The group is disabled and archived. No one can post a message to this group.  
    /// - When archiveOnly is false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. 
    /// - If archiveOnly is reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
    /// - ALL_MANAGERS_CAN_POST: Managers, including group owners, can post messages. 
    /// - ALL_MEMBERS_CAN_POST: Any group member can post a message. 
    /// - ALL_OWNERS_CAN_POST: Only group owners can post a message. 
    /// - ALL_IN_DOMAIN_CAN_POST: Anyone in the account can post a message.  
    /// - ANYONE_CAN_POST: Any Internet user who outside your account can access your Google Groups service and post a message. Note: When whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.
    #[serde(rename="whoCanPostMessage")]
    
    pub who_can_post_message: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanTakeTopics")]
    
    pub who_can_take_topics: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanUnassignTopic")]
    
    pub who_can_unassign_topic: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanUnmarkFavoriteReplyOnAnyTopic")]
    
    pub who_can_unmark_favorite_reply_on_any_topic: Option<String>,
    /// Permissions to view group messages. Possible values are:  
    /// - ANYONE_CAN_VIEW: Any Internet user can view the group's messages.  
    /// - ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. 
    /// - ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. 
    /// - ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages.
    #[serde(rename="whoCanViewGroup")]
    
    pub who_can_view_group: Option<String>,
    /// Permissions to view membership. Possible values are:  
    /// - ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.
    /// If a group already has external members, those members can still send email to this group.
    ///  
    /// - ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. 
    /// - ALL_MANAGERS_CAN_VIEW: The group managers can view group members list.
    #[serde(rename="whoCanViewMembership")]
    
    pub who_can_view_membership: Option<String>,
}

impl client::RequestValue for Groups {}
impl client::ResponseResult for Groups {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *group* resources.
/// It is not used directly, but through the [`Groupssettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_groupssettings1 as groupssettings1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use groupssettings1::{Groupssettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Groupssettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.groups();
/// # }
/// ```
pub struct GroupMethods<'a, S>
    where S: 'a {

    hub: &'a Groupssettings<S>,
}

impl<'a, S> client::MethodsBuilder for GroupMethods<'a, S> {}

impl<'a, S> GroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one resource by id.
    /// 
    /// # Arguments
    ///
    /// * `groupUniqueId` - The group's email address.
    pub fn get(&self, group_unique_id: &str) -> GroupGetCall<'a, S> {
        GroupGetCall {
            hub: self.hub,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `groupUniqueId` - The group's email address.
    pub fn patch(&self, request: Groups, group_unique_id: &str) -> GroupPatchCall<'a, S> {
        GroupPatchCall {
            hub: self.hub,
            _request: request,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `groupUniqueId` - The group's email address.
    pub fn update(&self, request: Groups, group_unique_id: &str) -> GroupUpdateCall<'a, S> {
        GroupUpdateCall {
            hub: self.hub,
            _request: request,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets one resource by id.
///
/// A builder for the *get* method supported by a *group* resource.
/// It is not used directly, but through a [`GroupMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_groupssettings1 as groupssettings1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use groupssettings1::{Groupssettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Groupssettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().get("groupUniqueId")
///              .doit().await;
/// # }
/// ```
pub struct GroupGetCall<'a, S>
    where S: 'a {

    hub: &'a Groupssettings<S>,
    _group_unique_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for GroupGetCall<'a, S> {}

impl<'a, S> GroupGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Groups)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "groupsSettings.groups.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "groupUniqueId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("groupUniqueId", self._group_unique_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{groupUniqueId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::AppGroupSetting.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{groupUniqueId}", "groupUniqueId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["groupUniqueId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The group's email address.
    ///
    /// Sets the *group unique id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn group_unique_id(mut self, new_value: &str) -> GroupGetCall<'a, S> {
        self._group_unique_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> GroupGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> GroupGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AppGroupSetting`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> GroupGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> GroupGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> GroupGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates an existing resource. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *group* resource.
/// It is not used directly, but through a [`GroupMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_groupssettings1 as groupssettings1;
/// use groupssettings1::api::Groups;
/// # async fn dox() {
/// # use std::default::Default;
/// # use groupssettings1::{Groupssettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Groupssettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Groups::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().patch(req, "groupUniqueId")
///              .doit().await;
/// # }
/// ```
pub struct GroupPatchCall<'a, S>
    where S: 'a {

    hub: &'a Groupssettings<S>,
    _request: Groups,
    _group_unique_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for GroupPatchCall<'a, S> {}

impl<'a, S> GroupPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Groups)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "groupsSettings.groups.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "groupUniqueId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("groupUniqueId", self._group_unique_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{groupUniqueId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::AppGroupSetting.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{groupUniqueId}", "groupUniqueId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["groupUniqueId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Groups) -> GroupPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The group's email address.
    ///
    /// Sets the *group unique id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn group_unique_id(mut self, new_value: &str) -> GroupPatchCall<'a, S> {
        self._group_unique_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> GroupPatchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> GroupPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AppGroupSetting`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> GroupPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> GroupPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> GroupPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates an existing resource.
///
/// A builder for the *update* method supported by a *group* resource.
/// It is not used directly, but through a [`GroupMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_groupssettings1 as groupssettings1;
/// use groupssettings1::api::Groups;
/// # async fn dox() {
/// # use std::default::Default;
/// # use groupssettings1::{Groupssettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Groupssettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Groups::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().update(req, "groupUniqueId")
///              .doit().await;
/// # }
/// ```
pub struct GroupUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Groupssettings<S>,
    _request: Groups,
    _group_unique_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for GroupUpdateCall<'a, S> {}

impl<'a, S> GroupUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Groups)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "groupsSettings.groups.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "groupUniqueId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("groupUniqueId", self._group_unique_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{groupUniqueId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::AppGroupSetting.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{groupUniqueId}", "groupUniqueId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["groupUniqueId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Groups) -> GroupUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The group's email address.
    ///
    /// Sets the *group unique id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn group_unique_id(mut self, new_value: &str) -> GroupUpdateCall<'a, S> {
        self._group_unique_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> GroupUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> GroupUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AppGroupSetting`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> GroupUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> GroupUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> GroupUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


