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
extern crate google_sqladmin1_beta4 as api;

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
    hub: api::SQLAdmin<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _backup_runs_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.backup_runs().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _backup_runs_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.backup_runs().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _databases_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.databases().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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

    fn _databases_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.databases().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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

    fn _databases_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Database::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "charset" => {
                        request.charset = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "collation" => {
                        request.collation = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["charset", "collation", "etag", "instance", "kind", "name", "project", "self-link"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.databases().insert(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _databases_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.databases().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _databases_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Database::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "charset" => {
                        request.charset = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "collation" => {
                        request.collation = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["charset", "collation", "etag", "instance", "kind", "name", "project", "self-link"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.databases().patch(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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

    fn _databases_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Database::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "charset" => {
                        request.charset = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "collation" => {
                        request.collation = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["charset", "collation", "etag", "instance", "kind", "name", "project", "self-link"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.databases().update(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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

    fn _flags_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.flags().list();
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

    fn _instances_clone(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::InstancesCloneRequest::default();
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
            fn request_clone_context_bin_log_coordinates_init(request: &mut api::InstancesCloneRequest) {
                request_clone_context_init(request);
                if request.clone_context.as_mut().unwrap().bin_log_coordinates.is_none() {
                    request.clone_context.as_mut().unwrap().bin_log_coordinates = Some(Default::default());
                }
            }
            
            fn request_clone_context_init(request: &mut api::InstancesCloneRequest) {
                if request.clone_context.is_none() {
                    request.clone_context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "clone-context.bin-log-coordinates.bin-log-position" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().bin_log_coordinates.as_mut().unwrap().bin_log_position = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.bin-log-coordinates.kind" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().bin_log_coordinates.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.bin-log-coordinates.bin-log-file-name" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().bin_log_coordinates.as_mut().unwrap().bin_log_file_name = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.kind" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.destination-instance-name" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().destination_instance_name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bin-log-coordinates", "bin-log-file-name", "bin-log-position", "clone-context", "destination-instance-name", "kind"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().clone(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_export(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::InstancesExportRequest::default();
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
            fn request_export_context_csv_export_options_init(request: &mut api::InstancesExportRequest) {
                request_export_context_init(request);
                if request.export_context.as_mut().unwrap().csv_export_options.is_none() {
                    request.export_context.as_mut().unwrap().csv_export_options = Some(Default::default());
                }
            }
            
            fn request_export_context_init(request: &mut api::InstancesExportRequest) {
                if request.export_context.is_none() {
                    request.export_context = Some(Default::default());
                }
            }
            
            fn request_export_context_sql_export_options_init(request: &mut api::InstancesExportRequest) {
                request_export_context_init(request);
                if request.export_context.as_mut().unwrap().sql_export_options.is_none() {
                    request.export_context.as_mut().unwrap().sql_export_options = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "export-context.kind" => {
                        request_export_context_init(&mut request);
                        request.export_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "export-context.file-type" => {
                        request_export_context_init(&mut request);
                        request.export_context.as_mut().unwrap().file_type = Some(value.unwrap_or("").to_string());
                    },
                "export-context.uri" => {
                        request_export_context_init(&mut request);
                        request.export_context.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "export-context.csv-export-options.select-query" => {
                        request_export_context_csv_export_options_init(&mut request);
                        request.export_context.as_mut().unwrap().csv_export_options.as_mut().unwrap().select_query = Some(value.unwrap_or("").to_string());
                    },
                "export-context.databases" => {
                        request_export_context_csv_export_options_init(&mut request);
                        if request.export_context.as_mut().unwrap().databases.is_none() {
                           request.export_context.as_mut().unwrap().databases = Some(Default::default());
                        }
                                        request.export_context.as_mut().unwrap().databases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "export-context.sql-export-options.tables" => {
                        request_export_context_sql_export_options_init(&mut request);
                        if request.export_context.as_mut().unwrap().sql_export_options.as_mut().unwrap().tables.is_none() {
                           request.export_context.as_mut().unwrap().sql_export_options.as_mut().unwrap().tables = Some(Default::default());
                        }
                                        request.export_context.as_mut().unwrap().sql_export_options.as_mut().unwrap().tables.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["csv-export-options", "databases", "export-context", "file-type", "kind", "select-query", "sql-export-options", "tables", "uri"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().export(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_import(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::InstancesImportRequest::default();
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
            fn request_import_context_csv_import_options_init(request: &mut api::InstancesImportRequest) {
                request_import_context_init(request);
                if request.import_context.as_mut().unwrap().csv_import_options.is_none() {
                    request.import_context.as_mut().unwrap().csv_import_options = Some(Default::default());
                }
            }
            
            fn request_import_context_init(request: &mut api::InstancesImportRequest) {
                if request.import_context.is_none() {
                    request.import_context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "import-context.file-type" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().file_type = Some(value.unwrap_or("").to_string());
                    },
                "import-context.database" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().database = Some(value.unwrap_or("").to_string());
                    },
                "import-context.uri" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "import-context.kind" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "import-context.csv-import-options.table" => {
                        request_import_context_csv_import_options_init(&mut request);
                        request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().table = Some(value.unwrap_or("").to_string());
                    },
                "import-context.csv-import-options.columns" => {
                        request_import_context_csv_import_options_init(&mut request);
                        if request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().columns.is_none() {
                           request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().columns = Some(Default::default());
                        }
                                        request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().columns.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["columns", "csv-import-options", "database", "file-type", "import-context", "kind", "table", "uri"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().import(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::DatabaseInstance::default();
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
            fn request_on_premises_configuration_init(request: &mut api::DatabaseInstance) {
                if request.on_premises_configuration.is_none() {
                    request.on_premises_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_init(request: &mut api::DatabaseInstance) {
                if request.replica_configuration.is_none() {
                    request.replica_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_mysql_replica_configuration_init(request: &mut api::DatabaseInstance) {
                request_replica_configuration_init(request);
                if request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.is_none() {
                    request.replica_configuration.as_mut().unwrap().mysql_replica_configuration = Some(Default::default());
                }
            }
            
            fn request_server_ca_cert_init(request: &mut api::DatabaseInstance) {
                if request.server_ca_cert.is_none() {
                    request.server_ca_cert = Some(Default::default());
                }
            }
            
            fn request_settings_backup_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().backup_configuration.is_none() {
                    request.settings.as_mut().unwrap().backup_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_init(request: &mut api::DatabaseInstance) {
                if request.settings.is_none() {
                    request.settings = Some(Default::default());
                }
            }
            
            fn request_settings_ip_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().ip_configuration.is_none() {
                    request.settings.as_mut().unwrap().ip_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_location_preference_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().location_preference.is_none() {
                    request.settings.as_mut().unwrap().location_preference = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "on-premises-configuration.kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "on-premises-configuration.host-port" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().host_port = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "max-disk-size" => {
                        request_on_premises_configuration_init(&mut request);
                        request.max_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "ipv6-address" => {
                        request_on_premises_configuration_init(&mut request);
                        request.ipv6_address = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert-serial-number" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert_serial_number = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.kind" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.sha1-fingerprint" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().sha1_fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.common-name" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().common_name = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.instance" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().instance = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.expiration-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.create-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().create_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.self-link" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "replica-names" => {
                        request_server_ca_cert_init(&mut request);
                        if request.replica_names.is_none() {
                           request.replica_names = Some(Default::default());
                        }
                                        request.replica_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request_server_ca_cert_init(&mut request);
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "region" => {
                        request_server_ca_cert_init(&mut request);
                        request.region = Some(value.unwrap_or("").to_string());
                    },
                "settings.kind" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.authorized-gae-applications" => {
                        request_settings_init(&mut request);
                        if request.settings.as_mut().unwrap().authorized_gae_applications.is_none() {
                           request.settings.as_mut().unwrap().authorized_gae_applications = Some(Default::default());
                        }
                                        request.settings.as_mut().unwrap().authorized_gae_applications.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "settings.activation-policy" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().activation_policy = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.kind" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.enabled", "boolean"));
                    },
                "settings.backup-configuration.start-time" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.binary-log-enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().binary_log_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.binary-log-enabled", "boolean"));
                    },
                "settings.ip-configuration.ipv4-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().ipv4_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.ipv4-enabled", "boolean"));
                    },
                "settings.ip-configuration.require-ssl" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().require_ssl = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.require-ssl", "boolean"));
                    },
                "settings.tier" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().tier = Some(value.unwrap_or("").to_string());
                    },
                "settings.database-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().database_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.database-replication-enabled", "boolean"));
                    },
                "settings.replication-type" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().replication_type = Some(value.unwrap_or("").to_string());
                    },
                "settings.crash-safe-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().crash_safe_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.crash-safe-replication-enabled", "boolean"));
                    },
                "settings.pricing-plan" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().pricing_plan = Some(value.unwrap_or("").to_string());
                    },
                "settings.settings-version" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().settings_version = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.kind" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.zone" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().zone = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.follow-gae-application" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().follow_gae_application = Some(value.unwrap_or("").to_string());
                    },
                "master-instance-name" => {
                        request_settings_init(&mut request);
                        request.master_instance_name = Some(value.unwrap_or("").to_string());
                    },
                "current-disk-size" => {
                        request_settings_init(&mut request);
                        request.current_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "state" => {
                        request_settings_init(&mut request);
                        request.state = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_settings_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "service-account-email-address" => {
                        request_settings_init(&mut request);
                        request.service_account_email_address = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.kind" => {
                        request_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.username" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().username = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.kind" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().connect_retry_interval = Some(arg_from_str(value.unwrap_or("-0"), err, "replica-configuration.mysql-replica-configuration.connect-retry-interval", "integer"));
                    },
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ssl_cipher = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.ca-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ca_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().master_heartbeat_period = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().verify_server_certificate = Some(arg_from_str(value.unwrap_or("false"), err, "replica-configuration.mysql-replica-configuration.verify-server-certificate", "boolean"));
                    },
                "replica-configuration.mysql-replica-configuration.dump-file-path" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().dump_file_path = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.password" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().password = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-key" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_key = Some(value.unwrap_or("").to_string());
                    },
                "database-version" => {
                        request_replica_configuration_init(&mut request);
                        request.database_version = Some(value.unwrap_or("").to_string());
                    },
                "instance-type" => {
                        request_replica_configuration_init(&mut request);
                        request.instance_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replica_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_replica_configuration_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "authorized-gae-applications", "backup-configuration", "binary-log-enabled", "ca-certificate", "cert", "cert-serial-number", "client-certificate", "client-key", "common-name", "connect-retry-interval", "crash-safe-replication-enabled", "create-time", "current-disk-size", "database-replication-enabled", "database-version", "dump-file-path", "enabled", "etag", "expiration-time", "follow-gae-application", "host-port", "instance", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "location-preference", "master-heartbeat-period", "master-instance-name", "max-disk-size", "mysql-replica-configuration", "name", "on-premises-configuration", "password", "pricing-plan", "project", "region", "replica-configuration", "replica-names", "replication-type", "require-ssl", "self-link", "server-ca-cert", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "ssl-cipher", "start-time", "state", "tier", "username", "verify-server-certificate", "zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().insert(request, opt.value_of("project").unwrap_or(""));
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

    fn _instances_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().list(opt.value_of("project").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _instances_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::DatabaseInstance::default();
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
            fn request_on_premises_configuration_init(request: &mut api::DatabaseInstance) {
                if request.on_premises_configuration.is_none() {
                    request.on_premises_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_init(request: &mut api::DatabaseInstance) {
                if request.replica_configuration.is_none() {
                    request.replica_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_mysql_replica_configuration_init(request: &mut api::DatabaseInstance) {
                request_replica_configuration_init(request);
                if request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.is_none() {
                    request.replica_configuration.as_mut().unwrap().mysql_replica_configuration = Some(Default::default());
                }
            }
            
            fn request_server_ca_cert_init(request: &mut api::DatabaseInstance) {
                if request.server_ca_cert.is_none() {
                    request.server_ca_cert = Some(Default::default());
                }
            }
            
            fn request_settings_backup_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().backup_configuration.is_none() {
                    request.settings.as_mut().unwrap().backup_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_init(request: &mut api::DatabaseInstance) {
                if request.settings.is_none() {
                    request.settings = Some(Default::default());
                }
            }
            
            fn request_settings_ip_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().ip_configuration.is_none() {
                    request.settings.as_mut().unwrap().ip_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_location_preference_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().location_preference.is_none() {
                    request.settings.as_mut().unwrap().location_preference = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "on-premises-configuration.kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "on-premises-configuration.host-port" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().host_port = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "max-disk-size" => {
                        request_on_premises_configuration_init(&mut request);
                        request.max_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "ipv6-address" => {
                        request_on_premises_configuration_init(&mut request);
                        request.ipv6_address = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert-serial-number" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert_serial_number = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.kind" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.sha1-fingerprint" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().sha1_fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.common-name" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().common_name = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.instance" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().instance = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.expiration-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.create-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().create_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.self-link" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "replica-names" => {
                        request_server_ca_cert_init(&mut request);
                        if request.replica_names.is_none() {
                           request.replica_names = Some(Default::default());
                        }
                                        request.replica_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request_server_ca_cert_init(&mut request);
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "region" => {
                        request_server_ca_cert_init(&mut request);
                        request.region = Some(value.unwrap_or("").to_string());
                    },
                "settings.kind" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.authorized-gae-applications" => {
                        request_settings_init(&mut request);
                        if request.settings.as_mut().unwrap().authorized_gae_applications.is_none() {
                           request.settings.as_mut().unwrap().authorized_gae_applications = Some(Default::default());
                        }
                                        request.settings.as_mut().unwrap().authorized_gae_applications.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "settings.activation-policy" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().activation_policy = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.kind" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.enabled", "boolean"));
                    },
                "settings.backup-configuration.start-time" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.binary-log-enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().binary_log_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.binary-log-enabled", "boolean"));
                    },
                "settings.ip-configuration.ipv4-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().ipv4_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.ipv4-enabled", "boolean"));
                    },
                "settings.ip-configuration.require-ssl" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().require_ssl = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.require-ssl", "boolean"));
                    },
                "settings.tier" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().tier = Some(value.unwrap_or("").to_string());
                    },
                "settings.database-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().database_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.database-replication-enabled", "boolean"));
                    },
                "settings.replication-type" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().replication_type = Some(value.unwrap_or("").to_string());
                    },
                "settings.crash-safe-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().crash_safe_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.crash-safe-replication-enabled", "boolean"));
                    },
                "settings.pricing-plan" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().pricing_plan = Some(value.unwrap_or("").to_string());
                    },
                "settings.settings-version" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().settings_version = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.kind" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.zone" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().zone = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.follow-gae-application" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().follow_gae_application = Some(value.unwrap_or("").to_string());
                    },
                "master-instance-name" => {
                        request_settings_init(&mut request);
                        request.master_instance_name = Some(value.unwrap_or("").to_string());
                    },
                "current-disk-size" => {
                        request_settings_init(&mut request);
                        request.current_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "state" => {
                        request_settings_init(&mut request);
                        request.state = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_settings_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "service-account-email-address" => {
                        request_settings_init(&mut request);
                        request.service_account_email_address = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.kind" => {
                        request_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.username" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().username = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.kind" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().connect_retry_interval = Some(arg_from_str(value.unwrap_or("-0"), err, "replica-configuration.mysql-replica-configuration.connect-retry-interval", "integer"));
                    },
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ssl_cipher = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.ca-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ca_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().master_heartbeat_period = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().verify_server_certificate = Some(arg_from_str(value.unwrap_or("false"), err, "replica-configuration.mysql-replica-configuration.verify-server-certificate", "boolean"));
                    },
                "replica-configuration.mysql-replica-configuration.dump-file-path" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().dump_file_path = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.password" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().password = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-key" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_key = Some(value.unwrap_or("").to_string());
                    },
                "database-version" => {
                        request_replica_configuration_init(&mut request);
                        request.database_version = Some(value.unwrap_or("").to_string());
                    },
                "instance-type" => {
                        request_replica_configuration_init(&mut request);
                        request.instance_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replica_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_replica_configuration_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "authorized-gae-applications", "backup-configuration", "binary-log-enabled", "ca-certificate", "cert", "cert-serial-number", "client-certificate", "client-key", "common-name", "connect-retry-interval", "crash-safe-replication-enabled", "create-time", "current-disk-size", "database-replication-enabled", "database-version", "dump-file-path", "enabled", "etag", "expiration-time", "follow-gae-application", "host-port", "instance", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "location-preference", "master-heartbeat-period", "master-instance-name", "max-disk-size", "mysql-replica-configuration", "name", "on-premises-configuration", "password", "pricing-plan", "project", "region", "replica-configuration", "replica-names", "replication-type", "require-ssl", "self-link", "server-ca-cert", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "ssl-cipher", "start-time", "state", "tier", "username", "verify-server-certificate", "zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().patch(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_promote_replica(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().promote_replica(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_reset_ssl_config(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().reset_ssl_config(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_restart(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().restart(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_restore_backup(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::InstancesRestoreBackupRequest::default();
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
            fn request_restore_backup_context_init(request: &mut api::InstancesRestoreBackupRequest) {
                if request.restore_backup_context.is_none() {
                    request.restore_backup_context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "restore-backup-context.kind" => {
                        request_restore_backup_context_init(&mut request);
                        request.restore_backup_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "restore-backup-context.backup-run-id" => {
                        request_restore_backup_context_init(&mut request);
                        request.restore_backup_context.as_mut().unwrap().backup_run_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-run-id", "kind", "restore-backup-context"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().restore_backup(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_start_replica(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().start_replica(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_stop_replica(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().stop_replica(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _instances_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::DatabaseInstance::default();
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
            fn request_on_premises_configuration_init(request: &mut api::DatabaseInstance) {
                if request.on_premises_configuration.is_none() {
                    request.on_premises_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_init(request: &mut api::DatabaseInstance) {
                if request.replica_configuration.is_none() {
                    request.replica_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_mysql_replica_configuration_init(request: &mut api::DatabaseInstance) {
                request_replica_configuration_init(request);
                if request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.is_none() {
                    request.replica_configuration.as_mut().unwrap().mysql_replica_configuration = Some(Default::default());
                }
            }
            
            fn request_server_ca_cert_init(request: &mut api::DatabaseInstance) {
                if request.server_ca_cert.is_none() {
                    request.server_ca_cert = Some(Default::default());
                }
            }
            
            fn request_settings_backup_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().backup_configuration.is_none() {
                    request.settings.as_mut().unwrap().backup_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_init(request: &mut api::DatabaseInstance) {
                if request.settings.is_none() {
                    request.settings = Some(Default::default());
                }
            }
            
            fn request_settings_ip_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().ip_configuration.is_none() {
                    request.settings.as_mut().unwrap().ip_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_location_preference_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().location_preference.is_none() {
                    request.settings.as_mut().unwrap().location_preference = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "on-premises-configuration.kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "on-premises-configuration.host-port" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().host_port = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "max-disk-size" => {
                        request_on_premises_configuration_init(&mut request);
                        request.max_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "ipv6-address" => {
                        request_on_premises_configuration_init(&mut request);
                        request.ipv6_address = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert-serial-number" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert_serial_number = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.kind" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.sha1-fingerprint" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().sha1_fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.common-name" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().common_name = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.instance" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().instance = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.expiration-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.create-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().create_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.self-link" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "replica-names" => {
                        request_server_ca_cert_init(&mut request);
                        if request.replica_names.is_none() {
                           request.replica_names = Some(Default::default());
                        }
                                        request.replica_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request_server_ca_cert_init(&mut request);
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "region" => {
                        request_server_ca_cert_init(&mut request);
                        request.region = Some(value.unwrap_or("").to_string());
                    },
                "settings.kind" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.authorized-gae-applications" => {
                        request_settings_init(&mut request);
                        if request.settings.as_mut().unwrap().authorized_gae_applications.is_none() {
                           request.settings.as_mut().unwrap().authorized_gae_applications = Some(Default::default());
                        }
                                        request.settings.as_mut().unwrap().authorized_gae_applications.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "settings.activation-policy" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().activation_policy = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.kind" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.enabled", "boolean"));
                    },
                "settings.backup-configuration.start-time" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.binary-log-enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().binary_log_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.binary-log-enabled", "boolean"));
                    },
                "settings.ip-configuration.ipv4-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().ipv4_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.ipv4-enabled", "boolean"));
                    },
                "settings.ip-configuration.require-ssl" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().require_ssl = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.require-ssl", "boolean"));
                    },
                "settings.tier" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().tier = Some(value.unwrap_or("").to_string());
                    },
                "settings.database-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().database_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.database-replication-enabled", "boolean"));
                    },
                "settings.replication-type" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().replication_type = Some(value.unwrap_or("").to_string());
                    },
                "settings.crash-safe-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().crash_safe_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.crash-safe-replication-enabled", "boolean"));
                    },
                "settings.pricing-plan" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().pricing_plan = Some(value.unwrap_or("").to_string());
                    },
                "settings.settings-version" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().settings_version = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.kind" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.zone" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().zone = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.follow-gae-application" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().follow_gae_application = Some(value.unwrap_or("").to_string());
                    },
                "master-instance-name" => {
                        request_settings_init(&mut request);
                        request.master_instance_name = Some(value.unwrap_or("").to_string());
                    },
                "current-disk-size" => {
                        request_settings_init(&mut request);
                        request.current_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "state" => {
                        request_settings_init(&mut request);
                        request.state = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_settings_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "service-account-email-address" => {
                        request_settings_init(&mut request);
                        request.service_account_email_address = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.kind" => {
                        request_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.username" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().username = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.kind" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().connect_retry_interval = Some(arg_from_str(value.unwrap_or("-0"), err, "replica-configuration.mysql-replica-configuration.connect-retry-interval", "integer"));
                    },
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ssl_cipher = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.ca-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ca_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().master_heartbeat_period = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().verify_server_certificate = Some(arg_from_str(value.unwrap_or("false"), err, "replica-configuration.mysql-replica-configuration.verify-server-certificate", "boolean"));
                    },
                "replica-configuration.mysql-replica-configuration.dump-file-path" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().dump_file_path = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.password" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().password = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-key" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_key = Some(value.unwrap_or("").to_string());
                    },
                "database-version" => {
                        request_replica_configuration_init(&mut request);
                        request.database_version = Some(value.unwrap_or("").to_string());
                    },
                "instance-type" => {
                        request_replica_configuration_init(&mut request);
                        request.instance_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replica_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_replica_configuration_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "authorized-gae-applications", "backup-configuration", "binary-log-enabled", "ca-certificate", "cert", "cert-serial-number", "client-certificate", "client-key", "common-name", "connect-retry-interval", "crash-safe-replication-enabled", "create-time", "current-disk-size", "database-replication-enabled", "database-version", "dump-file-path", "enabled", "etag", "expiration-time", "follow-gae-application", "host-port", "instance", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "location-preference", "master-heartbeat-period", "master-instance-name", "max-disk-size", "mysql-replica-configuration", "name", "on-premises-configuration", "password", "pricing-plan", "project", "region", "replica-configuration", "replica-names", "replication-type", "require-ssl", "self-link", "server-ca-cert", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "ssl-cipher", "start-time", "state", "tier", "username", "verify-server-certificate", "zone"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.instances().update(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _operations_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().get(opt.value_of("project").unwrap_or(""), opt.value_of("operation").unwrap_or(""));
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

    fn _operations_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _ssl_certs_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("sha1-fingerprint").unwrap_or(""));
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

    fn _ssl_certs_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("sha1-fingerprint").unwrap_or(""));
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

    fn _ssl_certs_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::SslCertsInsertRequest::default();
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
                "common-name" => {
                        request.common_name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["common-name"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.ssl_certs().insert(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _ssl_certs_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _tiers_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tiers().list(opt.value_of("project").unwrap_or(""));
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

    fn _users_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("host").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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

    fn _users_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::User::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "host" => {
                        request.host = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "host", "instance", "kind", "name", "password", "project"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().insert(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _users_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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

    fn _users_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::User::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "host" => {
                        request.host = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "host", "instance", "kind", "name", "password", "project"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().update(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("host").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("backup-runs", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._backup_runs_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._backup_runs_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("backup-runs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("databases", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._databases_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._databases_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._databases_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._databases_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._databases_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._databases_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("databases".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("flags", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._flags_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("flags".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("instances", Some(opt)) => {
                match opt.subcommand() {
                    ("clone", Some(opt)) => {
                        call_result = self._instances_clone(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._instances_delete(opt, dry_run, &mut err);
                    },
                    ("export", Some(opt)) => {
                        call_result = self._instances_export(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._instances_get(opt, dry_run, &mut err);
                    },
                    ("import", Some(opt)) => {
                        call_result = self._instances_import(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._instances_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._instances_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._instances_patch(opt, dry_run, &mut err);
                    },
                    ("promote-replica", Some(opt)) => {
                        call_result = self._instances_promote_replica(opt, dry_run, &mut err);
                    },
                    ("reset-ssl-config", Some(opt)) => {
                        call_result = self._instances_reset_ssl_config(opt, dry_run, &mut err);
                    },
                    ("restart", Some(opt)) => {
                        call_result = self._instances_restart(opt, dry_run, &mut err);
                    },
                    ("restore-backup", Some(opt)) => {
                        call_result = self._instances_restore_backup(opt, dry_run, &mut err);
                    },
                    ("start-replica", Some(opt)) => {
                        call_result = self._instances_start_replica(opt, dry_run, &mut err);
                    },
                    ("stop-replica", Some(opt)) => {
                        call_result = self._instances_stop_replica(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._instances_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("instances".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("operations", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._operations_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._operations_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("ssl-certs", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._ssl_certs_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._ssl_certs_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._ssl_certs_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._ssl_certs_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("ssl-certs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tiers", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._tiers_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tiers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("users", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._users_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._users_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._users_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._users_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("users".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "sqladmin1-beta4-secret.json", 
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
                                          program_name: "sqladmin1-beta4",
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
            hub: api::SQLAdmin::new(client, auth),
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
        ("backup-runs", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Retrieves a resource containing information about a backup run."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/backup-runs_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of this Backup Run."##),
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
                    Some(r##"Lists all backup runs associated with a given instance and configuration in the reverse chronological order of the enqueued time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/backup-runs_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("databases", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be deleted in the instance."##),
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
                    Some(r##"Retrieves a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database in the instance."##),
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
                    Some(r##"Inserts a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Lists databases in the specified Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Updates a resource containing information about a database inside a Cloud SQL instance. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_patch",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be updated in the instance."##),
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
                    Some(r##"Updates a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be updated in the instance."##),
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
        
        ("flags", "methods: 'list'", vec![
            ("list",  
                    Some(r##"List all available database flags for Google Cloud SQL instances."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/flags_list",
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
        
        ("instances", "methods: 'clone', 'delete', 'export', 'get', 'import', 'insert', 'list', 'patch', 'promote-replica', 'reset-ssl-config', 'restart', 'restore-backup', 'start-replica', 'stop-replica' and 'update'", vec![
            ("clone",  
                    Some(r##"Creates a Cloud SQL instance as a clone of the source instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_clone",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the source as well as the clone Cloud SQL instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"The ID of the Cloud SQL instance to be cloned (source). This does not include the project ID."##),
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
                    Some(r##"Deletes a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("export",  
                    Some(r##"Exports data from a Cloud SQL instance to a Google Cloud Storage bucket as a MySQL dump file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_export",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be exported."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("get",  
                    Some(r##"Retrieves a resource containing information about a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Imports data into a Cloud SQL instance from a MySQL dump file in Google Cloud Storage."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_import",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Creates a new Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project to which the newly created Cloud SQL instances should belong."##),
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
                    Some(r##"Lists instances under a given project in the alphabetical order of the instance name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
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
                    Some(r##"Updates settings of a Cloud SQL instance. Caution: This is not a partial update, so you must include values for all the settings that you want to retain. For partial updates, use patch.. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_patch",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("promote-replica",  
                    Some(r##"Promotes the read replica instance to be a stand-alone Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_promote-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("reset-ssl-config",  
                    Some(r##"Deletes all client certificates and generates a new server SSL certificate for the instance. The changes will not take effect until the instance is restarted. Existing instances without a server certificate will need to call this once to set a server certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_reset-ssl-config",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("restart",  
                    Some(r##"Restarts a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_restart",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be restarted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("restore-backup",  
                    Some(r##"Restores a backup of a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_restore-backup",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("start-replica",  
                    Some(r##"Starts the replication in the read replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_start-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("stop-replica",  
                    Some(r##"Stops the replication in the read replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_stop-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
                    Some(r##"Updates settings of a Cloud SQL instance. Caution: This is not a partial update, so you must include values for all the settings that you want to retain. For partial updates, use patch."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("operations", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Retrieves an instance operation that has been performed on an instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/operations_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operation"##),
                     None,
                     Some(r##"Instance operation ID."##),
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
                    Some(r##"Lists all instance operations that have been performed on the given Cloud SQL instance in the reverse chronological order of the start time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/operations_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("ssl-certs", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",  
                    Some(r##"Deletes the SSL certificate. The change will not take effect until the instance is restarted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sha1-fingerprint"##),
                     None,
                     Some(r##"Sha1 FingerPrint."##),
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
                    Some(r##"Retrieves a particular SSL certificate. Does not include the private key (required for usage). The private key must be saved from the response to initial creation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sha1-fingerprint"##),
                     None,
                     Some(r##"Sha1 FingerPrint."##),
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
                    Some(r##"Creates an SSL certificate and returns it along with the private key and server certificate authority. The new certificate will not be usable until the instance is restarted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project to which the newly created Cloud SQL instances should belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Lists all of the current SSL certificates for the instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("tiers", "methods: 'list'", vec![
            ("list",  
                    Some(r##"Lists all available service tiers for Google Cloud SQL, for example D1, D2. For related information, see Pricing."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/tiers_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list tiers."##),
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
        
        ("users", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes a user from a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"host"##),
                     None,
                     Some(r##"Host of the user in the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name of the user in the instance."##),
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
                    Some(r##"Creates a new user in a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Lists users in the specified Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Updates an existing user in a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"host"##),
                     None,
                     Some(r##"Host of the user in the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name of the user in the instance."##),
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
    
    let mut app = App::new("sqladmin1-beta4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150305")
           .about("API for Cloud SQL database instance management.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli")
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