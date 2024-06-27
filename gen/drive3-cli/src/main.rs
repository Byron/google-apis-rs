// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_drive3::{api, Error, oauth2, client::chrono, FieldMask};


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
        let mut call = self.hub.changes().list(opt.value_of("page-token").unwrap_or(""));
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
                "restrict-to-my-drive" => {
                    call = call.restrict_to_my_drive(        value.map(|v| arg_from_str(v, err, "restrict-to-my-drive", "boolean")).unwrap_or(false));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "include-team-drive-items" => {
                    call = call.include_team_drive_items(        value.map(|v| arg_from_str(v, err, "include-team-drive-items", "boolean")).unwrap_or(false));
                },
                "include-removed" => {
                    call = call.include_removed(        value.map(|v| arg_from_str(v, err, "include-removed", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["drive-id", "include-corpus-removals", "include-items-from-all-drives", "include-labels", "include-permissions-for-view", "include-removed", "include-team-drive-items", "page-size", "restrict-to-my-drive", "spaces", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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
        let mut call = self.hub.changes().watch(request, opt.value_of("page-token").unwrap_or(""));
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
                "restrict-to-my-drive" => {
                    call = call.restrict_to_my_drive(        value.map(|v| arg_from_str(v, err, "restrict-to-my-drive", "boolean")).unwrap_or(false));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "include-team-drive-items" => {
                    call = call.include_team_drive_items(        value.map(|v| arg_from_str(v, err, "include-team-drive-items", "boolean")).unwrap_or(false));
                },
                "include-removed" => {
                    call = call.include_removed(        value.map(|v| arg_from_str(v, err, "include-removed", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["drive-id", "include-corpus-removals", "include-items-from-all-drives", "include-labels", "include-permissions-for-view", "include-removed", "include-team-drive-items", "page-size", "restrict-to-my-drive", "spaces", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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

    async fn _comments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quoted-file-content.mime-type" => Some(("quotedFileContent.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quoted-file-content.value" => Some(("quotedFileContent.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resolved" => Some(("resolved", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["anchor", "author", "content", "created-time", "deleted", "display-name", "email-address", "html-content", "id", "kind", "me", "mime-type", "modified-time", "permission-id", "photo-link", "quoted-file-content", "resolved", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Comment = json::value::from_value(object).unwrap();
        let mut call = self.hub.comments().create(request, opt.value_of("file-id").unwrap_or(""));
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

    async fn _comments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-modified-time" => {
                    call = call.start_modified_time(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["include-deleted", "page-size", "page-token", "start-modified-time"].iter().map(|v|*v));
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
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quoted-file-content.mime-type" => Some(("quotedFileContent.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quoted-file-content.value" => Some(("quotedFileContent.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resolved" => Some(("resolved", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["anchor", "author", "content", "created-time", "deleted", "display-name", "email-address", "html-content", "id", "kind", "me", "mime-type", "modified-time", "permission-id", "photo-link", "quoted-file-content", "resolved", "value"]);
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

    async fn _drives_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-drive-background", "can-change-drive-members-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-rename", "can-rename-drive", "can-reset-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-time", "domain-users-only", "drive-members-only", "hidden", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "theme-id", "width", "x-coordinate", "y-coordinate"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Drive = json::value::from_value(object).unwrap();
        let mut call = self.hub.drives().create(request, opt.value_of("request-id").unwrap_or(""));
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
                                                                           v.extend(["page-size", "page-token", "q", "use-domain-admin-access"].iter().map(|v|*v));
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
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-drive-background", "can-change-drive-members-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-rename", "can-rename-drive", "can-reset-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-time", "domain-users-only", "drive-members-only", "hidden", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "theme-id", "width", "x-coordinate", "y-coordinate"]);
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
                    "app-properties" => Some(("appProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-viewers-can-copy-content" => Some(("capabilities.canChangeViewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "content-hints.indexable-text" => Some(("contentHints.indexableText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.image" => Some(("contentHints.thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.mime-type" => Some(("contentHints.thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "image-media-metadata.time" => Some(("imageMediaMetadata.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me" => Some(("modifiedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "modified-by-me-time" => Some(("modifiedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parents" => Some(("parents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "properties" => Some(("properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-time" => Some(("sharedWithMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.me" => Some(("sharingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.photo-link" => Some(("sharingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed" => Some(("trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashed-time" => Some(("trashedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.me" => Some(("trashingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.photo-link" => Some(("trashingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "viewed-by-me" => Some(("viewedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "viewed-by-me-time" => Some(("viewedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewers-can-copy-content" => Some(("viewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["altitude", "aperture", "app-properties", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-security-update-enabled", "can-change-viewers-can-copy-content", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "content-hints", "copy-requires-writer-permission", "created-time", "description", "display-name", "drive-id", "duration-millis", "email-address", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "iso-speed", "kind", "last-modifying-user", "latitude", "lens", "link-share-metadata", "location", "longitude", "max-aperture-value", "md5-checksum", "me", "metering-mode", "mime-type", "modified-by-me", "modified-by-me-time", "modified-time", "name", "original-filename", "owned-by-me", "parents", "permission-id", "permission-ids", "photo-link", "properties", "quota-bytes-used", "resource-key", "rotation", "security-update-eligible", "security-update-enabled", "sensor", "sha1-checksum", "sha256-checksum", "shared", "shared-with-me-time", "sharing-user", "shortcut-details", "size", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "thumbnail", "thumbnail-link", "thumbnail-version", "time", "trashed", "trashed-time", "trashing-user", "version", "video-media-metadata", "viewed-by-me", "viewed-by-me-time", "viewers-can-copy-content", "web-content-link", "web-view-link", "white-balance", "width", "writers-can-share"]);
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
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "keep-revision-forever" => {
                    call = call.keep_revision_forever(        value.map(|v| arg_from_str(v, err, "keep-revision-forever", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "ignore-default-visibility" => {
                    call = call.ignore_default_visibility(        value.map(|v| arg_from_str(v, err, "ignore-default-visibility", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent", "ignore-default-visibility", "include-labels", "include-permissions-for-view", "keep-revision-forever", "ocr-language", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _files_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-properties" => Some(("appProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-viewers-can-copy-content" => Some(("capabilities.canChangeViewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "content-hints.indexable-text" => Some(("contentHints.indexableText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.image" => Some(("contentHints.thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.mime-type" => Some(("contentHints.thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "image-media-metadata.time" => Some(("imageMediaMetadata.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me" => Some(("modifiedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "modified-by-me-time" => Some(("modifiedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parents" => Some(("parents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "properties" => Some(("properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-time" => Some(("sharedWithMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.me" => Some(("sharingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.photo-link" => Some(("sharingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed" => Some(("trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashed-time" => Some(("trashedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.me" => Some(("trashingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.photo-link" => Some(("trashingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "viewed-by-me" => Some(("viewedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "viewed-by-me-time" => Some(("viewedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewers-can-copy-content" => Some(("viewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["altitude", "aperture", "app-properties", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-security-update-enabled", "can-change-viewers-can-copy-content", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "content-hints", "copy-requires-writer-permission", "created-time", "description", "display-name", "drive-id", "duration-millis", "email-address", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "iso-speed", "kind", "last-modifying-user", "latitude", "lens", "link-share-metadata", "location", "longitude", "max-aperture-value", "md5-checksum", "me", "metering-mode", "mime-type", "modified-by-me", "modified-by-me-time", "modified-time", "name", "original-filename", "owned-by-me", "parents", "permission-id", "permission-ids", "photo-link", "properties", "quota-bytes-used", "resource-key", "rotation", "security-update-eligible", "security-update-enabled", "sensor", "sha1-checksum", "sha256-checksum", "shared", "shared-with-me-time", "sharing-user", "shortcut-details", "size", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "thumbnail", "thumbnail-link", "thumbnail-version", "time", "trashed", "trashed-time", "trashing-user", "version", "video-media-metadata", "viewed-by-me", "viewed-by-me-time", "viewers-can-copy-content", "web-content-link", "web-view-link", "white-balance", "width", "writers-can-share"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::File = json::value::from_value(object).unwrap();
        let mut call = self.hub.files().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(        value.map(|v| arg_from_str(v, err, "use-content-as-indexable-text", "boolean")).unwrap_or(false));
                },
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "keep-revision-forever" => {
                    call = call.keep_revision_forever(        value.map(|v| arg_from_str(v, err, "keep-revision-forever", "boolean")).unwrap_or(false));
                },
                "include-permissions-for-view" => {
                    call = call.include_permissions_for_view(value.unwrap_or(""));
                },
                "include-labels" => {
                    call = call.include_labels(value.unwrap_or(""));
                },
                "ignore-default-visibility" => {
                    call = call.ignore_default_visibility(        value.map(|v| arg_from_str(v, err, "ignore-default-visibility", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["enforce-single-parent", "ignore-default-visibility", "include-labels", "include-permissions-for-view", "keep-revision-forever", "ocr-language", "supports-all-drives", "supports-team-drives", "use-content-as-indexable-text"].iter().map(|v|*v));
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
                "count" => {
                    call = call.count(        value.map(|v| arg_from_str(v, err, "count", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["count", "space", "type"].iter().map(|v|*v));
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
                                                                           v.extend(["acknowledge-abuse", "include-labels", "include-permissions-for-view", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["corpora", "corpus", "drive-id", "include-items-from-all-drives", "include-labels", "include-permissions-for-view", "include-team-drive-items", "order-by", "page-size", "page-token", "q", "spaces", "supports-all-drives", "supports-team-drives", "team-drive-id"].iter().map(|v|*v));
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
                    "app-properties" => Some(("appProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "capabilities.can-accept-ownership" => Some(("capabilities.canAcceptOwnership", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-children" => Some(("capabilities.canAddChildren", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-folder-from-another-drive" => Some(("capabilities.canAddFolderFromAnotherDrive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-add-my-drive-parent" => Some(("capabilities.canAddMyDriveParent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-copy-requires-writer-permission" => Some(("capabilities.canChangeCopyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-security-update-enabled" => Some(("capabilities.canChangeSecurityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-change-viewers-can-copy-content" => Some(("capabilities.canChangeViewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "content-hints.indexable-text" => Some(("contentHints.indexableText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.image" => Some(("contentHints.thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.mime-type" => Some(("contentHints.thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copy-requires-writer-permission" => Some(("copyRequiresWriterPermission", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-id" => Some(("driveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "image-media-metadata.time" => Some(("imageMediaMetadata.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-eligible" => Some(("linkShareMetadata.securityUpdateEligible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "link-share-metadata.security-update-enabled" => Some(("linkShareMetadata.securityUpdateEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me" => Some(("modifiedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "modified-by-me-time" => Some(("modifiedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parents" => Some(("parents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permission-ids" => Some(("permissionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "properties" => Some(("properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-key" => Some(("resourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha1-checksum" => Some(("sha1Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sha256-checksum" => Some(("sha256Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "shared-with-me-time" => Some(("sharedWithMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.me" => Some(("sharingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.photo-link" => Some(("sharingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-id" => Some(("shortcutDetails.targetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-mime-type" => Some(("shortcutDetails.targetMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shortcut-details.target-resource-key" => Some(("shortcutDetails.targetResourceKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "team-drive-id" => Some(("teamDriveId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-version" => Some(("thumbnailVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashed" => Some(("trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashed-time" => Some(("trashedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.display-name" => Some(("trashingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.email-address" => Some(("trashingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.kind" => Some(("trashingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.me" => Some(("trashingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trashing-user.permission-id" => Some(("trashingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trashing-user.photo-link" => Some(("trashingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "viewed-by-me" => Some(("viewedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "viewed-by-me-time" => Some(("viewedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewers-can-copy-content" => Some(("viewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["altitude", "aperture", "app-properties", "camera-make", "camera-model", "can-accept-ownership", "can-add-children", "can-add-folder-from-another-drive", "can-add-my-drive-parent", "can-change-copy-requires-writer-permission", "can-change-security-update-enabled", "can-change-viewers-can-copy-content", "can-comment", "can-copy", "can-delete", "can-delete-children", "can-download", "can-edit", "can-list-children", "can-modify-content", "can-modify-content-restriction", "can-modify-editor-content-restriction", "can-modify-labels", "can-modify-owner-content-restriction", "can-move-children-out-of-drive", "can-move-children-out-of-team-drive", "can-move-children-within-drive", "can-move-children-within-team-drive", "can-move-item-into-team-drive", "can-move-item-out-of-drive", "can-move-item-out-of-team-drive", "can-move-item-within-drive", "can-move-item-within-team-drive", "can-move-team-drive-item", "can-read-drive", "can-read-labels", "can-read-revisions", "can-read-team-drive", "can-remove-children", "can-remove-content-restriction", "can-remove-my-drive-parent", "can-rename", "can-share", "can-trash", "can-trash-children", "can-untrash", "capabilities", "color-space", "content-hints", "copy-requires-writer-permission", "created-time", "description", "display-name", "drive-id", "duration-millis", "email-address", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "has-augmented-permissions", "has-thumbnail", "head-revision-id", "height", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "iso-speed", "kind", "last-modifying-user", "latitude", "lens", "link-share-metadata", "location", "longitude", "max-aperture-value", "md5-checksum", "me", "metering-mode", "mime-type", "modified-by-me", "modified-by-me-time", "modified-time", "name", "original-filename", "owned-by-me", "parents", "permission-id", "permission-ids", "photo-link", "properties", "quota-bytes-used", "resource-key", "rotation", "security-update-eligible", "security-update-enabled", "sensor", "sha1-checksum", "sha256-checksum", "shared", "shared-with-me-time", "sharing-user", "shortcut-details", "size", "spaces", "starred", "subject-distance", "target-id", "target-mime-type", "target-resource-key", "team-drive-id", "thumbnail", "thumbnail-link", "thumbnail-version", "time", "trashed", "trashed-time", "trashing-user", "version", "video-media-metadata", "viewed-by-me", "viewed-by-me-time", "viewers-can-copy-content", "web-content-link", "web-view-link", "white-balance", "width", "writers-can-share"]);
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
                "supports-team-drives" => {
                    call = call.supports_team_drives(        value.map(|v| arg_from_str(v, err, "supports-team-drives", "boolean")).unwrap_or(false));
                },
                "supports-all-drives" => {
                    call = call.supports_all_drives(        value.map(|v| arg_from_str(v, err, "supports-all-drives", "boolean")).unwrap_or(false));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "keep-revision-forever" => {
                    call = call.keep_revision_forever(        value.map(|v| arg_from_str(v, err, "keep-revision-forever", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["add-parents", "enforce-single-parent", "include-labels", "include-permissions-for-view", "keep-revision-forever", "ocr-language", "remove-parents", "supports-all-drives", "supports-team-drives", "use-content-as-indexable-text"].iter().map(|v|*v));
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
                                                                           v.extend(["acknowledge-abuse", "include-labels", "include-permissions-for-view", "supports-all-drives", "supports-team-drives"].iter().map(|v|*v));
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

    async fn _permissions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "allow-file-discovery" => Some(("allowFileDiscovery", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-owner" => Some(("pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view" => Some(("view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-file-discovery", "deleted", "display-name", "domain", "email-address", "expiration-time", "id", "kind", "pending-owner", "photo-link", "role", "type", "view"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Permission = json::value::from_value(object).unwrap();
        let mut call = self.hub.permissions().create(request, opt.value_of("file-id").unwrap_or(""));
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
                "send-notification-email" => {
                    call = call.send_notification_email(        value.map(|v| arg_from_str(v, err, "send-notification-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["email-message", "enforce-single-parent", "move-to-new-owners-root", "send-notification-email", "supports-all-drives", "supports-team-drives", "transfer-ownership", "use-domain-admin-access"].iter().map(|v|*v));
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
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["include-permissions-for-view", "page-size", "page-token", "supports-all-drives", "supports-team-drives", "use-domain-admin-access"].iter().map(|v|*v));
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
                    "allow-file-discovery" => Some(("allowFileDiscovery", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-owner" => Some(("pendingOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view" => Some(("view", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-file-discovery", "deleted", "display-name", "domain", "email-address", "expiration-time", "id", "kind", "pending-owner", "photo-link", "role", "type", "view"]);
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

    async fn _replies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "author", "content", "created-time", "deleted", "display-name", "email-address", "html-content", "id", "kind", "me", "modified-time", "permission-id", "photo-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Reply = json::value::from_value(object).unwrap();
        let mut call = self.hub.replies().create(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
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

    async fn _replies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().list(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["include-deleted", "page-size", "page-token"].iter().map(|v|*v));
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
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "author", "content", "created-time", "deleted", "display-name", "email-address", "html-content", "id", "kind", "me", "modified-time", "permission-id", "photo-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Reply = json::value::from_value(object).unwrap();
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
        let mut download_mode = false;
        let mut call = self.hub.revisions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["acknowledge-abuse"].iter().map(|v|*v));
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

    async fn _revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().list(opt.value_of("file-id").unwrap_or(""));
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
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keep-forever" => Some(("keepForever", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publish-auto" => Some(("publishAuto", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published" => Some(("published", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published-link" => Some(("publishedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published-outside-domain" => Some(("publishedOutsideDomain", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "email-address", "export-links", "id", "keep-forever", "kind", "last-modifying-user", "md5-checksum", "me", "mime-type", "modified-time", "original-filename", "permission-id", "photo-link", "publish-auto", "published", "published-link", "published-outside-domain", "size"]);
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

    async fn _teamdrives_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-change-team-drive-background", "can-change-team-members-only-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-team-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-remove-children", "can-rename", "can-rename-team-drive", "can-reset-team-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-time", "domain-users-only", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "team-members-only", "theme-id", "width", "x-coordinate", "y-coordinate"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TeamDrive = json::value::from_value(object).unwrap();
        let mut call = self.hub.teamdrives().create(request, opt.value_of("request-id").unwrap_or(""));
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
                                                                           v.extend(["page-size", "page-token", "q", "use-domain-admin-access"].iter().map(|v|*v));
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
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-managed-restrictions", "background-image-file", "background-image-link", "can-add-children", "can-change-copy-requires-writer-permission-restriction", "can-change-domain-users-only-restriction", "can-change-sharing-folders-requires-organizer-permission-restriction", "can-change-team-drive-background", "can-change-team-members-only-restriction", "can-comment", "can-copy", "can-delete-children", "can-delete-team-drive", "can-download", "can-edit", "can-list-children", "can-manage-members", "can-read-revisions", "can-remove-children", "can-rename", "can-rename-team-drive", "can-reset-team-drive-restrictions", "can-share", "can-trash-children", "capabilities", "color-rgb", "copy-requires-writer-permission", "created-time", "domain-users-only", "id", "kind", "name", "org-unit-id", "restrictions", "sharing-folders-requires-organizer-permission", "team-members-only", "theme-id", "width", "x-coordinate", "y-coordinate"]);
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
            ("comments", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._comments_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._comments_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._comments_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err).await;
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
                    ("create", Some(opt)) => {
                        call_result = self._drives_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._drives_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._drives_get(opt, dry_run, &mut err).await;
                    },
                    ("hide", Some(opt)) => {
                        call_result = self._drives_hide(opt, dry_run, &mut err).await;
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
                    ("create", Some(opt)) => {
                        call_result = self._files_create(opt, dry_run, &mut err).await;
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
                    ("list", Some(opt)) => {
                        call_result = self._files_list(opt, dry_run, &mut err).await;
                    },
                    ("list-labels", Some(opt)) => {
                        call_result = self._files_list_labels(opt, dry_run, &mut err).await;
                    },
                    ("modify-labels", Some(opt)) => {
                        call_result = self._files_modify_labels(opt, dry_run, &mut err).await;
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
            ("permissions", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._permissions_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._permissions_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._permissions_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._permissions_list(opt, dry_run, &mut err).await;
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
            ("replies", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._replies_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._replies_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._replies_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._replies_list(opt, dry_run, &mut err).await;
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
                    ("create", Some(opt)) => {
                        call_result = self._teamdrives_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._teamdrives_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._teamdrives_get(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "drive3-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/drive3", config_dir)).build().await.unwrap();

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
                    Some(r##"Gets information about the user, the user's Drive, and system capabilities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/about_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/apps_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/apps_list",
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
        
        ("changes", "methods: 'get-start-page-token', 'list' and 'watch'", vec![
            ("get-start-page-token",
                    Some(r##"Gets the starting pageToken for listing future changes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/changes_get-start-page-token",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/changes_list",
                  vec![
                    (Some(r##"page-token"##),
                     None,
                     Some(r##"The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method."##),
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
            ("watch",
                    Some(r##"Subscribes to changes for a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/changes_watch",
                  vec![
                    (Some(r##"page-token"##),
                     None,
                     Some(r##"The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method."##),
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
        
        ("channels", "methods: 'stop'", vec![
            ("stop",
                    Some(r##"Stops watching resources through this channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/channels_stop",
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
        
        ("comments", "methods: 'create', 'delete', 'get', 'list' and 'update'", vec![
            ("create",
                    Some(r##"Creates a comment on a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/comments_create",
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
            ("delete",
                    Some(r##"Deletes a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/comments_delete",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/comments_get",
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
            ("list",
                    Some(r##"Lists a file's comments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/comments_list",
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
            ("update",
                    Some(r##"Updates a comment with patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/comments_update",
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
        
        ("drives", "methods: 'create', 'delete', 'get', 'hide', 'list', 'unhide' and 'update'", vec![
            ("create",
                    Some(r##"Creates a shared drive."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_create",
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
            ("delete",
                    Some(r##"Permanently deletes a shared drive for which the user is an `organizer`. The shared drive cannot contain any untrashed items."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_delete",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_hide",
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
            ("list",
                    Some(r##" Lists the user's shared drives. This method accepts the `q` parameter, which is a search query combining one or more search terms. For more information, see the [Search for shared drives](/drive/api/guides/search-shareddrives) guide."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_unhide",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/drives_update",
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
        
        ("files", "methods: 'copy', 'create', 'delete', 'empty-trash', 'export', 'generate-ids', 'get', 'list', 'list-labels', 'modify-labels', 'update' and 'watch'", vec![
            ("copy",
                    Some(r##"Creates a copy of a file and applies any requested updates with patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_copy",
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
            ("create",
                    Some(r##" Creates a new file. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:*`*/*` Note: Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information on uploading files, see [Upload file data](/drive/api/guides/manage-uploads). Apps creating shortcuts with `files.create` must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `name` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"name": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `title` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the title. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_create",
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
            ("delete",
                    Some(r##"Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive, the user must be an `organizer` on the parent folder. If the target is a folder, all descendants owned by the user are also deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_delete",
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
                  ]),
            ("empty-trash",
                    Some(r##"Permanently deletes all of the user's trashed files."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_empty-trash",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("export",
                    Some(r##"Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_export",
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
                    Some(r##"Generates a set of file IDs which can be provided in create or copy requests."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_generate-ids",
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
                    Some(r##" Gets a file's metadata or content by ID. If you provide the URL parameter `alt=media`, then the response includes the file contents in the response body. Downloading content with `alt=media` only works if the file is stored in Drive. To download Google Docs, Sheets, and Slides use [`files.export`](/drive/api/reference/rest/v3/files/export) instead. For more information, see [Download & export files](/drive/api/guides/manage-downloads)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_get",
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
            ("list",
                    Some(r##" Lists the user's files. This method accepts the `q` parameter, which is a search query combining one or more search terms. For more information, see the [Search for files & folders](/drive/api/guides/search-files) guide. *Note:* This method returns *all* files by default, including trashed files. If you don't want trashed files to appear in the list, use the `trashed=false` query parameter to remove trashed files from the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_list-labels",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_modify-labels",
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
            ("update",
                    Some(r##" Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might be changed automatically, such as `modifiedDate`. This method supports patch semantics. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:*`*/*` Note: Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information on uploading files, see [Upload file data](/drive/api/guides/manage-uploads)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_update",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_watch",
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
            ]),
        
        ("permissions", "methods: 'create', 'delete', 'get', 'list' and 'update'", vec![
            ("create",
                    Some(r##"Creates a permission for a file or shared drive. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_create",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file or shared drive."##),
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
                    Some(r##"Deletes a permission. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file or shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID of the permission."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID of the permission."##),
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
                    Some(r##"Lists a file's or shared drive's permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_list",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file or shared drive."##),
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
                    Some(r##"Updates a permission with patch semantics. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file or shared drive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The ID of the permission."##),
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
        
        ("replies", "methods: 'create', 'delete', 'get', 'list' and 'update'", vec![
            ("create",
                    Some(r##"Creates a reply to a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/replies_create",
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
            ("delete",
                    Some(r##"Deletes a reply."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/replies_delete",
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
                    Some(r##"Gets a reply by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/replies_get",
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
            ("list",
                    Some(r##"Lists a comment's replies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/replies_list",
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
            ("update",
                    Some(r##"Updates a reply with patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/replies_update",
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
        
        ("revisions", "methods: 'delete', 'get', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/revisions_delete",
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
                    Some(r##"Gets a revision's metadata or content by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/revisions_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/revisions_list",
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
            ("update",
                    Some(r##"Updates a revision with patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/revisions_update",
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
        
        ("teamdrives", "methods: 'create', 'delete', 'get', 'list' and 'update'", vec![
            ("create",
                    Some(r##"Deprecated: Use `drives.create` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/teamdrives_create",
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
            ("delete",
                    Some(r##"Deprecated: Use `drives.delete` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/teamdrives_delete",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/teamdrives_get",
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
            ("list",
                    Some(r##"Deprecated: Use `drives.list` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/teamdrives_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/teamdrives_update",
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
    
    let mut app = App::new("drive3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240618")
           .about("The Google Drive API allows clients to access resources from Google Drive.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_drive3_cli")
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
