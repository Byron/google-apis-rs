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
extern crate google_gmail1 as api;

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
    hub: api::Gmail<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _users_drafts_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Draft::default();
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
            fn request_message_init(request: &mut api::Draft) {
                if request.message.is_none() {
                    request.message = Some(Default::default());
                }
            }
            
            fn request_message_payload_body_init(request: &mut api::Draft) {
                request_message_payload_init(request);
                if request.message.as_mut().unwrap().payload.as_mut().unwrap().body.is_none() {
                    request.message.as_mut().unwrap().payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_message_payload_init(request: &mut api::Draft) {
                request_message_init(request);
                if request.message.as_mut().unwrap().payload.is_none() {
                    request.message.as_mut().unwrap().payload = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "message.history-id" => {
                        request_message_init(&mut request);
                        request.message.as_mut().unwrap().history_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.data" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.attachment-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.size" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "message.payload.body.size", "integer"));
                    },
                "message.payload.mime-type" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.part-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.filename" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "message.snippet" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().snippet = Some(value.unwrap_or("").to_string());
                    },
                "message.raw" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().raw = Some(value.unwrap_or("").to_string());
                    },
                "message.size-estimate" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "message.size-estimate", "integer"));
                    },
                "message.thread-id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().thread_id = Some(value.unwrap_or("").to_string());
                    },
                "message.label-ids" => {
                        request_message_payload_init(&mut request);
                        if request.message.as_mut().unwrap().label_ids.is_none() {
                           request.message.as_mut().unwrap().label_ids = Some(Default::default());
                        }
                                        request.message.as_mut().unwrap().label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "message.id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_message_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attachment-id", "body", "data", "filename", "history-id", "id", "label-ids", "message", "mime-type", "part-id", "payload", "raw", "size", "size-estimate", "snippet", "thread-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().drafts_create(request, opt.value_of("user-id").unwrap_or(""));
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

    fn _users_drafts_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().drafts_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_drafts_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().drafts_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "format" => {
                    call = call.format(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["format"]
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

    fn _users_drafts_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().drafts_list(opt.value_of("user-id").unwrap_or(""));
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

    fn _users_drafts_send(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Draft::default();
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
            fn request_message_init(request: &mut api::Draft) {
                if request.message.is_none() {
                    request.message = Some(Default::default());
                }
            }
            
            fn request_message_payload_body_init(request: &mut api::Draft) {
                request_message_payload_init(request);
                if request.message.as_mut().unwrap().payload.as_mut().unwrap().body.is_none() {
                    request.message.as_mut().unwrap().payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_message_payload_init(request: &mut api::Draft) {
                request_message_init(request);
                if request.message.as_mut().unwrap().payload.is_none() {
                    request.message.as_mut().unwrap().payload = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "message.history-id" => {
                        request_message_init(&mut request);
                        request.message.as_mut().unwrap().history_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.data" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.attachment-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.size" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "message.payload.body.size", "integer"));
                    },
                "message.payload.mime-type" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.part-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.filename" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "message.snippet" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().snippet = Some(value.unwrap_or("").to_string());
                    },
                "message.raw" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().raw = Some(value.unwrap_or("").to_string());
                    },
                "message.size-estimate" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "message.size-estimate", "integer"));
                    },
                "message.thread-id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().thread_id = Some(value.unwrap_or("").to_string());
                    },
                "message.label-ids" => {
                        request_message_payload_init(&mut request);
                        if request.message.as_mut().unwrap().label_ids.is_none() {
                           request.message.as_mut().unwrap().label_ids = Some(Default::default());
                        }
                                        request.message.as_mut().unwrap().label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "message.id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_message_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attachment-id", "body", "data", "filename", "history-id", "id", "label-ids", "message", "mime-type", "part-id", "payload", "raw", "size", "size-estimate", "snippet", "thread-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().drafts_send(request, opt.value_of("user-id").unwrap_or(""));
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

    fn _users_drafts_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Draft::default();
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
            fn request_message_init(request: &mut api::Draft) {
                if request.message.is_none() {
                    request.message = Some(Default::default());
                }
            }
            
            fn request_message_payload_body_init(request: &mut api::Draft) {
                request_message_payload_init(request);
                if request.message.as_mut().unwrap().payload.as_mut().unwrap().body.is_none() {
                    request.message.as_mut().unwrap().payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_message_payload_init(request: &mut api::Draft) {
                request_message_init(request);
                if request.message.as_mut().unwrap().payload.is_none() {
                    request.message.as_mut().unwrap().payload = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "message.history-id" => {
                        request_message_init(&mut request);
                        request.message.as_mut().unwrap().history_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.data" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.attachment-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.size" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "message.payload.body.size", "integer"));
                    },
                "message.payload.mime-type" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.part-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.filename" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "message.snippet" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().snippet = Some(value.unwrap_or("").to_string());
                    },
                "message.raw" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().raw = Some(value.unwrap_or("").to_string());
                    },
                "message.size-estimate" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "message.size-estimate", "integer"));
                    },
                "message.thread-id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().thread_id = Some(value.unwrap_or("").to_string());
                    },
                "message.label-ids" => {
                        request_message_payload_init(&mut request);
                        if request.message.as_mut().unwrap().label_ids.is_none() {
                           request.message.as_mut().unwrap().label_ids = Some(Default::default());
                        }
                                        request.message.as_mut().unwrap().label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "message.id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_message_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attachment-id", "body", "data", "filename", "history-id", "id", "label-ids", "message", "mime-type", "part-id", "payload", "raw", "size", "size-estimate", "snippet", "thread-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().drafts_update(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_get_profile(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().get_profile(opt.value_of("user-id").unwrap_or(""));
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

    fn _users_history_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().history_list(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-history-id" => {
                    call = call.start_history_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "label-id" => {
                    call = call.label_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["page-token", "label-id", "start-history-id", "max-results"]
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

    fn _users_labels_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Label::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "messages-total" => {
                        request.messages_total = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-total", "integer"));
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "threads-total" => {
                        request.threads_total = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-total", "integer"));
                    },
                "label-list-visibility" => {
                        request.label_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "threads-unread" => {
                        request.threads_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-unread", "integer"));
                    },
                "message-list-visibility" => {
                        request.message_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "messages-unread" => {
                        request.messages_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-unread", "integer"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "label-list-visibility", "message-list-visibility", "messages-total", "messages-unread", "name", "threads-total", "threads-unread", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().labels_create(request, opt.value_of("user-id").unwrap_or(""));
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

    fn _users_labels_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().labels_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_labels_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().labels_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_labels_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().labels_list(opt.value_of("user-id").unwrap_or(""));
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

    fn _users_labels_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Label::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "messages-total" => {
                        request.messages_total = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-total", "integer"));
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "threads-total" => {
                        request.threads_total = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-total", "integer"));
                    },
                "label-list-visibility" => {
                        request.label_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "threads-unread" => {
                        request.threads_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-unread", "integer"));
                    },
                "message-list-visibility" => {
                        request.message_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "messages-unread" => {
                        request.messages_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-unread", "integer"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "label-list-visibility", "message-list-visibility", "messages-total", "messages-unread", "name", "threads-total", "threads-unread", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().labels_patch(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_labels_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Label::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "messages-total" => {
                        request.messages_total = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-total", "integer"));
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "threads-total" => {
                        request.threads_total = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-total", "integer"));
                    },
                "label-list-visibility" => {
                        request.label_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "threads-unread" => {
                        request.threads_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-unread", "integer"));
                    },
                "message-list-visibility" => {
                        request.message_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "messages-unread" => {
                        request.messages_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-unread", "integer"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "label-list-visibility", "message-list-visibility", "messages-total", "messages-unread", "name", "threads-total", "threads-unread", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().labels_update(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_messages_attachments_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().messages_attachments_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("message-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_messages_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().messages_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_messages_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().messages_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "metadata-headers" => {
                    call = call.add_metadata_headers(value.unwrap_or(""));
                },
                "format" => {
                    call = call.format(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["metadata-headers", "format"]
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

    fn _users_messages_import(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Message::default();
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
            fn request_payload_body_init(request: &mut api::Message) {
                request_payload_init(request);
                if request.payload.as_mut().unwrap().body.is_none() {
                    request.payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_payload_init(request: &mut api::Message) {
                if request.payload.is_none() {
                    request.payload = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "history-id" => {
                        request.history_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.data" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.attachment-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.size" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "payload.body.size", "integer"));
                    },
                "payload.mime-type" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "payload.part-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.filename" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request_payload_init(&mut request);
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "raw" => {
                        request_payload_init(&mut request);
                        request.raw = Some(value.unwrap_or("").to_string());
                    },
                "size-estimate" => {
                        request_payload_init(&mut request);
                        request.size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "size-estimate", "integer"));
                    },
                "thread-id" => {
                        request_payload_init(&mut request);
                        request.thread_id = Some(value.unwrap_or("").to_string());
                    },
                "label-ids" => {
                        request_payload_init(&mut request);
                        if request.label_ids.is_none() {
                           request.label_ids = Some(Default::default());
                        }
                                        request.label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_payload_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attachment-id", "body", "data", "filename", "history-id", "id", "label-ids", "mime-type", "part-id", "payload", "raw", "size", "size-estimate", "snippet", "thread-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().messages_import(request, opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "process-for-calendar" => {
                    call = call.process_for_calendar(arg_from_str(value.unwrap_or("false"), err, "process-for-calendar", "boolean"));
                },
                "never-mark-spam" => {
                    call = call.never_mark_spam(arg_from_str(value.unwrap_or("false"), err, "never-mark-spam", "boolean"));
                },
                "internal-date-source" => {
                    call = call.internal_date_source(value.unwrap_or(""));
                },
                "deleted" => {
                    call = call.deleted(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
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
                                                Vec::new() + &self.gp + &["process-for-calendar", "deleted", "internal-date-source", "never-mark-spam"]
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

    fn _users_messages_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Message::default();
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
            fn request_payload_body_init(request: &mut api::Message) {
                request_payload_init(request);
                if request.payload.as_mut().unwrap().body.is_none() {
                    request.payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_payload_init(request: &mut api::Message) {
                if request.payload.is_none() {
                    request.payload = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "history-id" => {
                        request.history_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.data" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.attachment-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.size" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "payload.body.size", "integer"));
                    },
                "payload.mime-type" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "payload.part-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.filename" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request_payload_init(&mut request);
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "raw" => {
                        request_payload_init(&mut request);
                        request.raw = Some(value.unwrap_or("").to_string());
                    },
                "size-estimate" => {
                        request_payload_init(&mut request);
                        request.size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "size-estimate", "integer"));
                    },
                "thread-id" => {
                        request_payload_init(&mut request);
                        request.thread_id = Some(value.unwrap_or("").to_string());
                    },
                "label-ids" => {
                        request_payload_init(&mut request);
                        if request.label_ids.is_none() {
                           request.label_ids = Some(Default::default());
                        }
                                        request.label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_payload_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attachment-id", "body", "data", "filename", "history-id", "id", "label-ids", "mime-type", "part-id", "payload", "raw", "size", "size-estimate", "snippet", "thread-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().messages_insert(request, opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "internal-date-source" => {
                    call = call.internal_date_source(value.unwrap_or(""));
                },
                "deleted" => {
                    call = call.deleted(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
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
                                                Vec::new() + &self.gp + &["deleted", "internal-date-source"]
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

    fn _users_messages_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().messages_list(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "label-ids" => {
                    call = call.add_label_ids(value.unwrap_or(""));
                },
                "include-spam-trash" => {
                    call = call.include_spam_trash(arg_from_str(value.unwrap_or("false"), err, "include-spam-trash", "boolean"));
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
                                                Vec::new() + &self.gp + &["q", "page-token", "include-spam-trash", "max-results", "label-ids"]
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

    fn _users_messages_modify(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ModifyMessageRequest::default();
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
                "remove-label-ids" => {
                        if request.remove_label_ids.is_none() {
                           request.remove_label_ids = Some(Default::default());
                        }
                                        request.remove_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "add-label-ids" => {
                        if request.add_label_ids.is_none() {
                           request.add_label_ids = Some(Default::default());
                        }
                                        request.add_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["add-label-ids", "remove-label-ids"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().messages_modify(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_messages_send(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Message::default();
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
            fn request_payload_body_init(request: &mut api::Message) {
                request_payload_init(request);
                if request.payload.as_mut().unwrap().body.is_none() {
                    request.payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_payload_init(request: &mut api::Message) {
                if request.payload.is_none() {
                    request.payload = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "history-id" => {
                        request.history_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.data" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.attachment-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.size" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "payload.body.size", "integer"));
                    },
                "payload.mime-type" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "payload.part-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.filename" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request_payload_init(&mut request);
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "raw" => {
                        request_payload_init(&mut request);
                        request.raw = Some(value.unwrap_or("").to_string());
                    },
                "size-estimate" => {
                        request_payload_init(&mut request);
                        request.size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "size-estimate", "integer"));
                    },
                "thread-id" => {
                        request_payload_init(&mut request);
                        request.thread_id = Some(value.unwrap_or("").to_string());
                    },
                "label-ids" => {
                        request_payload_init(&mut request);
                        if request.label_ids.is_none() {
                           request.label_ids = Some(Default::default());
                        }
                                        request.label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_payload_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attachment-id", "body", "data", "filename", "history-id", "id", "label-ids", "mime-type", "part-id", "payload", "raw", "size", "size-estimate", "snippet", "thread-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().messages_send(request, opt.value_of("user-id").unwrap_or(""));
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

    fn _users_messages_trash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().messages_trash(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_messages_untrash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().messages_untrash(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_threads_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().threads_delete(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_threads_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().threads_get(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "metadata-headers" => {
                    call = call.add_metadata_headers(value.unwrap_or(""));
                },
                "format" => {
                    call = call.format(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["metadata-headers", "format"]
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

    fn _users_threads_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().threads_list(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "label-ids" => {
                    call = call.add_label_ids(value.unwrap_or(""));
                },
                "include-spam-trash" => {
                    call = call.include_spam_trash(arg_from_str(value.unwrap_or("false"), err, "include-spam-trash", "boolean"));
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
                                                Vec::new() + &self.gp + &["q", "page-token", "include-spam-trash", "max-results", "label-ids"]
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

    fn _users_threads_modify(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ModifyThreadRequest::default();
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
                "remove-label-ids" => {
                        if request.remove_label_ids.is_none() {
                           request.remove_label_ids = Some(Default::default());
                        }
                                        request.remove_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "add-label-ids" => {
                        if request.add_label_ids.is_none() {
                           request.add_label_ids = Some(Default::default());
                        }
                                        request.add_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["add-label-ids", "remove-label-ids"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.users().threads_modify(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_threads_trash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().threads_trash(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _users_threads_untrash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().threads_untrash(opt.value_of("user-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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
            ("users", Some(opt)) => {
                match opt.subcommand() {
                    ("drafts-create", Some(opt)) => {
                        call_result = self._users_drafts_create(opt, dry_run, &mut err);
                    },
                    ("drafts-delete", Some(opt)) => {
                        call_result = self._users_drafts_delete(opt, dry_run, &mut err);
                    },
                    ("drafts-get", Some(opt)) => {
                        call_result = self._users_drafts_get(opt, dry_run, &mut err);
                    },
                    ("drafts-list", Some(opt)) => {
                        call_result = self._users_drafts_list(opt, dry_run, &mut err);
                    },
                    ("drafts-send", Some(opt)) => {
                        call_result = self._users_drafts_send(opt, dry_run, &mut err);
                    },
                    ("drafts-update", Some(opt)) => {
                        call_result = self._users_drafts_update(opt, dry_run, &mut err);
                    },
                    ("get-profile", Some(opt)) => {
                        call_result = self._users_get_profile(opt, dry_run, &mut err);
                    },
                    ("history-list", Some(opt)) => {
                        call_result = self._users_history_list(opt, dry_run, &mut err);
                    },
                    ("labels-create", Some(opt)) => {
                        call_result = self._users_labels_create(opt, dry_run, &mut err);
                    },
                    ("labels-delete", Some(opt)) => {
                        call_result = self._users_labels_delete(opt, dry_run, &mut err);
                    },
                    ("labels-get", Some(opt)) => {
                        call_result = self._users_labels_get(opt, dry_run, &mut err);
                    },
                    ("labels-list", Some(opt)) => {
                        call_result = self._users_labels_list(opt, dry_run, &mut err);
                    },
                    ("labels-patch", Some(opt)) => {
                        call_result = self._users_labels_patch(opt, dry_run, &mut err);
                    },
                    ("labels-update", Some(opt)) => {
                        call_result = self._users_labels_update(opt, dry_run, &mut err);
                    },
                    ("messages-attachments-get", Some(opt)) => {
                        call_result = self._users_messages_attachments_get(opt, dry_run, &mut err);
                    },
                    ("messages-delete", Some(opt)) => {
                        call_result = self._users_messages_delete(opt, dry_run, &mut err);
                    },
                    ("messages-get", Some(opt)) => {
                        call_result = self._users_messages_get(opt, dry_run, &mut err);
                    },
                    ("messages-import", Some(opt)) => {
                        call_result = self._users_messages_import(opt, dry_run, &mut err);
                    },
                    ("messages-insert", Some(opt)) => {
                        call_result = self._users_messages_insert(opt, dry_run, &mut err);
                    },
                    ("messages-list", Some(opt)) => {
                        call_result = self._users_messages_list(opt, dry_run, &mut err);
                    },
                    ("messages-modify", Some(opt)) => {
                        call_result = self._users_messages_modify(opt, dry_run, &mut err);
                    },
                    ("messages-send", Some(opt)) => {
                        call_result = self._users_messages_send(opt, dry_run, &mut err);
                    },
                    ("messages-trash", Some(opt)) => {
                        call_result = self._users_messages_trash(opt, dry_run, &mut err);
                    },
                    ("messages-untrash", Some(opt)) => {
                        call_result = self._users_messages_untrash(opt, dry_run, &mut err);
                    },
                    ("threads-delete", Some(opt)) => {
                        call_result = self._users_threads_delete(opt, dry_run, &mut err);
                    },
                    ("threads-get", Some(opt)) => {
                        call_result = self._users_threads_get(opt, dry_run, &mut err);
                    },
                    ("threads-list", Some(opt)) => {
                        call_result = self._users_threads_list(opt, dry_run, &mut err);
                    },
                    ("threads-modify", Some(opt)) => {
                        call_result = self._users_threads_modify(opt, dry_run, &mut err);
                    },
                    ("threads-trash", Some(opt)) => {
                        call_result = self._users_threads_trash(opt, dry_run, &mut err);
                    },
                    ("threads-untrash", Some(opt)) => {
                        call_result = self._users_threads_untrash(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "gmail1-secret.json", 
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
                                          program_name: "gmail1",
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
            hub: api::Gmail::new(client, auth),
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
        ("users", "methods: 'drafts-create', 'drafts-delete', 'drafts-get', 'drafts-list', 'drafts-send', 'drafts-update', 'get-profile', 'history-list', 'labels-create', 'labels-delete', 'labels-get', 'labels-list', 'labels-patch', 'labels-update', 'messages-attachments-get', 'messages-delete', 'messages-get', 'messages-import', 'messages-insert', 'messages-list', 'messages-modify', 'messages-send', 'messages-trash', 'messages-untrash', 'threads-delete', 'threads-get', 'threads-list', 'threads-modify', 'threads-trash' and 'threads-untrash'", vec![
            ("drafts-create",  
                    Some(r##"Creates a new draft with the DRAFT label."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_drafts-create",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("drafts-delete",  
                    Some(r##"Immediately and permanently deletes the specified draft. Does not simply trash it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_drafts-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the draft to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("drafts-get",  
                    Some(r##"Gets the specified draft."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_drafts-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the draft to retrieve."##),
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
            ("drafts-list",  
                    Some(r##"Lists the drafts in the user's mailbox."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_drafts-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("drafts-send",  
                    Some(r##"Sends the specified, existing draft to the recipients in the To, Cc, and Bcc headers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_drafts-send",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("drafts-update",  
                    Some(r##"Replaces a draft's content."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_drafts-update",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the draft to update."##),
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
            ("get-profile",  
                    Some(r##"Gets the current user's Gmail profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_get-profile",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("history-list",  
                    Some(r##"Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing historyId)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_history-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("labels-create",  
                    Some(r##"Creates a new label."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_labels-create",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("labels-delete",  
                    Some(r##"Immediately and permanently deletes the specified label and removes it from any messages and threads that it is applied to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_labels-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the label to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("labels-get",  
                    Some(r##"Gets the specified label."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_labels-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the label to retrieve."##),
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
            ("labels-list",  
                    Some(r##"Lists all labels in the user's mailbox."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_labels-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("labels-patch",  
                    Some(r##"Updates the specified label. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_labels-patch",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the label to update."##),
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
            ("labels-update",  
                    Some(r##"Updates the specified label."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_labels-update",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the label to update."##),
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
            ("messages-attachments-get",  
                    Some(r##"Gets the specified message attachment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-attachments-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"message-id"##),
                     None,
                     Some(r##"The ID of the message containing the attachment."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the attachment."##),
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
            ("messages-delete",  
                    Some(r##"Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer messages.trash instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the message to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("messages-get",  
                    Some(r##"Gets the specified message."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the message to retrieve."##),
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
            ("messages-import",  
                    Some(r##"Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. Does not send a message."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-import",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("messages-insert",  
                    Some(r##"Directly inserts a message into only this user's mailbox similar to IMAP APPEND, bypassing most scanning and classification. Does not send a message."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-insert",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("messages-list",  
                    Some(r##"Lists the messages in the user's mailbox."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("messages-modify",  
                    Some(r##"Modifies the labels on the specified message."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-modify",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the message to modify."##),
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
            ("messages-send",  
                    Some(r##"Sends the specified message to the recipients in the To, Cc, and Bcc headers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-send",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("messages-trash",  
                    Some(r##"Moves the specified message to the trash."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-trash",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the message to Trash."##),
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
            ("messages-untrash",  
                    Some(r##"Removes the specified message from the trash."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_messages-untrash",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the message to remove from Trash."##),
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
            ("threads-delete",  
                    Some(r##"Immediately and permanently deletes the specified thread. This operation cannot be undone. Prefer threads.trash instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_threads-delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"ID of the Thread to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("threads-get",  
                    Some(r##"Gets the specified thread."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_threads-get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the thread to retrieve."##),
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
            ("threads-list",  
                    Some(r##"Lists the threads in the user's mailbox."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_threads-list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
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
            ("threads-modify",  
                    Some(r##"Modifies the labels applied to the thread. This applies to all messages in the thread."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_threads-modify",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the thread to modify."##),
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
            ("threads-trash",  
                    Some(r##"Moves the specified thread to the trash."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_threads-trash",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the thread to Trash."##),
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
            ("threads-untrash",  
                    Some(r##"Removes the specified thread from the trash."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gmail1_cli/users_threads-untrash",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The user's email address. The special value me can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the thread to remove from Trash."##),
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
        
    ];
    
    let mut app = App::new("gmail1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150303")
           .about("The Gmail REST API.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_gmail1_cli")
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