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
extern crate google_storage1 as api;

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
    hub: api::Storage<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _bucket_access_controls_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bucket_access_controls().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _bucket_access_controls_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bucket_access_controls().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _bucket_access_controls_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::BucketAccessControl::default();
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
            fn request_project_team_init(request: &mut api::BucketAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "id", "kind", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.bucket_access_controls().insert(request, opt.value_of("bucket").unwrap_or(""));
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

    fn _bucket_access_controls_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bucket_access_controls().list(opt.value_of("bucket").unwrap_or(""));
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

    fn _bucket_access_controls_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::BucketAccessControl::default();
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
            fn request_project_team_init(request: &mut api::BucketAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "id", "kind", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.bucket_access_controls().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _bucket_access_controls_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::BucketAccessControl::default();
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
            fn request_project_team_init(request: &mut api::BucketAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "id", "kind", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.bucket_access_controls().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _buckets_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().delete(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-metageneration-not-match", "if-metageneration-match"]
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

    fn _buckets_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().get(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-metageneration-match", "if-metageneration-not-match", "projection"]
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

    fn _buckets_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Bucket::default();
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
            fn request_logging_init(request: &mut api::Bucket) {
                if request.logging.is_none() {
                    request.logging = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::Bucket) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            fn request_versioning_init(request: &mut api::Bucket) {
                if request.versioning.is_none() {
                    request.versioning = Some(Default::default());
                }
            }
            
            fn request_website_init(request: &mut api::Bucket) {
                if request.website.is_none() {
                    request.website = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website.not-found-page" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().not_found_page = Some(value.unwrap_or("").to_string());
                    },
                "website.main-page-suffix" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().main_page_suffix = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_website_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_website_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-object-prefix" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_object_prefix = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-bucket" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_bucket = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_logging_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "time-created" => {
                        request_logging_init(&mut request);
                        request.time_created = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_logging_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request_logging_init(&mut request);
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_logging_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_logging_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "versioning.enabled" => {
                        request_versioning_init(&mut request);
                        request.versioning.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "versioning.enabled", "boolean"));
                    },
                "storage-class" => {
                        request_versioning_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["enabled", "entity", "entity-id", "etag", "id", "kind", "location", "log-bucket", "log-object-prefix", "logging", "main-page-suffix", "metageneration", "name", "not-found-page", "owner", "project-number", "self-link", "storage-class", "time-created", "versioning", "website"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.buckets().insert(request, opt.value_of("project").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["predefined-acl", "projection", "predefined-default-object-acl"]
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

    fn _buckets_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().list(opt.value_of("project").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["page-token", "prefix", "projection", "max-results"]
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

    fn _buckets_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Bucket::default();
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
            fn request_logging_init(request: &mut api::Bucket) {
                if request.logging.is_none() {
                    request.logging = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::Bucket) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            fn request_versioning_init(request: &mut api::Bucket) {
                if request.versioning.is_none() {
                    request.versioning = Some(Default::default());
                }
            }
            
            fn request_website_init(request: &mut api::Bucket) {
                if request.website.is_none() {
                    request.website = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website.not-found-page" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().not_found_page = Some(value.unwrap_or("").to_string());
                    },
                "website.main-page-suffix" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().main_page_suffix = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_website_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_website_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-object-prefix" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_object_prefix = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-bucket" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_bucket = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_logging_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "time-created" => {
                        request_logging_init(&mut request);
                        request.time_created = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_logging_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request_logging_init(&mut request);
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_logging_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_logging_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "versioning.enabled" => {
                        request_versioning_init(&mut request);
                        request.versioning.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "versioning.enabled", "boolean"));
                    },
                "storage-class" => {
                        request_versioning_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["enabled", "entity", "entity-id", "etag", "id", "kind", "location", "log-bucket", "log-object-prefix", "logging", "main-page-suffix", "metageneration", "name", "not-found-page", "owner", "project-number", "self-link", "storage-class", "time-created", "versioning", "website"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.buckets().patch(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-metageneration-match", "if-metageneration-not-match", "predefined-acl", "projection", "predefined-default-object-acl"]
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

    fn _buckets_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Bucket::default();
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
            fn request_logging_init(request: &mut api::Bucket) {
                if request.logging.is_none() {
                    request.logging = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::Bucket) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            fn request_versioning_init(request: &mut api::Bucket) {
                if request.versioning.is_none() {
                    request.versioning = Some(Default::default());
                }
            }
            
            fn request_website_init(request: &mut api::Bucket) {
                if request.website.is_none() {
                    request.website = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website.not-found-page" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().not_found_page = Some(value.unwrap_or("").to_string());
                    },
                "website.main-page-suffix" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().main_page_suffix = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_website_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_website_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-object-prefix" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_object_prefix = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-bucket" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_bucket = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_logging_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "time-created" => {
                        request_logging_init(&mut request);
                        request.time_created = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_logging_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request_logging_init(&mut request);
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_logging_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_logging_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "versioning.enabled" => {
                        request_versioning_init(&mut request);
                        request.versioning.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "versioning.enabled", "boolean"));
                    },
                "storage-class" => {
                        request_versioning_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["enabled", "entity", "entity-id", "etag", "id", "kind", "location", "log-bucket", "log-object-prefix", "logging", "main-page-suffix", "metageneration", "name", "not-found-page", "owner", "project-number", "self-link", "storage-class", "time-created", "versioning", "website"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.buckets().update(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-metageneration-match", "if-metageneration-not-match", "predefined-acl", "projection", "predefined-default-object-acl"]
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

    fn _default_object_access_controls_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.default_object_access_controls().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _default_object_access_controls_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.default_object_access_controls().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _default_object_access_controls_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.default_object_access_controls().insert(request, opt.value_of("bucket").unwrap_or(""));
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

    fn _default_object_access_controls_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.default_object_access_controls().list(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-metageneration-not-match", "if-metageneration-match"]
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

    fn _default_object_access_controls_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.default_object_access_controls().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _default_object_access_controls_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.default_object_access_controls().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
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

    fn _object_access_controls_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.object_access_controls().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation"]
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

    fn _object_access_controls_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.object_access_controls().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation"]
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

    fn _object_access_controls_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.object_access_controls().insert(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation"]
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

    fn _object_access_controls_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.object_access_controls().list(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation"]
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

    fn _object_access_controls_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.object_access_controls().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation"]
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

    fn _object_access_controls_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.object_access_controls().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation"]
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

    fn _objects_compose(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ComposeRequest::default();
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
            fn request_destination_init(request: &mut api::ComposeRequest) {
                if request.destination.is_none() {
                    request.destination = Some(Default::default());
                }
            }
            
            fn request_destination_owner_init(request: &mut api::ComposeRequest) {
                request_destination_init(request);
                if request.destination.as_mut().unwrap().owner.is_none() {
                    request.destination.as_mut().unwrap().owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "destination.self-link" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "destination.generation" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().generation = Some(value.unwrap_or("").to_string());
                    },
                "destination.component-count" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "destination.component-count", "integer"));
                    },
                "destination.media-link" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().media_link = Some(value.unwrap_or("").to_string());
                    },
                "destination.owner.entity-id" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "destination.owner.entity" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "destination.cache-control" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().cache_control = Some(value.unwrap_or("").to_string());
                    },
                "destination.id" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "destination.size" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "destination.time-deleted" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "destination.md5-hash" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "destination.crc32c" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().crc32c = Some(value.unwrap_or("").to_string());
                    },
                "destination.etag" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "destination.metadata" => {
                        request_destination_owner_init(&mut request);
                        if request.destination.as_mut().unwrap().metadata.is_none() {
                           request.destination.as_mut().unwrap().metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.destination.as_mut().unwrap().metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "destination.updated" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-type" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_type = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-language" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_language = Some(value.unwrap_or("").to_string());
                    },
                "destination.metageneration" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().metageneration = Some(value.unwrap_or("").to_string());
                    },
                "destination.kind" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "destination.name" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "destination.bucket" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().bucket = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-encoding" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "destination.storage-class" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().storage_class = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-disposition" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "destination", "entity", "entity-id", "etag", "generation", "id", "kind", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "self-link", "size", "storage-class", "time-deleted", "updated"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.objects().compose(request, opt.value_of("destination-bucket").unwrap_or(""), opt.value_of("destination-object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-metageneration-match", "if-generation-match", "destination-predefined-acl"]
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _objects_copy(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "entity", "entity-id", "etag", "generation", "id", "kind", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "self-link", "size", "storage-class", "time-deleted", "updated"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.objects().copy(request, opt.value_of("source-bucket").unwrap_or(""), opt.value_of("source-object").unwrap_or(""), opt.value_of("destination-bucket").unwrap_or(""), opt.value_of("destination-object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source-generation" => {
                    call = call.source_generation(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-source-metageneration-not-match" => {
                    call = call.if_source_metageneration_not_match(value.unwrap_or(""));
                },
                "if-source-metageneration-match" => {
                    call = call.if_source_metageneration_match(value.unwrap_or(""));
                },
                "if-source-generation-not-match" => {
                    call = call.if_source_generation_not_match(value.unwrap_or(""));
                },
                "if-source-generation-match" => {
                    call = call.if_source_generation_match(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-source-generation-match", "projection", "if-source-metageneration-not-match", "if-metageneration-not-match", "source-generation", "destination-predefined-acl", "if-source-generation-not-match", "if-source-metageneration-match", "if-generation-match", "if-metageneration-match", "if-generation-not-match"]
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _objects_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.objects().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["generation", "if-metageneration-not-match", "if-generation-match", "if-metageneration-match", "if-generation-not-match"]
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

    fn _objects_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.objects().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["projection", "generation", "if-metageneration-match", "if-generation-match", "if-metageneration-not-match", "if-generation-not-match"]
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _objects_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "entity", "entity-id", "etag", "generation", "id", "kind", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "self-link", "size", "storage-class", "time-deleted", "updated"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.objects().insert(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "content-encoding" => {
                    call = call.content_encoding(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-generation-match", "if-generation-not-match", "content-encoding", "if-metageneration-match", "name", "predefined-acl", "if-metageneration-not-match", "projection"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    io::copy(&mut response, &mut ostream).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.objects().list(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "versions" => {
                    call = call.versions(arg_from_str(value.unwrap_or("false"), err, "versions", "boolean"));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["projection", "versions", "prefix", "max-results", "page-token", "delimiter"]
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

    fn _objects_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "entity", "entity-id", "etag", "generation", "id", "kind", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "self-link", "size", "storage-class", "time-deleted", "updated"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.objects().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-generation-match", "projection", "generation", "if-metageneration-match", "predefined-acl", "if-metageneration-not-match", "if-generation-not-match"]
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

    fn _objects_rewrite(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "entity", "entity-id", "etag", "generation", "id", "kind", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "self-link", "size", "storage-class", "time-deleted", "updated"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.objects().rewrite(request, opt.value_of("source-bucket").unwrap_or(""), opt.value_of("source-object").unwrap_or(""), opt.value_of("destination-bucket").unwrap_or(""), opt.value_of("destination-object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source-generation" => {
                    call = call.source_generation(value.unwrap_or(""));
                },
                "rewrite-token" => {
                    call = call.rewrite_token(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "max-bytes-rewritten-per-call" => {
                    call = call.max_bytes_rewritten_per_call(value.unwrap_or(""));
                },
                "if-source-metageneration-not-match" => {
                    call = call.if_source_metageneration_not_match(value.unwrap_or(""));
                },
                "if-source-metageneration-match" => {
                    call = call.if_source_metageneration_match(value.unwrap_or(""));
                },
                "if-source-generation-not-match" => {
                    call = call.if_source_generation_not_match(value.unwrap_or(""));
                },
                "if-source-generation-match" => {
                    call = call.if_source_generation_match(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-source-generation-match", "if-generation-match", "projection", "if-source-metageneration-not-match", "if-metageneration-not-match", "source-generation", "max-bytes-rewritten-per-call", "if-source-generation-not-match", "destination-predefined-acl", "if-source-metageneration-match", "rewrite-token", "if-metageneration-match", "if-generation-not-match"]
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

    fn _objects_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "entity", "entity-id", "etag", "generation", "id", "kind", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "self-link", "size", "storage-class", "time-deleted", "updated"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.objects().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["if-generation-match", "projection", "generation", "if-metageneration-match", "predefined-acl", "if-metageneration-not-match", "if-generation-not-match"]
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _objects_watch_all(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.objects().watch_all(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "versions" => {
                    call = call.versions(arg_from_str(value.unwrap_or("false"), err, "versions", "boolean"));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["projection", "versions", "prefix", "max-results", "page-token", "delimiter"]
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
            ("bucket-access-controls", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._bucket_access_controls_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._bucket_access_controls_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._bucket_access_controls_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._bucket_access_controls_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._bucket_access_controls_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._bucket_access_controls_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("bucket-access-controls".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("buckets", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._buckets_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._buckets_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._buckets_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._buckets_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._buckets_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._buckets_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("buckets".to_string()));
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
            ("default-object-access-controls", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._default_object_access_controls_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._default_object_access_controls_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._default_object_access_controls_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._default_object_access_controls_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._default_object_access_controls_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._default_object_access_controls_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("default-object-access-controls".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("object-access-controls", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._object_access_controls_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._object_access_controls_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._object_access_controls_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._object_access_controls_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._object_access_controls_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._object_access_controls_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("object-access-controls".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("objects", Some(opt)) => {
                match opt.subcommand() {
                    ("compose", Some(opt)) => {
                        call_result = self._objects_compose(opt, dry_run, &mut err);
                    },
                    ("copy", Some(opt)) => {
                        call_result = self._objects_copy(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._objects_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._objects_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._objects_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._objects_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._objects_patch(opt, dry_run, &mut err);
                    },
                    ("rewrite", Some(opt)) => {
                        call_result = self._objects_rewrite(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._objects_update(opt, dry_run, &mut err);
                    },
                    ("watch-all", Some(opt)) => {
                        call_result = self._objects_watch_all(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("objects".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "storage1-secret.json", 
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
                                          program_name: "storage1",
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
            hub: api::Storage::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("bucket-access-controls", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Permanently deletes the ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns the ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
                    Some(r##"Creates a new ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    Some(r##"Retrieves ACL entries on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    Some(r##"Updates an ACL entry on the specified bucket. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
                    Some(r##"Updates an ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
        
        ("buckets", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Permanently deletes an empty bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns metadata for the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    Some(r##"Creates a new bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"A valid API project identifier."##),
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
                    Some(r##"Retrieves a list of buckets for a given project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"A valid API project identifier."##),
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
                    Some(r##"Updates a bucket. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    Some(r##"Updates a bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/channels_stop",
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
        
        ("default-object-access-controls", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Permanently deletes the default object ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns the default object ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
                    Some(r##"Creates a new default object ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    Some(r##"Retrieves default object ACL entries on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
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
                    Some(r##"Updates a default object ACL entry on the specified bucket. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
                    Some(r##"Updates a default object ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
        
        ("object-access-controls", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Permanently deletes the ACL entry for the specified entity on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns the ACL entry for the specified entity on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
                    Some(r##"Creates a new ACL entry on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
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
                    Some(r##"Retrieves ACL entries on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
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
                    Some(r##"Updates an ACL entry on the specified object. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
                    Some(r##"Updates an ACL entry on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
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
        
        ("objects", "methods: 'compose', 'copy', 'delete', 'get', 'insert', 'list', 'patch', 'rewrite', 'update' and 'watch-all'", vec![
            ("compose",  
                    Some(r##"Concatenates a list of existing objects into a new object in the same bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_compose",
                  vec![
                    (Some(r##"destination-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-object"##),
                     None,
                     Some(r##"Name of the new object."##),
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
            ("copy",  
                    Some(r##"Copies an object to a specified location. Optionally overrides metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_copy",
                  vec![
                    (Some(r##"source-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to find the source object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"source-object"##),
                     None,
                     Some(r##"Name of the source object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-object"##),
                     None,
                     Some(r##"Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any."##),
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
                    Some(r##"Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Retrieves an object or its metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
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
                    Some(r##"Stores a new object and metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any."##),
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
            ("list",  
                    Some(r##"Retrieves a list of objects matching the criteria."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to look for objects."##),
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
                    Some(r##"Updates an object's metadata. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
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
            ("rewrite",  
                    Some(r##"Rewrites a source object to a destination object. Optionally overrides metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_rewrite",
                  vec![
                    (Some(r##"source-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to find the source object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"source-object"##),
                     None,
                     Some(r##"Name of the source object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-object"##),
                     None,
                     Some(r##"Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any."##),
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
                    Some(r##"Updates an object's metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object."##),
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
            ("watch-all",  
                    Some(r##"Watch for changes on all objects in a bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_watch-all",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to look for objects."##),
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
    
    let mut app = App::new("storage1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150326")
           .about("Lets you store and retrieve potentially-large, immutable data objects.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_storage1_cli")
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