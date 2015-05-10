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
extern crate google_fitness1 as api;

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
    hub: api::Fitness<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _users_data_sources_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::DataSource::default();
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
            fn request_application_init(request: &mut api::DataSource) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            fn request_data_type_init(request: &mut api::DataSource) {
                if request.data_type.is_none() {
                    request.data_type = Some(Default::default());
                }
            }
            
            fn request_device_init(request: &mut api::DataSource) {
                if request.device.is_none() {
                    request.device = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-name" => {
                        request.data_stream_name = Some(value.unwrap_or("").to_string());
                    },
                "data-type.name" => {
                        request_data_type_init(&mut request);
                        request.data_type.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = Some(value.unwrap_or("").to_string());
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = Some(value.unwrap_or("").to_string());
                    },
                "device.model" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().model = Some(value.unwrap_or("").to_string());
                    },
                "device.version" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "device.type" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "device.uid" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().uid = Some(value.unwrap_or("").to_string());
                    },
                "device.manufacturer" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().manufacturer = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-id" => {
                        request_device_init(&mut request);
                        request.data_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["application", "data-stream-id", "data-stream-name", "data-type", "details-url", "device", "manufacturer", "model", "name", "package-name", "type", "uid", "version"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().data_sources_create(request, opt.value_of("user-id").unwrap_or(""));
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

    fn _users_data_sources_datasets_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().data_sources_datasets_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "modified-time-millis" => {
                    call = call.modified_time_millis(value.unwrap_or(""));
                },
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["modified-time-millis", "current-time-millis"]
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

    fn _users_data_sources_datasets_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().data_sources_datasets_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "limit"]
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

    fn _users_data_sources_datasets_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
            match &temp_cursor.to_string()[..] {
                "min-start-time-ns" => {
                        request.min_start_time_ns = Some(value.unwrap_or("").to_string());
                    },
                "next-page-token" => {
                        request.next_page_token = Some(value.unwrap_or("").to_string());
                    },
                "max-end-time-ns" => {
                        request.max_end_time_ns = Some(value.unwrap_or("").to_string());
                    },
                "data-source-id" => {
                        request.data_source_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["data-source-id", "max-end-time-ns", "min-start-time-ns", "next-page-token"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().data_sources_datasets_patch(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["current-time-millis"]
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

    fn _users_data_sources_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().data_sources_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""));
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

    fn _users_data_sources_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().data_sources_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""));
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

    fn _users_data_sources_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().data_sources_list(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "data-type-name" => {
                    call = call.add_data_type_name(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["data-type-name"]
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

    fn _users_data_sources_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::DataSource::default();
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
            fn request_application_init(request: &mut api::DataSource) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            fn request_data_type_init(request: &mut api::DataSource) {
                if request.data_type.is_none() {
                    request.data_type = Some(Default::default());
                }
            }
            
            fn request_device_init(request: &mut api::DataSource) {
                if request.device.is_none() {
                    request.device = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-name" => {
                        request.data_stream_name = Some(value.unwrap_or("").to_string());
                    },
                "data-type.name" => {
                        request_data_type_init(&mut request);
                        request.data_type.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = Some(value.unwrap_or("").to_string());
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = Some(value.unwrap_or("").to_string());
                    },
                "device.model" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().model = Some(value.unwrap_or("").to_string());
                    },
                "device.version" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "device.type" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "device.uid" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().uid = Some(value.unwrap_or("").to_string());
                    },
                "device.manufacturer" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().manufacturer = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-id" => {
                        request_device_init(&mut request);
                        request.data_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["application", "data-stream-id", "data-stream-name", "data-type", "details-url", "device", "manufacturer", "model", "name", "package-name", "type", "uid", "version"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().data_sources_patch(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""));
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

    fn _users_data_sources_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::DataSource::default();
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
            fn request_application_init(request: &mut api::DataSource) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            fn request_data_type_init(request: &mut api::DataSource) {
                if request.data_type.is_none() {
                    request.data_type = Some(Default::default());
                }
            }
            
            fn request_device_init(request: &mut api::DataSource) {
                if request.device.is_none() {
                    request.device = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-name" => {
                        request.data_stream_name = Some(value.unwrap_or("").to_string());
                    },
                "data-type.name" => {
                        request_data_type_init(&mut request);
                        request.data_type.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = Some(value.unwrap_or("").to_string());
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = Some(value.unwrap_or("").to_string());
                    },
                "device.model" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().model = Some(value.unwrap_or("").to_string());
                    },
                "device.version" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "device.type" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "device.uid" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().uid = Some(value.unwrap_or("").to_string());
                    },
                "device.manufacturer" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().manufacturer = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-id" => {
                        request_device_init(&mut request);
                        request.data_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["application", "data-stream-id", "data-stream-name", "data-type", "details-url", "device", "manufacturer", "model", "name", "package-name", "type", "uid", "version"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().data_sources_update(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("data-source-id").unwrap_or(""));
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

    fn _users_sessions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().sessions_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("session-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["current-time-millis"]
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

    fn _users_sessions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().sessions_list(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-time" => {
                    call = call.start_time(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
                },
                "end-time" => {
                    call = call.end_time(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["page-token", "end-time", "include-deleted", "start-time"]
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

    fn _users_sessions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Session::default();
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
            fn request_application_init(request: &mut api::Session) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "modified-time-millis" => {
                        request.modified_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "end-time-millis" => {
                        request.end_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "activity-type" => {
                        request.activity_type = Some(arg_from_str(value.unwrap_or("-0"), err, "activity-type", "integer"));
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = Some(value.unwrap_or("").to_string());
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = Some(value.unwrap_or("").to_string());
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = Some(value.unwrap_or("").to_string());
                    },
                "start-time-millis" => {
                        request_application_init(&mut request);
                        request.start_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "active-time-millis" => {
                        request_application_init(&mut request);
                        request.active_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_application_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_application_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["active-time-millis", "activity-type", "application", "description", "details-url", "end-time-millis", "id", "modified-time-millis", "name", "package-name", "start-time-millis", "version"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().sessions_update(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("session-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["current-time-millis"]
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
            ("users", Some(opt)) => {
                match opt.subcommand() {
                    ("data-sources-create", Some(opt)) => {
                        call_result = self._users_data_sources_create(opt, dry_run, &mut err);
                    },
                    ("data-sources-datasets-delete", Some(opt)) => {
                        call_result = self._users_data_sources_datasets_delete(opt, dry_run, &mut err);
                    },
                    ("data-sources-datasets-get", Some(opt)) => {
                        call_result = self._users_data_sources_datasets_get(opt, dry_run, &mut err);
                    },
                    ("data-sources-datasets-patch", Some(opt)) => {
                        call_result = self._users_data_sources_datasets_patch(opt, dry_run, &mut err);
                    },
                    ("data-sources-delete", Some(opt)) => {
                        call_result = self._users_data_sources_delete(opt, dry_run, &mut err);
                    },
                    ("data-sources-get", Some(opt)) => {
                        call_result = self._users_data_sources_get(opt, dry_run, &mut err);
                    },
                    ("data-sources-list", Some(opt)) => {
                        call_result = self._users_data_sources_list(opt, dry_run, &mut err);
                    },
                    ("data-sources-patch", Some(opt)) => {
                        call_result = self._users_data_sources_patch(opt, dry_run, &mut err);
                    },
                    ("data-sources-update", Some(opt)) => {
                        call_result = self._users_data_sources_update(opt, dry_run, &mut err);
                    },
                    ("sessions-delete", Some(opt)) => {
                        call_result = self._users_sessions_delete(opt, dry_run, &mut err);
                    },
                    ("sessions-list", Some(opt)) => {
                        call_result = self._users_sessions_list(opt, dry_run, &mut err);
                    },
                    ("sessions-update", Some(opt)) => {
                        call_result = self._users_sessions_update(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "fitness1-secret.json", 
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
                                          program_name: "fitness1",
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
            hub: api::Fitness::new(client, auth),
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
        ("users", "methods: 'data-sources-create', 'data-sources-datasets-delete', 'data-sources-datasets-get', 'data-sources-datasets-patch', 'data-sources-delete', 'data-sources-get', 'data-sources-list', 'data-sources-patch', 'data-sources-update', 'sessions-delete', 'sessions-list' and 'sessions-update'", vec![
            ("data-sources-create",  
                    Some(r##"Creates a new data source that is unique across all data sources belonging to this user. The data stream ID field can be omitted and will be generated by the server with the correct format. The data stream ID is an ordered combination of some fields from the data source. In addition to the data source fields reflected into the data source ID, the developer project number that is authenticated when creating the data source is included. This developer project number is obfuscated when read by any other developer reading public data types."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-create",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Create the data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
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
            ("data-sources-datasets-delete",  
                    Some(r##"Performs an inclusive delete of all data points whose start and end times have any overlap with the time range specified by the dataset ID. For most data types, the entire data point will be deleted. For data types where the time span represents a consistent value (such as com.google.activity.segment), and a data point straddles either end point of the dataset, only the overlapping portion of the data point will be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-datasets-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Delete a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source that created the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: "startTime-endTime" where startTime and endTime are 64 bit integers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("data-sources-datasets-get",  
                    Some(r##"Returns a dataset containing all data points whose start and end times overlap with the specified range of the dataset minimum start time and maximum end time. Specifically, any data point whose start time is less than or equal to the dataset end time and whose end time is greater than or equal to the dataset start time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-datasets-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Retrieve a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source that created the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: "startTime-endTime" where startTime and endTime are 64 bit integers."##),
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
            ("data-sources-datasets-patch",  
                    Some(r##"Adds data points to a dataset. The dataset need not be previously created. All points within the given dataset will be returned with subsquent calls to retrieve this dataset. Data points can belong to more than one dataset. This method does not use patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-datasets-patch",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Patch a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source that created the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: "startTime-endTime" where startTime and endTime are 64 bit integers."##),
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
            ("data-sources-delete",  
                    Some(r##"Delete the data source if there are no datapoints associated with it"##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Retrieve a data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source to delete."##),
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
            ("data-sources-get",  
                    Some(r##"Returns a data source identified by a data stream ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Retrieve a data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source to retrieve."##),
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
            ("data-sources-list",  
                    Some(r##"Lists all data sources that are visible to the developer, using the OAuth scopes provided. The list is not exhaustive: the user may have private data sources that are only visible to other developers or calls using other scopes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"List data sources for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
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
            ("data-sources-patch",  
                    Some(r##"Updates a given data source. It is an error to modify the data source's data stream ID, data type, type, stream name or device information apart from the device version. Changing these fields would require a new unique data stream ID and separate data source.
        
        Data sources are identified by their data stream ID. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-patch",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Update the data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source to update."##),
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
            ("data-sources-update",  
                    Some(r##"Updates a given data source. It is an error to modify the data source's data stream ID, data type, type, stream name or device information apart from the device version. Changing these fields would require a new unique data stream ID and separate data source.
        
        Data sources are identified by their data stream ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_data-sources-update",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Update the data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"data-source-id"##),
                     None,
                     Some(r##"The data stream ID of the data source to update."##),
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
            ("sessions-delete",  
                    Some(r##"Deletes a session specified by the given session ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_sessions-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Delete a session for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"session-id"##),
                     None,
                     Some(r##"The ID of the session to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("sessions-list",  
                    Some(r##"Lists sessions previously created."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_sessions-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"List sessions for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
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
            ("sessions-update",  
                    Some(r##"Updates or insert a given session."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fitness1_cli/users_sessions-update",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Create sessions for the person identified. Use me to indicate the authenticated user. Only me is supported at this time."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"session-id"##),
                     None,
                     Some(r##"The ID of the session to be created."##),
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
    
    let mut app = App::new("fitness1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150326")
           .about("Google Fit API")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_fitness1_cli")
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