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
extern crate google_books1 as api;

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
    hub: api::Books<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _bookshelves_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bookshelves().get(opt.value_of("user-id").unwrap_or(""), opt.value_of("shelf").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _bookshelves_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bookshelves().list(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _bookshelves_volumes_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bookshelves().volumes_list(opt.value_of("user-id").unwrap_or(""), opt.value_of("shelf").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
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
                                                Vec::new() + &self.gp + &["source", "start-index", "show-preorders", "max-results"]
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

    fn _cloudloading_add_book(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cloudloading().add_book();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "upload-client-token" => {
                    call = call.upload_client_token(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "mime-type" => {
                    call = call.mime_type(value.unwrap_or(""));
                },
                "drive-document-id" => {
                    call = call.drive_document_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["mime-type", "name", "upload-client-token", "drive-document-id"]
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

    fn _cloudloading_delete_book(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cloudloading().delete_book(opt.value_of("volume-id").unwrap_or(""));
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

    fn _cloudloading_update_book(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::BooksCloudloadingResource::default();
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
                "title" => {
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "processing-state" => {
                        request.processing_state = Some(value.unwrap_or("").to_string());
                    },
                "volume-id" => {
                        request.volume_id = Some(value.unwrap_or("").to_string());
                    },
                "author" => {
                        request.author = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "processing-state", "title", "volume-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.cloudloading().update_book(request);
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

    fn _dictionary_list_offline_metadata(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.dictionary().list_offline_metadata(opt.value_of("cpksver").unwrap_or(""));
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

    fn _layers_annotation_data_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.layers().annotation_data_get(opt.value_of("volume-id").unwrap_or(""), opt.value_of("layer-id").unwrap_or(""), opt.value_of("annotation-data-id").unwrap_or(""), opt.value_of("content-version").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "w" => {
                    call = call.w(arg_from_str(value.unwrap_or("-0"), err, "w", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "scale" => {
                    call = call.scale(arg_from_str(value.unwrap_or("-0"), err, "scale", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "h" => {
                    call = call.h(arg_from_str(value.unwrap_or("-0"), err, "h", "integer"));
                },
                "allow-web-definitions" => {
                    call = call.allow_web_definitions(arg_from_str(value.unwrap_or("false"), err, "allow-web-definitions", "boolean"));
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
                                                Vec::new() + &self.gp + &["scale", "locale", "h", "source", "allow-web-definitions", "w"]
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

    fn _layers_annotation_data_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.layers().annotation_data_list(opt.value_of("volume-id").unwrap_or(""), opt.value_of("layer-id").unwrap_or(""), opt.value_of("content-version").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "w" => {
                    call = call.w(arg_from_str(value.unwrap_or("-0"), err, "w", "integer"));
                },
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "updated-max" => {
                    call = call.updated_max(value.unwrap_or(""));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "scale" => {
                    call = call.scale(arg_from_str(value.unwrap_or("-0"), err, "scale", "integer"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "h" => {
                    call = call.h(arg_from_str(value.unwrap_or("-0"), err, "h", "integer"));
                },
                "annotation-data-id" => {
                    call = call.add_annotation_data_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["scale", "source", "locale", "updated-min", "updated-max", "max-results", "annotation-data-id", "page-token", "w", "h"]
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

    fn _layers_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.layers().get(opt.value_of("volume-id").unwrap_or(""), opt.value_of("summary-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source", "content-version"]
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

    fn _layers_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.layers().list(opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source", "content-version", "max-results", "page-token"]
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

    fn _layers_volume_annotations_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.layers().volume_annotations_get(opt.value_of("volume-id").unwrap_or(""), opt.value_of("layer-id").unwrap_or(""), opt.value_of("annotation-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "source"]
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

    fn _layers_volume_annotations_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.layers().volume_annotations_list(opt.value_of("volume-id").unwrap_or(""), opt.value_of("layer-id").unwrap_or(""), opt.value_of("content-version").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "volume-annotations-version" => {
                    call = call.volume_annotations_version(value.unwrap_or(""));
                },
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "updated-max" => {
                    call = call.updated_max(value.unwrap_or(""));
                },
                "start-position" => {
                    call = call.start_position(value.unwrap_or(""));
                },
                "start-offset" => {
                    call = call.start_offset(value.unwrap_or(""));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "end-position" => {
                    call = call.end_position(value.unwrap_or(""));
                },
                "end-offset" => {
                    call = call.end_offset(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["show-deleted", "volume-annotations-version", "end-position", "updated-max", "start-position", "updated-min", "end-offset", "max-results", "source", "start-offset", "page-token", "locale"]
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

    fn _myconfig_get_user_settings(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.myconfig().get_user_settings();
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

    fn _myconfig_release_download_access(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.myconfig().release_download_access(&opt.values_of("volume-ids").unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>(), opt.value_of("cpksver").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "source"]
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

    fn _myconfig_request_access(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.myconfig().request_access(opt.value_of("source").unwrap_or(""), opt.value_of("volume-id").unwrap_or(""), opt.value_of("nonce").unwrap_or(""), opt.value_of("cpksver").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "license-types" => {
                    call = call.license_types(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "license-types"]
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

    fn _myconfig_sync_volume_licenses(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.myconfig().sync_volume_licenses(opt.value_of("source").unwrap_or(""), opt.value_of("nonce").unwrap_or(""), opt.value_of("cpksver").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "volume-ids" => {
                    call = call.add_volume_ids(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "features" => {
                    call = call.add_features(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "show-preorders", "volume-ids", "features"]
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

    fn _myconfig_update_user_settings(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Usersettings::default();
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
            fn request_notes_export_init(request: &mut api::Usersettings) {
                if request.notes_export.is_none() {
                    request.notes_export = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "notes-export.is-enabled" => {
                        request_notes_export_init(&mut request);
                        request.notes_export.as_mut().unwrap().is_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "notes-export.is-enabled", "boolean"));
                    },
                "notes-export.folder-name" => {
                        request_notes_export_init(&mut request);
                        request.notes_export.as_mut().unwrap().folder_name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["folder-name", "is-enabled", "kind", "notes-export"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.myconfig().update_user_settings(request);
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

    fn _mylibrary_annotations_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().annotations_delete(opt.value_of("annotation-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _mylibrary_annotations_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Annotation::default();
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
            fn request_client_version_ranges_cfi_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().cfi_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().cfi_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_gb_image_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().gb_image_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().gb_image_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_gb_text_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().gb_text_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().gb_text_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_image_cfi_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().image_cfi_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().image_cfi_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_init(request: &mut api::Annotation) {
                if request.client_version_ranges.is_none() {
                    request.client_version_ranges = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_cfi_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().cfi_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().cfi_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_gb_image_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().gb_image_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().gb_image_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_gb_text_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().gb_text_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().gb_text_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_image_cfi_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().image_cfi_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().image_cfi_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_init(request: &mut api::Annotation) {
                if request.current_version_ranges.is_none() {
                    request.current_version_ranges = Some(Default::default());
                }
            }
            
            fn request_layer_summary_init(request: &mut api::Annotation) {
                if request.layer_summary.is_none() {
                    request.layer_summary = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "page-ids" => {
                        if request.page_ids.is_none() {
                           request.page_ids = Some(Default::default());
                        }
                                        request.page_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "before-selected-text" => {
                        request.before_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.start-position" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.end-position" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.start-offset" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.end-offset" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.start-position" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.end-position" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.start-offset" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.end-offset" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.content-version" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().content_version = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.start-position" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.end-position" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.start-offset" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.end-offset" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.start-position" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.end-position" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.start-offset" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.end-offset" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "after-selected-text" => {
                        request_current_version_ranges_init(&mut request);
                        request.after_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_current_version_ranges_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "volume-id" => {
                        request_current_version_ranges_init(&mut request);
                        request.volume_id = Some(value.unwrap_or("").to_string());
                    },
                "layer-summary.limit-type" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().limit_type = Some(value.unwrap_or("").to_string());
                    },
                "layer-summary.remaining-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().remaining_character_count = Some(arg_from_str(value.unwrap_or("-0"), err, "layer-summary.remaining-character-count", "integer"));
                    },
                "layer-summary.allowed-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().allowed_character_count = Some(arg_from_str(value.unwrap_or("-0"), err, "layer-summary.allowed-character-count", "integer"));
                    },
                "selected-text" => {
                        request_layer_summary_init(&mut request);
                        request.selected_text = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.start-position" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.end-position" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.start-offset" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.end-offset" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.start-position" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.end-position" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.start-offset" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.end-offset" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.content-version" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().content_version = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.start-position" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.end-position" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.start-offset" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.end-offset" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.start-position" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.end-position" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.start-offset" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.end-offset" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "layer-id" => {
                        request_client_version_ranges_init(&mut request);
                        request.layer_id = Some(value.unwrap_or("").to_string());
                    },
                "highlight-style" => {
                        request_client_version_ranges_init(&mut request);
                        request.highlight_style = Some(value.unwrap_or("").to_string());
                    },
                "data" => {
                        request_client_version_ranges_init(&mut request);
                        request.data = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_client_version_ranges_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_client_version_ranges_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["after-selected-text", "allowed-character-count", "before-selected-text", "cfi-range", "client-version-ranges", "content-version", "created", "current-version-ranges", "data", "deleted", "end-offset", "end-position", "gb-image-range", "gb-text-range", "highlight-style", "id", "image-cfi-range", "kind", "layer-id", "layer-summary", "limit-type", "page-ids", "remaining-character-count", "selected-text", "self-link", "start-offset", "start-position", "updated", "volume-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.mylibrary().annotations_insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-only-summary-in-response" => {
                    call = call.show_only_summary_in_response(arg_from_str(value.unwrap_or("false"), err, "show-only-summary-in-response", "boolean"));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source", "show-only-summary-in-response", "country"]
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

    fn _mylibrary_annotations_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().annotations_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "volume-id" => {
                    call = call.volume_id(value.unwrap_or(""));
                },
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "updated-max" => {
                    call = call.updated_max(value.unwrap_or(""));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                "layer-ids" => {
                    call = call.add_layer_ids(value.unwrap_or(""));
                },
                "layer-id" => {
                    call = call.layer_id(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["page-token", "layer-ids", "show-deleted", "updated-min", "updated-max", "volume-id", "max-results", "source", "content-version", "layer-id"]
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

    fn _mylibrary_annotations_summary(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().annotations_summary(&opt.values_of("layer-ids").unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>(), opt.value_of("volume-id").unwrap_or(""));
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

    fn _mylibrary_annotations_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Annotation::default();
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
            fn request_client_version_ranges_cfi_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().cfi_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().cfi_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_gb_image_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().gb_image_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().gb_image_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_gb_text_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().gb_text_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().gb_text_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_image_cfi_range_init(request: &mut api::Annotation) {
                request_client_version_ranges_init(request);
                if request.client_version_ranges.as_mut().unwrap().image_cfi_range.is_none() {
                    request.client_version_ranges.as_mut().unwrap().image_cfi_range = Some(Default::default());
                }
            }
            
            fn request_client_version_ranges_init(request: &mut api::Annotation) {
                if request.client_version_ranges.is_none() {
                    request.client_version_ranges = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_cfi_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().cfi_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().cfi_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_gb_image_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().gb_image_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().gb_image_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_gb_text_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().gb_text_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().gb_text_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_image_cfi_range_init(request: &mut api::Annotation) {
                request_current_version_ranges_init(request);
                if request.current_version_ranges.as_mut().unwrap().image_cfi_range.is_none() {
                    request.current_version_ranges.as_mut().unwrap().image_cfi_range = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_init(request: &mut api::Annotation) {
                if request.current_version_ranges.is_none() {
                    request.current_version_ranges = Some(Default::default());
                }
            }
            
            fn request_layer_summary_init(request: &mut api::Annotation) {
                if request.layer_summary.is_none() {
                    request.layer_summary = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "page-ids" => {
                        if request.page_ids.is_none() {
                           request.page_ids = Some(Default::default());
                        }
                                        request.page_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "before-selected-text" => {
                        request.before_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.start-position" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.end-position" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.start-offset" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.end-offset" => {
                        request_current_version_ranges_image_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.start-position" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.end-position" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.start-offset" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-text-range.end-offset" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.content-version" => {
                        request_current_version_ranges_gb_text_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().content_version = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.start-position" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.end-position" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.start-offset" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.cfi-range.end-offset" => {
                        request_current_version_ranges_cfi_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.start-position" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.end-position" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.start-offset" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.gb-image-range.end-offset" => {
                        request_current_version_ranges_gb_image_range_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "after-selected-text" => {
                        request_current_version_ranges_init(&mut request);
                        request.after_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_current_version_ranges_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "volume-id" => {
                        request_current_version_ranges_init(&mut request);
                        request.volume_id = Some(value.unwrap_or("").to_string());
                    },
                "layer-summary.limit-type" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().limit_type = Some(value.unwrap_or("").to_string());
                    },
                "layer-summary.remaining-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().remaining_character_count = Some(arg_from_str(value.unwrap_or("-0"), err, "layer-summary.remaining-character-count", "integer"));
                    },
                "layer-summary.allowed-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().allowed_character_count = Some(arg_from_str(value.unwrap_or("-0"), err, "layer-summary.allowed-character-count", "integer"));
                    },
                "selected-text" => {
                        request_layer_summary_init(&mut request);
                        request.selected_text = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.start-position" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.end-position" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.start-offset" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.end-offset" => {
                        request_client_version_ranges_image_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.start-position" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.end-position" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.start-offset" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-text-range.end-offset" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.content-version" => {
                        request_client_version_ranges_gb_text_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().content_version = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.start-position" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.end-position" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.start-offset" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.cfi-range.end-offset" => {
                        request_client_version_ranges_cfi_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.start-position" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.end-position" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_position = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.start-offset" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().start_offset = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.gb-image-range.end-offset" => {
                        request_client_version_ranges_gb_image_range_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.as_mut().unwrap().end_offset = Some(value.unwrap_or("").to_string());
                    },
                "layer-id" => {
                        request_client_version_ranges_init(&mut request);
                        request.layer_id = Some(value.unwrap_or("").to_string());
                    },
                "highlight-style" => {
                        request_client_version_ranges_init(&mut request);
                        request.highlight_style = Some(value.unwrap_or("").to_string());
                    },
                "data" => {
                        request_client_version_ranges_init(&mut request);
                        request.data = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_client_version_ranges_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_client_version_ranges_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["after-selected-text", "allowed-character-count", "before-selected-text", "cfi-range", "client-version-ranges", "content-version", "created", "current-version-ranges", "data", "deleted", "end-offset", "end-position", "gb-image-range", "gb-text-range", "highlight-style", "id", "image-cfi-range", "kind", "layer-id", "layer-summary", "limit-type", "page-ids", "remaining-character-count", "selected-text", "self-link", "start-offset", "start-position", "updated", "volume-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.mylibrary().annotations_update(request, opt.value_of("annotation-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _mylibrary_bookshelves_add_volume(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().bookshelves_add_volume(opt.value_of("shelf").unwrap_or(""), opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "reason" => {
                    call = call.reason(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source", "reason"]
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

    fn _mylibrary_bookshelves_clear_volumes(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().bookshelves_clear_volumes(opt.value_of("shelf").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _mylibrary_bookshelves_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().bookshelves_get(opt.value_of("shelf").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _mylibrary_bookshelves_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().bookshelves_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _mylibrary_bookshelves_move_volume(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let volume_position: i32 = arg_from_str(&opt.value_of("volume-position").unwrap_or(""), err, "<volume-position>", "integer");
        let mut call = self.hub.mylibrary().bookshelves_move_volume(opt.value_of("shelf").unwrap_or(""), opt.value_of("volume-id").unwrap_or(""), volume_position);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source"]
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

    fn _mylibrary_bookshelves_remove_volume(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().bookshelves_remove_volume(opt.value_of("shelf").unwrap_or(""), opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "reason" => {
                    call = call.reason(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source", "reason"]
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

    fn _mylibrary_bookshelves_volumes_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().bookshelves_volumes_list(opt.value_of("shelf").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["projection", "country", "show-preorders", "max-results", "q", "source", "start-index"]
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

    fn _mylibrary_readingpositions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().readingpositions_get(opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["source", "content-version"]
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

    fn _mylibrary_readingpositions_set_position(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.mylibrary().readingpositions_set_position(opt.value_of("volume-id").unwrap_or(""), opt.value_of("timestamp").unwrap_or(""), opt.value_of("position").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "device-cookie" => {
                    call = call.device_cookie(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
                },
                "action" => {
                    call = call.action(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["action", "source", "content-version", "device-cookie"]
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

    fn _onboarding_list_categories(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.onboarding().list_categories();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale"]
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

    fn _onboarding_list_category_volumes(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.onboarding().list_category_volumes();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "category-id" => {
                    call = call.add_category_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "page-token", "category-id", "page-size"]
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

    fn _promooffer_accept(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.promooffer().accept();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "volume-id" => {
                    call = call.volume_id(value.unwrap_or(""));
                },
                "serial" => {
                    call = call.serial(value.unwrap_or(""));
                },
                "product" => {
                    call = call.product(value.unwrap_or(""));
                },
                "offer-id" => {
                    call = call.offer_id(value.unwrap_or(""));
                },
                "model" => {
                    call = call.model(value.unwrap_or(""));
                },
                "manufacturer" => {
                    call = call.manufacturer(value.unwrap_or(""));
                },
                "device" => {
                    call = call.device(value.unwrap_or(""));
                },
                "android-id" => {
                    call = call.android_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["product", "volume-id", "offer-id", "android-id", "device", "model", "serial", "manufacturer"]
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

    fn _promooffer_dismiss(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.promooffer().dismiss();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "serial" => {
                    call = call.serial(value.unwrap_or(""));
                },
                "product" => {
                    call = call.product(value.unwrap_or(""));
                },
                "offer-id" => {
                    call = call.offer_id(value.unwrap_or(""));
                },
                "model" => {
                    call = call.model(value.unwrap_or(""));
                },
                "manufacturer" => {
                    call = call.manufacturer(value.unwrap_or(""));
                },
                "device" => {
                    call = call.device(value.unwrap_or(""));
                },
                "android-id" => {
                    call = call.android_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["product", "offer-id", "android-id", "device", "model", "serial", "manufacturer"]
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

    fn _promooffer_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.promooffer().get();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "serial" => {
                    call = call.serial(value.unwrap_or(""));
                },
                "product" => {
                    call = call.product(value.unwrap_or(""));
                },
                "model" => {
                    call = call.model(value.unwrap_or(""));
                },
                "manufacturer" => {
                    call = call.manufacturer(value.unwrap_or(""));
                },
                "device" => {
                    call = call.device(value.unwrap_or(""));
                },
                "android-id" => {
                    call = call.android_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["product", "android-id", "device", "model", "serial", "manufacturer"]
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

    fn _volumes_associated_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().associated_list(opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "association" => {
                    call = call.association(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "source", "association"]
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

    fn _volumes_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().get(opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-library-consistent-read" => {
                    call = call.user_library_consistent_read(arg_from_str(value.unwrap_or("false"), err, "user-library-consistent-read", "boolean"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "partner" => {
                    call = call.partner(value.unwrap_or(""));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["country", "source", "user-library-consistent-read", "projection", "partner"]
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

    fn _volumes_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().list(opt.value_of("q").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "print-type" => {
                    call = call.print_type(value.unwrap_or(""));
                },
                "partner" => {
                    call = call.partner(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "library-restrict" => {
                    call = call.library_restrict(value.unwrap_or(""));
                },
                "lang-restrict" => {
                    call = call.lang_restrict(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "download" => {
                    call = call.download(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["order-by", "projection", "library-restrict", "lang-restrict", "print-type", "show-preorders", "max-results", "filter", "source", "start-index", "download", "partner"]
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

    fn _volumes_mybooks_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().mybooks_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "processing-state" => {
                    call = call.add_processing_state(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "acquire-method" => {
                    call = call.add_acquire_method(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "acquire-method", "max-results", "source", "start-index", "processing-state"]
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

    fn _volumes_recommended_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().recommended_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "source"]
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

    fn _volumes_recommended_rate(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().recommended_rate(opt.value_of("rating").unwrap_or(""), opt.value_of("volume-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "source"]
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

    fn _volumes_useruploaded_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.volumes().useruploaded_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "volume-id" => {
                    call = call.add_volume_id(value.unwrap_or(""));
                },
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "processing-state" => {
                    call = call.add_processing_state(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["locale", "volume-id", "max-results", "source", "start-index", "processing-state"]
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
            ("bookshelves", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._bookshelves_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._bookshelves_list(opt, dry_run, &mut err);
                    },
                    ("volumes-list", Some(opt)) => {
                        call_result = self._bookshelves_volumes_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("bookshelves".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("cloudloading", Some(opt)) => {
                match opt.subcommand() {
                    ("add-book", Some(opt)) => {
                        call_result = self._cloudloading_add_book(opt, dry_run, &mut err);
                    },
                    ("delete-book", Some(opt)) => {
                        call_result = self._cloudloading_delete_book(opt, dry_run, &mut err);
                    },
                    ("update-book", Some(opt)) => {
                        call_result = self._cloudloading_update_book(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("cloudloading".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("dictionary", Some(opt)) => {
                match opt.subcommand() {
                    ("list-offline-metadata", Some(opt)) => {
                        call_result = self._dictionary_list_offline_metadata(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("dictionary".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("layers", Some(opt)) => {
                match opt.subcommand() {
                    ("annotation-data-get", Some(opt)) => {
                        call_result = self._layers_annotation_data_get(opt, dry_run, &mut err);
                    },
                    ("annotation-data-list", Some(opt)) => {
                        call_result = self._layers_annotation_data_list(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._layers_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._layers_list(opt, dry_run, &mut err);
                    },
                    ("volume-annotations-get", Some(opt)) => {
                        call_result = self._layers_volume_annotations_get(opt, dry_run, &mut err);
                    },
                    ("volume-annotations-list", Some(opt)) => {
                        call_result = self._layers_volume_annotations_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("layers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("myconfig", Some(opt)) => {
                match opt.subcommand() {
                    ("get-user-settings", Some(opt)) => {
                        call_result = self._myconfig_get_user_settings(opt, dry_run, &mut err);
                    },
                    ("release-download-access", Some(opt)) => {
                        call_result = self._myconfig_release_download_access(opt, dry_run, &mut err);
                    },
                    ("request-access", Some(opt)) => {
                        call_result = self._myconfig_request_access(opt, dry_run, &mut err);
                    },
                    ("sync-volume-licenses", Some(opt)) => {
                        call_result = self._myconfig_sync_volume_licenses(opt, dry_run, &mut err);
                    },
                    ("update-user-settings", Some(opt)) => {
                        call_result = self._myconfig_update_user_settings(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("myconfig".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("mylibrary", Some(opt)) => {
                match opt.subcommand() {
                    ("annotations-delete", Some(opt)) => {
                        call_result = self._mylibrary_annotations_delete(opt, dry_run, &mut err);
                    },
                    ("annotations-insert", Some(opt)) => {
                        call_result = self._mylibrary_annotations_insert(opt, dry_run, &mut err);
                    },
                    ("annotations-list", Some(opt)) => {
                        call_result = self._mylibrary_annotations_list(opt, dry_run, &mut err);
                    },
                    ("annotations-summary", Some(opt)) => {
                        call_result = self._mylibrary_annotations_summary(opt, dry_run, &mut err);
                    },
                    ("annotations-update", Some(opt)) => {
                        call_result = self._mylibrary_annotations_update(opt, dry_run, &mut err);
                    },
                    ("bookshelves-add-volume", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_add_volume(opt, dry_run, &mut err);
                    },
                    ("bookshelves-clear-volumes", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_clear_volumes(opt, dry_run, &mut err);
                    },
                    ("bookshelves-get", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_get(opt, dry_run, &mut err);
                    },
                    ("bookshelves-list", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_list(opt, dry_run, &mut err);
                    },
                    ("bookshelves-move-volume", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_move_volume(opt, dry_run, &mut err);
                    },
                    ("bookshelves-remove-volume", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_remove_volume(opt, dry_run, &mut err);
                    },
                    ("bookshelves-volumes-list", Some(opt)) => {
                        call_result = self._mylibrary_bookshelves_volumes_list(opt, dry_run, &mut err);
                    },
                    ("readingpositions-get", Some(opt)) => {
                        call_result = self._mylibrary_readingpositions_get(opt, dry_run, &mut err);
                    },
                    ("readingpositions-set-position", Some(opt)) => {
                        call_result = self._mylibrary_readingpositions_set_position(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("mylibrary".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("onboarding", Some(opt)) => {
                match opt.subcommand() {
                    ("list-categories", Some(opt)) => {
                        call_result = self._onboarding_list_categories(opt, dry_run, &mut err);
                    },
                    ("list-category-volumes", Some(opt)) => {
                        call_result = self._onboarding_list_category_volumes(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("onboarding".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("promooffer", Some(opt)) => {
                match opt.subcommand() {
                    ("accept", Some(opt)) => {
                        call_result = self._promooffer_accept(opt, dry_run, &mut err);
                    },
                    ("dismiss", Some(opt)) => {
                        call_result = self._promooffer_dismiss(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._promooffer_get(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("promooffer".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("volumes", Some(opt)) => {
                match opt.subcommand() {
                    ("associated-list", Some(opt)) => {
                        call_result = self._volumes_associated_list(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._volumes_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._volumes_list(opt, dry_run, &mut err);
                    },
                    ("mybooks-list", Some(opt)) => {
                        call_result = self._volumes_mybooks_list(opt, dry_run, &mut err);
                    },
                    ("recommended-list", Some(opt)) => {
                        call_result = self._volumes_recommended_list(opt, dry_run, &mut err);
                    },
                    ("recommended-rate", Some(opt)) => {
                        call_result = self._volumes_recommended_rate(opt, dry_run, &mut err);
                    },
                    ("useruploaded-list", Some(opt)) => {
                        call_result = self._volumes_useruploaded_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("volumes".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "books1-secret.json", 
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
                                          program_name: "books1",
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
            hub: api::Books::new(client, auth),
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
        ("bookshelves", "methods: 'get', 'list' and 'volumes-list'", vec![
            ("get",  
                    Some(r##"Retrieves metadata for a specific bookshelf for the specified user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/bookshelves_get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"ID of user for whom to retrieve bookshelves."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf to retrieve."##),
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
                    Some(r##"Retrieves a list of public bookshelves for the specified user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/bookshelves_list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"ID of user for whom to retrieve bookshelves."##),
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
            ("volumes-list",  
                    Some(r##"Retrieves volumes in a specific bookshelf for the specified user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/bookshelves_volumes-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"ID of user for whom to retrieve bookshelf volumes."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf to retrieve volumes."##),
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
        
        ("cloudloading", "methods: 'add-book', 'delete-book' and 'update-book'", vec![
            ("add-book",  
                    Some(r##""##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/cloudloading_add-book",
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
            ("delete-book",  
                    Some(r##"Remove the book and its contents"##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/cloudloading_delete-book",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The id of the book to be removed."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("update-book",  
                    Some(r##""##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/cloudloading_update-book",
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
        
        ("dictionary", "methods: 'list-offline-metadata'", vec![
            ("list-offline-metadata",  
                    Some(r##"Returns a list of offline dictionary meatadata available"##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/dictionary_list-offline-metadata",
                  vec![
                    (Some(r##"cpksver"##),
                     None,
                     Some(r##"The device/version ID from which to request the data."##),
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
        
        ("layers", "methods: 'annotation-data-get', 'annotation-data-list', 'get', 'list', 'volume-annotations-get' and 'volume-annotations-list'", vec![
            ("annotation-data-get",  
                    Some(r##"Gets the annotation data."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/layers_annotation-data-get",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to retrieve annotations for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"layer-id"##),
                     None,
                     Some(r##"The ID for the layer to get the annotations."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"annotation-data-id"##),
                     None,
                     Some(r##"The ID of the annotation data to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"content-version"##),
                     None,
                     Some(r##"The content version for the volume you are trying to retrieve."##),
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
            ("annotation-data-list",  
                    Some(r##"Gets the annotation data for a volume and layer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/layers_annotation-data-list",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to retrieve annotation data for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"layer-id"##),
                     None,
                     Some(r##"The ID for the layer to get the annotation data."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"content-version"##),
                     None,
                     Some(r##"The content version for the requested volume."##),
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
                    Some(r##"Gets the layer summary for a volume."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/layers_get",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to retrieve layers for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"summary-id"##),
                     None,
                     Some(r##"The ID for the layer to get the summary for."##),
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
                    Some(r##"List the layer summaries for a volume."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/layers_list",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to retrieve layers for."##),
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
            ("volume-annotations-get",  
                    Some(r##"Gets the volume annotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/layers_volume-annotations-get",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to retrieve annotations for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"layer-id"##),
                     None,
                     Some(r##"The ID for the layer to get the annotations."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"annotation-id"##),
                     None,
                     Some(r##"The ID of the volume annotation to retrieve."##),
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
            ("volume-annotations-list",  
                    Some(r##"Gets the volume annotations for a volume and layer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/layers_volume-annotations-list",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to retrieve annotations for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"layer-id"##),
                     None,
                     Some(r##"The ID for the layer to get the annotations."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"content-version"##),
                     None,
                     Some(r##"The content version for the requested volume."##),
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
        
        ("myconfig", "methods: 'get-user-settings', 'release-download-access', 'request-access', 'sync-volume-licenses' and 'update-user-settings'", vec![
            ("get-user-settings",  
                    Some(r##"Gets the current settings for the user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/myconfig_get-user-settings",
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
            ("release-download-access",  
                    Some(r##"Release downloaded content access restriction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/myconfig_release-download-access",
                  vec![
                    (Some(r##"volume-ids"##),
                     None,
                     Some(r##"The volume(s) to release restrictions for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cpksver"##),
                     None,
                     Some(r##"The device/version ID from which to release the restriction."##),
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
            ("request-access",  
                    Some(r##"Request concurrent and download access restrictions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/myconfig_request-access",
                  vec![
                    (Some(r##"source"##),
                     None,
                     Some(r##"String to identify the originator of this request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"The volume to request concurrent/download restrictions for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"nonce"##),
                     None,
                     Some(r##"The client nonce value."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cpksver"##),
                     None,
                     Some(r##"The device/version ID from which to request the restrictions."##),
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
            ("sync-volume-licenses",  
                    Some(r##"Request downloaded content access for specified volumes on the My eBooks shelf."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/myconfig_sync-volume-licenses",
                  vec![
                    (Some(r##"source"##),
                     None,
                     Some(r##"String to identify the originator of this request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"nonce"##),
                     None,
                     Some(r##"The client nonce value."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cpksver"##),
                     None,
                     Some(r##"The device/version ID from which to release the restriction."##),
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
            ("update-user-settings",  
                    Some(r##"Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/myconfig_update-user-settings",
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
        
        ("mylibrary", "methods: 'annotations-delete', 'annotations-insert', 'annotations-list', 'annotations-summary', 'annotations-update', 'bookshelves-add-volume', 'bookshelves-clear-volumes', 'bookshelves-get', 'bookshelves-list', 'bookshelves-move-volume', 'bookshelves-remove-volume', 'bookshelves-volumes-list', 'readingpositions-get' and 'readingpositions-set-position'", vec![
            ("annotations-delete",  
                    Some(r##"Deletes an annotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_annotations-delete",
                  vec![
                    (Some(r##"annotation-id"##),
                     None,
                     Some(r##"The ID for the annotation to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("annotations-insert",  
                    Some(r##"Inserts a new annotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_annotations-insert",
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
            ("annotations-list",  
                    Some(r##"Retrieves a list of annotations, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_annotations-list",
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
            ("annotations-summary",  
                    Some(r##"Gets the summary of specified layers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_annotations-summary",
                  vec![
                    (Some(r##"layer-ids"##),
                     None,
                     Some(r##"Array of layer IDs to get the summary for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"Volume id to get the summary for."##),
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
            ("annotations-update",  
                    Some(r##"Updates an existing annotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_annotations-update",
                  vec![
                    (Some(r##"annotation-id"##),
                     None,
                     Some(r##"The ID for the annotation to update."##),
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
            ("bookshelves-add-volume",  
                    Some(r##"Adds a volume to a bookshelf."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-add-volume",
                  vec![
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf to which to add a volume."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of volume to add."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("bookshelves-clear-volumes",  
                    Some(r##"Clears all volumes from a bookshelf."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-clear-volumes",
                  vec![
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf from which to remove a volume."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("bookshelves-get",  
                    Some(r##"Retrieves metadata for a specific bookshelf belonging to the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-get",
                  vec![
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf to retrieve."##),
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
            ("bookshelves-list",  
                    Some(r##"Retrieves a list of bookshelves belonging to the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-list",
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
            ("bookshelves-move-volume",  
                    Some(r##"Moves a volume within a bookshelf."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-move-volume",
                  vec![
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf with the volume."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of volume to move."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-position"##),
                     None,
                     Some(r##"Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("bookshelves-remove-volume",  
                    Some(r##"Removes a volume from a bookshelf."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-remove-volume",
                  vec![
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"ID of bookshelf from which to remove a volume."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of volume to remove."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("bookshelves-volumes-list",  
                    Some(r##"Gets volume information for volumes on a bookshelf."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_bookshelves-volumes-list",
                  vec![
                    (Some(r##"shelf"##),
                     None,
                     Some(r##"The bookshelf ID or name retrieve volumes for."##),
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
            ("readingpositions-get",  
                    Some(r##"Retrieves my reading position information for a volume."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_readingpositions-get",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of volume for which to retrieve a reading position."##),
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
            ("readingpositions-set-position",  
                    Some(r##"Sets my reading position information for a volume."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/mylibrary_readingpositions-set-position",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of volume for which to update the reading position."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"timestamp"##),
                     None,
                     Some(r##"RFC 3339 UTC format timestamp associated with this reading position."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"position"##),
                     None,
                     Some(r##"Position string for the new volume reading position."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
        ("onboarding", "methods: 'list-categories' and 'list-category-volumes'", vec![
            ("list-categories",  
                    Some(r##"List categories for onboarding experience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/onboarding_list-categories",
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
            ("list-category-volumes",  
                    Some(r##"List available volumes under categories for onboarding experience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/onboarding_list-category-volumes",
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
        
        ("promooffer", "methods: 'accept', 'dismiss' and 'get'", vec![
            ("accept",  
                    Some(r##""##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/promooffer_accept",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("dismiss",  
                    Some(r##""##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/promooffer_dismiss",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Returns a list of promo offers available to the user"##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/promooffer_get",
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
        
        ("volumes", "methods: 'associated-list', 'get', 'list', 'mybooks-list', 'recommended-list', 'recommended-rate' and 'useruploaded-list'", vec![
            ("associated-list",  
                    Some(r##"Return a list of associated books."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_associated-list",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of the source volume."##),
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
                    Some(r##"Gets volume information for a single volume."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_get",
                  vec![
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of volume to retrieve."##),
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
                    Some(r##"Performs a book search."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_list",
                  vec![
                    (Some(r##"q"##),
                     None,
                     Some(r##"Full-text search query string."##),
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
            ("mybooks-list",  
                    Some(r##"Return a list of books in My Library."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_mybooks-list",
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
            ("recommended-list",  
                    Some(r##"Return a list of recommended books for the current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_recommended-list",
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
            ("recommended-rate",  
                    Some(r##"Rate a recommended book for the current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_recommended-rate",
                  vec![
                    (Some(r##"rating"##),
                     None,
                     Some(r##"Rating to be given to the volume."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"volume-id"##),
                     None,
                     Some(r##"ID of the source volume."##),
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
            ("useruploaded-list",  
                    Some(r##"Return a list of books uploaded by the current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_books1_cli/volumes_useruploaded-list",
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
        
    ];
    
    let mut app = App::new("books1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150401")
           .about("Lets you search for books and manage your Google Books library.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_books1_cli")
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