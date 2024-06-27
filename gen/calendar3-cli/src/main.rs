// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_calendar3::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CalendarHub<S>,
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
    async fn _acl_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.acl().delete(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
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

    async fn _acl_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.acl().get(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
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

    async fn _acl_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope.type" => Some(("scope.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope.value" => Some(("scope.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "id", "kind", "role", "scope", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AclRule = json::value::from_value(object).unwrap();
        let mut call = self.hub.acl().insert(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["send-notifications"].iter().map(|v|*v));
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

    async fn _acl_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.acl().list(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["max-results", "page-token", "show-deleted", "sync-token"].iter().map(|v|*v));
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

    async fn _acl_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope.type" => Some(("scope.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope.value" => Some(("scope.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "id", "kind", "role", "scope", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AclRule = json::value::from_value(object).unwrap();
        let mut call = self.hub.acl().patch(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["send-notifications"].iter().map(|v|*v));
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

    async fn _acl_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope.type" => Some(("scope.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope.value" => Some(("scope.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "id", "kind", "role", "scope", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AclRule = json::value::from_value(object).unwrap();
        let mut call = self.hub.acl().update(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["send-notifications"].iter().map(|v|*v));
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

    async fn _acl_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.acl().watch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["max-results", "page-token", "show-deleted", "sync-token"].iter().map(|v|*v));
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

    async fn _calendar_list_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendar_list().delete(opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _calendar_list_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendar_list().get(opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _calendar_list_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-role" => Some(("accessRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-color" => Some(("backgroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-properties.allowed-conference-solution-types" => Some(("conferenceProperties.allowedConferenceSolutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "foreground-color" => Some(("foregroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hidden" => Some(("hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary" => Some(("primary", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "selected" => Some(("selected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary-override" => Some(("summaryOverride", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-role", "allowed-conference-solution-types", "background-color", "color-id", "conference-properties", "deleted", "description", "etag", "foreground-color", "hidden", "id", "kind", "location", "primary", "selected", "summary", "summary-override", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CalendarListEntry = json::value::from_value(object).unwrap();
        let mut call = self.hub.calendar_list().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(        value.map(|v| arg_from_str(v, err, "color-rgb-format", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["color-rgb-format"].iter().map(|v|*v));
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

    async fn _calendar_list_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendar_list().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-hidden" => {
                    call = call.show_hidden(        value.map(|v| arg_from_str(v, err, "show-hidden", "boolean")).unwrap_or(false));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-access-role" => {
                    call = call.min_access_role(value.unwrap_or(""));
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
                                                                           v.extend(["max-results", "min-access-role", "page-token", "show-deleted", "show-hidden", "sync-token"].iter().map(|v|*v));
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

    async fn _calendar_list_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-role" => Some(("accessRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-color" => Some(("backgroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-properties.allowed-conference-solution-types" => Some(("conferenceProperties.allowedConferenceSolutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "foreground-color" => Some(("foregroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hidden" => Some(("hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary" => Some(("primary", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "selected" => Some(("selected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary-override" => Some(("summaryOverride", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-role", "allowed-conference-solution-types", "background-color", "color-id", "conference-properties", "deleted", "description", "etag", "foreground-color", "hidden", "id", "kind", "location", "primary", "selected", "summary", "summary-override", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CalendarListEntry = json::value::from_value(object).unwrap();
        let mut call = self.hub.calendar_list().patch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(        value.map(|v| arg_from_str(v, err, "color-rgb-format", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["color-rgb-format"].iter().map(|v|*v));
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

    async fn _calendar_list_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-role" => Some(("accessRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "background-color" => Some(("backgroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-properties.allowed-conference-solution-types" => Some(("conferenceProperties.allowedConferenceSolutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "foreground-color" => Some(("foregroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hidden" => Some(("hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary" => Some(("primary", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "selected" => Some(("selected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary-override" => Some(("summaryOverride", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-role", "allowed-conference-solution-types", "background-color", "color-id", "conference-properties", "deleted", "description", "etag", "foreground-color", "hidden", "id", "kind", "location", "primary", "selected", "summary", "summary-override", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CalendarListEntry = json::value::from_value(object).unwrap();
        let mut call = self.hub.calendar_list().update(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(        value.map(|v| arg_from_str(v, err, "color-rgb-format", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["color-rgb-format"].iter().map(|v|*v));
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

    async fn _calendar_list_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.calendar_list().watch(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-hidden" => {
                    call = call.show_hidden(        value.map(|v| arg_from_str(v, err, "show-hidden", "boolean")).unwrap_or(false));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-access-role" => {
                    call = call.min_access_role(value.unwrap_or(""));
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
                                                                           v.extend(["max-results", "min-access-role", "page-token", "show-deleted", "show-hidden", "sync-token"].iter().map(|v|*v));
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

    async fn _calendars_clear(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendars().clear(opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _calendars_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendars().delete(opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _calendars_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendars().get(opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _calendars_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "conference-properties.allowed-conference-solution-types" => Some(("conferenceProperties.allowedConferenceSolutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-conference-solution-types", "conference-properties", "description", "etag", "id", "kind", "location", "summary", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Calendar = json::value::from_value(object).unwrap();
        let mut call = self.hub.calendars().insert(request);
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

    async fn _calendars_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "conference-properties.allowed-conference-solution-types" => Some(("conferenceProperties.allowedConferenceSolutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-conference-solution-types", "conference-properties", "description", "etag", "id", "kind", "location", "summary", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Calendar = json::value::from_value(object).unwrap();
        let mut call = self.hub.calendars().patch(request, opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _calendars_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "conference-properties.allowed-conference-solution-types" => Some(("conferenceProperties.allowedConferenceSolutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-conference-solution-types", "conference-properties", "description", "etag", "id", "kind", "location", "summary", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Calendar = json::value::from_value(object).unwrap();
        let mut call = self.hub.calendars().update(request, opt.value_of("calendar-id").unwrap_or(""));
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

    async fn _colors_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.colors().get();
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

    async fn _events_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().delete(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-updates" => {
                    call = call.send_updates(value.unwrap_or(""));
                },
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["send-notifications", "send-updates"].iter().map(|v|*v));
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

    async fn _events_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().get(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "always-include-email" => {
                    call = call.always_include_email(        value.map(|v| arg_from_str(v, err, "always-include-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["always-include-email", "max-attendees", "time-zone"].iter().map(|v|*v));
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

    async fn _events_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anyone-can-add-self" => Some(("anyoneCanAddSelf", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "attendees-omitted" => Some(("attendeesOmitted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-id" => Some(("conferenceData.conferenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.icon-uri" => Some(("conferenceData.conferenceSolution.iconUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.key.type" => Some(("conferenceData.conferenceSolution.key.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.name" => Some(("conferenceData.conferenceSolution.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.conference-solution-key.type" => Some(("conferenceData.createRequest.conferenceSolutionKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.request-id" => Some(("conferenceData.createRequest.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.status.status-code" => Some(("conferenceData.createRequest.status.statusCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.notes" => Some(("conferenceData.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.parameters.add-on-parameters.parameters" => Some(("conferenceData.parameters.addOnParameters.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conference-data.signature" => Some(("conferenceData.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.display-name" => Some(("creator.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.email" => Some(("creator.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.id" => Some(("creator.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.self" => Some(("creator.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date" => Some(("end.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date-time" => Some(("end.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.time-zone" => Some(("end.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time-unspecified" => Some(("endTimeUnspecified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "extended-properties.private" => Some(("extendedProperties.private", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "extended-properties.shared" => Some(("extendedProperties.shared", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "focus-time-properties.auto-decline-mode" => Some(("focusTimeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.chat-status" => Some(("focusTimeProperties.chatStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.decline-message" => Some(("focusTimeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.display" => Some(("gadget.display", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.height" => Some(("gadget.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "gadget.icon-link" => Some(("gadget.iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.link" => Some(("gadget.link", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.preferences" => Some(("gadget.preferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "gadget.title" => Some(("gadget.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.type" => Some(("gadget.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.width" => Some(("gadget.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "guests-can-invite-others" => Some(("guestsCanInviteOthers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-modify" => Some(("guestsCanModify", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-see-other-guests" => Some(("guestsCanSeeOtherGuests", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "hangout-link" => Some(("hangoutLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-link" => Some(("htmlLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "i-cal-uid" => Some(("iCalUID", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locked" => Some(("locked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "organizer.display-name" => Some(("organizer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.email" => Some(("organizer.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.id" => Some(("organizer.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.self" => Some(("organizer.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-start-time.date" => Some(("originalStartTime.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.date-time" => Some(("originalStartTime.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.time-zone" => Some(("originalStartTime.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.auto-decline-mode" => Some(("outOfOfficeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.decline-message" => Some(("outOfOfficeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-copy" => Some(("privateCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "recurrence" => Some(("recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recurring-event-id" => Some(("recurringEventId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reminders.use-default" => Some(("reminders.useDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sequence" => Some(("sequence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "source.title" => Some(("source.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.url" => Some(("source.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date" => Some(("start.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date-time" => Some(("start.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.time-zone" => Some(("start.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transparency" => Some(("transparency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.custom-location.label" => Some(("workingLocationProperties.customLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.building-id" => Some(("workingLocationProperties.officeLocation.buildingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.desk-id" => Some(("workingLocationProperties.officeLocation.deskId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-id" => Some(("workingLocationProperties.officeLocation.floorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-section-id" => Some(("workingLocationProperties.officeLocation.floorSectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.label" => Some(("workingLocationProperties.officeLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.type" => Some(("workingLocationProperties.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-on-parameters", "anyone-can-add-self", "attendees-omitted", "auto-decline-mode", "building-id", "chat-status", "color-id", "conference-data", "conference-id", "conference-solution", "conference-solution-key", "create-request", "created", "creator", "custom-location", "date", "date-time", "decline-message", "description", "desk-id", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "event-type", "extended-properties", "floor-id", "floor-section-id", "focus-time-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "icon-uri", "id", "key", "kind", "label", "link", "location", "locked", "name", "notes", "office-location", "organizer", "original-start-time", "out-of-office-properties", "parameters", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "request-id", "self", "sequence", "shared", "signature", "source", "start", "status", "status-code", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width", "working-location-properties"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Event = json::value::from_value(object).unwrap();
        let mut call = self.hub.events().import(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-attachments" => {
                    call = call.supports_attachments(        value.map(|v| arg_from_str(v, err, "supports-attachments", "boolean")).unwrap_or(false));
                },
                "conference-data-version" => {
                    call = call.conference_data_version(        value.map(|v| arg_from_str(v, err, "conference-data-version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["conference-data-version", "supports-attachments"].iter().map(|v|*v));
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

    async fn _events_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anyone-can-add-self" => Some(("anyoneCanAddSelf", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "attendees-omitted" => Some(("attendeesOmitted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-id" => Some(("conferenceData.conferenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.icon-uri" => Some(("conferenceData.conferenceSolution.iconUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.key.type" => Some(("conferenceData.conferenceSolution.key.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.name" => Some(("conferenceData.conferenceSolution.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.conference-solution-key.type" => Some(("conferenceData.createRequest.conferenceSolutionKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.request-id" => Some(("conferenceData.createRequest.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.status.status-code" => Some(("conferenceData.createRequest.status.statusCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.notes" => Some(("conferenceData.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.parameters.add-on-parameters.parameters" => Some(("conferenceData.parameters.addOnParameters.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conference-data.signature" => Some(("conferenceData.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.display-name" => Some(("creator.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.email" => Some(("creator.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.id" => Some(("creator.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.self" => Some(("creator.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date" => Some(("end.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date-time" => Some(("end.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.time-zone" => Some(("end.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time-unspecified" => Some(("endTimeUnspecified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "extended-properties.private" => Some(("extendedProperties.private", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "extended-properties.shared" => Some(("extendedProperties.shared", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "focus-time-properties.auto-decline-mode" => Some(("focusTimeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.chat-status" => Some(("focusTimeProperties.chatStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.decline-message" => Some(("focusTimeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.display" => Some(("gadget.display", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.height" => Some(("gadget.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "gadget.icon-link" => Some(("gadget.iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.link" => Some(("gadget.link", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.preferences" => Some(("gadget.preferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "gadget.title" => Some(("gadget.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.type" => Some(("gadget.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.width" => Some(("gadget.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "guests-can-invite-others" => Some(("guestsCanInviteOthers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-modify" => Some(("guestsCanModify", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-see-other-guests" => Some(("guestsCanSeeOtherGuests", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "hangout-link" => Some(("hangoutLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-link" => Some(("htmlLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "i-cal-uid" => Some(("iCalUID", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locked" => Some(("locked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "organizer.display-name" => Some(("organizer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.email" => Some(("organizer.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.id" => Some(("organizer.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.self" => Some(("organizer.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-start-time.date" => Some(("originalStartTime.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.date-time" => Some(("originalStartTime.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.time-zone" => Some(("originalStartTime.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.auto-decline-mode" => Some(("outOfOfficeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.decline-message" => Some(("outOfOfficeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-copy" => Some(("privateCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "recurrence" => Some(("recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recurring-event-id" => Some(("recurringEventId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reminders.use-default" => Some(("reminders.useDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sequence" => Some(("sequence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "source.title" => Some(("source.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.url" => Some(("source.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date" => Some(("start.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date-time" => Some(("start.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.time-zone" => Some(("start.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transparency" => Some(("transparency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.custom-location.label" => Some(("workingLocationProperties.customLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.building-id" => Some(("workingLocationProperties.officeLocation.buildingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.desk-id" => Some(("workingLocationProperties.officeLocation.deskId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-id" => Some(("workingLocationProperties.officeLocation.floorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-section-id" => Some(("workingLocationProperties.officeLocation.floorSectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.label" => Some(("workingLocationProperties.officeLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.type" => Some(("workingLocationProperties.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-on-parameters", "anyone-can-add-self", "attendees-omitted", "auto-decline-mode", "building-id", "chat-status", "color-id", "conference-data", "conference-id", "conference-solution", "conference-solution-key", "create-request", "created", "creator", "custom-location", "date", "date-time", "decline-message", "description", "desk-id", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "event-type", "extended-properties", "floor-id", "floor-section-id", "focus-time-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "icon-uri", "id", "key", "kind", "label", "link", "location", "locked", "name", "notes", "office-location", "organizer", "original-start-time", "out-of-office-properties", "parameters", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "request-id", "self", "sequence", "shared", "signature", "source", "start", "status", "status-code", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width", "working-location-properties"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Event = json::value::from_value(object).unwrap();
        let mut call = self.hub.events().insert(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-attachments" => {
                    call = call.supports_attachments(        value.map(|v| arg_from_str(v, err, "supports-attachments", "boolean")).unwrap_or(false));
                },
                "send-updates" => {
                    call = call.send_updates(value.unwrap_or(""));
                },
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
                },
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "conference-data-version" => {
                    call = call.conference_data_version(        value.map(|v| arg_from_str(v, err, "conference-data-version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["conference-data-version", "max-attendees", "send-notifications", "send-updates", "supports-attachments"].iter().map(|v|*v));
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

    async fn _events_instances(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().instances(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(        value.map(|v| arg_from_str(v, err, "time-min", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "time-max" => {
                    call = call.time_max(        value.map(|v| arg_from_str(v, err, "time-max", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "original-start" => {
                    call = call.original_start(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "always-include-email" => {
                    call = call.always_include_email(        value.map(|v| arg_from_str(v, err, "always-include-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["always-include-email", "max-attendees", "max-results", "original-start", "page-token", "show-deleted", "time-max", "time-min", "time-zone"].iter().map(|v|*v));
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

    async fn _events_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().list(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(        value.map(|v| arg_from_str(v, err, "updated-min", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(        value.map(|v| arg_from_str(v, err, "time-min", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "time-max" => {
                    call = call.time_max(        value.map(|v| arg_from_str(v, err, "time-max", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "single-events" => {
                    call = call.single_events(        value.map(|v| arg_from_str(v, err, "single-events", "boolean")).unwrap_or(false));
                },
                "show-hidden-invitations" => {
                    call = call.show_hidden_invitations(        value.map(|v| arg_from_str(v, err, "show-hidden-invitations", "boolean")).unwrap_or(false));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
                },
                "shared-extended-property" => {
                    call = call.add_shared_extended_property(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "private-extended-property" => {
                    call = call.add_private_extended_property(value.unwrap_or(""));
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
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "i-cal-uid" => {
                    call = call.i_cal_uid(value.unwrap_or(""));
                },
                "event-types" => {
                    call = call.add_event_types(value.unwrap_or(""));
                },
                "always-include-email" => {
                    call = call.always_include_email(        value.map(|v| arg_from_str(v, err, "always-include-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["always-include-email", "event-types", "i-cal-uid", "max-attendees", "max-results", "order-by", "page-token", "private-extended-property", "q", "shared-extended-property", "show-deleted", "show-hidden-invitations", "single-events", "sync-token", "time-max", "time-min", "time-zone", "updated-min"].iter().map(|v|*v));
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

    async fn _events_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().move_(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""), opt.value_of("destination").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-updates" => {
                    call = call.send_updates(value.unwrap_or(""));
                },
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["send-notifications", "send-updates"].iter().map(|v|*v));
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

    async fn _events_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anyone-can-add-self" => Some(("anyoneCanAddSelf", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "attendees-omitted" => Some(("attendeesOmitted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-id" => Some(("conferenceData.conferenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.icon-uri" => Some(("conferenceData.conferenceSolution.iconUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.key.type" => Some(("conferenceData.conferenceSolution.key.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.name" => Some(("conferenceData.conferenceSolution.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.conference-solution-key.type" => Some(("conferenceData.createRequest.conferenceSolutionKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.request-id" => Some(("conferenceData.createRequest.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.status.status-code" => Some(("conferenceData.createRequest.status.statusCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.notes" => Some(("conferenceData.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.parameters.add-on-parameters.parameters" => Some(("conferenceData.parameters.addOnParameters.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conference-data.signature" => Some(("conferenceData.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.display-name" => Some(("creator.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.email" => Some(("creator.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.id" => Some(("creator.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.self" => Some(("creator.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date" => Some(("end.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date-time" => Some(("end.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.time-zone" => Some(("end.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time-unspecified" => Some(("endTimeUnspecified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "extended-properties.private" => Some(("extendedProperties.private", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "extended-properties.shared" => Some(("extendedProperties.shared", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "focus-time-properties.auto-decline-mode" => Some(("focusTimeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.chat-status" => Some(("focusTimeProperties.chatStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.decline-message" => Some(("focusTimeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.display" => Some(("gadget.display", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.height" => Some(("gadget.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "gadget.icon-link" => Some(("gadget.iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.link" => Some(("gadget.link", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.preferences" => Some(("gadget.preferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "gadget.title" => Some(("gadget.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.type" => Some(("gadget.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.width" => Some(("gadget.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "guests-can-invite-others" => Some(("guestsCanInviteOthers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-modify" => Some(("guestsCanModify", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-see-other-guests" => Some(("guestsCanSeeOtherGuests", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "hangout-link" => Some(("hangoutLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-link" => Some(("htmlLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "i-cal-uid" => Some(("iCalUID", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locked" => Some(("locked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "organizer.display-name" => Some(("organizer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.email" => Some(("organizer.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.id" => Some(("organizer.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.self" => Some(("organizer.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-start-time.date" => Some(("originalStartTime.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.date-time" => Some(("originalStartTime.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.time-zone" => Some(("originalStartTime.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.auto-decline-mode" => Some(("outOfOfficeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.decline-message" => Some(("outOfOfficeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-copy" => Some(("privateCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "recurrence" => Some(("recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recurring-event-id" => Some(("recurringEventId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reminders.use-default" => Some(("reminders.useDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sequence" => Some(("sequence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "source.title" => Some(("source.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.url" => Some(("source.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date" => Some(("start.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date-time" => Some(("start.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.time-zone" => Some(("start.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transparency" => Some(("transparency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.custom-location.label" => Some(("workingLocationProperties.customLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.building-id" => Some(("workingLocationProperties.officeLocation.buildingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.desk-id" => Some(("workingLocationProperties.officeLocation.deskId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-id" => Some(("workingLocationProperties.officeLocation.floorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-section-id" => Some(("workingLocationProperties.officeLocation.floorSectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.label" => Some(("workingLocationProperties.officeLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.type" => Some(("workingLocationProperties.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-on-parameters", "anyone-can-add-self", "attendees-omitted", "auto-decline-mode", "building-id", "chat-status", "color-id", "conference-data", "conference-id", "conference-solution", "conference-solution-key", "create-request", "created", "creator", "custom-location", "date", "date-time", "decline-message", "description", "desk-id", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "event-type", "extended-properties", "floor-id", "floor-section-id", "focus-time-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "icon-uri", "id", "key", "kind", "label", "link", "location", "locked", "name", "notes", "office-location", "organizer", "original-start-time", "out-of-office-properties", "parameters", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "request-id", "self", "sequence", "shared", "signature", "source", "start", "status", "status-code", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width", "working-location-properties"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Event = json::value::from_value(object).unwrap();
        let mut call = self.hub.events().patch(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-attachments" => {
                    call = call.supports_attachments(        value.map(|v| arg_from_str(v, err, "supports-attachments", "boolean")).unwrap_or(false));
                },
                "send-updates" => {
                    call = call.send_updates(value.unwrap_or(""));
                },
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
                },
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "conference-data-version" => {
                    call = call.conference_data_version(        value.map(|v| arg_from_str(v, err, "conference-data-version", "int32")).unwrap_or(-0));
                },
                "always-include-email" => {
                    call = call.always_include_email(        value.map(|v| arg_from_str(v, err, "always-include-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["always-include-email", "conference-data-version", "max-attendees", "send-notifications", "send-updates", "supports-attachments"].iter().map(|v|*v));
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

    async fn _events_quick_add(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().quick_add(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("text").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-updates" => {
                    call = call.send_updates(value.unwrap_or(""));
                },
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["send-notifications", "send-updates"].iter().map(|v|*v));
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

    async fn _events_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "anyone-can-add-self" => Some(("anyoneCanAddSelf", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "attendees-omitted" => Some(("attendeesOmitted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "color-id" => Some(("colorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-id" => Some(("conferenceData.conferenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.icon-uri" => Some(("conferenceData.conferenceSolution.iconUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.key.type" => Some(("conferenceData.conferenceSolution.key.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.conference-solution.name" => Some(("conferenceData.conferenceSolution.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.conference-solution-key.type" => Some(("conferenceData.createRequest.conferenceSolutionKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.request-id" => Some(("conferenceData.createRequest.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.create-request.status.status-code" => Some(("conferenceData.createRequest.status.statusCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.notes" => Some(("conferenceData.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conference-data.parameters.add-on-parameters.parameters" => Some(("conferenceData.parameters.addOnParameters.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conference-data.signature" => Some(("conferenceData.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.display-name" => Some(("creator.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.email" => Some(("creator.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.id" => Some(("creator.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.self" => Some(("creator.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date" => Some(("end.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.date-time" => Some(("end.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end.time-zone" => Some(("end.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time-unspecified" => Some(("endTimeUnspecified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "extended-properties.private" => Some(("extendedProperties.private", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "extended-properties.shared" => Some(("extendedProperties.shared", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "focus-time-properties.auto-decline-mode" => Some(("focusTimeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.chat-status" => Some(("focusTimeProperties.chatStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "focus-time-properties.decline-message" => Some(("focusTimeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.display" => Some(("gadget.display", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.height" => Some(("gadget.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "gadget.icon-link" => Some(("gadget.iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.link" => Some(("gadget.link", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.preferences" => Some(("gadget.preferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "gadget.title" => Some(("gadget.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.type" => Some(("gadget.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gadget.width" => Some(("gadget.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "guests-can-invite-others" => Some(("guestsCanInviteOthers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-modify" => Some(("guestsCanModify", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "guests-can-see-other-guests" => Some(("guestsCanSeeOtherGuests", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "hangout-link" => Some(("hangoutLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-link" => Some(("htmlLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "i-cal-uid" => Some(("iCalUID", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locked" => Some(("locked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "organizer.display-name" => Some(("organizer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.email" => Some(("organizer.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.id" => Some(("organizer.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organizer.self" => Some(("organizer.self", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-start-time.date" => Some(("originalStartTime.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.date-time" => Some(("originalStartTime.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-start-time.time-zone" => Some(("originalStartTime.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.auto-decline-mode" => Some(("outOfOfficeProperties.autoDeclineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "out-of-office-properties.decline-message" => Some(("outOfOfficeProperties.declineMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-copy" => Some(("privateCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "recurrence" => Some(("recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recurring-event-id" => Some(("recurringEventId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reminders.use-default" => Some(("reminders.useDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sequence" => Some(("sequence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "source.title" => Some(("source.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.url" => Some(("source.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date" => Some(("start.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.date-time" => Some(("start.dateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start.time-zone" => Some(("start.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transparency" => Some(("transparency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.custom-location.label" => Some(("workingLocationProperties.customLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.building-id" => Some(("workingLocationProperties.officeLocation.buildingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.desk-id" => Some(("workingLocationProperties.officeLocation.deskId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-id" => Some(("workingLocationProperties.officeLocation.floorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.floor-section-id" => Some(("workingLocationProperties.officeLocation.floorSectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.office-location.label" => Some(("workingLocationProperties.officeLocation.label", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "working-location-properties.type" => Some(("workingLocationProperties.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-on-parameters", "anyone-can-add-self", "attendees-omitted", "auto-decline-mode", "building-id", "chat-status", "color-id", "conference-data", "conference-id", "conference-solution", "conference-solution-key", "create-request", "created", "creator", "custom-location", "date", "date-time", "decline-message", "description", "desk-id", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "event-type", "extended-properties", "floor-id", "floor-section-id", "focus-time-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "icon-uri", "id", "key", "kind", "label", "link", "location", "locked", "name", "notes", "office-location", "organizer", "original-start-time", "out-of-office-properties", "parameters", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "request-id", "self", "sequence", "shared", "signature", "source", "start", "status", "status-code", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width", "working-location-properties"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Event = json::value::from_value(object).unwrap();
        let mut call = self.hub.events().update(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "supports-attachments" => {
                    call = call.supports_attachments(        value.map(|v| arg_from_str(v, err, "supports-attachments", "boolean")).unwrap_or(false));
                },
                "send-updates" => {
                    call = call.send_updates(value.unwrap_or(""));
                },
                "send-notifications" => {
                    call = call.send_notifications(        value.map(|v| arg_from_str(v, err, "send-notifications", "boolean")).unwrap_or(false));
                },
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "conference-data-version" => {
                    call = call.conference_data_version(        value.map(|v| arg_from_str(v, err, "conference-data-version", "int32")).unwrap_or(-0));
                },
                "always-include-email" => {
                    call = call.always_include_email(        value.map(|v| arg_from_str(v, err, "always-include-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["always-include-email", "conference-data-version", "max-attendees", "send-notifications", "send-updates", "supports-attachments"].iter().map(|v|*v));
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

    async fn _events_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.events().watch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(        value.map(|v| arg_from_str(v, err, "updated-min", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(        value.map(|v| arg_from_str(v, err, "time-min", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "time-max" => {
                    call = call.time_max(        value.map(|v| arg_from_str(v, err, "time-max", "date-time")).unwrap_or(chrono::Utc::now()));
                },
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "single-events" => {
                    call = call.single_events(        value.map(|v| arg_from_str(v, err, "single-events", "boolean")).unwrap_or(false));
                },
                "show-hidden-invitations" => {
                    call = call.show_hidden_invitations(        value.map(|v| arg_from_str(v, err, "show-hidden-invitations", "boolean")).unwrap_or(false));
                },
                "show-deleted" => {
                    call = call.show_deleted(        value.map(|v| arg_from_str(v, err, "show-deleted", "boolean")).unwrap_or(false));
                },
                "shared-extended-property" => {
                    call = call.add_shared_extended_property(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "private-extended-property" => {
                    call = call.add_private_extended_property(value.unwrap_or(""));
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
                "max-attendees" => {
                    call = call.max_attendees(        value.map(|v| arg_from_str(v, err, "max-attendees", "int32")).unwrap_or(-0));
                },
                "i-cal-uid" => {
                    call = call.i_cal_uid(value.unwrap_or(""));
                },
                "event-types" => {
                    call = call.add_event_types(value.unwrap_or(""));
                },
                "always-include-email" => {
                    call = call.always_include_email(        value.map(|v| arg_from_str(v, err, "always-include-email", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["always-include-email", "event-types", "i-cal-uid", "max-attendees", "max-results", "order-by", "page-token", "private-extended-property", "q", "shared-extended-property", "show-deleted", "show-hidden-invitations", "single-events", "sync-token", "time-max", "time-min", "time-zone", "updated-min"].iter().map(|v|*v));
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

    async fn _freebusy_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "calendar-expansion-max" => Some(("calendarExpansionMax", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "group-expansion-max" => Some(("groupExpansionMax", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "time-max" => Some(("timeMax", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-min" => Some(("timeMin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["calendar-expansion-max", "group-expansion-max", "time-max", "time-min", "time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FreeBusyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.freebusy().query(request);
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

    async fn _settings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.settings().get(opt.value_of("setting").unwrap_or(""));
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

    async fn _settings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.settings().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
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
                                                                           v.extend(["max-results", "page-token", "sync-token"].iter().map(|v|*v));
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

    async fn _settings_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.settings().watch(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
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
                                                                           v.extend(["max-results", "page-token", "sync-token"].iter().map(|v|*v));
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
            ("acl", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._acl_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._acl_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._acl_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._acl_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._acl_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._acl_update(opt, dry_run, &mut err).await;
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._acl_watch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("acl".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("calendar-list", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._calendar_list_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._calendar_list_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._calendar_list_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._calendar_list_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._calendar_list_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._calendar_list_update(opt, dry_run, &mut err).await;
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._calendar_list_watch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("calendar-list".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("calendars", Some(opt)) => {
                match opt.subcommand() {
                    ("clear", Some(opt)) => {
                        call_result = self._calendars_clear(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._calendars_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._calendars_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._calendars_insert(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._calendars_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._calendars_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("calendars".to_string()));
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
            ("colors", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._colors_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("colors".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("events", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._events_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._events_get(opt, dry_run, &mut err).await;
                    },
                    ("import", Some(opt)) => {
                        call_result = self._events_import(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._events_insert(opt, dry_run, &mut err).await;
                    },
                    ("instances", Some(opt)) => {
                        call_result = self._events_instances(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._events_list(opt, dry_run, &mut err).await;
                    },
                    ("move", Some(opt)) => {
                        call_result = self._events_move(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._events_patch(opt, dry_run, &mut err).await;
                    },
                    ("quick-add", Some(opt)) => {
                        call_result = self._events_quick_add(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._events_update(opt, dry_run, &mut err).await;
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._events_watch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("events".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("freebusy", Some(opt)) => {
                match opt.subcommand() {
                    ("query", Some(opt)) => {
                        call_result = self._freebusy_query(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("freebusy".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("settings", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._settings_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._settings_list(opt, dry_run, &mut err).await;
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._settings_watch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("settings".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "calendar3-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/calendar3", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CalendarHub::new(client, auth),
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
        ("acl", "methods: 'delete', 'get', 'insert', 'list', 'patch', 'update' and 'watch'", vec![
            ("delete",
                    Some(r##"Deletes an access control rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_delete",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"ACL rule identifier."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns an access control rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_get",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"ACL rule identifier."##),
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
                    Some(r##"Creates an access control rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_insert",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Returns the rules in the access control list for the calendar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_list",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Updates an access control rule. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_patch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"ACL rule identifier."##),
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
                    Some(r##"Updates an access control rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_update",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"ACL rule identifier."##),
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
            ("watch",
                    Some(r##"Watch for changes to ACL resources."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_watch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
        
        ("calendar-list", "methods: 'delete', 'get', 'insert', 'list', 'patch', 'update' and 'watch'", vec![
            ("delete",
                    Some(r##"Removes a calendar from the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_delete",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns a calendar from the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_get",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Inserts an existing calendar into the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_insert",
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
            ("list",
                    Some(r##"Returns the calendars on the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_list",
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
            ("patch",
                    Some(r##"Updates an existing calendar on the user's calendar list. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_patch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Updates an existing calendar on the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_update",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
            ("watch",
                    Some(r##"Watch for changes to CalendarList resources."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_watch",
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
        
        ("calendars", "methods: 'clear', 'delete', 'get', 'insert', 'patch' and 'update'", vec![
            ("clear",
                    Some(r##"Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendars_clear",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("delete",
                    Some(r##"Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendars_delete",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns metadata for a calendar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendars_get",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Creates a secondary calendar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendars_insert",
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
            ("patch",
                    Some(r##"Updates metadata for a calendar. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendars_patch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Updates metadata for a calendar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendars_update",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
                    Some(r##"Stop watching resources through this channel"##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/channels_stop",
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
        
        ("colors", "methods: 'get'", vec![
            ("get",
                    Some(r##"Returns the color definitions for calendars and events."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/colors_get",
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
        
        ("events", "methods: 'delete', 'get', 'import', 'insert', 'instances', 'list', 'move', 'patch', 'quick-add', 'update' and 'watch'", vec![
            ("delete",
                    Some(r##"Deletes an event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_delete",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"event-id"##),
                     None,
                     Some(r##"Event identifier."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns an event based on its Google Calendar ID. To retrieve an event using its iCalendar ID, call the events.list method using the iCalUID parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_get",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"event-id"##),
                     None,
                     Some(r##"Event identifier."##),
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
            ("import",
                    Some(r##"Imports an event. This operation is used to add a private copy of an existing event to a calendar. Only events with an eventType of default may be imported.
        Deprecated behavior: If a non-default event is imported, its type will be changed to default and any event-type-specific properties it may have will be dropped."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_import",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
            ("insert",
                    Some(r##"Creates an event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_insert",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
            ("instances",
                    Some(r##"Returns instances of the specified recurring event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_instances",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"event-id"##),
                     None,
                     Some(r##"Recurring event identifier."##),
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
                    Some(r##"Returns events on the specified calendar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_list",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
            ("move",
                    Some(r##"Moves an event to another calendar, i.e. changes an event's organizer. Note that only default events can be moved; outOfOffice, focusTime, workingLocation and fromGmail events cannot be moved."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_move",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier of the source calendar where the event currently is on."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"event-id"##),
                     None,
                     Some(r##"Event identifier."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination"##),
                     None,
                     Some(r##"Calendar identifier of the target calendar where the event is to be moved to."##),
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
                    Some(r##"Updates an event. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_patch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"event-id"##),
                     None,
                     Some(r##"Event identifier."##),
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
            ("quick-add",
                    Some(r##"Creates an event based on a simple text string."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_quick-add",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"text"##),
                     None,
                     Some(r##"The text describing the event to be created."##),
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
                    Some(r##"Updates an event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_update",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"event-id"##),
                     None,
                     Some(r##"Event identifier."##),
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
            ("watch",
                    Some(r##"Watch for changes to Events resources."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_watch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword."##),
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
        
        ("freebusy", "methods: 'query'", vec![
            ("query",
                    Some(r##"Returns free/busy information for a set of calendars."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/freebusy_query",
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
        
        ("settings", "methods: 'get', 'list' and 'watch'", vec![
            ("get",
                    Some(r##"Returns a single user setting."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/settings_get",
                  vec![
                    (Some(r##"setting"##),
                     None,
                     Some(r##"The id of the user setting."##),
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
                    Some(r##"Returns all user settings for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/settings_list",
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
                    Some(r##"Watch for changes to Settings resources."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/settings_watch",
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
        
    ];
    
    let mut app = App::new("calendar3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240523")
           .about("Manipulates events and other calendar data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_calendar3_cli")
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
