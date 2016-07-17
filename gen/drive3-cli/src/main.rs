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
extern crate google_drive3 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
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

    fn _changes_get_start_page_token(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().get_start_page_token();
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
        let mut call = self.hub.changes().list(opt.value_of("page-token").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "restrict-to-my-drive" => {
                    call = call.restrict_to_my_drive(arg_from_str(value.unwrap_or("false"), err, "restrict-to-my-drive", "boolean"));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "include-removed" => {
                    call = call.include_removed(arg_from_str(value.unwrap_or("false"), err, "include-removed", "boolean"));
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
                                                                           v.extend(["spaces", "page-size", "restrict-to-my-drive", "include-removed"].iter().map(|v|*v));
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
        let mut call = self.hub.changes().watch(request, opt.value_of("page-token").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "spaces" => {
                    call = call.spaces(value.unwrap_or(""));
                },
                "restrict-to-my-drive" => {
                    call = call.restrict_to_my_drive(arg_from_str(value.unwrap_or("false"), err, "restrict-to-my-drive", "boolean"));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "include-removed" => {
                    call = call.include_removed(arg_from_str(value.unwrap_or("false"), err, "include-removed", "boolean"));
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
                                                                           v.extend(["spaces", "page-size", "restrict-to-my-drive", "include-removed"].iter().map(|v|*v));
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

    fn _comments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "resolved" => Some(("resolved", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quoted-file-content.mime-type" => Some(("quotedFileContent.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quoted-file-content.value" => Some(("quotedFileContent.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _comments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "include-deleted", "page-size", "start-modified-time"].iter().map(|v|*v));
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
                    "resolved" => Some(("resolved", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quoted-file-content.mime-type" => Some(("quotedFileContent.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quoted-file-content.value" => Some(("quotedFileContent.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "anchor" => Some(("anchor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "modified-by-me-time" => Some(("modifiedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "viewed-by-me-time" => Some(("viewedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.me" => Some(("sharingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.photo-link" => Some(("sharingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-properties" => Some(("appProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parents" => Some(("parents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-time" => Some(("sharedWithMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewers-can-copy-content" => Some(("viewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewed-by-me" => Some(("viewedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties" => Some(("properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
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
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.time" => Some(("imageMediaMetadata.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trashed" => Some(("trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-hints.indexable-text" => Some(("contentHints.indexableText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.mime-type" => Some(("contentHints.thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.image" => Some(("contentHints.thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["altitude", "aperture", "app-properties", "camera-make", "camera-model", "can-comment", "can-copy", "can-edit", "can-read-revisions", "can-share", "capabilities", "color-space", "content-hints", "created-time", "description", "display-name", "duration-millis", "email-address", "explicitly-trashed", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "iso-speed", "kind", "last-modifying-user", "latitude", "lens", "location", "longitude", "max-aperture-value", "md5-checksum", "me", "metering-mode", "mime-type", "modified-by-me-time", "modified-time", "name", "original-filename", "owned-by-me", "parents", "permission-id", "photo-link", "properties", "quota-bytes-used", "rotation", "sensor", "shared", "shared-with-me-time", "sharing-user", "size", "spaces", "starred", "subject-distance", "thumbnail", "thumbnail-link", "time", "trashed", "version", "video-media-metadata", "viewed-by-me", "viewed-by-me-time", "viewers-can-copy-content", "web-content-link", "web-view-link", "white-balance", "width", "writers-can-share"]);
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
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "keep-revision-forever" => {
                    call = call.keep_revision_forever(arg_from_str(value.unwrap_or("false"), err, "keep-revision-forever", "boolean"));
                },
                "ignore-default-visibility" => {
                    call = call.ignore_default_visibility(arg_from_str(value.unwrap_or("false"), err, "ignore-default-visibility", "boolean"));
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
                                                                           v.extend(["keep-revision-forever", "ocr-language", "ignore-default-visibility"].iter().map(|v|*v));
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

    fn _files_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "modified-by-me-time" => Some(("modifiedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "viewed-by-me-time" => Some(("viewedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.me" => Some(("sharingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.photo-link" => Some(("sharingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-properties" => Some(("appProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parents" => Some(("parents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-time" => Some(("sharedWithMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewers-can-copy-content" => Some(("viewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewed-by-me" => Some(("viewedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties" => Some(("properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
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
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.time" => Some(("imageMediaMetadata.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trashed" => Some(("trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-hints.indexable-text" => Some(("contentHints.indexableText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.mime-type" => Some(("contentHints.thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.image" => Some(("contentHints.thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["altitude", "aperture", "app-properties", "camera-make", "camera-model", "can-comment", "can-copy", "can-edit", "can-read-revisions", "can-share", "capabilities", "color-space", "content-hints", "created-time", "description", "display-name", "duration-millis", "email-address", "explicitly-trashed", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "iso-speed", "kind", "last-modifying-user", "latitude", "lens", "location", "longitude", "max-aperture-value", "md5-checksum", "me", "metering-mode", "mime-type", "modified-by-me-time", "modified-time", "name", "original-filename", "owned-by-me", "parents", "permission-id", "photo-link", "properties", "quota-bytes-used", "rotation", "sensor", "shared", "shared-with-me-time", "sharing-user", "size", "spaces", "starred", "subject-distance", "thumbnail", "thumbnail-link", "time", "trashed", "version", "video-media-metadata", "viewed-by-me", "viewed-by-me-time", "viewers-can-copy-content", "web-content-link", "web-view-link", "white-balance", "width", "writers-can-share"]);
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
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "keep-revision-forever" => {
                    call = call.keep_revision_forever(arg_from_str(value.unwrap_or("false"), err, "keep-revision-forever", "boolean"));
                },
                "ignore-default-visibility" => {
                    call = call.ignore_default_visibility(arg_from_str(value.unwrap_or("false"), err, "ignore-default-visibility", "boolean"));
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
                                                                           v.extend(["keep-revision-forever", "use-content-as-indexable-text", "ocr-language", "ignore-default-visibility"].iter().map(|v|*v));
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
                "count" => {
                    call = call.count(arg_from_str(value.unwrap_or("-0"), err, "count", "integer"));
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
                                                                           v.extend(["count", "space"].iter().map(|v|*v));
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
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-size", "q", "page-token", "spaces", "corpus"].iter().map(|v|*v));
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
                    "modified-by-me-time" => Some(("modifiedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-link" => Some(("thumbnailLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explicitly-trashed" => Some(("explicitlyTrashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-app-authorized" => Some(("isAppAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "writers-can-share" => Some(("writersCanShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "owned-by-me" => Some(("ownedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "viewed-by-me-time" => Some(("viewedByMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.me" => Some(("sharingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sharing-user.kind" => Some(("sharingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.display-name" => Some(("sharingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.permission-id" => Some(("sharingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.email-address" => Some(("sharingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sharing-user.photo-link" => Some(("sharingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.width" => Some(("videoMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-media-metadata.duration-millis" => Some(("videoMediaMetadata.durationMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-media-metadata.height" => Some(("videoMediaMetadata.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-color-rgb" => Some(("folderColorRgb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-properties" => Some(("appProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "capabilities.can-read-revisions" => Some(("capabilities.canReadRevisions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-copy" => Some(("capabilities.canCopy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-share" => Some(("capabilities.canShare", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-edit" => Some(("capabilities.canEdit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "capabilities.can-comment" => Some(("capabilities.canComment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-view-link" => Some(("webViewLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parents" => Some(("parents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "shared-with-me-time" => Some(("sharedWithMeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared" => Some(("shared", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "full-file-extension" => Some(("fullFileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewers-can-copy-content" => Some(("viewersCanCopyContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewed-by-me" => Some(("viewedByMe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "quota-bytes-used" => Some(("quotaBytesUsed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-extension" => Some(("fileExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties" => Some(("properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "icon-link" => Some(("iconLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-bias" => Some(("imageMediaMetadata.exposureBias", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.exposure-time" => Some(("imageMediaMetadata.exposureTime", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.max-aperture-value" => Some(("imageMediaMetadata.maxApertureValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.iso-speed" => Some(("imageMediaMetadata.isoSpeed", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
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
                    "image-media-metadata.width" => Some(("imageMediaMetadata.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.metering-mode" => Some(("imageMediaMetadata.meteringMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.flash-used" => Some(("imageMediaMetadata.flashUsed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-media-metadata.time" => Some(("imageMediaMetadata.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.aperture" => Some(("imageMediaMetadata.aperture", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "image-media-metadata.rotation" => Some(("imageMediaMetadata.rotation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image-media-metadata.sensor" => Some(("imageMediaMetadata.sensor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.white-balance" => Some(("imageMediaMetadata.whiteBalance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-media-metadata.camera-model" => Some(("imageMediaMetadata.cameraModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-content-link" => Some(("webContentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spaces" => Some(("spaces", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trashed" => Some(("trashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-hints.indexable-text" => Some(("contentHints.indexableText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.mime-type" => Some(("contentHints.thumbnail.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-hints.thumbnail.image" => Some(("contentHints.thumbnail.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "head-revision-id" => Some(("headRevisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["altitude", "aperture", "app-properties", "camera-make", "camera-model", "can-comment", "can-copy", "can-edit", "can-read-revisions", "can-share", "capabilities", "color-space", "content-hints", "created-time", "description", "display-name", "duration-millis", "email-address", "explicitly-trashed", "exposure-bias", "exposure-mode", "exposure-time", "file-extension", "flash-used", "focal-length", "folder-color-rgb", "full-file-extension", "head-revision-id", "height", "icon-link", "id", "image", "image-media-metadata", "indexable-text", "is-app-authorized", "iso-speed", "kind", "last-modifying-user", "latitude", "lens", "location", "longitude", "max-aperture-value", "md5-checksum", "me", "metering-mode", "mime-type", "modified-by-me-time", "modified-time", "name", "original-filename", "owned-by-me", "parents", "permission-id", "photo-link", "properties", "quota-bytes-used", "rotation", "sensor", "shared", "shared-with-me-time", "sharing-user", "size", "spaces", "starred", "subject-distance", "thumbnail", "thumbnail-link", "time", "trashed", "version", "video-media-metadata", "viewed-by-me", "viewed-by-me-time", "viewers-can-copy-content", "web-content-link", "web-view-link", "white-balance", "width", "writers-can-share"]);
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
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "keep-revision-forever" => {
                    call = call.keep_revision_forever(arg_from_str(value.unwrap_or("false"), err, "keep-revision-forever", "boolean"));
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
                                                                           v.extend(["add-parents", "keep-revision-forever", "use-content-as-indexable-text", "ocr-language", "remove-parents"].iter().map(|v|*v));
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

    fn _permissions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-file-discovery" => Some(("allowFileDiscovery", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-file-discovery", "display-name", "domain", "email-address", "id", "kind", "photo-link", "role", "type"]);
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
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
                },
                "send-notification-email" => {
                    call = call.send_notification_email(arg_from_str(value.unwrap_or("false"), err, "send-notification-email", "boolean"));
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
                                                                           v.extend(["send-notification-email", "email-message", "transfer-ownership"].iter().map(|v|*v));
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
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-file-discovery" => Some(("allowFileDiscovery", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "photo-link" => Some(("photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-file-discovery", "display-name", "domain", "email-address", "id", "kind", "photo-link", "role", "type"]);
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

    fn _replies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _replies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().list(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "include-deleted", "page-size"].iter().map(|v|*v));
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
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.me" => Some(("author.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author.kind" => Some(("author.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.permission-id" => Some(("author.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.email-address" => Some(("author.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.photo-link" => Some(("author.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "html-content" => Some(("htmlContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-time" => Some(("createdTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
        let mut download_mode = false;
        let mut call = self.hub.revisions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "modified-time" => Some(("modifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.me" => Some(("lastModifyingUser.me", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "last-modifying-user.kind" => Some(("lastModifyingUser.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.display-name" => Some(("lastModifyingUser.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.permission-id" => Some(("lastModifyingUser.permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.email-address" => Some(("lastModifyingUser.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modifying-user.photo-link" => Some(("lastModifyingUser.photoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keep-forever" => Some(("keepForever", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "publish-auto" => Some(("publishAuto", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "original-filename" => Some(("originalFilename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published" => Some(("published", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-checksum" => Some(("md5Checksum", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published-outside-domain" => Some(("publishedOutsideDomain", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "email-address", "id", "keep-forever", "kind", "last-modifying-user", "md5-checksum", "me", "mime-type", "modified-time", "original-filename", "permission-id", "photo-link", "publish-auto", "published", "published-outside-domain", "size"]);
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
            ("changes", Some(opt)) => {
                match opt.subcommand() {
                    ("get-start-page-token", Some(opt)) => {
                        call_result = self._changes_get_start_page_token(opt, dry_run, &mut err);
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
            ("comments", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._comments_create(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._comments_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._comments_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err);
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
                    ("create", Some(opt)) => {
                        call_result = self._files_create(opt, dry_run, &mut err);
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
                    ("list", Some(opt)) => {
                        call_result = self._files_list(opt, dry_run, &mut err);
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
            ("permissions", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._permissions_create(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._permissions_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._permissions_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._permissions_list(opt, dry_run, &mut err);
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
            ("replies", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._replies_create(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._replies_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._replies_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._replies_list(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "drive3-secret.json",
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
                                          program_name: "drive3",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

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
                    Some(r##"Lists changes for a user."##),
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
                    Some(r##"Stop watching resources through this channel"##),
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
                    Some(r##"Creates a new comment on a file."##),
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
        
        ("files", "methods: 'copy', 'create', 'delete', 'empty-trash', 'export', 'generate-ids', 'get', 'list', 'update' and 'watch'", vec![
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
                    Some(r##"Creates a new file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_create",
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
            ("delete",
                    Some(r##"Permanently deletes a file owned by the user without moving it to the trash. If the target is a folder, all descendants owned by the user are also deleted."##),
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
                    Some(r##"Exports a Google Doc to the requested MIME type and returns the exported content."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/files_export",
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
                    Some(r##"Generates a set of file IDs which can be provided in create requests."##),
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
                    Some(r##"Gets a file's metadata or content by ID."##),
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
                    Some(r##"Lists or searches files."##),
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
            ("update",
                    Some(r##"Updates a file's metadata and/or content with patch semantics."##),
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
                    Some(r##"Subscribes to changes to a file"##),
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
                    Some(r##"Creates a permission for a file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_create",
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
                    Some(r##"Deletes a permission."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_delete",
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
                    Some(r##"Lists a file's permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_list",
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
                    Some(r##"Updates a permission with patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_drive3_cli/permissions_update",
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
                    Some(r##"Creates a new reply to a comment."##),
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
                    Some(r##"Permanently deletes a revision. This method is only applicable to files with binary content in Drive."##),
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
        
    ];
    
    let mut app = App::new("drive3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.5+20160331")
           .about("The API to interact with Drive.")
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