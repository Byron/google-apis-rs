// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_androidmanagement1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::AndroidManagement<S>,
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
    async fn _enterprises_applications_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().applications_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _enterprises_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-auto-approval-enabled" => Some(("appAutoApprovalEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "contact-info.contact-email" => Some(("contactInfo.contactEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.data-protection-officer-email" => Some(("contactInfo.dataProtectionOfficerEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.data-protection-officer-name" => Some(("contactInfo.dataProtectionOfficerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.data-protection-officer-phone" => Some(("contactInfo.dataProtectionOfficerPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.eu-representative-email" => Some(("contactInfo.euRepresentativeEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.eu-representative-name" => Some(("contactInfo.euRepresentativeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.eu-representative-phone" => Some(("contactInfo.euRepresentativePhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enabled-notification-types" => Some(("enabledNotificationTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enterprise-display-name" => Some(("enterpriseDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "google-authentication-settings.google-authentication-required" => Some(("googleAuthenticationSettings.googleAuthenticationRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logo.sha256-hash" => Some(("logo.sha256Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logo.url" => Some(("logo.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-color" => Some(("primaryColor", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pubsub-topic" => Some(("pubsubTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-auto-approval-enabled", "contact-email", "contact-info", "data-protection-officer-email", "data-protection-officer-name", "data-protection-officer-phone", "enabled-notification-types", "enterprise-display-name", "eu-representative-email", "eu-representative-name", "eu-representative-phone", "google-authentication-required", "google-authentication-settings", "logo", "name", "primary-color", "pubsub-topic", "sha256-hash", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Enterprise = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "signup-url-name" => {
                    call = call.signup_url_name(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "enterprise-token" => {
                    call = call.enterprise_token(value.unwrap_or(""));
                },
                "agreement-accepted" => {
                    call = call.agreement_accepted(        value.map(|v| arg_from_str(v, err, "agreement-accepted", "boolean")).unwrap_or(false));
                },
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
                                                                           v.extend(["agreement-accepted", "enterprise-token", "project-id", "signup-url-name"].iter().map(|v|*v));
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

    async fn _enterprises_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().delete(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_devices_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().devices_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "wipe-reason-message" => {
                    call = call.wipe_reason_message(value.unwrap_or(""));
                },
                "wipe-data-flags" => {
                    call = call.add_wipe_data_flags(value.unwrap_or(""));
                },
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
                                                                           v.extend(["wipe-data-flags", "wipe-reason-message"].iter().map(|v|*v));
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

    async fn _enterprises_devices_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().devices_get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_devices_issue_command(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "clear-apps-data-params.package-names" => Some(("clearAppsDataParams.packageNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "duration" => Some(("duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error-code" => Some(("errorCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "new-password" => Some(("newPassword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reset-password-flags" => Some(("resetPasswordFlags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "start-lost-mode-params.lost-email-address" => Some(("startLostModeParams.lostEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-lost-mode-params.lost-message.default-message" => Some(("startLostModeParams.lostMessage.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-lost-mode-params.lost-message.localized-messages" => Some(("startLostModeParams.lostMessage.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "start-lost-mode-params.lost-organization.default-message" => Some(("startLostModeParams.lostOrganization.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-lost-mode-params.lost-organization.localized-messages" => Some(("startLostModeParams.lostOrganization.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "start-lost-mode-params.lost-phone-number.default-message" => Some(("startLostModeParams.lostPhoneNumber.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-lost-mode-params.lost-phone-number.localized-messages" => Some(("startLostModeParams.lostPhoneNumber.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "start-lost-mode-params.lost-street-address.default-message" => Some(("startLostModeParams.lostStreetAddress.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-lost-mode-params.lost-street-address.localized-messages" => Some(("startLostModeParams.lostStreetAddress.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "start-lost-mode-status.status" => Some(("startLostModeStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stop-lost-mode-status.status" => Some(("stopLostModeStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-name" => Some(("userName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["clear-apps-data-params", "create-time", "default-message", "duration", "error-code", "localized-messages", "lost-email-address", "lost-message", "lost-organization", "lost-phone-number", "lost-street-address", "new-password", "package-names", "reset-password-flags", "start-lost-mode-params", "start-lost-mode-status", "status", "stop-lost-mode-status", "type", "user-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Command = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().devices_issue_command(request, opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _enterprises_devices_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().devices_operations_cancel(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_devices_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().devices_operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_devices_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().devices_operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _enterprises_devices_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-level" => Some(("apiLevel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "applied-policy-name" => Some(("appliedPolicyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "applied-policy-version" => Some(("appliedPolicyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "applied-state" => Some(("appliedState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-criteria-mode-info.common-criteria-mode-status" => Some(("commonCriteriaModeInfo.commonCriteriaModeStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-settings.adb-enabled" => Some(("deviceSettings.adbEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-settings.development-settings-enabled" => Some(("deviceSettings.developmentSettingsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-settings.encryption-status" => Some(("deviceSettings.encryptionStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-settings.is-device-secure" => Some(("deviceSettings.isDeviceSecure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-settings.is-encrypted" => Some(("deviceSettings.isEncrypted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-settings.unknown-sources-enabled" => Some(("deviceSettings.unknownSourcesEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-settings.verify-apps-enabled" => Some(("deviceSettings.verifyAppsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "disabled-reason.default-message" => Some(("disabledReason.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled-reason.localized-messages" => Some(("disabledReason.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "dpc-migration-info.additional-data" => Some(("dpcMigrationInfo.additionalData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dpc-migration-info.previous-dpc" => Some(("dpcMigrationInfo.previousDpc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enrollment-time" => Some(("enrollmentTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enrollment-token-data" => Some(("enrollmentTokenData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enrollment-token-name" => Some(("enrollmentTokenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.battery-shutdown-temperatures" => Some(("hardwareInfo.batteryShutdownTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.battery-throttling-temperatures" => Some(("hardwareInfo.batteryThrottlingTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.brand" => Some(("hardwareInfo.brand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.cpu-shutdown-temperatures" => Some(("hardwareInfo.cpuShutdownTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.cpu-throttling-temperatures" => Some(("hardwareInfo.cpuThrottlingTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.device-baseband-version" => Some(("hardwareInfo.deviceBasebandVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.enterprise-specific-id" => Some(("hardwareInfo.enterpriseSpecificId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.gpu-shutdown-temperatures" => Some(("hardwareInfo.gpuShutdownTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.gpu-throttling-temperatures" => Some(("hardwareInfo.gpuThrottlingTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.hardware" => Some(("hardwareInfo.hardware", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.manufacturer" => Some(("hardwareInfo.manufacturer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.model" => Some(("hardwareInfo.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.serial-number" => Some(("hardwareInfo.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hardware-info.skin-shutdown-temperatures" => Some(("hardwareInfo.skinShutdownTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hardware-info.skin-throttling-temperatures" => Some(("hardwareInfo.skinThrottlingTemperatures", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "last-policy-compliance-report-time" => Some(("lastPolicyComplianceReportTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-policy-sync-time" => Some(("lastPolicySyncTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-status-report-time" => Some(("lastStatusReportTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-mode" => Some(("managementMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "memory-info.total-internal-storage" => Some(("memoryInfo.totalInternalStorage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "memory-info.total-ram" => Some(("memoryInfo.totalRam", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-info.imei" => Some(("networkInfo.imei", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-info.meid" => Some(("networkInfo.meid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-info.network-operator-name" => Some(("networkInfo.networkOperatorName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-info.wifi-mac-address" => Some(("networkInfo.wifiMacAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ownership" => Some(("ownership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy-compliant" => Some(("policyCompliant", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policy-name" => Some(("policyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "previous-device-names" => Some(("previousDeviceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "security-posture.device-posture" => Some(("securityPosture.devicePosture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.android-build-number" => Some(("softwareInfo.androidBuildNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.android-build-time" => Some(("softwareInfo.androidBuildTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.android-device-policy-version-code" => Some(("softwareInfo.androidDevicePolicyVersionCode", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "software-info.android-device-policy-version-name" => Some(("softwareInfo.androidDevicePolicyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.android-version" => Some(("softwareInfo.androidVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.bootloader-version" => Some(("softwareInfo.bootloaderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.device-build-signature" => Some(("softwareInfo.deviceBuildSignature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.device-kernel-version" => Some(("softwareInfo.deviceKernelVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.primary-language-code" => Some(("softwareInfo.primaryLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.security-patch-level" => Some(("softwareInfo.securityPatchLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.system-update-info.update-received-time" => Some(("softwareInfo.systemUpdateInfo.updateReceivedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "software-info.system-update-info.update-status" => Some(("softwareInfo.systemUpdateInfo.updateStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "system-properties" => Some(("systemProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "user.account-identifier" => Some(("user.accountIdentifier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-name" => Some(("userName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-identifier", "adb-enabled", "additional-data", "android-build-number", "android-build-time", "android-device-policy-version-code", "android-device-policy-version-name", "android-version", "api-level", "applied-policy-name", "applied-policy-version", "applied-state", "battery-shutdown-temperatures", "battery-throttling-temperatures", "bootloader-version", "brand", "common-criteria-mode-info", "common-criteria-mode-status", "cpu-shutdown-temperatures", "cpu-throttling-temperatures", "default-message", "development-settings-enabled", "device-baseband-version", "device-build-signature", "device-kernel-version", "device-posture", "device-settings", "disabled-reason", "dpc-migration-info", "encryption-status", "enrollment-time", "enrollment-token-data", "enrollment-token-name", "enterprise-specific-id", "gpu-shutdown-temperatures", "gpu-throttling-temperatures", "hardware", "hardware-info", "imei", "is-device-secure", "is-encrypted", "last-policy-compliance-report-time", "last-policy-sync-time", "last-status-report-time", "localized-messages", "management-mode", "manufacturer", "meid", "memory-info", "model", "name", "network-info", "network-operator-name", "ownership", "policy-compliant", "policy-name", "previous-device-names", "previous-dpc", "primary-language-code", "security-patch-level", "security-posture", "serial-number", "skin-shutdown-temperatures", "skin-throttling-temperatures", "software-info", "state", "system-properties", "system-update-info", "total-internal-storage", "total-ram", "unknown-sources-enabled", "update-received-time", "update-status", "user", "user-name", "verify-apps-enabled", "wifi-mac-address"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Device = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().devices_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _enterprises_enrollment_tokens_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "additional-data" => Some(("additionalData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-personal-usage" => Some(("allowPersonalUsage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "duration" => Some(("duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-timestamp" => Some(("expirationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "one-time-only" => Some(("oneTimeOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policy-name" => Some(("policyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "qr-code" => Some(("qrCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user.account-identifier" => Some(("user.accountIdentifier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-identifier", "additional-data", "allow-personal-usage", "duration", "expiration-timestamp", "name", "one-time-only", "policy-name", "qr-code", "user", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EnrollmentToken = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().enrollment_tokens_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _enterprises_enrollment_tokens_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().enrollment_tokens_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_enrollment_tokens_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().enrollment_tokens_get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_enrollment_tokens_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().enrollment_tokens_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _enterprises_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
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
                                                                           v.extend(["page-size", "page-token", "project-id", "view"].iter().map(|v|*v));
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

    async fn _enterprises_migration_tokens_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "additional-data" => Some(("additionalData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device" => Some(("device", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-id" => Some(("deviceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-mode" => Some(("managementMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy" => Some(("policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl" => Some(("ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-id" => Some(("userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-data", "create-time", "device", "device-id", "expire-time", "management-mode", "name", "policy", "ttl", "user-id", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::MigrationToken = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().migration_tokens_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _enterprises_migration_tokens_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().migration_tokens_get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_migration_tokens_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().migration_tokens_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _enterprises_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-auto-approval-enabled" => Some(("appAutoApprovalEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "contact-info.contact-email" => Some(("contactInfo.contactEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.data-protection-officer-email" => Some(("contactInfo.dataProtectionOfficerEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.data-protection-officer-name" => Some(("contactInfo.dataProtectionOfficerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.data-protection-officer-phone" => Some(("contactInfo.dataProtectionOfficerPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.eu-representative-email" => Some(("contactInfo.euRepresentativeEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.eu-representative-name" => Some(("contactInfo.euRepresentativeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-info.eu-representative-phone" => Some(("contactInfo.euRepresentativePhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enabled-notification-types" => Some(("enabledNotificationTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enterprise-display-name" => Some(("enterpriseDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "google-authentication-settings.google-authentication-required" => Some(("googleAuthenticationSettings.googleAuthenticationRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logo.sha256-hash" => Some(("logo.sha256Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logo.url" => Some(("logo.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-color" => Some(("primaryColor", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pubsub-topic" => Some(("pubsubTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-auto-approval-enabled", "contact-email", "contact-info", "data-protection-officer-email", "data-protection-officer-name", "data-protection-officer-phone", "enabled-notification-types", "enterprise-display-name", "eu-representative-email", "eu-representative-name", "eu-representative-phone", "google-authentication-required", "google-authentication-settings", "logo", "name", "primary-color", "pubsub-topic", "sha256-hash", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Enterprise = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _enterprises_policies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().policies_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_policies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().policies_get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_policies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().policies_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _enterprises_policies_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-types-with-management-disabled" => Some(("accountTypesWithManagementDisabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "add-user-disabled" => Some(("addUserDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "adjust-volume-disabled" => Some(("adjustVolumeDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-security-overrides.common-criteria-mode" => Some(("advancedSecurityOverrides.commonCriteriaMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-security-overrides.developer-settings" => Some(("advancedSecurityOverrides.developerSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-security-overrides.google-play-protect-verify-apps" => Some(("advancedSecurityOverrides.googlePlayProtectVerifyApps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-security-overrides.mte-policy" => Some(("advancedSecurityOverrides.mtePolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-security-overrides.personal-apps-that-can-read-work-notifications" => Some(("advancedSecurityOverrides.personalAppsThatCanReadWorkNotifications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "advanced-security-overrides.untrusted-apps-policy" => Some(("advancedSecurityOverrides.untrustedAppsPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "always-on-vpn-package.lockdown-enabled" => Some(("alwaysOnVpnPackage.lockdownEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "always-on-vpn-package.package-name" => Some(("alwaysOnVpnPackage.packageName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "android-device-policy-tracks" => Some(("androidDevicePolicyTracks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "app-auto-update-policy" => Some(("appAutoUpdatePolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auto-date-and-time-zone" => Some(("autoDateAndTimeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auto-time-required" => Some(("autoTimeRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "block-applications-enabled" => Some(("blockApplicationsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bluetooth-config-disabled" => Some(("bluetoothConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bluetooth-contact-sharing-disabled" => Some(("bluetoothContactSharingDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bluetooth-disabled" => Some(("bluetoothDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "camera-access" => Some(("cameraAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "camera-disabled" => Some(("cameraDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cell-broadcasts-config-disabled" => Some(("cellBroadcastsConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "create-windows-disabled" => Some(("createWindowsDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "credential-provider-policy-default" => Some(("credentialProviderPolicyDefault", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "credentials-config-disabled" => Some(("credentialsConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cross-profile-policies.cross-profile-copy-paste" => Some(("crossProfilePolicies.crossProfileCopyPaste", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cross-profile-policies.cross-profile-data-sharing" => Some(("crossProfilePolicies.crossProfileDataSharing", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cross-profile-policies.exemptions-to-show-work-contacts-in-personal-profile.package-names" => Some(("crossProfilePolicies.exemptionsToShowWorkContactsInPersonalProfile.packageNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cross-profile-policies.show-work-contacts-in-personal-profile" => Some(("crossProfilePolicies.showWorkContactsInPersonalProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cross-profile-policies.work-profile-widgets-default" => Some(("crossProfilePolicies.workProfileWidgetsDefault", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-roaming-disabled" => Some(("dataRoamingDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "debugging-features-allowed" => Some(("debuggingFeaturesAllowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-permission-policy" => Some(("defaultPermissionPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-connectivity-management.configure-wifi" => Some(("deviceConnectivityManagement.configureWifi", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-connectivity-management.tethering-settings" => Some(("deviceConnectivityManagement.tetheringSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-connectivity-management.usb-data-access" => Some(("deviceConnectivityManagement.usbDataAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-connectivity-management.wifi-direct-settings" => Some(("deviceConnectivityManagement.wifiDirectSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-connectivity-management.wifi-ssid-policy.wifi-ssid-policy-type" => Some(("deviceConnectivityManagement.wifiSsidPolicy.wifiSsidPolicyType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner-lock-screen-info.default-message" => Some(("deviceOwnerLockScreenInfo.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner-lock-screen-info.localized-messages" => Some(("deviceOwnerLockScreenInfo.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "device-radio-state.airplane-mode-state" => Some(("deviceRadioState.airplaneModeState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-radio-state.cellular-two-g-state" => Some(("deviceRadioState.cellularTwoGState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-radio-state.minimum-wifi-security-level" => Some(("deviceRadioState.minimumWifiSecurityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-radio-state.ultra-wideband-state" => Some(("deviceRadioState.ultraWidebandState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-radio-state.wifi-state" => Some(("deviceRadioState.wifiState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-settings.screen-brightness-settings.screen-brightness" => Some(("displaySettings.screenBrightnessSettings.screenBrightness", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "display-settings.screen-brightness-settings.screen-brightness-mode" => Some(("displaySettings.screenBrightnessSettings.screenBrightnessMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-settings.screen-timeout-settings.screen-timeout" => Some(("displaySettings.screenTimeoutSettings.screenTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-settings.screen-timeout-settings.screen-timeout-mode" => Some(("displaySettings.screenTimeoutSettings.screenTimeoutMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-policy" => Some(("encryptionPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ensure-verify-apps-enabled" => Some(("ensureVerifyAppsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "factory-reset-disabled" => Some(("factoryResetDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "frp-admin-emails" => Some(("frpAdminEmails", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fun-disabled" => Some(("funDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "install-apps-disabled" => Some(("installAppsDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "install-unknown-sources-allowed" => Some(("installUnknownSourcesAllowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "keyguard-disabled" => Some(("keyguardDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "keyguard-disabled-features" => Some(("keyguardDisabledFeatures", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kiosk-custom-launcher-enabled" => Some(("kioskCustomLauncherEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kiosk-customization.device-settings" => Some(("kioskCustomization.deviceSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kiosk-customization.power-button-actions" => Some(("kioskCustomization.powerButtonActions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kiosk-customization.status-bar" => Some(("kioskCustomization.statusBar", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kiosk-customization.system-error-warnings" => Some(("kioskCustomization.systemErrorWarnings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kiosk-customization.system-navigation" => Some(("kioskCustomization.systemNavigation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-mode" => Some(("locationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-support-message.default-message" => Some(("longSupportMessage.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-support-message.localized-messages" => Some(("longSupportMessage.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "maximum-time-to-lock" => Some(("maximumTimeToLock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "microphone-access" => Some(("microphoneAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-api-level" => Some(("minimumApiLevel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "mobile-networks-config-disabled" => Some(("mobileNetworksConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "modify-accounts-disabled" => Some(("modifyAccountsDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "mount-physical-media-disabled" => Some(("mountPhysicalMediaDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-escape-hatch-enabled" => Some(("networkEscapeHatchEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "network-reset-disabled" => Some(("networkResetDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outgoing-beam-disabled" => Some(("outgoingBeamDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outgoing-calls-disabled" => Some(("outgoingCallsDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "password-requirements.maximum-failed-passwords-for-wipe" => Some(("passwordRequirements.maximumFailedPasswordsForWipe", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-expiration-timeout" => Some(("passwordRequirements.passwordExpirationTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "password-requirements.password-history-length" => Some(("passwordRequirements.passwordHistoryLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-length" => Some(("passwordRequirements.passwordMinimumLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-letters" => Some(("passwordRequirements.passwordMinimumLetters", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-lower-case" => Some(("passwordRequirements.passwordMinimumLowerCase", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-non-letter" => Some(("passwordRequirements.passwordMinimumNonLetter", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-numeric" => Some(("passwordRequirements.passwordMinimumNumeric", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-symbols" => Some(("passwordRequirements.passwordMinimumSymbols", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-minimum-upper-case" => Some(("passwordRequirements.passwordMinimumUpperCase", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "password-requirements.password-quality" => Some(("passwordRequirements.passwordQuality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "password-requirements.password-scope" => Some(("passwordRequirements.passwordScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "password-requirements.require-password-unlock" => Some(("passwordRequirements.requirePasswordUnlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "password-requirements.unified-lock-settings" => Some(("passwordRequirements.unifiedLockSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permitted-accessibility-services.package-names" => Some(("permittedAccessibilityServices.packageNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permitted-input-methods.package-names" => Some(("permittedInputMethods.packageNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "personal-usage-policies.account-types-with-management-disabled" => Some(("personalUsagePolicies.accountTypesWithManagementDisabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "personal-usage-policies.camera-disabled" => Some(("personalUsagePolicies.cameraDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "personal-usage-policies.max-days-with-work-off" => Some(("personalUsagePolicies.maxDaysWithWorkOff", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "personal-usage-policies.personal-play-store-mode" => Some(("personalUsagePolicies.personalPlayStoreMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "personal-usage-policies.screen-capture-disabled" => Some(("personalUsagePolicies.screenCaptureDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "play-store-mode" => Some(("playStoreMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preferential-network-service" => Some(("preferentialNetworkService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "printing-policy" => Some(("printingPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-key-selection-enabled" => Some(("privateKeySelectionEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "recommended-global-proxy.excluded-hosts" => Some(("recommendedGlobalProxy.excludedHosts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recommended-global-proxy.host" => Some(("recommendedGlobalProxy.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recommended-global-proxy.pac-uri" => Some(("recommendedGlobalProxy.pacUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recommended-global-proxy.port" => Some(("recommendedGlobalProxy.port", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "remove-user-disabled" => Some(("removeUserDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "safe-boot-disabled" => Some(("safeBootDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "screen-capture-disabled" => Some(("screenCaptureDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "set-user-icon-disabled" => Some(("setUserIconDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "set-wallpaper-disabled" => Some(("setWallpaperDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "share-location-disabled" => Some(("shareLocationDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "short-support-message.default-message" => Some(("shortSupportMessage.defaultMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "short-support-message.localized-messages" => Some(("shortSupportMessage.localizedMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "skip-first-use-hints-enabled" => Some(("skipFirstUseHintsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sms-disabled" => Some(("smsDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-bar-disabled" => Some(("statusBarDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.application-reporting-settings.include-removed-apps" => Some(("statusReportingSettings.applicationReportingSettings.includeRemovedApps", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.application-reports-enabled" => Some(("statusReportingSettings.applicationReportsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.common-criteria-mode-enabled" => Some(("statusReportingSettings.commonCriteriaModeEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.device-settings-enabled" => Some(("statusReportingSettings.deviceSettingsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.display-info-enabled" => Some(("statusReportingSettings.displayInfoEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.hardware-status-enabled" => Some(("statusReportingSettings.hardwareStatusEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.memory-info-enabled" => Some(("statusReportingSettings.memoryInfoEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.network-info-enabled" => Some(("statusReportingSettings.networkInfoEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.power-management-events-enabled" => Some(("statusReportingSettings.powerManagementEventsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.software-info-enabled" => Some(("statusReportingSettings.softwareInfoEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status-reporting-settings.system-properties-enabled" => Some(("statusReportingSettings.systemPropertiesEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "stay-on-plugged-modes" => Some(("stayOnPluggedModes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "system-update.end-minutes" => Some(("systemUpdate.endMinutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "system-update.start-minutes" => Some(("systemUpdate.startMinutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "system-update.type" => Some(("systemUpdate.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tethering-config-disabled" => Some(("tetheringConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "uninstall-apps-disabled" => Some(("uninstallAppsDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "unmute-microphone-disabled" => Some(("unmuteMicrophoneDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "usage-log.enabled-log-types" => Some(("usageLog.enabledLogTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "usage-log.upload-on-cellular-allowed" => Some(("usageLog.uploadOnCellularAllowed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "usb-file-transfer-disabled" => Some(("usbFileTransferDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "usb-mass-storage-enabled" => Some(("usbMassStorageEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vpn-config-disabled" => Some(("vpnConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wifi-config-disabled" => Some(("wifiConfigDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wifi-configs-lockdown-enabled" => Some(("wifiConfigsLockdownEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-types-with-management-disabled", "add-user-disabled", "adjust-volume-disabled", "advanced-security-overrides", "airplane-mode-state", "always-on-vpn-package", "android-device-policy-tracks", "app-auto-update-policy", "application-reporting-settings", "application-reports-enabled", "auto-date-and-time-zone", "auto-time-required", "block-applications-enabled", "bluetooth-config-disabled", "bluetooth-contact-sharing-disabled", "bluetooth-disabled", "camera-access", "camera-disabled", "cell-broadcasts-config-disabled", "cellular-two-g-state", "common-criteria-mode", "common-criteria-mode-enabled", "configure-wifi", "create-windows-disabled", "credential-provider-policy-default", "credentials-config-disabled", "cross-profile-copy-paste", "cross-profile-data-sharing", "cross-profile-policies", "data-roaming-disabled", "debugging-features-allowed", "default-message", "default-permission-policy", "developer-settings", "device-connectivity-management", "device-owner-lock-screen-info", "device-radio-state", "device-settings", "device-settings-enabled", "display-info-enabled", "display-settings", "enabled-log-types", "encryption-policy", "end-minutes", "ensure-verify-apps-enabled", "excluded-hosts", "exemptions-to-show-work-contacts-in-personal-profile", "factory-reset-disabled", "frp-admin-emails", "fun-disabled", "google-play-protect-verify-apps", "hardware-status-enabled", "host", "include-removed-apps", "install-apps-disabled", "install-unknown-sources-allowed", "keyguard-disabled", "keyguard-disabled-features", "kiosk-custom-launcher-enabled", "kiosk-customization", "localized-messages", "location-mode", "lockdown-enabled", "long-support-message", "max-days-with-work-off", "maximum-failed-passwords-for-wipe", "maximum-time-to-lock", "memory-info-enabled", "microphone-access", "minimum-api-level", "minimum-wifi-security-level", "mobile-networks-config-disabled", "modify-accounts-disabled", "mount-physical-media-disabled", "mte-policy", "name", "network-escape-hatch-enabled", "network-info-enabled", "network-reset-disabled", "outgoing-beam-disabled", "outgoing-calls-disabled", "pac-uri", "package-name", "package-names", "password-expiration-timeout", "password-history-length", "password-minimum-length", "password-minimum-letters", "password-minimum-lower-case", "password-minimum-non-letter", "password-minimum-numeric", "password-minimum-symbols", "password-minimum-upper-case", "password-quality", "password-requirements", "password-scope", "permitted-accessibility-services", "permitted-input-methods", "personal-apps-that-can-read-work-notifications", "personal-play-store-mode", "personal-usage-policies", "play-store-mode", "port", "power-button-actions", "power-management-events-enabled", "preferential-network-service", "printing-policy", "private-key-selection-enabled", "recommended-global-proxy", "remove-user-disabled", "require-password-unlock", "safe-boot-disabled", "screen-brightness", "screen-brightness-mode", "screen-brightness-settings", "screen-capture-disabled", "screen-timeout", "screen-timeout-mode", "screen-timeout-settings", "set-user-icon-disabled", "set-wallpaper-disabled", "share-location-disabled", "short-support-message", "show-work-contacts-in-personal-profile", "skip-first-use-hints-enabled", "sms-disabled", "software-info-enabled", "start-minutes", "status-bar", "status-bar-disabled", "status-reporting-settings", "stay-on-plugged-modes", "system-error-warnings", "system-navigation", "system-properties-enabled", "system-update", "tethering-config-disabled", "tethering-settings", "type", "ultra-wideband-state", "unified-lock-settings", "uninstall-apps-disabled", "unmute-microphone-disabled", "untrusted-apps-policy", "upload-on-cellular-allowed", "usage-log", "usb-data-access", "usb-file-transfer-disabled", "usb-mass-storage-enabled", "version", "vpn-config-disabled", "wifi-config-disabled", "wifi-configs-lockdown-enabled", "wifi-direct-settings", "wifi-ssid-policy", "wifi-ssid-policy-type", "wifi-state", "work-profile-widgets-default"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Policy = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().policies_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _enterprises_web_apps_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-mode" => Some(("displayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-url" => Some(("startUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-code" => Some(("versionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-mode", "name", "start-url", "title", "version-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WebApp = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().web_apps_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _enterprises_web_apps_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().web_apps_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_web_apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().web_apps_get(opt.value_of("name").unwrap_or(""));
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

    async fn _enterprises_web_apps_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.enterprises().web_apps_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _enterprises_web_apps_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-mode" => Some(("displayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-url" => Some(("startUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-code" => Some(("versionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-mode", "name", "start-url", "title", "version-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WebApp = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().web_apps_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _enterprises_web_tokens_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "enabled-features" => Some(("enabledFeatures", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-frame-url" => Some(("parentFrameUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["enabled-features", "name", "parent-frame-url", "permissions", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WebToken = json::value::from_value(object).unwrap();
        let mut call = self.hub.enterprises().web_tokens_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _provisioning_info_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.provisioning_info().get(opt.value_of("name").unwrap_or(""));
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

    async fn _signup_urls_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.signup_urls().create();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "callback-url" => {
                    call = call.callback_url(value.unwrap_or(""));
                },
                "admin-email" => {
                    call = call.admin_email(value.unwrap_or(""));
                },
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
                                                                           v.extend(["admin-email", "callback-url", "project-id"].iter().map(|v|*v));
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
            ("enterprises", Some(opt)) => {
                match opt.subcommand() {
                    ("applications-get", Some(opt)) => {
                        call_result = self._enterprises_applications_get(opt, dry_run, &mut err).await;
                    },
                    ("create", Some(opt)) => {
                        call_result = self._enterprises_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._enterprises_delete(opt, dry_run, &mut err).await;
                    },
                    ("devices-delete", Some(opt)) => {
                        call_result = self._enterprises_devices_delete(opt, dry_run, &mut err).await;
                    },
                    ("devices-get", Some(opt)) => {
                        call_result = self._enterprises_devices_get(opt, dry_run, &mut err).await;
                    },
                    ("devices-issue-command", Some(opt)) => {
                        call_result = self._enterprises_devices_issue_command(opt, dry_run, &mut err).await;
                    },
                    ("devices-list", Some(opt)) => {
                        call_result = self._enterprises_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("devices-operations-cancel", Some(opt)) => {
                        call_result = self._enterprises_devices_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("devices-operations-get", Some(opt)) => {
                        call_result = self._enterprises_devices_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("devices-operations-list", Some(opt)) => {
                        call_result = self._enterprises_devices_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("devices-patch", Some(opt)) => {
                        call_result = self._enterprises_devices_patch(opt, dry_run, &mut err).await;
                    },
                    ("enrollment-tokens-create", Some(opt)) => {
                        call_result = self._enterprises_enrollment_tokens_create(opt, dry_run, &mut err).await;
                    },
                    ("enrollment-tokens-delete", Some(opt)) => {
                        call_result = self._enterprises_enrollment_tokens_delete(opt, dry_run, &mut err).await;
                    },
                    ("enrollment-tokens-get", Some(opt)) => {
                        call_result = self._enterprises_enrollment_tokens_get(opt, dry_run, &mut err).await;
                    },
                    ("enrollment-tokens-list", Some(opt)) => {
                        call_result = self._enterprises_enrollment_tokens_list(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._enterprises_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._enterprises_list(opt, dry_run, &mut err).await;
                    },
                    ("migration-tokens-create", Some(opt)) => {
                        call_result = self._enterprises_migration_tokens_create(opt, dry_run, &mut err).await;
                    },
                    ("migration-tokens-get", Some(opt)) => {
                        call_result = self._enterprises_migration_tokens_get(opt, dry_run, &mut err).await;
                    },
                    ("migration-tokens-list", Some(opt)) => {
                        call_result = self._enterprises_migration_tokens_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._enterprises_patch(opt, dry_run, &mut err).await;
                    },
                    ("policies-delete", Some(opt)) => {
                        call_result = self._enterprises_policies_delete(opt, dry_run, &mut err).await;
                    },
                    ("policies-get", Some(opt)) => {
                        call_result = self._enterprises_policies_get(opt, dry_run, &mut err).await;
                    },
                    ("policies-list", Some(opt)) => {
                        call_result = self._enterprises_policies_list(opt, dry_run, &mut err).await;
                    },
                    ("policies-patch", Some(opt)) => {
                        call_result = self._enterprises_policies_patch(opt, dry_run, &mut err).await;
                    },
                    ("web-apps-create", Some(opt)) => {
                        call_result = self._enterprises_web_apps_create(opt, dry_run, &mut err).await;
                    },
                    ("web-apps-delete", Some(opt)) => {
                        call_result = self._enterprises_web_apps_delete(opt, dry_run, &mut err).await;
                    },
                    ("web-apps-get", Some(opt)) => {
                        call_result = self._enterprises_web_apps_get(opt, dry_run, &mut err).await;
                    },
                    ("web-apps-list", Some(opt)) => {
                        call_result = self._enterprises_web_apps_list(opt, dry_run, &mut err).await;
                    },
                    ("web-apps-patch", Some(opt)) => {
                        call_result = self._enterprises_web_apps_patch(opt, dry_run, &mut err).await;
                    },
                    ("web-tokens-create", Some(opt)) => {
                        call_result = self._enterprises_web_tokens_create(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("enterprises".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("provisioning-info", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._provisioning_info_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("provisioning-info".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("signup-urls", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._signup_urls_create(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("signup-urls".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "androidmanagement1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/androidmanagement1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::AndroidManagement::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
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
        ("enterprises", "methods: 'applications-get', 'create', 'delete', 'devices-delete', 'devices-get', 'devices-issue-command', 'devices-list', 'devices-operations-cancel', 'devices-operations-get', 'devices-operations-list', 'devices-patch', 'enrollment-tokens-create', 'enrollment-tokens-delete', 'enrollment-tokens-get', 'enrollment-tokens-list', 'get', 'list', 'migration-tokens-create', 'migration-tokens-get', 'migration-tokens-list', 'patch', 'policies-delete', 'policies-get', 'policies-list', 'policies-patch', 'web-apps-create', 'web-apps-delete', 'web-apps-get', 'web-apps-list', 'web-apps-patch' and 'web-tokens-create'", vec![
            ("applications-get",
                    Some(r##"Gets info about an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_applications-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the application in the form enterprises/{enterpriseId}/applications/{package_name}."##),
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
            ("create",
                    Some(r##"Creates an enterprise. This is the last step in the enterprise signup flow. See also: SigninDetail"##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_create",
                  vec![
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
            ("delete",
                    Some(r##"Permanently deletes an enterprise and all accounts and data associated with it. Warning: this will result in a cascaded deletion of all AM API devices associated with the deleted enterprise. Only available for EMM-managed enterprises."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("devices-delete",
                    Some(r##"Deletes a device. This operation wipes the device. Deleted devices do not show up in enterprises.devices.list calls and a 404 is returned from enterprises.devices.get."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."##),
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
            ("devices-get",
                    Some(r##"Gets a device. Deleted devices will respond with a 404 error."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."##),
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
            ("devices-issue-command",
                    Some(r##"Issues a command to a device. The Operation resource returned contains a Command in its metadata field. Use the get operation method to get the status of the command."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-issue-command",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."##),
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
            ("devices-list",
                    Some(r##"Lists devices for a given enterprise. Deleted devices are not returned in the response."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("devices-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("devices-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("devices-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("devices-patch",
                    Some(r##"Updates a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_devices-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."##),
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
            ("enrollment-tokens-create",
                    Some(r##"Creates an enrollment token for a given enterprise. It's up to the caller's responsibility to manage the lifecycle of newly created tokens and deleting them when they're not intended to be used anymore. Once an enrollment token has been created, it's not possible to retrieve the token's content anymore using AM API. It is recommended for EMMs to securely store the token if it's intended to be reused."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_enrollment-tokens-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("enrollment-tokens-delete",
                    Some(r##"Deletes an enrollment token. This operation invalidates the token, preventing its future use."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_enrollment-tokens-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the enrollment token in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}."##),
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
            ("enrollment-tokens-get",
                    Some(r##"Gets an active, unexpired enrollment token. A partial view of the enrollment token is returned. Only the following fields are populated: name, expirationTimestamp, allowPersonalUsage, value, qrCode. This method is meant to help manage active enrollment tokens lifecycle. For security reasons, it's recommended to delete active enrollment tokens as soon as they're not intended to be used anymore."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_enrollment-tokens-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the enrollment token in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}."##),
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
            ("enrollment-tokens-list",
                    Some(r##"Lists active, unexpired enrollment tokens for a given enterprise. The list items contain only a partial view of EnrollmentToken object. Only the following fields are populated: name, expirationTimestamp, allowPersonalUsage, value, qrCode. This method is meant to help manage active enrollment tokens lifecycle. For security reasons, it's recommended to delete active enrollment tokens as soon as they're not intended to be used anymore."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_enrollment-tokens-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("get",
                    Some(r##"Gets an enterprise."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("list",
                    Some(r##"Lists EMM-managed enterprises. Only BASIC fields are returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_list",
                  vec![
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
            ("migration-tokens-create",
                    Some(r##"Creates a migration token, to migrate an existing device from being managed by the EMM's Device Policy Controller (DPC) to being managed by the Android Management API. See the guide (https://developers.google.com/android/management/dpc-migration) for more details."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_migration-tokens-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The enterprise in which this migration token is created. This must be the same enterprise which already manages the device in the Play EMM API. Format: enterprises/{enterprise}"##),
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
            ("migration-tokens-get",
                    Some(r##"Gets a migration token."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_migration-tokens-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the migration token to retrieve. Format: enterprises/{enterprise}/migrationTokens/{migration_token}"##),
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
            ("migration-tokens-list",
                    Some(r##"Lists migration tokens."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_migration-tokens-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The enterprise which the migration tokens belong to. Format: enterprises/{enterprise}"##),
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
                    Some(r##"Updates an enterprise. See also: SigninDetail"##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("policies-delete",
                    Some(r##"Deletes a policy. This operation is only permitted if no devices are currently referencing the policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_policies-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}."##),
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
            ("policies-get",
                    Some(r##"Gets a policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_policies-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}."##),
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
            ("policies-list",
                    Some(r##"Lists policies for a given enterprise."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_policies-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("policies-patch",
                    Some(r##"Updates or creates a policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_policies-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}."##),
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
            ("web-apps-create",
                    Some(r##"Creates a web app."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_web-apps-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("web-apps-delete",
                    Some(r##"Deletes a web app."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_web-apps-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the web app in the form enterprises/{enterpriseId}/webApps/{packageName}."##),
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
            ("web-apps-get",
                    Some(r##"Gets a web app."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_web-apps-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the web app in the form enterprises/{enterpriseId}/webApp/{packageName}."##),
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
            ("web-apps-list",
                    Some(r##"Lists web apps for a given enterprise."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_web-apps-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
            ("web-apps-patch",
                    Some(r##"Updates a web app."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_web-apps-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the web app in the form enterprises/{enterpriseId}/webApps/{packageName}."##),
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
            ("web-tokens-create",
                    Some(r##"Creates a web token to access an embeddable managed Google Play web UI for a given enterprise."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/enterprises_web-tokens-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the enterprise in the form enterprises/{enterpriseId}."##),
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
        
        ("provisioning-info", "methods: 'get'", vec![
            ("get",
                    Some(r##"Get the device provisioning information by the identifier provided in the sign-in url."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/provisioning-info_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The identifier that Android Device Policy passes to the 3P sign-in page in the form of provisioningInfo/{provisioning_info}."##),
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
            ]),
        
        ("signup-urls", "methods: 'create'", vec![
            ("create",
                    Some(r##"Creates an enterprise signup URL."##),
                    "Details at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli/signup-urls_create",
                  vec![
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
    
    let mut app = App::new("androidmanagement1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240626")
           .about("The Android Management API provides remote enterprise management of Android devices and apps.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_androidmanagement1_cli")
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
