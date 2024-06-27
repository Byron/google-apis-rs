// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_groupssettings1::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::Groupssettings<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _groups_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.groups().get(opt.value_of("group-unique-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _groups_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "allow-external-members" => Some(("allowExternalMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-google-communication" => Some(("allowGoogleCommunication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-web-posting" => Some(("allowWebPosting", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "archive-only" => Some(("archiveOnly", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-footer-text" => Some(("customFooterText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-reply-to" => Some(("customReplyTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-roles-enabled-for-settings-to-be-merged" => Some(("customRolesEnabledForSettingsToBeMerged", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-message-deny-notification-text" => Some(("defaultMessageDenyNotificationText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-sender" => Some(("default_sender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-collaborative-inbox" => Some(("enableCollaborativeInbox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "favorite-replies-on-top" => Some(("favoriteRepliesOnTop", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-custom-footer" => Some(("includeCustomFooter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-in-global-address-list" => Some(("includeInGlobalAddressList", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-archived" => Some(("isArchived", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-message-bytes" => Some(("maxMessageBytes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "members-can-post-as-the-group" => Some(("membersCanPostAsTheGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "message-display-font" => Some(("messageDisplayFont", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "message-moderation-level" => Some(("messageModerationLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-language" => Some(("primaryLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-to" => Some(("replyTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "send-message-deny-notification" => Some(("sendMessageDenyNotification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "show-in-group-directory" => Some(("showInGroupDirectory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spam-moderation-level" => Some(("spamModerationLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-add" => Some(("whoCanAdd", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-add-references" => Some(("whoCanAddReferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-approve-members" => Some(("whoCanApproveMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-approve-messages" => Some(("whoCanApproveMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-assign-topics" => Some(("whoCanAssignTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-assist-content" => Some(("whoCanAssistContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-ban-users" => Some(("whoCanBanUsers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-contact-owner" => Some(("whoCanContactOwner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-delete-any-post" => Some(("whoCanDeleteAnyPost", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-delete-topics" => Some(("whoCanDeleteTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-discover-group" => Some(("whoCanDiscoverGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-enter-free-form-tags" => Some(("whoCanEnterFreeFormTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-hide-abuse" => Some(("whoCanHideAbuse", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-invite" => Some(("whoCanInvite", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-join" => Some(("whoCanJoin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-leave-group" => Some(("whoCanLeaveGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-lock-topics" => Some(("whoCanLockTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-make-topics-sticky" => Some(("whoCanMakeTopicsSticky", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-duplicate" => Some(("whoCanMarkDuplicate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-favorite-reply-on-any-topic" => Some(("whoCanMarkFavoriteReplyOnAnyTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-favorite-reply-on-own-topic" => Some(("whoCanMarkFavoriteReplyOnOwnTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-no-response-needed" => Some(("whoCanMarkNoResponseNeeded", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-moderate-content" => Some(("whoCanModerateContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-moderate-members" => Some(("whoCanModerateMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-modify-members" => Some(("whoCanModifyMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-modify-tags-and-categories" => Some(("whoCanModifyTagsAndCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-move-topics-in" => Some(("whoCanMoveTopicsIn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-move-topics-out" => Some(("whoCanMoveTopicsOut", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-post-announcements" => Some(("whoCanPostAnnouncements", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-post-message" => Some(("whoCanPostMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-take-topics" => Some(("whoCanTakeTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-unassign-topic" => Some(("whoCanUnassignTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-unmark-favorite-reply-on-any-topic" => Some(("whoCanUnmarkFavoriteReplyOnAnyTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-view-group" => Some(("whoCanViewGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-view-membership" => Some(("whoCanViewMembership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-external-members", "allow-google-communication", "allow-web-posting", "archive-only", "custom-footer-text", "custom-reply-to", "custom-roles-enabled-for-settings-to-be-merged", "default-message-deny-notification-text", "default-sender", "description", "email", "enable-collaborative-inbox", "favorite-replies-on-top", "include-custom-footer", "include-in-global-address-list", "is-archived", "kind", "max-message-bytes", "members-can-post-as-the-group", "message-display-font", "message-moderation-level", "name", "primary-language", "reply-to", "send-message-deny-notification", "show-in-group-directory", "spam-moderation-level", "who-can-add", "who-can-add-references", "who-can-approve-members", "who-can-approve-messages", "who-can-assign-topics", "who-can-assist-content", "who-can-ban-users", "who-can-contact-owner", "who-can-delete-any-post", "who-can-delete-topics", "who-can-discover-group", "who-can-enter-free-form-tags", "who-can-hide-abuse", "who-can-invite", "who-can-join", "who-can-leave-group", "who-can-lock-topics", "who-can-make-topics-sticky", "who-can-mark-duplicate", "who-can-mark-favorite-reply-on-any-topic", "who-can-mark-favorite-reply-on-own-topic", "who-can-mark-no-response-needed", "who-can-moderate-content", "who-can-moderate-members", "who-can-modify-members", "who-can-modify-tags-and-categories", "who-can-move-topics-in", "who-can-move-topics-out", "who-can-post-announcements", "who-can-post-message", "who-can-take-topics", "who-can-unassign-topic", "who-can-unmark-favorite-reply-on-any-topic", "who-can-view-group", "who-can-view-membership"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Groups = json::value::from_value(object).unwrap();
        let mut call = self.hub.groups().patch(request, opt.value_of("group-unique-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _groups_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "allow-external-members" => Some(("allowExternalMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-google-communication" => Some(("allowGoogleCommunication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-web-posting" => Some(("allowWebPosting", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "archive-only" => Some(("archiveOnly", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-footer-text" => Some(("customFooterText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-reply-to" => Some(("customReplyTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-roles-enabled-for-settings-to-be-merged" => Some(("customRolesEnabledForSettingsToBeMerged", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-message-deny-notification-text" => Some(("defaultMessageDenyNotificationText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-sender" => Some(("default_sender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-collaborative-inbox" => Some(("enableCollaborativeInbox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "favorite-replies-on-top" => Some(("favoriteRepliesOnTop", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-custom-footer" => Some(("includeCustomFooter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-in-global-address-list" => Some(("includeInGlobalAddressList", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-archived" => Some(("isArchived", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-message-bytes" => Some(("maxMessageBytes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "members-can-post-as-the-group" => Some(("membersCanPostAsTheGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "message-display-font" => Some(("messageDisplayFont", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "message-moderation-level" => Some(("messageModerationLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-language" => Some(("primaryLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-to" => Some(("replyTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "send-message-deny-notification" => Some(("sendMessageDenyNotification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "show-in-group-directory" => Some(("showInGroupDirectory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spam-moderation-level" => Some(("spamModerationLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-add" => Some(("whoCanAdd", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-add-references" => Some(("whoCanAddReferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-approve-members" => Some(("whoCanApproveMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-approve-messages" => Some(("whoCanApproveMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-assign-topics" => Some(("whoCanAssignTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-assist-content" => Some(("whoCanAssistContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-ban-users" => Some(("whoCanBanUsers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-contact-owner" => Some(("whoCanContactOwner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-delete-any-post" => Some(("whoCanDeleteAnyPost", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-delete-topics" => Some(("whoCanDeleteTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-discover-group" => Some(("whoCanDiscoverGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-enter-free-form-tags" => Some(("whoCanEnterFreeFormTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-hide-abuse" => Some(("whoCanHideAbuse", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-invite" => Some(("whoCanInvite", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-join" => Some(("whoCanJoin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-leave-group" => Some(("whoCanLeaveGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-lock-topics" => Some(("whoCanLockTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-make-topics-sticky" => Some(("whoCanMakeTopicsSticky", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-duplicate" => Some(("whoCanMarkDuplicate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-favorite-reply-on-any-topic" => Some(("whoCanMarkFavoriteReplyOnAnyTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-favorite-reply-on-own-topic" => Some(("whoCanMarkFavoriteReplyOnOwnTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-mark-no-response-needed" => Some(("whoCanMarkNoResponseNeeded", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-moderate-content" => Some(("whoCanModerateContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-moderate-members" => Some(("whoCanModerateMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-modify-members" => Some(("whoCanModifyMembers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-modify-tags-and-categories" => Some(("whoCanModifyTagsAndCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-move-topics-in" => Some(("whoCanMoveTopicsIn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-move-topics-out" => Some(("whoCanMoveTopicsOut", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-post-announcements" => Some(("whoCanPostAnnouncements", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-post-message" => Some(("whoCanPostMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-take-topics" => Some(("whoCanTakeTopics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-unassign-topic" => Some(("whoCanUnassignTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-unmark-favorite-reply-on-any-topic" => Some(("whoCanUnmarkFavoriteReplyOnAnyTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-view-group" => Some(("whoCanViewGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "who-can-view-membership" => Some(("whoCanViewMembership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-external-members", "allow-google-communication", "allow-web-posting", "archive-only", "custom-footer-text", "custom-reply-to", "custom-roles-enabled-for-settings-to-be-merged", "default-message-deny-notification-text", "default-sender", "description", "email", "enable-collaborative-inbox", "favorite-replies-on-top", "include-custom-footer", "include-in-global-address-list", "is-archived", "kind", "max-message-bytes", "members-can-post-as-the-group", "message-display-font", "message-moderation-level", "name", "primary-language", "reply-to", "send-message-deny-notification", "show-in-group-directory", "spam-moderation-level", "who-can-add", "who-can-add-references", "who-can-approve-members", "who-can-approve-messages", "who-can-assign-topics", "who-can-assist-content", "who-can-ban-users", "who-can-contact-owner", "who-can-delete-any-post", "who-can-delete-topics", "who-can-discover-group", "who-can-enter-free-form-tags", "who-can-hide-abuse", "who-can-invite", "who-can-join", "who-can-leave-group", "who-can-lock-topics", "who-can-make-topics-sticky", "who-can-mark-duplicate", "who-can-mark-favorite-reply-on-any-topic", "who-can-mark-favorite-reply-on-own-topic", "who-can-mark-no-response-needed", "who-can-moderate-content", "who-can-moderate-members", "who-can-modify-members", "who-can-modify-tags-and-categories", "who-can-move-topics-in", "who-can-move-topics-out", "who-can-post-announcements", "who-can-post-message", "who-can-take-topics", "who-can-unassign-topic", "who-can-unmark-favorite-reply-on-any-topic", "who-can-view-group", "who-can-view-membership"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Groups = json::value::from_value(object).unwrap();
        let mut call = self.hub.groups().update(request, opt.value_of("group-unique-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("groups", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._groups_get(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._groups_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._groups_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("groups".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "groupssettings1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/groupssettings1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Groupssettings::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
                ]
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("groups", "methods: 'get', 'patch' and 'update'", vec![
            ("get",
                    Some(r##"Gets one resource by id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_groupssettings1_cli/groups_get",
                  vec![
                    (Some(r##"group-unique-id"##),
                     None,
                     Some(r##"The group's email address."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing resource. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_groupssettings1_cli/groups_patch",
                  vec![
                    (Some(r##"group-unique-id"##),
                     None,
                     Some(r##"The group's email address."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_groupssettings1_cli/groups_update",
                  vec![
                    (Some(r##"group-unique-id"##),
                     None,
                     Some(r##"The group's email address."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("groupssettings1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20220614")
           .about("Manages permission levels and related settings of a group.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_groupssettings1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Debug print all errors")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
