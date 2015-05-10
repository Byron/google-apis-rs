// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_calendar3 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde::json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: api::CalendarHub<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _acl_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.acl().delete(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _acl_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.acl().get(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _acl_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AclRule::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_scope_init(request: &mut api::AclRule) {
                if request.scope.is_none() {
                    request.scope = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "scope.type" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "scope.value" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_scope_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_scope_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request_scope_init(&mut request);
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_scope_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "id", "kind", "role", "scope", "type", "value"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.acl().insert(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _acl_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.acl().list(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sync-token", "max-results", "page-token", "show-deleted"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _acl_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AclRule::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_scope_init(request: &mut api::AclRule) {
                if request.scope.is_none() {
                    request.scope = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "scope.type" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "scope.value" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_scope_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_scope_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request_scope_init(&mut request);
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_scope_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "id", "kind", "role", "scope", "type", "value"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.acl().patch(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _acl_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AclRule::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_scope_init(request: &mut api::AclRule) {
                if request.scope.is_none() {
                    request.scope = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "scope.type" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "scope.value" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_scope_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_scope_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request_scope_init(&mut request);
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_scope_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "id", "kind", "role", "scope", "type", "value"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.acl().update(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _acl_watch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.acl().watch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sync-token", "max-results", "page-token", "show-deleted"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendar_list_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendar_list().delete(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _calendar_list_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendar_list().get(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendar_list_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CalendarListEntry::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "foreground-color" => {
                        request.foreground_color = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "color-id" => {
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "selected" => {
                        request.selected = Some(arg_from_str(value.unwrap_or("false"), err, "selected", "boolean"));
                    },
                "primary" => {
                        request.primary = Some(arg_from_str(value.unwrap_or("false"), err, "primary", "boolean"));
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "background-color" => {
                        request.background_color = Some(value.unwrap_or("").to_string());
                    },
                "summary-override" => {
                        request.summary_override = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "hidden" => {
                        request.hidden = Some(arg_from_str(value.unwrap_or("false"), err, "hidden", "boolean"));
                    },
                "access-role" => {
                        request.access_role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["access-role", "background-color", "color-id", "deleted", "description", "etag", "foreground-color", "hidden", "id", "kind", "location", "primary", "selected", "summary", "summary-override", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendar_list().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(arg_from_str(value.unwrap_or("false"), err, "color-rgb-format", "boolean"));
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
                                                Vec::new() + &self.gp + &["color-rgb-format"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendar_list_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendar_list().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-hidden" => {
                    call = call.show_hidden(arg_from_str(value.unwrap_or("false"), err, "show-hidden", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-access-role" => {
                    call = call.min_access_role(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sync-token", "min-access-role", "show-deleted", "max-results", "page-token", "show-hidden"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendar_list_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CalendarListEntry::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "foreground-color" => {
                        request.foreground_color = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "color-id" => {
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "selected" => {
                        request.selected = Some(arg_from_str(value.unwrap_or("false"), err, "selected", "boolean"));
                    },
                "primary" => {
                        request.primary = Some(arg_from_str(value.unwrap_or("false"), err, "primary", "boolean"));
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "background-color" => {
                        request.background_color = Some(value.unwrap_or("").to_string());
                    },
                "summary-override" => {
                        request.summary_override = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "hidden" => {
                        request.hidden = Some(arg_from_str(value.unwrap_or("false"), err, "hidden", "boolean"));
                    },
                "access-role" => {
                        request.access_role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["access-role", "background-color", "color-id", "deleted", "description", "etag", "foreground-color", "hidden", "id", "kind", "location", "primary", "selected", "summary", "summary-override", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendar_list().patch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(arg_from_str(value.unwrap_or("false"), err, "color-rgb-format", "boolean"));
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
                                                Vec::new() + &self.gp + &["color-rgb-format"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendar_list_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CalendarListEntry::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "foreground-color" => {
                        request.foreground_color = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "color-id" => {
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "selected" => {
                        request.selected = Some(arg_from_str(value.unwrap_or("false"), err, "selected", "boolean"));
                    },
                "primary" => {
                        request.primary = Some(arg_from_str(value.unwrap_or("false"), err, "primary", "boolean"));
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "background-color" => {
                        request.background_color = Some(value.unwrap_or("").to_string());
                    },
                "summary-override" => {
                        request.summary_override = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "hidden" => {
                        request.hidden = Some(arg_from_str(value.unwrap_or("false"), err, "hidden", "boolean"));
                    },
                "access-role" => {
                        request.access_role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["access-role", "background-color", "color-id", "deleted", "description", "etag", "foreground-color", "hidden", "id", "kind", "location", "primary", "selected", "summary", "summary-override", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendar_list().update(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(arg_from_str(value.unwrap_or("false"), err, "color-rgb-format", "boolean"));
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
                                                Vec::new() + &self.gp + &["color-rgb-format"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendar_list_watch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendar_list().watch(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-hidden" => {
                    call = call.show_hidden(arg_from_str(value.unwrap_or("false"), err, "show-hidden", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-access-role" => {
                    call = call.min_access_role(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sync-token", "min-access-role", "show-deleted", "max-results", "page-token", "show-hidden"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendars_clear(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendars().clear(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _calendars_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendars().delete(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _calendars_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.calendars().get(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendars_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Calendar::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "etag", "id", "kind", "location", "summary", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendars().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendars_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Calendar::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "etag", "id", "kind", "location", "summary", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendars().patch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _calendars_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Calendar::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "etag", "id", "kind", "location", "summary", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.calendars().update(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channels_stop(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.channels().stop(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _colors_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.colors().get();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().delete(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
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
                                                Vec::new() + &self.gp + &["send-notifications"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _events_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().get(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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
                                                Vec::new() + &self.gp + &["time-zone", "always-include-email", "max-attendees"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_import(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Event::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["anyone-can-add-self", "attendees-omitted", "color-id", "created", "creator", "date", "date-time", "description", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "extended-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "id", "kind", "link", "location", "locked", "organizer", "original-start-time", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "self", "sequence", "shared", "source", "start", "status", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.events().import(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Event::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["anyone-can-add-self", "attendees-omitted", "color-id", "created", "creator", "date", "date-time", "description", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "extended-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "id", "kind", "link", "location", "locked", "organizer", "original-start-time", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "self", "sequence", "shared", "source", "start", "status", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.events().insert(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
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
                                                Vec::new() + &self.gp + &["max-attendees", "send-notifications"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_instances(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().instances(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(value.unwrap_or(""));
                },
                "time-max" => {
                    call = call.time_max(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "original-start" => {
                    call = call.original_start(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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
                                                Vec::new() + &self.gp + &["show-deleted", "time-max", "always-include-email", "max-results", "page-token", "time-min", "time-zone", "original-start", "max-attendees"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().list(opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(value.unwrap_or(""));
                },
                "time-max" => {
                    call = call.time_max(value.unwrap_or(""));
                },
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "single-events" => {
                    call = call.single_events(arg_from_str(value.unwrap_or("false"), err, "single-events", "boolean"));
                },
                "show-hidden-invitations" => {
                    call = call.show_hidden_invitations(arg_from_str(value.unwrap_or("false"), err, "show-hidden-invitations", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "i-cal-uid" => {
                    call = call.i_cal_uid(value.unwrap_or(""));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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
                                                Vec::new() + &self.gp + &["show-hidden-invitations", "sync-token", "page-token", "time-max", "updated-min", "single-events", "i-cal-uid", "always-include-email", "order-by", "q", "show-deleted", "max-results", "time-min", "time-zone", "private-extended-property", "shared-extended-property", "max-attendees"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_move(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().move_(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""), opt.value_of("destination").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
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
                                                Vec::new() + &self.gp + &["send-notifications"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Event::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["anyone-can-add-self", "attendees-omitted", "color-id", "created", "creator", "date", "date-time", "description", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "extended-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "id", "kind", "link", "location", "locked", "organizer", "original-start-time", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "self", "sequence", "shared", "source", "start", "status", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.events().patch(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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
                                                Vec::new() + &self.gp + &["max-attendees", "always-include-email", "send-notifications"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_quick_add(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().quick_add(opt.value_of("calendar-id").unwrap_or(""), opt.value_of("text").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
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
                                                Vec::new() + &self.gp + &["send-notifications"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Event::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["anyone-can-add-self", "attendees-omitted", "color-id", "created", "creator", "date", "date-time", "description", "display", "display-name", "email", "end", "end-time-unspecified", "etag", "extended-properties", "gadget", "guests-can-invite-others", "guests-can-modify", "guests-can-see-other-guests", "hangout-link", "height", "html-link", "i-cal-uid", "icon-link", "id", "kind", "link", "location", "locked", "organizer", "original-start-time", "preferences", "private", "private-copy", "recurrence", "recurring-event-id", "reminders", "self", "sequence", "shared", "source", "start", "status", "summary", "time-zone", "title", "transparency", "type", "updated", "url", "use-default", "visibility", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.events().update(request, opt.value_of("calendar-id").unwrap_or(""), opt.value_of("event-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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
                                                Vec::new() + &self.gp + &["max-attendees", "always-include-email", "send-notifications"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_watch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.events().watch(request, opt.value_of("calendar-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(value.unwrap_or(""));
                },
                "time-max" => {
                    call = call.time_max(value.unwrap_or(""));
                },
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "single-events" => {
                    call = call.single_events(arg_from_str(value.unwrap_or("false"), err, "single-events", "boolean"));
                },
                "show-hidden-invitations" => {
                    call = call.show_hidden_invitations(arg_from_str(value.unwrap_or("false"), err, "show-hidden-invitations", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "i-cal-uid" => {
                    call = call.i_cal_uid(value.unwrap_or(""));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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
                                                Vec::new() + &self.gp + &["show-hidden-invitations", "sync-token", "page-token", "time-max", "updated-min", "single-events", "i-cal-uid", "always-include-email", "order-by", "q", "show-deleted", "max-results", "time-min", "time-zone", "private-extended-property", "shared-extended-property", "max-attendees"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _freebusy_query(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::FreeBusyRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "time-max" => {
                        request.time_max = Some(value.unwrap_or("").to_string());
                    },
                "calendar-expansion-max" => {
                        request.calendar_expansion_max = Some(arg_from_str(value.unwrap_or("-0"), err, "calendar-expansion-max", "integer"));
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "time-min" => {
                        request.time_min = Some(value.unwrap_or("").to_string());
                    },
                "group-expansion-max" => {
                        request.group_expansion_max = Some(arg_from_str(value.unwrap_or("-0"), err, "group-expansion-max", "integer"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["calendar-expansion-max", "group-expansion-max", "time-max", "time-min", "time-zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.freebusy().query(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _settings_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.settings().get(opt.value_of("setting").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _settings_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.settings().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sync-token", "max-results", "page-token"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _settings_watch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.settings().watch(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sync-token", "max-results", "page-token"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
            ("acl", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._acl_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._acl_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._acl_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._acl_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._acl_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._acl_update(opt, dry_run, &mut err);
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._acl_watch(opt, dry_run, &mut err);
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
                        call_result = self._calendar_list_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._calendar_list_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._calendar_list_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._calendar_list_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._calendar_list_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._calendar_list_update(opt, dry_run, &mut err);
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._calendar_list_watch(opt, dry_run, &mut err);
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
                        call_result = self._calendars_clear(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._calendars_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._calendars_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._calendars_insert(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._calendars_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._calendars_update(opt, dry_run, &mut err);
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
                        call_result = self._channels_stop(opt, dry_run, &mut err);
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
                        call_result = self._colors_get(opt, dry_run, &mut err);
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
                        call_result = self._events_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._events_get(opt, dry_run, &mut err);
                    },
                    ("import", Some(opt)) => {
                        call_result = self._events_import(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._events_insert(opt, dry_run, &mut err);
                    },
                    ("instances", Some(opt)) => {
                        call_result = self._events_instances(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._events_list(opt, dry_run, &mut err);
                    },
                    ("move", Some(opt)) => {
                        call_result = self._events_move(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._events_patch(opt, dry_run, &mut err);
                    },
                    ("quick-add", Some(opt)) => {
                        call_result = self._events_quick_add(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._events_update(opt, dry_run, &mut err);
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._events_watch(opt, dry_run, &mut err);
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
                        call_result = self._freebusy_query(opt, dry_run, &mut err);
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
                        call_result = self._settings_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._settings_list(opt, dry_run, &mut err);
                    },
                    ("watch", Some(opt)) => {
                        call_result = self._settings_watch(opt, dry_run, &mut err);
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "calendar3-secret.json", 
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpConnector(None) 
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "calendar3",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
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
    let arg_data = [
        ("acl", "methods: 'delete', 'get', 'insert', 'list', 'patch', 'update' and 'watch'", vec![
            ("delete",  
                    Some(r##"Deletes an access control rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/acl_delete",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                    Some(r##"Deletes an entry on the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_delete",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns an entry on the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_get",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
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
                    Some(r##"Adds an entry to the user's calendar list."##),
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
                    Some(r##"Returns entries on the user's calendar list."##),
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
                    Some(r##"Updates an entry on the user's calendar list. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_patch",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
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
                    Some(r##"Updates an entry on the user's calendar list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/calendar-list_update",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                    Some(r##"Returns an event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_get",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
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
                    Some(r##"Imports an event. This operation is used to add a private copy of an existing event to a calendar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_calendar3_cli/events_import",
                  vec![
                    (Some(r##"calendar-id"##),
                     None,
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                    Some(r##"Moves an event to another calendar, i.e. changes an event's organizer."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
                     Some(r##"Calendar identifier."##),
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
           .version("0.2.0+20150326")
           .about("Lets you manipulate events and other calendar data.")
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
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, ref about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::new(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::new(sub_command_name);
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
                       let mut arg = Arg::with_name(arg_name_str);
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

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            env::set_exit_status(err.exit_code);
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                env::set_exit_status(1);
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }
}