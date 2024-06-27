// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_drive2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::DriveHub<S>,
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
    async fn _about_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.about().get();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(        value.map(|v| arg_from_str(v, err, "start-change-id", "int64")).unwrap_or(-0));
                },
                "max-change-id-count" => {
                    call = call.max_change_id_count(        value.map(|v| arg_from_str(v, err, "max-change-id-count", "int64")).unwrap_or(-0));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(        value.map(|v| arg_from_str(v, err, "include-subscribed", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-subscribed", "max-change-id-count", "start-change-id"].iter().map(|v|*v));
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

    async fn _apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().get(opt.value_of("app-id").unwrap_or(""));
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

    async fn _apps_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "app-filter-mime-types" => {
                    call = call.app_filter_mime_types(value.unwrap_or(""));
                },
                "app-filter-extensions" => {
                    call = call.app_filter_extensions(value.unwrap_or(""));
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
                                                                           v.extend(["app-filter-extensions", "app-filter-mime-types", "language-code"].iter().map(|v|*v));
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

    async fn _changes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().get(opt.value_of("change-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "team-drive-id" => {
                    call = call.team_drive_id(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "drive-id" => {
                    call = call.drive_id(value.unwrap_or(""));
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
                                                                           v.extend(["drive-id", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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

    async fn _changes_get_start_page_token(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().get_start_page_token();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "team-drive-id" => {
                    call = call.team_drive_id(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "drive-id" => {
                    call = call.drive_id(value.unwrap_or(""));
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
                                                                           v.extend(["drive-id", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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

    async fn _changes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "team-drive-id" => {
                    call = call.team_drive_id(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "start-change-id" => {
                    call = call.start_change_id(        value.map(|v| arg_from_str(v, err, "start-change-id", "int64")).unwrap_or(-0));
                },
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-team-drive-items" => {
                    call = call.include_team_drive_items(        value.map(|v| arg_from_str(v, err, "include-team-drive-items", "boolean")).unwrap_or(false));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(        value.map(|v| arg_from_str(v, err, "include-subscribed", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "include-items-from-all-drives" => {
                    call = call.include_items_from_all_drives(        value.map(|v| arg_from_str(v, err, "include-items-from-all-drives", "boolean")).unwrap_or(false));
                },
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
                },
                "include-corpus-removals" => {
                    call = call.include_corpus_removals(        value.map(|v| arg_from_str(v, err, "include-corpus-removals", "boolean")).unwrap_or(false));
                },
                "drive-id" => {
                    call = call.drive_id(value.unwrap_or(""));
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
                                                                           v.extend(["drive-id", "include-corpus-removals", "include-deleted", "include-items-from-all-drives", "include-labels", "include-permissions-for-view", "include-subscribed", "include-team-drive-items", "max-results", "page-token", "spaces", "start-change-id", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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

    async fn _changes_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.changes().watch(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "team-drive-id" => {
                    call = call.team_drive_id(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "start-change-id" => {
                    call = call.start_change_id(        value.map(|v| arg_from_str(v, err, "start-change-id", "int64")).unwrap_or(-0));
                },
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-team-drive-items" => {
                    call = call.include_team_drive_items(        value.map(|v| arg_from_str(v, err, "include-team-drive-items", "boolean")).unwrap_or(false));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(        value.map(|v| arg_from_str(v, err, "include-subscribed", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "include-items-from-all-drives" => {
                    call = call.include_items_from_all_drives(        value.map(|v| arg_from_str(v, err, "include-items-from-all-drives", "boolean")).unwrap_or(false));
                },
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
                },
                "include-corpus-removals" => {
                    call = call.include_corpus_removals(        value.map(|v| arg_from_str(v, err, "include-corpus-removals", "boolean")).unwrap_or(false));
                },
                "drive-id" => {
                    call = call.drive_id(value.unwrap_or(""));
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
                                                                           v.extend(["drive-id", "include-corpus-removals", "include-deleted", "include-items-from-all-drives", "include-labels", "include-permissions-for-view", "include-subscribed", "include-team-drive-items", "max-results", "page-token", "spaces", "start-change-id", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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

    async fn _channels_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.channels().stop(request);
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _children_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().delete(opt.value_of("folder-id").unwrap_or(""), opt.value_of("child-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _children_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().get(opt.value_of("folder-id").unwrap_or(""), opt.value_of("child-id").unwrap_or(""));
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

    async fn _children_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "child-link" => Some(("childLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["child-link", "id", "kind", "self-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ChildReference = json::value::from_value(object).unwrap();
        let mut call = self.hub.children().insert(request, opt.value_of("folder-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _children_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().list(opt.value_of("folder-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "order-by", "page-token", "q"].iter().map(|v|*v));
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

    async fn _comments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _comments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-deleted"].iter().map(|v|*v));
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

    async fn _comments_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comment-id" => Some(("commentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.type" => Some(("context.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.value" => Some(("context.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "file-id" => Some(("fileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-title" => Some(("fileTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["anchor", "author", "comment-id", "content", "context", "created-date", "deleted", "display-name", "email-address", "file-id", "file-title", "html-content", "is-authenticated-user", "kind", "modified-date", "permission-id", "picture", "self-link", "status", "type", "url", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Comment = json::value::from_value(object).unwrap();
        let mut call = self.hub.comments().insert(request, opt.value_of("file-id").unwrap_or(""));
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

    async fn _comments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-deleted", "max-results", "page-token", "updated-min"].iter().map(|v|*v));
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

    async fn _comments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comment-id" => Some(("commentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.type" => Some(("context.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.value" => Some(("context.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "file-id" => Some(("fileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-title" => Some(("fileTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["anchor", "author", "comment-id", "content", "context", "created-date", "deleted", "display-name", "email-address", "file-id", "file-title", "html-content", "is-authenticated-user", "kind", "modified-date", "permission-id", "picture", "self-link", "status", "type", "url", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Comment = json::value::from_value(object).unwrap();
        let mut call = self.hub.comments().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
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

    async fn _comments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comment-id" => Some(("commentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.type" => Some(("context.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.value" => Some(("context.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "file-id" => Some(("fileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-title" => Some(("fileTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["anchor", "author", "comment-id", "content", "context", "created-date", "deleted", "display-name", "email-address", "file-id", "file-title", "html-content", "is-authenticated-user", "kind", "modified-date", "permission-id", "picture", "self-link", "status", "type", "url", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Comment = json::value::from_value(object).unwrap();
        let mut call = self.hub.comments().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
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

    async fn _drives_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.drives().delete(opt.value_of("drive-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "allow-item-deletion" => {
                    call = call.allow_item_deletion(        value.map(|v| arg_from_str(v, err, "allow-item-deletion", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["allow-item-deletion", "use-domain-admin-access"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _drives_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.drives().get(opt.value_of("drive-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _drives_hide(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.drives().hide(opt.value_of("drive-id").unwrap_or(""));
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

    async fn _drives_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "background-image-file.id" => Some(("backgroundImageFile.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-image-file.width" => Some(("backgroundImageFile.width", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.x-coordinate" => Some(("backgroundImageFile.xCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.y-coordinate" => Some(("backgroundImageFile.yCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-link" => Some(("backgroundImageLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission-restriction" => Some(("capabilities.canChangeCopyRequiresWriterPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-domain-users-only-restriction" => Some(("capabilities.canChangeDomainUsersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-drive-background" => Some(("capabilities.canChangeDriveBackground", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-drive-members-only-restriction" => Some(("capabilities.canChangeDriveMembersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-sharing-folders-requires-organizer-permission-restriction" => Some(("capabilities.canChangeSharingFoldersRequiresOrganizerPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-drive" => Some(("capabilities.canDeleteDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-manage-members" => Some(("capabilities.canManageMembers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename-drive" => Some(("capabilities.canRenameDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-reset-drive-restrictions" => Some(("capabilities.canResetDriveRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-rgb" => Some(("colorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hidden" => Some(("hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "org-unit-id" => Some(("orgUnitId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restrictions.admin-managed-restrictions" => Some(("restrictions.adminManagedRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.copy-requires-writer-permission" => Some(("restrictions.copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.domain-users-only" => Some(("restrictions.domainUsersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.drive-members-only" => Some(("restrictions.driveMembersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.sharing-folders-requires-organizer-permission" => Some(("restrictions.sharingFoldersRequiresOrganizerPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "theme-id" => Some(("themeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-drive-background", "can-change-drive-members-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-rename", "can-rename-drive", "can-reset-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-date", "domain-users-only", "drive-members-only", "hidden", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "theme-id", "width", "x-coordinate", "y-coordinate"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Drive = json::value::from_value(object).unwrap();
        let mut call = self.hub.drives().insert(request, opt.value_of("request-id").unwrap_or(""));
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

    async fn _drives_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.drives().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "page-token", "q", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _drives_unhide(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.drives().unhide(opt.value_of("drive-id").unwrap_or(""));
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

    async fn _drives_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "background-image-file.id" => Some(("backgroundImageFile.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-image-file.width" => Some(("backgroundImageFile.width", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.x-coordinate" => Some(("backgroundImageFile.xCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.y-coordinate" => Some(("backgroundImageFile.yCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-link" => Some(("backgroundImageLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission-restriction" => Some(("capabilities.canChangeCopyRequiresWriterPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-domain-users-only-restriction" => Some(("capabilities.canChangeDomainUsersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-drive-background" => Some(("capabilities.canChangeDriveBackground", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-drive-members-only-restriction" => Some(("capabilities.canChangeDriveMembersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-sharing-folders-requires-organizer-permission-restriction" => Some(("capabilities.canChangeSharingFoldersRequiresOrganizerPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-drive" => Some(("capabilities.canDeleteDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-manage-members" => Some(("capabilities.canManageMembers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename-drive" => Some(("capabilities.canRenameDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-reset-drive-restrictions" => Some(("capabilities.canResetDriveRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-rgb" => Some(("colorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hidden" => Some(("hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "org-unit-id" => Some(("orgUnitId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restrictions.admin-managed-restrictions" => Some(("restrictions.adminManagedRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.copy-requires-writer-permission" => Some(("restrictions.copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.domain-users-only" => Some(("restrictions.domainUsersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.drive-members-only" => Some(("restrictions.driveMembersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.sharing-folders-requires-organizer-permission" => Some(("restrictions.sharingFoldersRequiresOrganizerPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "theme-id" => Some(("themeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-drive-background", "can-change-drive-members-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-rename", "can-rename-drive", "can-reset-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-date", "domain-users-only", "drive-members-only", "hidden", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "theme-id", "width", "x-coordinate", "y-coordinate"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Drive = json::value::from_value(object).unwrap();
        let mut call = self.hub.drives().update(request, opt.value_of("drive-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _files_copy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-restricted-download" => Some(("capabilities.canChangeRestrictedDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete" => Some(("capabilities.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content" => Some(("capabilities.canModifyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content-restriction" => Some(("capabilities.canModifyContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-editor-content-restriction" => Some(("capabilities.canModifyEditorContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-labels" => Some(("capabilities.canModifyLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-owner-content-restriction" => Some(("capabilities.canModifyOwnerContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-drive" => Some(("capabilities.canMoveChildrenOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-team-drive" => Some(("capabilities.canMoveChildrenOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-drive" => Some(("capabilities.canMoveChildrenWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-team-drive" => Some(("capabilities.canMoveChildrenWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-into-team-drive" => Some(("capabilities.canMoveItemIntoTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-drive" => Some(("capabilities.canMoveItemOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-team-drive" => Some(("capabilities.canMoveItemOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-drive" => Some(("capabilities.canMoveItemWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-team-drive" => Some(("capabilities.canMoveItemWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-team-drive-item" => Some(("capabilities.canMoveTeamDriveItem", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-drive" => Some(("capabilities.canReadDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-labels" => Some(("capabilities.canReadLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-team-drive" => Some(("capabilities.canReadTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-children" => Some(("capabilities.canRemoveChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-content-restriction" => Some(("capabilities.canRemoveContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-my-drive-parent" => Some(("capabilities.canRemoveMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash" => Some(("capabilities.canTrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-untrash" => Some(("capabilities.canUntrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-augmented-permissions" => Some(("hasAugmentedPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "has-thumbnail" => Some(("hasThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.modified" => Some(("labels.modified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed-date" => Some(("trashedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.is-authenticated-user" => Some(("trashingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.picture.url" => Some(("trashingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.deleted" => Some(("userPermission.deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.expiration-date" => Some(("userPermission.expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.pending-owner" => Some(("userPermission.pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.view" => Some(("userPermission.view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-restricted-download", "can-change-security-update-enabled", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "copy-requires-writer-permission", "copyable", "created-date", "date", "default-open-with-link", "deleted", "description", "display-name", "domain", "download-url", "drive-id", "duration-millis", "editable", "email-address", "embed-link", "etag", "expiration-date", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "link-share-metadata", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "pending-owner", "permission-id", "permission-ids", "photo-link", "picture", "quota-bytes-used", "resource-key", "restricted", "role", "rotation", "security-update-eligible", "security-update-enabled", "self-link", "sensor", "sha1-checksum", "sha256-checksum", "shareable", "shared", "shared-with-me-date", "sharing-user", "shortcut-details", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "text", "thumbnail", "thumbnail-link", "thumbnail-version", "title", "trashed", "trashed-date", "trashing-user", "type", "url", "user-permission", "value", "version", "video-media-metadata", "view", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::File = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().copy(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "pinned" => {
                    call = call.pinned(        value.map(|v| arg_from_str(v, err, "pinned", "boolean")).unwrap_or(false));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(        value.map(|v| arg_from_str(v, err, "ocr", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
                },
                "convert" => {
                    call = call.convert(        value.map(|v| arg_from_str(v, err, "convert", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["convert", "enforce-single-parent", "include-labels", "include-permissions-for-view", "ocr", "ocr-language", "pinned", "supports-all-drives", "supports-team-drives", "timed-text-language", "timed-text-track-name", "visibility"].iter().map(|v|*v));
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

    async fn _files_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().delete(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _files_empty_trash(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().empty_trash();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
                },
                "drive-id" => {
                    call = call.drive_id(value.unwrap_or(""));
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
                                                                           v.extend(["drive-id", "enforce-single-parent"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _files_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.files().export(opt.value_of("file-id").unwrap_or(""), opt.value_of("mime-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
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
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("a string as API currently is inefficient").to_vec();
                    ostream.write_all(&bytes).expect("write to be complete");
                    ostream.flush().expect("io to never fail which should really be fixed one day");
                    }
                    Ok(())
                }
            }
        }
    }

    async fn _files_generate_ids(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().generate_ids();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "space" => {
                    call = call.space(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "space", "type"].iter().map(|v|*v));
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

    async fn _files_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.files().get(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-viewed-date" => {
                    call = call.update_viewed_date(        value.map(|v| arg_from_str(v, err, "update-viewed-date", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "revision-id" => {
                    call = call.revision_id(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "acknowledge-abuse" => {
                    call = call.acknowledge_abuse(        value.map(|v| arg_from_str(v, err, "acknowledge-abuse", "boolean")).unwrap_or(false));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["acknowledge-abuse", "include-labels", "include-permissions-for-view", "projection", "revision-id", "supports-all-drives", "supports-team-drives", "update-viewed-date"].iter().map(|v|*v));
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    } else {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("a string as API currently is inefficient").to_vec();
                    ostream.write_all(&bytes).expect("write to be complete");
                    ostream.flush().expect("io to never fail which should really be fixed one day");
                    }
                    Ok(())
                }
            }
        }
    }

    async fn _files_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-restricted-download" => Some(("capabilities.canChangeRestrictedDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete" => Some(("capabilities.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content" => Some(("capabilities.canModifyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content-restriction" => Some(("capabilities.canModifyContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-editor-content-restriction" => Some(("capabilities.canModifyEditorContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-labels" => Some(("capabilities.canModifyLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-owner-content-restriction" => Some(("capabilities.canModifyOwnerContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-drive" => Some(("capabilities.canMoveChildrenOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-team-drive" => Some(("capabilities.canMoveChildrenOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-drive" => Some(("capabilities.canMoveChildrenWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-team-drive" => Some(("capabilities.canMoveChildrenWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-into-team-drive" => Some(("capabilities.canMoveItemIntoTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-drive" => Some(("capabilities.canMoveItemOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-team-drive" => Some(("capabilities.canMoveItemOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-drive" => Some(("capabilities.canMoveItemWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-team-drive" => Some(("capabilities.canMoveItemWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-team-drive-item" => Some(("capabilities.canMoveTeamDriveItem", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-drive" => Some(("capabilities.canReadDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-labels" => Some(("capabilities.canReadLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-team-drive" => Some(("capabilities.canReadTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-children" => Some(("capabilities.canRemoveChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-content-restriction" => Some(("capabilities.canRemoveContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-my-drive-parent" => Some(("capabilities.canRemoveMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash" => Some(("capabilities.canTrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-untrash" => Some(("capabilities.canUntrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-augmented-permissions" => Some(("hasAugmentedPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "has-thumbnail" => Some(("hasThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.modified" => Some(("labels.modified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed-date" => Some(("trashedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.is-authenticated-user" => Some(("trashingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.picture.url" => Some(("trashingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.deleted" => Some(("userPermission.deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.expiration-date" => Some(("userPermission.expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.pending-owner" => Some(("userPermission.pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.view" => Some(("userPermission.view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-restricted-download", "can-change-security-update-enabled", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "copy-requires-writer-permission", "copyable", "created-date", "date", "default-open-with-link", "deleted", "description", "display-name", "domain", "download-url", "drive-id", "duration-millis", "editable", "email-address", "embed-link", "etag", "expiration-date", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "link-share-metadata", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "pending-owner", "permission-id", "permission-ids", "photo-link", "picture", "quota-bytes-used", "resource-key", "restricted", "role", "rotation", "security-update-eligible", "security-update-enabled", "self-link", "sensor", "sha1-checksum", "sha256-checksum", "shareable", "shared", "shared-with-me-date", "sharing-user", "shortcut-details", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "text", "thumbnail", "thumbnail-link", "thumbnail-version", "title", "trashed", "trashed-date", "trashing-user", "type", "url", "user-permission", "value", "version", "video-media-metadata", "view", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::File = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(        value.map(|v| arg_from_str(v, err, "use-content-as-indexable-text", "boolean")).unwrap_or(false));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "pinned" => {
                    call = call.pinned(        value.map(|v| arg_from_str(v, err, "pinned", "boolean")).unwrap_or(false));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(        value.map(|v| arg_from_str(v, err, "ocr", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
                },
                "convert" => {
                    call = call.convert(        value.map(|v| arg_from_str(v, err, "convert", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["convert", "enforce-single-parent", "include-labels", "include-permissions-for-view", "ocr", "ocr-language", "pinned", "supports-all-drives", "supports-team-drives", "timed-text-language", "timed-text-track-name", "use-content-as-indexable-text", "visibility"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _files_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "team-drive-id" => {
                    call = call.team_drive_id(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-team-drive-items" => {
                    call = call.include_team_drive_items(        value.map(|v| arg_from_str(v, err, "include-team-drive-items", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "include-items-from-all-drives" => {
                    call = call.include_items_from_all_drives(        value.map(|v| arg_from_str(v, err, "include-items-from-all-drives", "boolean")).unwrap_or(false));
                },
                "drive-id" => {
                    call = call.drive_id(value.unwrap_or(""));
                },
                "corpus" => {
                    call = call.corpus(value.unwrap_or(""));
                },
                "corpora" => {
                    call = call.corpora(value.unwrap_or(""));
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
                                                                           v.extend(["corpora", "corpus", "drive-id", "include-items-from-all-drives", "include-labels", "include-permissions-for-view", "include-team-drive-items", "max-results", "order-by", "page-token", "projection", "q", "spaces", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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

    async fn _files_list_labels(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().list_labels(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
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

    async fn _files_modify_labels(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["kind"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ModifyLabelsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().modify_labels(request, opt.value_of("file-id").unwrap_or(""));
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

    async fn _files_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-restricted-download" => Some(("capabilities.canChangeRestrictedDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete" => Some(("capabilities.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content" => Some(("capabilities.canModifyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content-restriction" => Some(("capabilities.canModifyContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-editor-content-restriction" => Some(("capabilities.canModifyEditorContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-labels" => Some(("capabilities.canModifyLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-owner-content-restriction" => Some(("capabilities.canModifyOwnerContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-drive" => Some(("capabilities.canMoveChildrenOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-team-drive" => Some(("capabilities.canMoveChildrenOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-drive" => Some(("capabilities.canMoveChildrenWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-team-drive" => Some(("capabilities.canMoveChildrenWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-into-team-drive" => Some(("capabilities.canMoveItemIntoTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-drive" => Some(("capabilities.canMoveItemOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-team-drive" => Some(("capabilities.canMoveItemOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-drive" => Some(("capabilities.canMoveItemWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-team-drive" => Some(("capabilities.canMoveItemWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-team-drive-item" => Some(("capabilities.canMoveTeamDriveItem", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-drive" => Some(("capabilities.canReadDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-labels" => Some(("capabilities.canReadLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-team-drive" => Some(("capabilities.canReadTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-children" => Some(("capabilities.canRemoveChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-content-restriction" => Some(("capabilities.canRemoveContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-my-drive-parent" => Some(("capabilities.canRemoveMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash" => Some(("capabilities.canTrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-untrash" => Some(("capabilities.canUntrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-augmented-permissions" => Some(("hasAugmentedPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "has-thumbnail" => Some(("hasThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.modified" => Some(("labels.modified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed-date" => Some(("trashedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.is-authenticated-user" => Some(("trashingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.picture.url" => Some(("trashingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.deleted" => Some(("userPermission.deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.expiration-date" => Some(("userPermission.expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.pending-owner" => Some(("userPermission.pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.view" => Some(("userPermission.view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-restricted-download", "can-change-security-update-enabled", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "copy-requires-writer-permission", "copyable", "created-date", "date", "default-open-with-link", "deleted", "description", "display-name", "domain", "download-url", "drive-id", "duration-millis", "editable", "email-address", "embed-link", "etag", "expiration-date", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "link-share-metadata", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "pending-owner", "permission-id", "permission-ids", "photo-link", "picture", "quota-bytes-used", "resource-key", "restricted", "role", "rotation", "security-update-eligible", "security-update-enabled", "self-link", "sensor", "sha1-checksum", "sha256-checksum", "shareable", "shared", "shared-with-me-date", "sharing-user", "shortcut-details", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "text", "thumbnail", "thumbnail-link", "thumbnail-version", "title", "trashed", "trashed-date", "trashing-user", "type", "url", "user-permission", "value", "version", "video-media-metadata", "view", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::File = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().patch(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(        value.map(|v| arg_from_str(v, err, "use-content-as-indexable-text", "boolean")).unwrap_or(false));
                },
                "update-viewed-date" => {
                    call = call.update_viewed_date(        value.map(|v| arg_from_str(v, err, "update-viewed-date", "boolean")).unwrap_or(false));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "set-modified-date" => {
                    call = call.set_modified_date(        value.map(|v| arg_from_str(v, err, "set-modified-date", "boolean")).unwrap_or(false));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(        value.map(|v| arg_from_str(v, err, "pinned", "boolean")).unwrap_or(false));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(        value.map(|v| arg_from_str(v, err, "ocr", "boolean")).unwrap_or(false));
                },
                "new-revision" => {
                    call = call.new_revision(        value.map(|v| arg_from_str(v, err, "new-revision", "boolean")).unwrap_or(false));
                },
                "modified-date-behavior" => {
                    call = call.modified_date_behavior(value.unwrap_or(""));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
                },
                "convert" => {
                    call = call.convert(        value.map(|v| arg_from_str(v, err, "convert", "boolean")).unwrap_or(false));
                },
                "add-parents" => {
                    call = call.add_parents(value.unwrap_or(""));
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
                                                                           v.extend(["add-parents", "convert", "enforce-single-parent", "include-labels", "include-permissions-for-view", "modified-date-behavior", "new-revision", "ocr", "ocr-language", "pinned", "remove-parents", "set-modified-date", "supports-all-drives", "supports-team-drives", "timed-text-language", "timed-text-track-name", "update-viewed-date", "use-content-as-indexable-text"].iter().map(|v|*v));
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

    async fn _files_touch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().touch(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
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
                                                                           v.extend(["include-labels", "include-permissions-for-view", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _files_trash(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().trash(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
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
                                                                           v.extend(["include-labels", "include-permissions-for-view", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _files_untrash(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().untrash(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
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
                                                                           v.extend(["include-labels", "include-permissions-for-view", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _files_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-restricted-download" => Some(("capabilities.canChangeRestrictedDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete" => Some(("capabilities.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content" => Some(("capabilities.canModifyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-content-restriction" => Some(("capabilities.canModifyContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-editor-content-restriction" => Some(("capabilities.canModifyEditorContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-labels" => Some(("capabilities.canModifyLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-modify-owner-content-restriction" => Some(("capabilities.canModifyOwnerContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-drive" => Some(("capabilities.canMoveChildrenOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-out-of-team-drive" => Some(("capabilities.canMoveChildrenOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-drive" => Some(("capabilities.canMoveChildrenWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-children-within-team-drive" => Some(("capabilities.canMoveChildrenWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-into-team-drive" => Some(("capabilities.canMoveItemIntoTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-drive" => Some(("capabilities.canMoveItemOutOfDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-out-of-team-drive" => Some(("capabilities.canMoveItemOutOfTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-drive" => Some(("capabilities.canMoveItemWithinDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-item-within-team-drive" => Some(("capabilities.canMoveItemWithinTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-move-team-drive-item" => Some(("capabilities.canMoveTeamDriveItem", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-drive" => Some(("capabilities.canReadDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-labels" => Some(("capabilities.canReadLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-team-drive" => Some(("capabilities.canReadTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-children" => Some(("capabilities.canRemoveChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-content-restriction" => Some(("capabilities.canRemoveContentRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-my-drive-parent" => Some(("capabilities.canRemoveMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash" => Some(("capabilities.canTrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-untrash" => Some(("capabilities.canUntrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-augmented-permissions" => Some(("hasAugmentedPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "has-thumbnail" => Some(("hasThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.modified" => Some(("labels.modified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed-date" => Some(("trashedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.is-authenticated-user" => Some(("trashingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.picture.url" => Some(("trashingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.deleted" => Some(("userPermission.deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.expiration-date" => Some(("userPermission.expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.pending-owner" => Some(("userPermission.pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.view" => Some(("userPermission.view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-restricted-download", "can-change-security-update-enabled", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "copy-requires-writer-permission", "copyable", "created-date", "date", "default-open-with-link", "deleted", "description", "display-name", "domain", "download-url", "drive-id", "duration-millis", "editable", "email-address", "embed-link", "etag", "expiration-date", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "link-share-metadata", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "pending-owner", "permission-id", "permission-ids", "photo-link", "picture", "quota-bytes-used", "resource-key", "restricted", "role", "rotation", "security-update-eligible", "security-update-enabled", "self-link", "sensor", "sha1-checksum", "sha256-checksum", "shareable", "shared", "shared-with-me-date", "sharing-user", "shortcut-details", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "text", "thumbnail", "thumbnail-link", "thumbnail-version", "title", "trashed", "trashed-date", "trashing-user", "type", "url", "user-permission", "value", "version", "video-media-metadata", "view", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::File = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().update(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(        value.map(|v| arg_from_str(v, err, "use-content-as-indexable-text", "boolean")).unwrap_or(false));
                },
                "update-viewed-date" => {
                    call = call.update_viewed_date(        value.map(|v| arg_from_str(v, err, "update-viewed-date", "boolean")).unwrap_or(false));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "set-modified-date" => {
                    call = call.set_modified_date(        value.map(|v| arg_from_str(v, err, "set-modified-date", "boolean")).unwrap_or(false));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(        value.map(|v| arg_from_str(v, err, "pinned", "boolean")).unwrap_or(false));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(        value.map(|v| arg_from_str(v, err, "ocr", "boolean")).unwrap_or(false));
                },
                "new-revision" => {
                    call = call.new_revision(        value.map(|v| arg_from_str(v, err, "new-revision", "boolean")).unwrap_or(false));
                },
                "modified-date-behavior" => {
                    call = call.modified_date_behavior(value.unwrap_or(""));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
                },
                "convert" => {
                    call = call.convert(        value.map(|v| arg_from_str(v, err, "convert", "boolean")).unwrap_or(false));
                },
                "add-parents" => {
                    call = call.add_parents(value.unwrap_or(""));
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
                                                                           v.extend(["add-parents", "convert", "enforce-single-parent", "include-labels", "include-permissions-for-view", "modified-date-behavior", "new-revision", "ocr", "ocr-language", "pinned", "remove-parents", "set-modified-date", "supports-all-drives", "supports-team-drives", "timed-text-language", "timed-text-track-name", "update-viewed-date", "use-content-as-indexable-text"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _files_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().watch(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-viewed-date" => {
                    call = call.update_viewed_date(        value.map(|v| arg_from_str(v, err, "update-viewed-date", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "revision-id" => {
                    call = call.revision_id(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "acknowledge-abuse" => {
                    call = call.acknowledge_abuse(        value.map(|v| arg_from_str(v, err, "acknowledge-abuse", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["acknowledge-abuse", "include-labels", "include-permissions-for-view", "projection", "revision-id", "supports-all-drives", "supports-team-drives", "update-viewed-date"].iter().map(|v|*v));
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

    async fn _parents_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("parent-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _parents_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("parent-id").unwrap_or(""));
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

    async fn _parents_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-root" => Some(("isRoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link" => Some(("parentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "is-root", "kind", "parent-link", "self-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ParentReference = json::value::from_value(object).unwrap();
        let mut call = self.hub.parents().insert(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _parents_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().list(opt.value_of("file-id").unwrap_or(""));
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

    async fn _permissions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["supports-all-drives", "supports-team-drives", "use-domain-admin-access"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _permissions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["supports-all-drives", "supports-team-drives", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _permissions_get_id_for_email(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().get_id_for_email(opt.value_of("email").unwrap_or(""));
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

    async fn _permissions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "additional-roles" => Some(("additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "auth-key" => Some(("authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-date" => Some(("expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-owner" => Some(("pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view" => Some(("view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "with-link" => Some(("withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "auth-key", "deleted", "domain", "email-address", "etag", "expiration-date", "id", "kind", "name", "pending-owner", "photo-link", "role", "self-link", "type", "value", "view", "with-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Permission = json::value::from_value(object).unwrap();
        let mut call = self.hub.permissions().insert(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "send-notification-emails" => {
                    call = call.send_notification_emails(        value.map(|v| arg_from_str(v, err, "send-notification-emails", "boolean")).unwrap_or(false));
                },
                "move-to-new-owners-root" => {
                    call = call.move_to_new_owners_root(        value.map(|v| arg_from_str(v, err, "move-to-new-owners-root", "boolean")).unwrap_or(false));
                },
                "enforce-single-parent" => {
                    call = call.enforce_single_parent(        value.map(|v| arg_from_str(v, err, "enforce-single-parent", "boolean")).unwrap_or(false));
                },
                "email-message" => {
                    call = call.email_message(value.unwrap_or(""));
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
                                                                           v.extend(["email-message", "enforce-single-parent", "move-to-new-owners-root", "send-notification-emails", "supports-all-drives", "supports-team-drives", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _permissions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
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
                                                                           v.extend(["include-permissions-for-view", "max-results", "page-token", "supports-all-drives", "supports-team-drives", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _permissions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "additional-roles" => Some(("additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "auth-key" => Some(("authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-date" => Some(("expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-owner" => Some(("pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view" => Some(("view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "with-link" => Some(("withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "auth-key", "deleted", "domain", "email-address", "etag", "expiration-date", "id", "kind", "name", "pending-owner", "photo-link", "role", "self-link", "type", "value", "view", "with-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Permission = json::value::from_value(object).unwrap();
        let mut call = self.hub.permissions().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "transfer-ownership" => {
                    call = call.transfer_ownership(        value.map(|v| arg_from_str(v, err, "transfer-ownership", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "remove-expiration" => {
                    call = call.remove_expiration(        value.map(|v| arg_from_str(v, err, "remove-expiration", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["remove-expiration", "supports-all-drives", "supports-team-drives", "transfer-ownership", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _permissions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "additional-roles" => Some(("additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "auth-key" => Some(("authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-date" => Some(("expirationDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-owner" => Some(("pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view" => Some(("view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "with-link" => Some(("withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "auth-key", "deleted", "domain", "email-address", "etag", "expiration-date", "id", "kind", "name", "pending-owner", "photo-link", "role", "self-link", "type", "value", "view", "with-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Permission = json::value::from_value(object).unwrap();
        let mut call = self.hub.permissions().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "transfer-ownership" => {
                    call = call.transfer_ownership(        value.map(|v| arg_from_str(v, err, "transfer-ownership", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "remove-expiration" => {
                    call = call.remove_expiration(        value.map(|v| arg_from_str(v, err, "remove-expiration", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["remove-expiration", "supports-all-drives", "supports-team-drives", "transfer-ownership", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _properties_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
                                                                           v.extend(["visibility"].iter().map(|v|*v));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _properties_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
                                                                           v.extend(["visibility"].iter().map(|v|*v));
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

    async fn _properties_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "key", "kind", "self-link", "value", "visibility"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Property = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().insert(request, opt.value_of("file-id").unwrap_or(""));
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

    async fn _properties_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().list(opt.value_of("file-id").unwrap_or(""));
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

    async fn _properties_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "key", "kind", "self-link", "value", "visibility"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Property = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
                                                                           v.extend(["visibility"].iter().map(|v|*v));
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

    async fn _properties_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "key", "kind", "self-link", "value", "visibility"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Property = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
                                                                           v.extend(["visibility"].iter().map(|v|*v));
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

    async fn _replies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _replies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-deleted"].iter().map(|v|*v));
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

    async fn _replies_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-id" => Some(("replyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verb" => Some(("verb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "content", "created-date", "deleted", "display-name", "email-address", "html-content", "is-authenticated-user", "kind", "modified-date", "permission-id", "picture", "reply-id", "url", "verb"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommentReply = json::value::from_value(object).unwrap();
        let mut call = self.hub.replies().insert(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
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

    async fn _replies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().list(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-deleted", "max-results", "page-token"].iter().map(|v|*v));
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

    async fn _replies_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-id" => Some(("replyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verb" => Some(("verb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "content", "created-date", "deleted", "display-name", "email-address", "html-content", "is-authenticated-user", "kind", "modified-date", "permission-id", "picture", "reply-id", "url", "verb"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommentReply = json::value::from_value(object).unwrap();
        let mut call = self.hub.replies().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
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

    async fn _replies_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-id" => Some(("replyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verb" => Some(("verb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "content", "created-date", "deleted", "display-name", "email-address", "html-content", "is-authenticated-user", "kind", "modified-date", "permission-id", "picture", "reply-id", "url", "verb"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommentReply = json::value::from_value(object).unwrap();
        let mut call = self.hub.replies().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
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

    async fn _revisions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _revisions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
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

    async fn _revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
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

    async fn _revisions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pinned" => Some(("pinned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "publish-auto" => Some(("publishAuto", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published" => Some(("published", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published-link" => Some(("publishedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published-outside-domain" => Some(("publishedOutsideDomain", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "download-url", "email-address", "etag", "export-links", "file-size", "id", "is-authenticated-user", "kind", "last-modifying-user", "last-modifying-user-name", "md5-checksum", "mime-type", "modified-date", "original-filename", "permission-id", "picture", "pinned", "publish-auto", "published", "published-link", "published-outside-domain", "self-link", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Revision = json::value::from_value(object).unwrap();
        let mut call = self.hub.revisions().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
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

    async fn _revisions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pinned" => Some(("pinned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "publish-auto" => Some(("publishAuto", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published" => Some(("published", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published-link" => Some(("publishedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published-outside-domain" => Some(("publishedOutsideDomain", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "download-url", "email-address", "etag", "export-links", "file-size", "id", "is-authenticated-user", "kind", "last-modifying-user", "last-modifying-user-name", "md5-checksum", "mime-type", "modified-date", "original-filename", "permission-id", "picture", "pinned", "publish-auto", "published", "published-link", "published-outside-domain", "self-link", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Revision = json::value::from_value(object).unwrap();
        let mut call = self.hub.revisions().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
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

    async fn _teamdrives_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.teamdrives().delete(opt.value_of("team-drive-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _teamdrives_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.teamdrives().get(opt.value_of("team-drive-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _teamdrives_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "background-image-file.id" => Some(("backgroundImageFile.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-image-file.width" => Some(("backgroundImageFile.width", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.x-coordinate" => Some(("backgroundImageFile.xCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.y-coordinate" => Some(("backgroundImageFile.yCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-link" => Some(("backgroundImageLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission-restriction" => Some(("capabilities.canChangeCopyRequiresWriterPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-domain-users-only-restriction" => Some(("capabilities.canChangeDomainUsersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-sharing-folders-requires-organizer-permission-restriction" => Some(("capabilities.canChangeSharingFoldersRequiresOrganizerPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-team-drive-background" => Some(("capabilities.canChangeTeamDriveBackground", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-team-members-only-restriction" => Some(("capabilities.canChangeTeamMembersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-team-drive" => Some(("capabilities.canDeleteTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-manage-members" => Some(("capabilities.canManageMembers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-children" => Some(("capabilities.canRemoveChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename-team-drive" => Some(("capabilities.canRenameTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-reset-team-drive-restrictions" => Some(("capabilities.canResetTeamDriveRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-rgb" => Some(("colorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "org-unit-id" => Some(("orgUnitId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restrictions.admin-managed-restrictions" => Some(("restrictions.adminManagedRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.copy-requires-writer-permission" => Some(("restrictions.copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.domain-users-only" => Some(("restrictions.domainUsersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.sharing-folders-requires-organizer-permission" => Some(("restrictions.sharingFoldersRequiresOrganizerPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.team-members-only" => Some(("restrictions.teamMembersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "theme-id" => Some(("themeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-change-team-drive-background", "can-change-team-members-only-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-team-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-remove-children", "can-rename", "can-rename-team-drive", "can-reset-team-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-date", "domain-users-only", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "team-members-only", "theme-id", "width", "x-coordinate", "y-coordinate"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TeamDrive = json::value::from_value(object).unwrap();
        let mut call = self.hub.teamdrives().insert(request, opt.value_of("request-id").unwrap_or(""));
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

    async fn _teamdrives_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.teamdrives().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "page-token", "q", "use-domain-admin-access"].iter().map(|v|*v));
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

    async fn _teamdrives_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "background-image-file.id" => Some(("backgroundImageFile.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-image-file.width" => Some(("backgroundImageFile.width", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.x-coordinate" => Some(("backgroundImageFile.xCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-file.y-coordinate" => Some(("backgroundImageFile.yCoordinate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "background-image-link" => Some(("backgroundImageLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission-restriction" => Some(("capabilities.canChangeCopyRequiresWriterPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-domain-users-only-restriction" => Some(("capabilities.canChangeDomainUsersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-sharing-folders-requires-organizer-permission-restriction" => Some(("capabilities.canChangeSharingFoldersRequiresOrganizerPermissionRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-team-drive-background" => Some(("capabilities.canChangeTeamDriveBackground", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-team-members-only-restriction" => Some(("capabilities.canChangeTeamMembersOnlyRestriction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-children" => Some(("capabilities.canDeleteChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-delete-team-drive" => Some(("capabilities.canDeleteTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-download" => Some(("capabilities.canDownload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-list-children" => Some(("capabilities.canListChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-manage-members" => Some(("capabilities.canManageMembers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-remove-children" => Some(("capabilities.canRemoveChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename" => Some(("capabilities.canRename", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-rename-team-drive" => Some(("capabilities.canRenameTeamDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-reset-team-drive-restrictions" => Some(("capabilities.canResetTeamDriveRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-trash-children" => Some(("capabilities.canTrashChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-rgb" => Some(("colorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "org-unit-id" => Some(("orgUnitId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restrictions.admin-managed-restrictions" => Some(("restrictions.adminManagedRestrictions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.copy-requires-writer-permission" => Some(("restrictions.copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.domain-users-only" => Some(("restrictions.domainUsersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.sharing-folders-requires-organizer-permission" => Some(("restrictions.sharingFoldersRequiresOrganizerPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restrictions.team-members-only" => Some(("restrictions.teamMembersOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "theme-id" => Some(("themeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-change-team-drive-background", "can-change-team-members-only-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-team-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-remove-children", "can-rename", "can-rename-team-drive", "can-reset-team-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-date", "domain-users-only", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "team-members-only", "theme-id", "width", "x-coordinate", "y-coordinate"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TeamDrive = json::value::from_value(object).unwrap();
        let mut call = self.hub.teamdrives().update(request, opt.value_of("team-drive-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-domain-admin-access" => {
                    call = call.use_domain_admin_access(        value.map(|v| arg_from_str(v, err, "use-domain-admin-access", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["use-domain-admin-access"].iter().map(|v|*v));
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
            ("about", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._about_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("about".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("apps", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._apps_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._apps_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("apps".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("changes", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._changes_get(opt, dry_run, &mut err).await;
                    },
                    ("get-start-page-token", Some(opt)) => {
                        call_result = self._changes_get_start_page_token(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._changes_list(opt, dry_run, &mut err).await;
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._changes_watch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("changes".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("channels", Some(opt)) => {
                match opt.subcommand() {
                    ("stop", Some(opt)) => {
                        call_result = self._channels_stop(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("channels".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("children", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._children_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._children_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._children_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._children_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("children".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("comments", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._comments_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._comments_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._comments_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._comments_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._comments_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("comments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("drives", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._drives_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._drives_get(opt, dry_run, &mut err).await;
                    },
                    ("hide", Some(opt)) => {
                        call_result = self._drives_hide(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._drives_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._drives_list(opt, dry_run, &mut err).await;
                    },
                    ("unhide", Some(opt)) => {
                        call_result = self._drives_unhide(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._drives_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("drives".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("files", Some(opt)) => {
                match opt.subcommand() {
                    ("copy", Some(opt)) => {
                        call_result = self._files_copy(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._files_delete(opt, dry_run, &mut err).await;
                    },
                    ("empty-trash", Some(opt)) => {
                        call_result = self._files_empty_trash(opt, dry_run, &mut err).await;
                    },
                    ("export", Some(opt)) => {
                        call_result = self._files_export(opt, dry_run, &mut err).await;
                    },
                    ("generate-ids", Some(opt)) => {
                        call_result = self._files_generate_ids(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._files_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._files_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._files_list(opt, dry_run, &mut err).await;
                    },
                    ("list-labels", Some(opt)) => {
                        call_result = self._files_list_labels(opt, dry_run, &mut err).await;
                    },
                    ("modify-labels", Some(opt)) => {
                        call_result = self._files_modify_labels(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._files_patch(opt, dry_run, &mut err).await;
                    },
                    ("touch", Some(opt)) => {
                        call_result = self._files_touch(opt, dry_run, &mut err).await;
                    },
                    ("trash", Some(opt)) => {
                        call_result = self._files_trash(opt, dry_run, &mut err).await;
                    },
                    ("untrash", Some(opt)) => {
                        call_result = self._files_untrash(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._files_update(opt, dry_run, &mut err).await;
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._files_watch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("files".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("parents", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._parents_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._parents_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._parents_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._parents_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("parents".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("permissions", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._permissions_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._permissions_get(opt, dry_run, &mut err).await;
                    },
                    ("get-id-for-email", Some(opt)) => {
                        call_result = self._permissions_get_id_for_email(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._permissions_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._permissions_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._permissions_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._permissions_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("permissions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("properties", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._properties_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._properties_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._properties_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._properties_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._properties_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._properties_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("properties".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("replies", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._replies_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._replies_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._replies_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._replies_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._replies_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._replies_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("replies".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("revisions", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._revisions_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._revisions_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._revisions_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._revisions_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._revisions_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("revisions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("teamdrives", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._teamdrives_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._teamdrives_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._teamdrives_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._teamdrives_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._teamdrives_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("teamdrives".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "drive2-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"De0ub0IbWruJbBXUyseFYvZ-\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"276875258587-5gbp23a7aqnrl6p06c0jt5fskuktactq.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/drive2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::DriveHub::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("about", "methods: 'get'", vec![
            ("get",
                    Some(r##"Gets the information about the current user along with Drive API settings"##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/about_get",
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
        
        ("apps", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Gets a specific app."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/apps_get",
                  vec![
                    (Some(r##"app-id"##),
                     None,
                     Some(r##"The ID of the app."##),
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
                    Some(r##"Lists a user's installed apps."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/apps_list",
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
        
        ("changes", "methods: 'get', 'get-start-page-token', 'list' and 'watch'", vec![
            ("get",
                    Some(r##"Deprecated: Use `changes.getStartPageToken` and `changes.list` to retrieve recent changes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/changes_get",
                  vec![
                    (Some(r##"change-id"##),
                     None,
                     Some(r##"The ID of the change."##),
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
            ("get-start-page-token",
                    Some(r##"Gets the starting pageToken for listing future changes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/changes_get-start-page-token",
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
            ("list",
                    Some(r##"Lists the changes for a user or shared drive."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/changes_list",
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
            ("watch",
                    Some(r##"Subscribe to changes for a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/changes_watch",
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
            ]),
        
        ("channels", "methods: 'stop'", vec![
            ("stop",
                    Some(r##"Stops watching resources through this channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/channels_stop",
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
                  ]),
            ]),
        
        ("children", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Removes a child from a folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/children_delete",
                  vec![
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The ID of the folder."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"child-id"##),
                     None,
                     Some(r##"The ID of the child."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a specific child reference."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/children_get",
                  vec![
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The ID of the folder."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"child-id"##),
                     None,
                     Some(r##"The ID of the child."##),
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
            ("insert",
                    Some(r##"Inserts a file into a folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/children_insert",
                  vec![
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The ID of the folder."##),
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
            ("list",
                    Some(r##"Lists a folder's children."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/children_list",
                  vec![
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The ID of the folder."##),
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
        
        ("comments", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/comments_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a comment by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/comments_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
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
            ("insert",
                    Some(r##"Creates a new comment on the given file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/comments_insert",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
            ("list",
                    Some(r##"Lists a file's comments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/comments_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
                    Some(r##"Updates an existing comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/comments_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
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
                    Some(r##"Updates an existing comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/comments_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
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
        
        ("drives", "methods: 'delete', 'get', 'hide', 'insert', 'list', 'unhide' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes a shared drive for which the user is an `organizer`. The shared drive cannot contain any untrashed items."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_delete",
                  vec![
                    (Some(r##"drive-id"##),
                     None,
                     Some(r##"The ID of the shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a shared drive's metadata by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_get",
                  vec![
                    (Some(r##"drive-id"##),
                     None,
                     Some(r##"The ID of the shared drive."##),
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
            ("hide",
                    Some(r##"Hides a shared drive from the default view."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_hide",
                  vec![
                    (Some(r##"drive-id"##),
                     None,
                     Some(r##"The ID of the shared drive."##),
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
            ("insert",
                    Some(r##"Creates a new shared drive."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_insert",
                  vec![
                    (Some(r##"request-id"##),
                     None,
                     Some(r##"Required. An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned."##),
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
            ("list",
                    Some(r##" Lists the user's shared drives. This method accepts the `q` parameter, which is a search query combining one or more search terms. For more information, see the [Search for shared drives](/drive/api/guides/search-shareddrives) guide."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_list",
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
            ("unhide",
                    Some(r##"Restores a shared drive to the default view."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_unhide",
                  vec![
                    (Some(r##"drive-id"##),
                     None,
                     Some(r##"The ID of the shared drive."##),
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
            ("update",
                    Some(r##"Updates the metadata for a shared drive."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/drives_update",
                  vec![
                    (Some(r##"drive-id"##),
                     None,
                     Some(r##"The ID of the shared drive."##),
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
        
        ("files", "methods: 'copy', 'delete', 'empty-trash', 'export', 'generate-ids', 'get', 'insert', 'list', 'list-labels', 'modify-labels', 'patch', 'touch', 'trash', 'untrash', 'update' and 'watch'", vec![
            ("copy",
                    Some(r##"Creates a copy of the specified file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_copy",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to copy."##),
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
            ("delete",
                    Some(r##"Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive, the user must be an `organizer` on the parent folder. If the target is a folder, all descendants owned by the user are also deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("empty-trash",
                    Some(r##"Permanently deletes all of the user's trashed files."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_empty-trash",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("export",
                    Some(r##"Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_export",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mime-type"##),
                     None,
                     Some(r##"Required. The MIME type of the format requested for this export."##),
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
            ("generate-ids",
                    Some(r##"Generates a set of file IDs which can be provided in insert or copy requests."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_generate-ids",
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
            ("get",
                    Some(r##" Gets a file's metadata or content by ID. If you provide the URL parameter `alt=media`, then the response includes the file contents in the response body. Downloading content with `alt=media` only works if the file is stored in Drive. To download Google Docs, Sheets, and Slides use [`files.export`](/drive/api/reference/rest/v2/files/export) instead. For more information, see [Download & export files](/drive/api/guides/manage-downloads)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file in question."##),
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
            ("insert",
                    Some(r##" Inserts a new file. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:*`*/*` Note: Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information on uploading files, see [Upload file data](/drive/api/guides/manage-uploads). Apps creating shortcuts with `files.insert` must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `title` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"title": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `title` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the title. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
            ("list",
                    Some(r##" Lists the user's files. This method accepts the `q` parameter, which is a search query combining one or more search terms. For more information, see the [Search for files & folders](/drive/api/guides/search-files) guide. *Note:* This method returns *all* files by default, including trashed files. If you don't want trashed files to appear in the list, use the `trashed=false` query parameter to remove trashed files from the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_list",
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
            ("list-labels",
                    Some(r##"Lists the labels on a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_list-labels",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
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
            ("modify-labels",
                    Some(r##"Modifies the set of labels applied to a file. Returns a list of the labels that were added or modified."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_modify-labels",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to which the labels belong."##),
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
            ("patch",
                    Some(r##"Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might change automatically, such as modifiedDate. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to update."##),
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
            ("touch",
                    Some(r##"Set the file's updated time to the current server time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_touch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to update."##),
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
            ("trash",
                    Some(r##"Moves a file to the trash. The currently authenticated user must own the file or be at least a `fileOrganizer` on the parent for shared drive files."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_trash",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to trash."##),
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
            ("untrash",
                    Some(r##"Restores a file from the trash. The currently authenticated user must own the file or be at least a `fileOrganizer` on the parent for shared drive files."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_untrash",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to untrash."##),
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
            ("update",
                    Some(r##" Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might be changed automatically, such as `modifiedDate`. This method supports patch semantics. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:*`*/*` Note: Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information on uploading files, see [Upload file data](/drive/api/guides/manage-uploads)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
            ("watch",
                    Some(r##"Subscribes to changes to a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_watch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file in question."##),
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
        
        ("parents", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Removes a parent from a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/parents_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"parent-id"##),
                     None,
                     Some(r##"The ID of the parent."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a specific parent reference."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/parents_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"parent-id"##),
                     None,
                     Some(r##"The ID of the parent."##),
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
            ("insert",
                    Some(r##"Adds a parent folder for a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/parents_insert",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
            ("list",
                    Some(r##"Lists a file's parents."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/parents_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
        
        ("permissions", "methods: 'delete', 'get', 'get-id-for-email', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a permission from a file or shared drive. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file or shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID for the permission."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a permission by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file or shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID for the permission."##),
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
            ("get-id-for-email",
                    Some(r##"Returns the permission ID for an email address."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_get-id-for-email",
                  vec![
                    (Some(r##"email"##),
                     None,
                     Some(r##"The email address for which to return a permission ID"##),
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
            ("insert",
                    Some(r##"Inserts a permission for a file or shared drive. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_insert",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file or shared drive."##),
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
            ("list",
                    Some(r##"Lists a file's or shared drive's permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file or shared drive."##),
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
                    Some(r##"Updates a permission using patch semantics. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file or shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID for the permission."##),
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
                    Some(r##"Updates a permission. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file or shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID for the permission."##),
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
        
        ("properties", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/properties_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"property-key"##),
                     None,
                     Some(r##"The key of the property."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a property by its key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/properties_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"property-key"##),
                     None,
                     Some(r##"The key of the property."##),
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
            ("insert",
                    Some(r##"Adds a property to a file, or updates it if it already exists."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/properties_insert",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
            ("list",
                    Some(r##"Lists a file's properties."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/properties_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
                    Some(r##"Updates a property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/properties_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"property-key"##),
                     None,
                     Some(r##"The key of the property."##),
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
                    Some(r##"Updates a property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/properties_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"property-key"##),
                     None,
                     Some(r##"The key of the property."##),
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
        
        ("replies", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a reply."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/replies_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"reply-id"##),
                     None,
                     Some(r##"The ID of the reply."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a reply."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/replies_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"reply-id"##),
                     None,
                     Some(r##"The ID of the reply."##),
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
            ("insert",
                    Some(r##"Creates a new reply to the given comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/replies_insert",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
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
            ("list",
                    Some(r##"Lists all of the replies to a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/replies_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
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
                    Some(r##"Updates an existing reply."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/replies_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"reply-id"##),
                     None,
                     Some(r##"The ID of the reply."##),
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
                    Some(r##"Updates an existing reply."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/replies_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"reply-id"##),
                     None,
                     Some(r##"The ID of the reply."##),
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
        
        ("revisions", "methods: 'delete', 'get', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes a file version. You can only delete revisions for files with binary content, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/revisions_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-id"##),
                     None,
                     Some(r##"The ID of the revision."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets a specific revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/revisions_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-id"##),
                     None,
                     Some(r##"The ID of the revision."##),
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
                    Some(r##"Lists a file's revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/revisions_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
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
                    Some(r##"Updates a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/revisions_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-id"##),
                     None,
                     Some(r##"The ID for the revision."##),
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
                    Some(r##"Updates a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/revisions_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-id"##),
                     None,
                     Some(r##"The ID for the revision."##),
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
        
        ("teamdrives", "methods: 'delete', 'get', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deprecated: Use `drives.delete` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/teamdrives_delete",
                  vec![
                    (Some(r##"team-drive-id"##),
                     None,
                     Some(r##"The ID of the Team Drive"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Deprecated: Use `drives.get` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/teamdrives_get",
                  vec![
                    (Some(r##"team-drive-id"##),
                     None,
                     Some(r##"The ID of the Team Drive"##),
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
            ("insert",
                    Some(r##"Deprecated: Use `drives.insert` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/teamdrives_insert",
                  vec![
                    (Some(r##"request-id"##),
                     None,
                     Some(r##"Required. An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned."##),
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
            ("list",
                    Some(r##"Deprecated: Use `drives.list` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/teamdrives_list",
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
            ("update",
                    Some(r##"Deprecated: Use `drives.update` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/teamdrives_update",
                  vec![
                    (Some(r##"team-drive-id"##),
                     None,
                     Some(r##"The ID of the Team Drive"##),
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
    
    let mut app = App::new("drive2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240618")
           .about("The Google Drive API allows clients to access resources from Google Drive.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_drive2_cli")
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
                       if arg_name_str == "mode" {
                           arg = arg.number_of_values(2);
                           arg = arg.value_names(&upload_value_names);
           
                           scmd = scmd.arg(Arg::with_name("mime")
                                               .short("m")
                                               .requires("mode")
                                               .required(false)
                                               .help("The file's mime time, like 'application/octet-stream'")
                                               .takes_value(true));
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
