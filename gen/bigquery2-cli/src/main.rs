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
extern crate google_bigquery2 as api;

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
    hub: api::Bigquery<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _datasets_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "delete-contents" => {
                    call = call.delete_contents(arg_from_str(value.unwrap_or("false"), err, "delete-contents", "boolean"));
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
                                                Vec::new() + &self.gp + &["delete-contents"]
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

    fn _datasets_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    fn _datasets_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Dataset::default();
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
            fn request_dataset_reference_init(request: &mut api::Dataset) {
                if request.dataset_reference.is_none() {
                    request.dataset_reference = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.project-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.dataset-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_dataset_reference_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "default-table-expiration-ms" => {
                        request_dataset_reference_init(&mut request);
                        request.default_table_expiration_ms = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_dataset_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_dataset_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_dataset_reference_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_dataset_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_dataset_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "dataset-reference", "default-table-expiration-ms", "description", "etag", "friendly-name", "id", "kind", "last-modified-time", "project-id", "self-link"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().insert(request, opt.value_of("project-id").unwrap_or(""));
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

    fn _datasets_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "all" => {
                    call = call.all(arg_from_str(value.unwrap_or("false"), err, "all", "boolean"));
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
                                                Vec::new() + &self.gp + &["page-token", "all", "max-results"]
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

    fn _datasets_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Dataset::default();
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
            fn request_dataset_reference_init(request: &mut api::Dataset) {
                if request.dataset_reference.is_none() {
                    request.dataset_reference = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.project-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.dataset-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_dataset_reference_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "default-table-expiration-ms" => {
                        request_dataset_reference_init(&mut request);
                        request.default_table_expiration_ms = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_dataset_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_dataset_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_dataset_reference_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_dataset_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_dataset_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "dataset-reference", "default-table-expiration-ms", "description", "etag", "friendly-name", "id", "kind", "last-modified-time", "project-id", "self-link"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    fn _datasets_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Dataset::default();
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
            fn request_dataset_reference_init(request: &mut api::Dataset) {
                if request.dataset_reference.is_none() {
                    request.dataset_reference = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.project-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.dataset-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_dataset_reference_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "default-table-expiration-ms" => {
                        request_dataset_reference_init(&mut request);
                        request.default_table_expiration_ms = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_dataset_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_dataset_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_dataset_reference_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_dataset_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_dataset_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "dataset-reference", "default-table-expiration-ms", "description", "etag", "friendly-name", "id", "kind", "last-modified-time", "project-id", "self-link"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    fn _jobs_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
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

    fn _jobs_get_query_results(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().get_query_results(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "timeout-ms" => {
                    call = call.timeout_ms(arg_from_str(value.unwrap_or("-0"), err, "timeout-ms", "integer"));
                },
                "start-index" => {
                    call = call.start_index(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["timeout-ms", "page-token", "start-index", "max-results"]
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

    fn _jobs_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Job::default();
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
            fn request_configuration_copy_destination_table_init(request: &mut api::Job) {
                request_configuration_copy_init(request);
                if request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_copy_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().copy.is_none() {
                    request.configuration.as_mut().unwrap().copy = Some(Default::default());
                }
            }
            
            fn request_configuration_copy_source_table_init(request: &mut api::Job) {
                request_configuration_copy_init(request);
                if request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.is_none() {
                    request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table = Some(Default::default());
                }
            }
            
            fn request_configuration_extract_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().extract.is_none() {
                    request.configuration.as_mut().unwrap().extract = Some(Default::default());
                }
            }
            
            fn request_configuration_extract_source_table_init(request: &mut api::Job) {
                request_configuration_extract_init(request);
                if request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.is_none() {
                    request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table = Some(Default::default());
                }
            }
            
            fn request_configuration_init(request: &mut api::Job) {
                if request.configuration.is_none() {
                    request.configuration = Some(Default::default());
                }
            }
            
            fn request_configuration_link_destination_table_init(request: &mut api::Job) {
                request_configuration_link_init(request);
                if request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_link_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().link.is_none() {
                    request.configuration.as_mut().unwrap().link = Some(Default::default());
                }
            }
            
            fn request_configuration_load_destination_table_init(request: &mut api::Job) {
                request_configuration_load_init(request);
                if request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_load_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().load.is_none() {
                    request.configuration.as_mut().unwrap().load = Some(Default::default());
                }
            }
            
            fn request_configuration_query_default_dataset_init(request: &mut api::Job) {
                request_configuration_query_init(request);
                if request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset.is_none() {
                    request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset = Some(Default::default());
                }
            }
            
            fn request_configuration_query_destination_table_init(request: &mut api::Job) {
                request_configuration_query_init(request);
                if request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_query_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().query.is_none() {
                    request.configuration.as_mut().unwrap().query = Some(Default::default());
                }
            }
            
            fn request_job_reference_init(request: &mut api::Job) {
                if request.job_reference.is_none() {
                    request.job_reference = Some(Default::default());
                }
            }
            
            fn request_statistics_extract_init(request: &mut api::Job) {
                request_statistics_init(request);
                if request.statistics.as_mut().unwrap().extract.is_none() {
                    request.statistics.as_mut().unwrap().extract = Some(Default::default());
                }
            }
            
            fn request_statistics_init(request: &mut api::Job) {
                if request.statistics.is_none() {
                    request.statistics = Some(Default::default());
                }
            }
            
            fn request_statistics_load_init(request: &mut api::Job) {
                request_statistics_init(request);
                if request.statistics.as_mut().unwrap().load.is_none() {
                    request.statistics.as_mut().unwrap().load = Some(Default::default());
                }
            }
            
            fn request_statistics_query_init(request: &mut api::Job) {
                request_statistics_init(request);
                if request.statistics.as_mut().unwrap().query.is_none() {
                    request.statistics.as_mut().unwrap().query = Some(Default::default());
                }
            }
            
            fn request_status_error_result_init(request: &mut api::Job) {
                request_status_init(request);
                if request.status.as_mut().unwrap().error_result.is_none() {
                    request.status.as_mut().unwrap().error_result = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Job) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.state" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().state = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.debug-info" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().debug_info = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.message" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().message = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.reason" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().reason = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.location" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().location = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.output-rows" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().output_rows = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.input-files" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().input_files = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.input-file-bytes" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().input_file_bytes = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.output-bytes" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().output_bytes = Some(value.unwrap_or("").to_string());
                    },
                "statistics.creation-time" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().creation_time = Some(value.unwrap_or("").to_string());
                    },
                "statistics.total-bytes-processed" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().total_bytes_processed = Some(value.unwrap_or("").to_string());
                    },
                "statistics.start-time" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "statistics.query.cache-hit" => {
                        request_statistics_query_init(&mut request);
                        request.statistics.as_mut().unwrap().query.as_mut().unwrap().cache_hit = Some(arg_from_str(value.unwrap_or("false"), err, "statistics.query.cache-hit", "boolean"));
                    },
                "statistics.query.total-bytes-processed" => {
                        request_statistics_query_init(&mut request);
                        request.statistics.as_mut().unwrap().query.as_mut().unwrap().total_bytes_processed = Some(value.unwrap_or("").to_string());
                    },
                "statistics.end-time" => {
                        request_statistics_query_init(&mut request);
                        request.statistics.as_mut().unwrap().end_time = Some(value.unwrap_or("").to_string());
                    },
                "statistics.extract.destination-uri-file-counts" => {
                        request_statistics_extract_init(&mut request);
                        if request.statistics.as_mut().unwrap().extract.as_mut().unwrap().destination_uri_file_counts.is_none() {
                           request.statistics.as_mut().unwrap().extract.as_mut().unwrap().destination_uri_file_counts = Some(Default::default());
                        }
                                        request.statistics.as_mut().unwrap().extract.as_mut().unwrap().destination_uri_file_counts.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "statistics.extract.destination-uri-file-counts", "int64"));
                    },
                "job-reference.project-id" => {
                        request_job_reference_init(&mut request);
                        request.job_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "job-reference.job-id" => {
                        request_job_reference_init(&mut request);
                        request.job_reference.as_mut().unwrap().job_id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_job_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "user-email" => {
                        request_job_reference_init(&mut request);
                        request.user_email = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.encoding" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().encoding = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.skip-leading-rows" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().skip_leading_rows = Some(arg_from_str(value.unwrap_or("-0"), err, "configuration.load.skip-leading-rows", "integer"));
                    },
                "configuration.load.quote" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().quote = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.source-format" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_format = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.destination-table.project-id" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.destination-table.table-id" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.destination-table.dataset-id" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.max-bad-records" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().max_bad_records = Some(arg_from_str(value.unwrap_or("-0"), err, "configuration.load.max-bad-records", "integer"));
                    },
                "configuration.load.allow-jagged-rows" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().allow_jagged_rows = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.load.allow-jagged-rows", "boolean"));
                    },
                "configuration.load.write-disposition" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.source-uris" => {
                        request_configuration_load_destination_table_init(&mut request);
                        if request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_uris.is_none() {
                           request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_uris = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.load.field-delimiter" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().field_delimiter = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.create-disposition" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.schema-inline-format" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().schema_inline_format = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.schema-inline" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().schema_inline = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.allow-quoted-newlines" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().allow_quoted_newlines = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.load.allow-quoted-newlines", "boolean"));
                    },
                "configuration.load.projection-fields" => {
                        request_configuration_load_destination_table_init(&mut request);
                        if request.configuration.as_mut().unwrap().load.as_mut().unwrap().projection_fields.is_none() {
                           request.configuration.as_mut().unwrap().load.as_mut().unwrap().projection_fields = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().projection_fields.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.load.ignore-unknown-values" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().ignore_unknown_values = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.load.ignore-unknown-values", "boolean"));
                    },
                "configuration.dry-run" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().dry_run = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.dry-run", "boolean"));
                    },
                "configuration.link.create-disposition" => {
                        request_configuration_link_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.write-disposition" => {
                        request_configuration_link_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.destination-table.project-id" => {
                        request_configuration_link_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.destination-table.table-id" => {
                        request_configuration_link_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.destination-table.dataset-id" => {
                        request_configuration_link_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.source-uri" => {
                        request_configuration_link_destination_table_init(&mut request);
                        if request.configuration.as_mut().unwrap().link.as_mut().unwrap().source_uri.is_none() {
                           request.configuration.as_mut().unwrap().link.as_mut().unwrap().source_uri = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().source_uri.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.query.flatten-results" => {
                        request_configuration_query_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().flatten_results = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.flatten-results", "boolean"));
                    },
                "configuration.query.use-query-cache" => {
                        request_configuration_query_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().use_query_cache = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.use-query-cache", "boolean"));
                    },
                "configuration.query.default-dataset.project-id" => {
                        request_configuration_query_default_dataset_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.default-dataset.dataset-id" => {
                        request_configuration_query_default_dataset_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.destination-table.project-id" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.destination-table.table-id" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.destination-table.dataset-id" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.priority" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().priority = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.write-disposition" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.allow-large-results" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().allow_large_results = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.allow-large-results", "boolean"));
                    },
                "configuration.query.create-disposition" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.query" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.preserve-nulls" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().preserve_nulls = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.preserve-nulls", "boolean"));
                    },
                "configuration.copy.create-disposition" => {
                        request_configuration_copy_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.write-disposition" => {
                        request_configuration_copy_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.destination-table.project-id" => {
                        request_configuration_copy_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.destination-table.table-id" => {
                        request_configuration_copy_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.destination-table.dataset-id" => {
                        request_configuration_copy_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.source-table.project-id" => {
                        request_configuration_copy_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.source-table.table-id" => {
                        request_configuration_copy_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.source-table.dataset-id" => {
                        request_configuration_copy_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.destination-uri" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uri = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.compression" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().compression = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.field-delimiter" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().field_delimiter = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.destination-format" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_format = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.print-header" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().print_header = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.extract.print-header", "boolean"));
                    },
                "configuration.extract.destination-uris" => {
                        request_configuration_extract_init(&mut request);
                        if request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uris.is_none() {
                           request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uris = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.extract.source-table.project-id" => {
                        request_configuration_extract_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.source-table.table-id" => {
                        request_configuration_extract_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.source-table.dataset-id" => {
                        request_configuration_extract_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_configuration_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-jagged-rows", "allow-large-results", "allow-quoted-newlines", "cache-hit", "compression", "configuration", "copy", "create-disposition", "creation-time", "dataset-id", "debug-info", "default-dataset", "destination-format", "destination-table", "destination-uri", "destination-uri-file-counts", "destination-uris", "dry-run", "encoding", "end-time", "error-result", "etag", "extract", "field-delimiter", "flatten-results", "id", "ignore-unknown-values", "input-file-bytes", "input-files", "job-id", "job-reference", "kind", "link", "load", "location", "max-bad-records", "message", "output-bytes", "output-rows", "preserve-nulls", "print-header", "priority", "project-id", "projection-fields", "query", "quote", "reason", "schema-inline", "schema-inline-format", "self-link", "skip-leading-rows", "source-format", "source-table", "source-uri", "source-uris", "start-time", "state", "statistics", "status", "table-id", "total-bytes-processed", "use-query-cache", "user-email", "write-disposition"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.jobs().insert(request, opt.value_of("project-id").unwrap_or(""));
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
                    Ok(())
                }
            }
        }
    }

    fn _jobs_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "state-filter" => {
                    call = call.add_state_filter(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "all-users" => {
                    call = call.all_users(arg_from_str(value.unwrap_or("false"), err, "all-users", "boolean"));
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
                                                Vec::new() + &self.gp + &["page-token", "all-users", "max-results", "projection", "state-filter"]
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

    fn _jobs_query(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::QueryRequest::default();
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
            fn request_default_dataset_init(request: &mut api::QueryRequest) {
                if request.default_dataset.is_none() {
                    request.default_dataset = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "timeout-ms" => {
                        request.timeout_ms = Some(arg_from_str(value.unwrap_or("-0"), err, "timeout-ms", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "dry-run" => {
                        request.dry_run = Some(arg_from_str(value.unwrap_or("false"), err, "dry-run", "boolean"));
                    },
                "use-query-cache" => {
                        request.use_query_cache = Some(arg_from_str(value.unwrap_or("false"), err, "use-query-cache", "boolean"));
                    },
                "default-dataset.project-id" => {
                        request_default_dataset_init(&mut request);
                        request.default_dataset.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "default-dataset.dataset-id" => {
                        request_default_dataset_init(&mut request);
                        request.default_dataset.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "max-results" => {
                        request_default_dataset_init(&mut request);
                        request.max_results = Some(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                    },
                "query" => {
                        request_default_dataset_init(&mut request);
                        request.query = Some(value.unwrap_or("").to_string());
                    },
                "preserve-nulls" => {
                        request_default_dataset_init(&mut request);
                        request.preserve_nulls = Some(arg_from_str(value.unwrap_or("false"), err, "preserve-nulls", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["dataset-id", "default-dataset", "dry-run", "kind", "max-results", "preserve-nulls", "project-id", "query", "timeout-ms", "use-query-cache"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.jobs().query(request, opt.value_of("project-id").unwrap_or(""));
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

    fn _projects_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().list();
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

    fn _tabledata_insert_all(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::TableDataInsertAllRequest::default();
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
                "ignore-unknown-values" => {
                        request.ignore_unknown_values = Some(arg_from_str(value.unwrap_or("false"), err, "ignore-unknown-values", "boolean"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "skip-invalid-rows" => {
                        request.skip_invalid_rows = Some(arg_from_str(value.unwrap_or("false"), err, "skip-invalid-rows", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["ignore-unknown-values", "kind", "skip-invalid-rows"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.tabledata().insert_all(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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

    fn _tabledata_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tabledata().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["page-token", "start-index", "max-results"]
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

    fn _tables_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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

    fn _tables_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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

    fn _tables_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Table::default();
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
            fn request_table_reference_init(request: &mut api::Table) {
                if request.table_reference.is_none() {
                    request.table_reference = Some(Default::default());
                }
            }
            
            fn request_view_init(request: &mut api::Table) {
                if request.view.is_none() {
                    request.view = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.project-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.table-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.dataset-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "num-rows" => {
                        request_table_reference_init(&mut request);
                        request.num_rows = Some(value.unwrap_or("").to_string());
                    },
                "num-bytes" => {
                        request_table_reference_init(&mut request);
                        request.num_bytes = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_table_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_table_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "expiration-time" => {
                        request_table_reference_init(&mut request);
                        request.expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_table_reference_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_table_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_table_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "view.query" => {
                        request_view_init(&mut request);
                        request.view.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "description", "etag", "expiration-time", "friendly-name", "id", "kind", "last-modified-time", "num-bytes", "num-rows", "project-id", "query", "self-link", "table-id", "table-reference", "type", "view"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.tables().insert(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    fn _tables_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    fn _tables_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Table::default();
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
            fn request_table_reference_init(request: &mut api::Table) {
                if request.table_reference.is_none() {
                    request.table_reference = Some(Default::default());
                }
            }
            
            fn request_view_init(request: &mut api::Table) {
                if request.view.is_none() {
                    request.view = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.project-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.table-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.dataset-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "num-rows" => {
                        request_table_reference_init(&mut request);
                        request.num_rows = Some(value.unwrap_or("").to_string());
                    },
                "num-bytes" => {
                        request_table_reference_init(&mut request);
                        request.num_bytes = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_table_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_table_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "expiration-time" => {
                        request_table_reference_init(&mut request);
                        request.expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_table_reference_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_table_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_table_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "view.query" => {
                        request_view_init(&mut request);
                        request.view.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "description", "etag", "expiration-time", "friendly-name", "id", "kind", "last-modified-time", "num-bytes", "num-rows", "project-id", "query", "self-link", "table-id", "table-reference", "type", "view"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.tables().patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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

    fn _tables_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Table::default();
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
            fn request_table_reference_init(request: &mut api::Table) {
                if request.table_reference.is_none() {
                    request.table_reference = Some(Default::default());
                }
            }
            
            fn request_view_init(request: &mut api::Table) {
                if request.view.is_none() {
                    request.view = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.project-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.table-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.dataset-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "num-rows" => {
                        request_table_reference_init(&mut request);
                        request.num_rows = Some(value.unwrap_or("").to_string());
                    },
                "num-bytes" => {
                        request_table_reference_init(&mut request);
                        request.num_bytes = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_table_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_table_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "expiration-time" => {
                        request_table_reference_init(&mut request);
                        request.expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_table_reference_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_table_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_table_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "view.query" => {
                        request_view_init(&mut request);
                        request.view.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "description", "etag", "expiration-time", "friendly-name", "id", "kind", "last-modified-time", "num-bytes", "num-rows", "project-id", "query", "self-link", "table-id", "table-reference", "type", "view"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.tables().update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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
            ("datasets", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._datasets_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._datasets_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._datasets_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._datasets_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._datasets_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._datasets_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("datasets".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("jobs", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._jobs_get(opt, dry_run, &mut err);
                    },
                    ("get-query-results", Some(opt)) => {
                        call_result = self._jobs_get_query_results(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._jobs_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._jobs_list(opt, dry_run, &mut err);
                    },
                    ("query", Some(opt)) => {
                        call_result = self._jobs_query(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("jobs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._projects_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tabledata", Some(opt)) => {
                match opt.subcommand() {
                    ("insert-all", Some(opt)) => {
                        call_result = self._tabledata_insert_all(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._tabledata_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tabledata".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tables", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._tables_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._tables_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._tables_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._tables_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._tables_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._tables_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tables".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "bigquery2-secret.json", 
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
                                          program_name: "bigquery2",
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
            hub: api::Bigquery::new(client, auth),
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
        ("datasets", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the dataset being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of dataset being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns the dataset specified by datasetID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the requested dataset"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the requested dataset"##),
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
                    Some(r##"Creates a new empty dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the new dataset"##),
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
                    Some(r##"Lists all datasets in the specified project to which you have been granted the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the datasets to be listed"##),
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
                    Some(r##"Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the dataset being updated"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the dataset being updated"##),
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
                    Some(r##"Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the dataset being updated"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the dataset being updated"##),
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
        
        ("jobs", "methods: 'get', 'get-query-results', 'insert', 'list' and 'query'", vec![
            ("get",  
                    Some(r##"Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the requested job"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Job ID of the requested job"##),
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
            ("get-query-results",  
                    Some(r##"Retrieves the results of a query job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_get-query-results",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the query job"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Job ID of the query job"##),
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
                    Some(r##"Starts a new asynchronous job. Requires the Can View project role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the project that will be billed for the job"##),
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
                    Some(r##"Lists all jobs that you started in the specified project. The job list returns in reverse chronological order of when the jobs were created, starting with the most recent job created. Requires the Can View project role, or the Is Owner project role if you set the allUsers property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the jobs to list"##),
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
            ("query",  
                    Some(r##"Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_query",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the project billed for the query"##),
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
        
        ("projects", "methods: 'list'", vec![
            ("list",  
                    Some(r##"Lists all projects to which you have been granted any project role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/projects_list",
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
        
        ("tabledata", "methods: 'insert-all' and 'list'", vec![
            ("insert-all",  
                    Some(r##"Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tabledata_insert-all",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the destination table."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the destination table."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the destination table."##),
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
                    Some(r##"Retrieves table data from a specified set of rows. Requires the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tabledata_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to read"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to read"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to read"##),
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
        
        ("tables", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the requested table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the requested table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the requested table"##),
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
                    Some(r##"Creates a new, empty table in the dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the new table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the new table"##),
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
                    Some(r##"Lists all tables in the specified dataset. Requires the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the tables to list"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the tables to list"##),
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
                    Some(r##"Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to update"##),
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
                    Some(r##"Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to update"##),
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
    
    let mut app = App::new("bigquery2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150326")
           .about("A data platform for customers to create, manage, share and query data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_bigquery2_cli")
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