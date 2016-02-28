// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_drive2 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _about_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.about().get();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
                },
                "max-change-id-count" => {
                    call = call.max_change_id_count(arg_from_str(value.unwrap_or("-0"), err, "max-change-id-count", "int64"));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(arg_from_str(value.unwrap_or("false"), err, "include-subscribed", "boolean"));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["language-code", "app-filter-extensions", "app-filter-mime-types"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _changes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().get(opt.value_of("change-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _changes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
                },
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(arg_from_str(value.unwrap_or("false"), err, "include-subscribed", "boolean"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
                                                                           v.extend(["include-subscribed", "include-deleted", "max-results", "page-token", "spaces", "start-change-id"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _changes_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
                },
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(arg_from_str(value.unwrap_or("false"), err, "include-subscribed", "boolean"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
                                                                           v.extend(["include-subscribed", "include-deleted", "max-results", "page-token", "spaces", "start-change-id"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channels_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _children_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().delete(opt.value_of("folder-id").unwrap_or(""), opt.value_of("child-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _children_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _children_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "child-link" => Some(("childLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _children_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["q", "page-token", "max-results", "order-by"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _comments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-title" => Some(("fileTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.type" => Some(("context.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.value" => Some(("context.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comment-id" => Some(("commentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-id" => Some(("fileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
                                                                           v.extend(["updated-min", "include-deleted", "max-results", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-title" => Some(("fileTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.type" => Some(("context.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.value" => Some(("context.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comment-id" => Some(("commentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-id" => Some(("fileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-title" => Some(("fileTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.type" => Some(("context.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.value" => Some(("context.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comment-id" => Some(("commentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-id" => Some(("fileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_copy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-comment", "can-read-revisions", "color-space", "copyable", "created-date", "date", "default-open-with-link", "description", "display-name", "domain", "download-url", "duration-millis", "editable", "email-address", "embed-link", "etag", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "permission-id", "photo-link", "picture", "quota-bytes-used", "restricted", "role", "rotation", "self-link", "sensor", "shareable", "shared", "shared-with-me-date", "sharing-user", "spaces", "starred", "subject-distance", "text", "thumbnail", "thumbnail-link", "title", "trashed", "type", "url", "user-permission", "value", "version", "video-media-metadata", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
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
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
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
                                                                           v.extend(["convert", "ocr-language", "visibility", "pinned", "ocr", "timed-text-track-name", "timed-text-language"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().delete(opt.value_of("file-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _files_empty_trash(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().empty_trash();
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _files_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    ostream.flush().unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _files_generate_ids(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().generate_ids();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "space" => {
                    call = call.space(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["max-results", "space"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.files().get(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "revision-id" => {
                    call = call.revision_id(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "acknowledge-abuse" => {
                    call = call.acknowledge_abuse(arg_from_str(value.unwrap_or("false"), err, "acknowledge-abuse", "boolean"));
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
                                                                           v.extend(["revision-id", "update-viewed-date", "acknowledge-abuse", "projection"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    ostream.flush().unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _files_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-comment", "can-read-revisions", "color-space", "copyable", "created-date", "date", "default-open-with-link", "description", "display-name", "domain", "download-url", "duration-millis", "editable", "email-address", "embed-link", "etag", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "permission-id", "photo-link", "picture", "quota-bytes-used", "restricted", "role", "rotation", "self-link", "sensor", "shareable", "shared", "shared-with-me-date", "sharing-user", "spaces", "starred", "subject-distance", "text", "thumbnail", "thumbnail-link", "title", "trashed", "type", "url", "user-permission", "value", "version", "video-media-metadata", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
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
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
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
                                                                           v.extend(["convert", "use-content-as-indexable-text", "ocr-language", "visibility", "pinned", "ocr", "timed-text-track-name", "timed-text-language"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "corpus" => {
                    call = call.corpus(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "projection", "max-results", "q", "page-token", "spaces", "corpus"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-comment", "can-read-revisions", "color-space", "copyable", "created-date", "date", "default-open-with-link", "description", "display-name", "domain", "download-url", "duration-millis", "editable", "email-address", "embed-link", "etag", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "permission-id", "photo-link", "picture", "quota-bytes-used", "restricted", "role", "rotation", "self-link", "sensor", "shareable", "shared", "shared-with-me-date", "sharing-user", "spaces", "starred", "subject-distance", "text", "thumbnail", "thumbnail-link", "title", "trashed", "type", "url", "user-permission", "value", "version", "video-media-metadata", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
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
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "set-modified-date" => {
                    call = call.set_modified_date(arg_from_str(value.unwrap_or("false"), err, "set-modified-date", "boolean"));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "new-revision" => {
                    call = call.new_revision(arg_from_str(value.unwrap_or("false"), err, "new-revision", "boolean"));
                },
                "modified-date-behavior" => {
                    call = call.modified_date_behavior(value.unwrap_or(""));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
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
                                                                           v.extend(["add-parents", "convert", "ocr", "set-modified-date", "modified-date-behavior", "use-content-as-indexable-text", "ocr-language", "new-revision", "pinned", "remove-parents", "update-viewed-date", "timed-text-track-name", "timed-text-language"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_touch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().touch(opt.value_of("file-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_trash(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().trash(opt.value_of("file-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_untrash(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().untrash(opt.value_of("file-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-viewed-by-me-date" => Some(("lastViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-data-contents" => Some(("appDataContents", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.restricted" => Some(("labels.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.hidden" => Some(("labels.hidden", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.viewed" => Some(("labels.viewed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.starred" => Some(("labels.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels.trashed" => Some(("labels.trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "indexable-text.text" => Some(("indexableText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.picture.url" => Some(("sharingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.is-authenticated-user" => Some(("sharingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "copyable" => Some(("copyable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner-names" => Some(("ownerNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-date" => Some(("sharedWithMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-with-links" => Some(("openWithLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "can-read-revisions" => Some(("canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.focal-length" => Some(("imageMediaMetadata.focalLength", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-make" => Some(("imageMediaMetadata.cameraMake", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-mode" => Some(("imageMediaMetadata.exposureMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.color-space" => Some(("imageMediaMetadata.colorSpace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.latitude" => Some(("imageMediaMetadata.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.altitude" => Some(("imageMediaMetadata.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.location.longitude" => Some(("imageMediaMetadata.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.subject-distance" => Some(("imageMediaMetadata.subjectDistance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.height" => Some(("imageMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.lens" => Some(("imageMediaMetadata.lens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.date" => Some(("imageMediaMetadata.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable" => Some(("editable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "marked-viewed-by-me-date" => Some(("markedViewedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "can-comment" => Some(("canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "embed-link" => Some(("embedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-open-with-link" => Some(("defaultOpenWithLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "alternate-link" => Some(("alternateLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-by-me-date" => Some(("modifiedByMeDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.mime-type" => Some(("thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail.image" => Some(("thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.with-link" => Some(("userPermission.withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-permission.domain" => Some(("userPermission.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.name" => Some(("userPermission.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.kind" => Some(("userPermission.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.value" => Some(("userPermission.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.id" => Some(("userPermission.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.auth-key" => Some(("userPermission.authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.etag" => Some(("userPermission.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.email-address" => Some(("userPermission.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.photo-link" => Some(("userPermission.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.role" => Some(("userPermission.role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.type" => Some(("userPermission.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-permission.additional-roles" => Some(("userPermission.additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-permission.self-link" => Some(("userPermission.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shareable" => Some(("shareable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "alternate-link", "altitude", "aperture", "app-data-contents", "auth-key", "camera-make", "camera-model", "can-comment", "can-read-revisions", "color-space", "copyable", "created-date", "date", "default-open-with-link", "description", "display-name", "domain", "download-url", "duration-millis", "editable", "email-address", "embed-link", "etag", "explicitly-trashed", "export-links", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "file-size", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "hidden", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-authenticated-user", "iso-speed", "kind", "labels", "last-modifying-user", "last-modifying-user-name", "last-viewed-by-me-date", "latitude", "lens", "location", "longitude", "marked-viewed-by-me-date", "max-aperture-value", "md5-checksum", "metering-mode", "mime-type", "modified-by-me-date", "modified-date", "name", "open-with-links", "original-filename", "owned-by-me", "owner-names", "permission-id", "photo-link", "picture", "quota-bytes-used", "restricted", "role", "rotation", "self-link", "sensor", "shareable", "shared", "shared-with-me-date", "sharing-user", "spaces", "starred", "subject-distance", "text", "thumbnail", "thumbnail-link", "title", "trashed", "type", "url", "user-permission", "value", "version", "video-media-metadata", "viewed", "web-content-link", "web-view-link", "white-balance", "width", "with-link", "writers-can-share"]);
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
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "set-modified-date" => {
                    call = call.set_modified_date(arg_from_str(value.unwrap_or("false"), err, "set-modified-date", "boolean"));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "new-revision" => {
                    call = call.new_revision(arg_from_str(value.unwrap_or("false"), err, "new-revision", "boolean"));
                },
                "modified-date-behavior" => {
                    call = call.modified_date_behavior(value.unwrap_or(""));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
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
                                                                           v.extend(["add-parents", "convert", "ocr", "set-modified-date", "modified-date-behavior", "use-content-as-indexable-text", "ocr-language", "new-revision", "pinned", "remove-parents", "update-viewed-date", "timed-text-track-name", "timed-text-language"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
        let mut download_mode = false;
        let mut call = self.hub.files().watch(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "revision-id" => {
                    call = call.revision_id(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "acknowledge-abuse" => {
                    call = call.acknowledge_abuse(arg_from_str(value.unwrap_or("false"), err, "acknowledge-abuse", "boolean"));
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
                                                                           v.extend(["revision-id", "update-viewed-date", "acknowledge-abuse", "projection"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    ostream.flush().unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _parents_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("parent-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _parents_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _parents_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-root" => Some(("isRoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parent-link" => Some(("parentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _parents_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _permissions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_get_id_for_email(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "with-link" => Some(("withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-key" => Some(("authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "additional-roles" => Some(("additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "auth-key", "domain", "email-address", "etag", "id", "kind", "name", "photo-link", "role", "self-link", "type", "value", "with-link"]);
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
                "send-notification-emails" => {
                    call = call.send_notification_emails(arg_from_str(value.unwrap_or("false"), err, "send-notification-emails", "boolean"));
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
                                                                           v.extend(["email-message", "send-notification-emails"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().list(opt.value_of("file-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "with-link" => Some(("withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-key" => Some(("authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "additional-roles" => Some(("additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "auth-key", "domain", "email-address", "etag", "id", "kind", "name", "photo-link", "role", "self-link", "type", "value", "with-link"]);
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
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
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
                                                                           v.extend(["transfer-ownership"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "with-link" => Some(("withLink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-key" => Some(("authKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "additional-roles" => Some(("additionalRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-roles", "auth-key", "domain", "email-address", "etag", "id", "kind", "name", "photo-link", "role", "self-link", "type", "value", "with-link"]);
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
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
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
                                                                           v.extend(["transfer-ownership"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _properties_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility" => Some(("visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _realtime_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.realtime().get(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "revision" => {
                    call = call.revision(arg_from_str(value.unwrap_or("-0"), err, "revision", "integer"));
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
                                                                           v.extend(["revision"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    ostream.flush().unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _realtime_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.realtime().update(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "base-revision" => {
                    call = call.base_revision(value.unwrap_or(""));
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
                                                                           v.extend(["base-revision"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _replies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _replies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verb" => Some(("verb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-id" => Some(("replyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().list(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
                                                                           v.extend(["page-token", "include-deleted", "max-results"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verb" => Some(("verb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-id" => Some(("replyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.picture.url" => Some(("author.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.is-authenticated-user" => Some(("author.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verb" => Some(("verb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reply-id" => Some(("replyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-date" => Some(("createdDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _revisions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().list(opt.value_of("file-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pinned" => Some(("pinned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publish-auto" => Some(("publishAuto", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published-outside-domain" => Some(("publishedOutsideDomain", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published-link" => Some(("publishedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published" => Some(("published", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "mime-type" => Some(("mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pinned" => Some(("pinned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publish-auto" => Some(("publishAuto", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "published-outside-domain" => Some(("publishedOutsideDomain", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.picture.url" => Some(("lastModifyingUser.picture.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.is-authenticated-user" => Some(("lastModifyingUser.isAuthenticatedUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published-link" => Some(("publishedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-url" => Some(("downloadUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-size" => Some(("fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-links" => Some(("exportLinks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modifying-user-name" => Some(("lastModifyingUserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-date" => Some(("modifiedDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published" => Some(("published", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("about", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._about_get(opt, dry_run, &mut err);
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
                        call_result = self._apps_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._apps_list(opt, dry_run, &mut err);
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
                        call_result = self._changes_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._changes_list(opt, dry_run, &mut err);
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._changes_watch(opt, dry_run, &mut err);
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
                        call_result = self._channels_stop(opt, dry_run, &mut err);
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
                        call_result = self._children_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._children_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._children_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._children_list(opt, dry_run, &mut err);
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
                        call_result = self._comments_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._comments_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._comments_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._comments_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._comments_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("comments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("files", Some(opt)) => {
                match opt.subcommand() {
                    ("copy", Some(opt)) => {
                        call_result = self._files_copy(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._files_delete(opt, dry_run, &mut err);
                    },
                    ("empty-trash", Some(opt)) => {
                        call_result = self._files_empty_trash(opt, dry_run, &mut err);
                    },
                    ("export", Some(opt)) => {
                        call_result = self._files_export(opt, dry_run, &mut err);
                    },
                    ("generate-ids", Some(opt)) => {
                        call_result = self._files_generate_ids(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._files_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._files_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._files_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._files_patch(opt, dry_run, &mut err);
                    },
                    ("touch", Some(opt)) => {
                        call_result = self._files_touch(opt, dry_run, &mut err);
                    },
                    ("trash", Some(opt)) => {
                        call_result = self._files_trash(opt, dry_run, &mut err);
                    },
                    ("untrash", Some(opt)) => {
                        call_result = self._files_untrash(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._files_update(opt, dry_run, &mut err);
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._files_watch(opt, dry_run, &mut err);
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
                        call_result = self._parents_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._parents_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._parents_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._parents_list(opt, dry_run, &mut err);
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
                        call_result = self._permissions_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._permissions_get(opt, dry_run, &mut err);
                    },
                    ("get-id-for-email", Some(opt)) => {
                        call_result = self._permissions_get_id_for_email(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._permissions_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._permissions_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._permissions_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._permissions_update(opt, dry_run, &mut err);
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
                        call_result = self._properties_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._properties_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._properties_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._properties_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._properties_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._properties_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("properties".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("realtime", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._realtime_get(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._realtime_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("realtime".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("replies", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._replies_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._replies_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._replies_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._replies_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._replies_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._replies_update(opt, dry_run, &mut err);
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
                        call_result = self._revisions_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._revisions_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._revisions_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._revisions_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._revisions_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("revisions".to_string()));
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
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "drive2-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"De0ub0IbWruJbBXUyseFYvZ-\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"276875258587-5gbp23a7aqnrl6p06c0jt5fskuktactq.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "drive2",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Drive::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
                ]
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
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
        
        ("changes", "methods: 'get', 'list' and 'watch'", vec![
            ("get",
                    Some(r##"Gets a specific change."##),
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
            ("list",
                    Some(r##"Lists the changes for a user."##),
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
                    Some(r##"Stop watching resources through this channel"##),
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
                    Some(r##"Updates an existing comment. This method supports patch semantics."##),
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
        
        ("files", "methods: 'copy', 'delete', 'empty-trash', 'export', 'generate-ids', 'get', 'insert', 'list', 'patch', 'touch', 'trash', 'untrash', 'update' and 'watch'", vec![
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
                    Some(r##"Permanently deletes a file by ID. Skips the trash. The currently authenticated user must own the file."##),
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
                    Some(r##"Exports a Google Doc to the requested MIME type and returns the exported content."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_export",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mime-type"##),
                     None,
                     Some(r##"The MIME type of the format requested for this export."##),
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
                    Some(r##"Generates a set of file IDs which can be provided in insert requests."##),
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
                    Some(r##"Gets a file's metadata by ID."##),
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
                    Some(r##"Insert a new file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/files_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Lists the user's files."##),
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
            ("patch",
                    Some(r##"Updates file metadata and/or content. This method supports patch semantics."##),
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
                    Some(r##"Moves a file to the trash. The currently authenticated user must own the file."##),
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
                    Some(r##"Restores a file from the trash."##),
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
                    Some(r##"Updates file metadata and/or content."##),
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
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Subscribe to changes on a file"##),
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
                    Some(r##"Deletes a permission from a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_delete",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
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
                     Some(r##"The ID for the file."##),
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
                    Some(r##"Inserts a permission for a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_insert",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
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
                    Some(r##"Lists a file's permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_list",
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
            ("patch",
                    Some(r##"Updates a permission using patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_patch",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
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
                    Some(r##"Updates a permission."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/permissions_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID for the file."##),
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
                    Some(r##"Updates a property, or adds it if it doesn't exist. This method supports patch semantics."##),
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
                    Some(r##"Updates a property, or adds it if it doesn't exist."##),
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
        
        ("realtime", "methods: 'get' and 'update'", vec![
            ("get",
                    Some(r##"Exports the contents of the Realtime API data model associated with this file as JSON."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/realtime_get",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file that the Realtime API data model is associated with."##),
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
                    Some(r##"Overwrites the Realtime API data model associated with this file with the provided JSON data model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive2_cli/realtime_update",
                  vec![
                    (Some(r##"file-id"##),
                     None,
                     Some(r##"The ID of the file that the Realtime API data model is associated with."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
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
                    Some(r##"Updates an existing reply. This method supports patch semantics."##),
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
                    Some(r##"Removes a revision."##),
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
                    Some(r##"Updates a revision. This method supports patch semantics."##),
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
        
    ];
    
    let mut app = App::new("drive2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.3+20160222")
           .about("The API to interact with Drive.")
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
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
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

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
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