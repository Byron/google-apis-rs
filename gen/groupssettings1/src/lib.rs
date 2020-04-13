// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *groupssettings* crate version *1.0.13+20190725*, where *20190725* is the exact revision of the *groupssettings:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.
//! 
//! Everything else about the *groupssettings* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/google-apps/groups-settings/get_started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/groupssettings1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Groupssettings.html) ... 
//! 
//! * groups
//!  * [*get*](struct.GroupGetCall.html), [*patch*](struct.GroupPatchCall.html) and [*update*](struct.GroupUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Groupssettings.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.groups().update(...).doit()
//! let r = hub.groups().patch(...).doit()
//! let r = hub.groups().get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-groupssettings1 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_groupssettings1 as groupssettings1;
//! use groupssettings1::Groups;
//! use groupssettings1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use groupssettings1::Groupssettings;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Groupssettings::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Groups::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.groups().update(req, "groupUniqueId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


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
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_groupssettings1 as groupssettings1;
/// use groupssettings1::Groups;
/// use groupssettings1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use groupssettings1::Groupssettings;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Groupssettings::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Groups::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().update(req, "groupUniqueId")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct Groupssettings<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Groupssettings<C, A> {}

impl<'a, C, A> Groupssettings<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Groupssettings<C, A> {
        Groupssettings {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.13".to_string(),
            _base_url: "https://www.googleapis.com/groups/v1/groups/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn groups(&'a self) -> GroupMethods<'a, C, A> {
        GroupMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.13`.
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
/// * [update groups](struct.GroupUpdateCall.html) (request|response)
/// * [patch groups](struct.GroupPatchCall.html) (request|response)
/// * [get groups](struct.GroupGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Groups {
    /// Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanBanUsers")]
    pub who_can_ban_users: Option<String>,
    /// Specifies who can moderate metadata. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanAssistContent")]
    pub who_can_assist_content: Option<String>,
    /// Identifies whether members external to your organization can join the group. Possible values are:  
    /// - true: G Suite users external to your organization can become members of this group. 
    /// - false: Users not belonging to the organization are not allowed to become members of this group.
    #[serde(rename="allowExternalMembers")]
    pub allow_external_members: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanEnterFreeFormTags")]
    pub who_can_enter_free_form_tags: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanApproveMessages")]
    pub who_can_approve_messages: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkDuplicate")]
    pub who_can_mark_duplicate: Option<String>,
    /// Permission to join group. Possible values are:  
    /// - ANYONE_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. 
    /// - ALL_IN_DOMAIN_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. 
    /// - INVITED_CAN_JOIN: Candidates for membership can be invited to join.  
    /// - CAN_REQUEST_TO_JOIN: Non members can request an invitation to join.
    #[serde(rename="whoCanJoin")]
    pub who_can_join: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModifyTagsAndCategories")]
    pub who_can_modify_tags_and_categories: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkNoResponseNeeded")]
    pub who_can_mark_no_response_needed: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanUnmarkFavoriteReplyOnAnyTopic")]
    pub who_can_unmark_favorite_reply_on_any_topic: Option<String>,
    /// Specifies who can moderate content. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModerateContent")]
    pub who_can_moderate_content: Option<String>,
    /// The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags.
    #[serde(rename="primaryLanguage")]
    pub primary_language: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkFavoriteReplyOnOwnTopic")]
    pub who_can_mark_favorite_reply_on_own_topic: Option<String>,
    /// Permissions to view membership. Possible values are:  
    /// - ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.
    /// If a group already has external members, those members can still send email to this group.
    ///  
    /// - ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. 
    /// - ALL_MANAGERS_CAN_VIEW: The group managers can view group members list.
    #[serde(rename="whoCanViewMembership")]
    pub who_can_view_membership: Option<String>,
    /// Indicates if favorite replies should be displayed above other replies.  
    /// - true: Favorite replies will be displayed above other replies. 
    /// - false: Favorite replies will not be displayed above other replies.
    #[serde(rename="favoriteRepliesOnTop")]
    pub favorite_replies_on_top: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMarkFavoriteReplyOnAnyTopic")]
    pub who_can_mark_favorite_reply_on_any_topic: Option<String>,
    /// Whether to include custom footer. Possible values are:  
    /// - true 
    /// - false
    #[serde(rename="includeCustomFooter")]
    pub include_custom_footer: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMoveTopicsOut")]
    pub who_can_move_topics_out: Option<String>,
    /// When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true.
    #[serde(rename="defaultMessageDenyNotificationText")]
    pub default_message_deny_notification_text: Option<String>,
    /// Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:  
    /// - true: Group is included in the Global Address List. 
    /// - false: Group is not included in the Global Address List.
    #[serde(rename="includeInGlobalAddressList")]
    pub include_in_global_address_list: Option<String>,
    /// Allows the group to be archived only. Possible values are:  
    /// - true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.  
    /// - If true, the whoCanPostMessage property is set to NONE_CAN_POST.  
    /// - If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
    /// - false: The group is active and can receive messages.  
    /// - When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error.
    #[serde(rename="archiveOnly")]
    pub archive_only: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanDeleteTopics")]
    pub who_can_delete_topics: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanDeleteAnyPost")]
    pub who_can_delete_any_post: Option<String>,
    /// Allows the Group contents to be archived. Possible values are:  
    /// - true: Archive messages sent to the group. 
    /// - false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive.
    #[serde(rename="isArchived")]
    pub is_archived: Option<String>,
    /// Enables members to post messages as the group. Possible values are:  
    /// - true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. 
    /// - false: Members can not post in behalf of the group's email address.
    #[serde(rename="membersCanPostAsTheGroup")]
    pub members_can_post_as_the_group: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMakeTopicsSticky")]
    pub who_can_make_topics_sticky: Option<String>,
    /// Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:  
    /// - true 
    /// - false
    #[serde(rename="customRolesEnabledForSettingsToBeMerged")]
    pub custom_roles_enabled_for_settings_to_be_merged: Option<String>,
    /// The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.
    /// When you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address.
    pub email: Option<String>,
    /// Specifies the set of users for whom this group is discoverable. Possible values are:  
    /// - ANYONE_CAN_DISCOVER 
    /// - ALL_IN_DOMAIN_CAN_DISCOVER 
    /// - ALL_MEMBERS_CAN_DISCOVER
    #[serde(rename="whoCanDiscoverGroup")]
    pub who_can_discover_group: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModifyMembers")]
    pub who_can_modify_members: Option<String>,
    /// Moderation level of incoming messages. Possible values are:  
    /// - MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
    /// - MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
    /// - MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
    /// - MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.
    /// When memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group.
    #[serde(rename="messageModerationLevel")]
    pub message_moderation_level: Option<String>,
    /// Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters.
    pub description: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanUnassignTopic")]
    pub who_can_unassign_topic: Option<String>,
    /// Specifies who should the default reply go to. Possible values are:  
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
    /// An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.  
    /// - When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. 
    /// - If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned.
    #[serde(rename="customReplyTo")]
    pub custom_reply_to: Option<String>,
    /// Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:  
    /// - true: When a message is rejected, send the deny message notification to the message author.
    /// The defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.
    ///  
    /// - false: When a message is rejected, no notification is sent.
    #[serde(rename="sendMessageDenyNotification")]
    pub send_message_deny_notification: Option<String>,
    /// Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:  
    /// - true 
    /// - false
    #[serde(rename="enableCollaborativeInbox")]
    pub enable_collaborative_inbox: Option<String>,
    /// Permission to contact owner of the group via web UI. Possible values are:  
    /// - ALL_IN_DOMAIN_CAN_CONTACT 
    /// - ALL_MANAGERS_CAN_CONTACT 
    /// - ALL_MEMBERS_CAN_CONTACT 
    /// - ANYONE_CAN_CONTACT
    #[serde(rename="whoCanContactOwner")]
    pub who_can_contact_owner: Option<String>,
    /// Deprecated. The default message display font always has a value of "DEFAULT_FONT".
    #[serde(rename="messageDisplayFont")]
    pub message_display_font: Option<String>,
    /// Permission to leave the group. Possible values are:  
    /// - ALL_MANAGERS_CAN_LEAVE 
    /// - ALL_MEMBERS_CAN_LEAVE 
    /// - NONE_CAN_LEAVE
    #[serde(rename="whoCanLeaveGroup")]
    pub who_can_leave_group: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:  
    /// - ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. 
    /// - ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. 
    /// - ALL_OWNERS_CAN_ADD: Only owners can directly add new members. 
    /// - NONE_CAN_ADD: No one can directly add new members.
    #[serde(rename="whoCanAdd")]
    pub who_can_add: Option<String>,
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
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanMoveTopicsIn")]
    pub who_can_move_topics_in: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanTakeTopics")]
    pub who_can_take_topics: Option<String>,
    /// Name of the group, which has a maximum size of 75 characters.
    pub name: Option<String>,
    /// The type of the resource. It is always groupsSettings#groups.
    pub kind: Option<String>,
    /// Deprecated. The maximum size of a message is 25Mb.
    #[serde(rename="maxMessageBytes")]
    pub max_message_bytes: Option<i32>,
    /// Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:  
    /// - ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. 
    /// - ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. 
    /// - ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. 
    /// - NONE_CAN_INVITE: No one can invite a new member candidate.
    #[serde(rename="whoCanInvite")]
    pub who_can_invite: Option<String>,
    /// Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
    /// - ALL_MEMBERS_CAN_APPROVE 
    /// - ALL_MANAGERS_CAN_APPROVE 
    /// - ALL_OWNERS_CAN_APPROVE 
    /// - NONE_CAN_APPROVE
    #[serde(rename="whoCanApproveMembers")]
    pub who_can_approve_members: Option<String>,
    /// Specifies moderation levels for messages detected as spam. Possible values are:  
    /// - ALLOW: Post the message to the group. 
    /// - MODERATE: Send the message to the moderation queue. This is the default. 
    /// - SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. 
    /// - REJECT: Immediately reject the message.
    #[serde(rename="spamModerationLevel")]
    pub spam_moderation_level: Option<String>,
    /// Allows posting from web. Possible values are:  
    /// - true: Allows any member to post to the group forum. 
    /// - false: Members only use Gmail to communicate with the group.
    #[serde(rename="allowWebPosting")]
    pub allow_web_posting: Option<String>,
    /// Specifies who can manage members. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanModerateMembers")]
    pub who_can_moderate_members: Option<String>,
    /// Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always "NONE".
    #[serde(rename="whoCanAddReferences")]
    pub who_can_add_references: Option<String>,
    /// Permissions to view group messages. Possible values are:  
    /// - ANYONE_CAN_VIEW: Any Internet user can view the group's messages.  
    /// - ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. 
    /// - ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. 
    /// - ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages.
    #[serde(rename="whoCanViewGroup")]
    pub who_can_view_group: Option<String>,
    /// Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:  
    /// - true: All groups in the account are listed in the Groups directory. 
    /// - false: All groups in the account are not listed in the directory.
    #[serde(rename="showInGroupDirectory")]
    pub show_in_group_directory: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanPostAnnouncements")]
    pub who_can_post_announcements: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanLockTopics")]
    pub who_can_lock_topics: Option<String>,
    /// Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - MANAGERS_ONLY 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanAssignTopics")]
    pub who_can_assign_topics: Option<String>,
    /// Set the content of custom footer text. The maximum number of characters is 1,000.
    #[serde(rename="customFooterText")]
    pub custom_footer_text: Option<String>,
    /// Deprecated. Allows Google to contact administrator of the group.  
    /// - true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. 
    /// - false: Google can not contact managers of this group.
    #[serde(rename="allowGoogleCommunication")]
    pub allow_google_communication: Option<String>,
    /// Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:  
    /// - ALL_MEMBERS 
    /// - OWNERS_AND_MANAGERS 
    /// - OWNERS_ONLY 
    /// - NONE
    #[serde(rename="whoCanHideAbuse")]
    pub who_can_hide_abuse: Option<String>,
}

impl RequestValue for Groups {}
impl ResponseResult for Groups {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *group* resources.
/// It is not used directly, but through the `Groupssettings` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_groupssettings1 as groupssettings1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use groupssettings1::Groupssettings;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Groupssettings::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.groups();
/// # }
/// ```
pub struct GroupMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Groupssettings<C, A>,
}

impl<'a, C, A> MethodsBuilder for GroupMethods<'a, C, A> {}

impl<'a, C, A> GroupMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `groupUniqueId` - The group's email address.
    pub fn update(&self, request: Groups, group_unique_id: &str) -> GroupUpdateCall<'a, C, A> {
        GroupUpdateCall {
            hub: self.hub,
            _request: request,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
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
    pub fn patch(&self, request: Groups, group_unique_id: &str) -> GroupPatchCall<'a, C, A> {
        GroupPatchCall {
            hub: self.hub,
            _request: request,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one resource by id.
    /// 
    /// # Arguments
    ///
    /// * `groupUniqueId` - The group's email address.
    pub fn get(&self, group_unique_id: &str) -> GroupGetCall<'a, C, A> {
        GroupGetCall {
            hub: self.hub,
            _group_unique_id: group_unique_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Updates an existing resource.
///
/// A builder for the *update* method supported by a *group* resource.
/// It is not used directly, but through a `GroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_groupssettings1 as groupssettings1;
/// use groupssettings1::Groups;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use groupssettings1::Groupssettings;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Groupssettings::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Groups::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().update(req, "groupUniqueId")
///              .doit();
/// # }
/// ```
pub struct GroupUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Groupssettings<C, A>,
    _request: Groups,
    _group_unique_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for GroupUpdateCall<'a, C, A> {}

impl<'a, C, A> GroupUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Groups)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "groupsSettings.groups.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("groupUniqueId", self._group_unique_id.to_string()));
        for &field in ["alt", "groupUniqueId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{groupUniqueId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppGroupSetting.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{groupUniqueId}", "groupUniqueId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["groupUniqueId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: Groups) -> GroupUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The group's email address.
    ///
    /// Sets the *group unique id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn group_unique_id(mut self, new_value: &str) -> GroupUpdateCall<'a, C, A> {
        self._group_unique_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> GroupUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> GroupUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppGroupSetting`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> GroupUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an existing resource. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *group* resource.
/// It is not used directly, but through a `GroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_groupssettings1 as groupssettings1;
/// use groupssettings1::Groups;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use groupssettings1::Groupssettings;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Groupssettings::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Groups::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().patch(req, "groupUniqueId")
///              .doit();
/// # }
/// ```
pub struct GroupPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Groupssettings<C, A>,
    _request: Groups,
    _group_unique_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for GroupPatchCall<'a, C, A> {}

impl<'a, C, A> GroupPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Groups)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "groupsSettings.groups.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("groupUniqueId", self._group_unique_id.to_string()));
        for &field in ["alt", "groupUniqueId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{groupUniqueId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppGroupSetting.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{groupUniqueId}", "groupUniqueId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["groupUniqueId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: Groups) -> GroupPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The group's email address.
    ///
    /// Sets the *group unique id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn group_unique_id(mut self, new_value: &str) -> GroupPatchCall<'a, C, A> {
        self._group_unique_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> GroupPatchCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> GroupPatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppGroupSetting`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> GroupPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets one resource by id.
///
/// A builder for the *get* method supported by a *group* resource.
/// It is not used directly, but through a `GroupMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_groupssettings1 as groupssettings1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use groupssettings1::Groupssettings;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Groupssettings::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.groups().get("groupUniqueId")
///              .doit();
/// # }
/// ```
pub struct GroupGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Groupssettings<C, A>,
    _group_unique_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for GroupGetCall<'a, C, A> {}

impl<'a, C, A> GroupGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Groups)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "groupsSettings.groups.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("groupUniqueId", self._group_unique_id.to_string()));
        for &field in ["alt", "groupUniqueId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{groupUniqueId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppGroupSetting.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{groupUniqueId}", "groupUniqueId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["groupUniqueId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn group_unique_id(mut self, new_value: &str) -> GroupGetCall<'a, C, A> {
        self._group_unique_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> GroupGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> GroupGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppGroupSetting`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> GroupGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


