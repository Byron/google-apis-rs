// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_drive2 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          protocol_from_str};

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
    hub: api::Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _about_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.about().get();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().get(opt.value_of("app-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _changes_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().get(opt.value_of("change-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _changes_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.changes().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _changes_watch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.changes().watch(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.channels().stop(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _children_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().delete(opt.value_of("folder-id").unwrap_or(""), opt.value_of("child-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _children_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().get(opt.value_of("folder-id").unwrap_or(""), opt.value_of("child-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _children_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ChildReference::default();
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
                "child-link" => {
                        request.child_link = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.children().insert(request, opt.value_of("folder-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _children_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.children().list(opt.value_of("folder-id").unwrap_or(""));
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _comments_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Comment::default();
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
            fn request_author_init(request: &mut api::Comment) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::Comment) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_context_init(request: &mut api::Comment) {
                if request.context.is_none() {
                    request.context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "file-title" => {
                        request_author_init(&mut request);
                        request.file_title = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "context.type" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "context.value" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_context_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "comment-id" => {
                        request_context_init(&mut request);
                        request.comment_id = Some(value.unwrap_or("").to_string());
                    },
                "anchor" => {
                        request_context_init(&mut request);
                        request.anchor = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_context_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "file-id" => {
                        request_context_init(&mut request);
                        request.file_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.comments().insert(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Comment::default();
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
            fn request_author_init(request: &mut api::Comment) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::Comment) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_context_init(request: &mut api::Comment) {
                if request.context.is_none() {
                    request.context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "file-title" => {
                        request_author_init(&mut request);
                        request.file_title = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "context.type" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "context.value" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_context_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "comment-id" => {
                        request_context_init(&mut request);
                        request.comment_id = Some(value.unwrap_or("").to_string());
                    },
                "anchor" => {
                        request_context_init(&mut request);
                        request.anchor = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_context_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "file-id" => {
                        request_context_init(&mut request);
                        request.file_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.comments().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Comment::default();
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
            fn request_author_init(request: &mut api::Comment) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::Comment) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_context_init(request: &mut api::Comment) {
                if request.context.is_none() {
                    request.context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "file-title" => {
                        request_author_init(&mut request);
                        request.file_title = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "context.type" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "context.value" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_context_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "comment-id" => {
                        request_context_init(&mut request);
                        request.comment_id = Some(value.unwrap_or("").to_string());
                    },
                "anchor" => {
                        request_context_init(&mut request);
                        request.anchor = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_context_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "file-id" => {
                        request_context_init(&mut request);
                        request.file_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.comments().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_copy(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::File::default();
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().copy(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().delete(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _files_empty_trash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().empty_trash();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _files_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.files().get(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _files_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::File::default();
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = protocol_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "q" => {
                    call = call.q(value.unwrap_or(""));
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
                "corpus" => {
                    call = call.corpus(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::File::default();
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().patch(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
                },
                "add-parents" => {
                    call = call.add_parents(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_touch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().touch(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_trash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().trash(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_untrash(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.files().untrash(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::File::default();
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().update(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
                },
                "add-parents" => {
                    call = call.add_parents(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = protocol_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _files_watch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.files().watch(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _parents_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("parent-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _parents_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("parent-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _parents_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ParentReference::default();
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
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "is-root" => {
                        request.is_root = Some(arg_from_str(value.unwrap_or("false"), err, "is-root", "boolean"));
                    },
                "parent-link" => {
                        request.parent_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.parents().insert(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _parents_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.parents().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _permissions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_get_id_for_email(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().get_id_for_email(opt.value_of("email").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Permission::default();
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
                "with-link" => {
                        request.with_link = Some(arg_from_str(value.unwrap_or("false"), err, "with-link", "boolean"));
                    },
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "auth-key" => {
                        request.auth_key = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "photo-link" => {
                        request.photo_link = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "additional-roles" => {
                        if request.additional_roles.is_none() {
                           request.additional_roles = Some(Default::default());
                        }
                                        request.additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.permissions().insert(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notification-emails" => {
                    call = call.send_notification_emails(arg_from_str(value.unwrap_or("false"), err, "send-notification-emails", "boolean"));
                },
                "email-message" => {
                    call = call.email_message(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.permissions().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Permission::default();
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
                "with-link" => {
                        request.with_link = Some(arg_from_str(value.unwrap_or("false"), err, "with-link", "boolean"));
                    },
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "auth-key" => {
                        request.auth_key = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "photo-link" => {
                        request.photo_link = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "additional-roles" => {
                        if request.additional_roles.is_none() {
                           request.additional_roles = Some(Default::default());
                        }
                                        request.additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.permissions().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _permissions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Permission::default();
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
                "with-link" => {
                        request.with_link = Some(arg_from_str(value.unwrap_or("false"), err, "with-link", "boolean"));
                    },
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "auth-key" => {
                        request.auth_key = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "photo-link" => {
                        request.photo_link = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "additional-roles" => {
                        if request.additional_roles.is_none() {
                           request.additional_roles = Some(Default::default());
                        }
                                        request.additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.permissions().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _properties_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Property::default();
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
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "visibility" => {
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "key" => {
                        request.key = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.properties().insert(request, opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Property::default();
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
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "visibility" => {
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "key" => {
                        request.key = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.properties().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _properties_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Property::default();
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
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "visibility" => {
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "key" => {
                        request.key = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.properties().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("property-key").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _realtime_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.realtime().get(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "revision" => {
                    call = call.revision(arg_from_str(value.unwrap_or("-0"), err, "revision", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _realtime_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.realtime().update(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "base-revision" => {
                    call = call.base_revision(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = protocol_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _replies_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _replies_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CommentReply::default();
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
            fn request_author_init(request: &mut api::CommentReply) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::CommentReply) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_author_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "reply-id" => {
                        request_author_init(&mut request);
                        request.reply_id = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_author_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.replies().insert(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.replies().list(opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CommentReply::default();
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
            fn request_author_init(request: &mut api::CommentReply) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::CommentReply) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_author_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "reply-id" => {
                        request_author_init(&mut request);
                        request.reply_id = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_author_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.replies().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _replies_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CommentReply::default();
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
            fn request_author_init(request: &mut api::CommentReply) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::CommentReply) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_author_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "reply-id" => {
                        request_author_init(&mut request);
                        request.reply_id = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_author_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.replies().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("comment-id").unwrap_or(""), opt.value_of("reply-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().delete(opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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

    fn _revisions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().get(opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.revisions().list(opt.value_of("file-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Revision::default();
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
            fn request_last_modifying_user_init(request: &mut api::Revision) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::Revision) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "pinned" => {
                        request.pinned = Some(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "publish-auto" => {
                        request.publish_auto = Some(arg_from_str(value.unwrap_or("false"), err, "publish-auto", "boolean"));
                    },
                "published-outside-domain" => {
                        request.published_outside_domain = Some(arg_from_str(value.unwrap_or("false"), err, "published-outside-domain", "boolean"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "published-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.published_link = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_last_modifying_user_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_last_modifying_user_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_last_modifying_user_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_last_modifying_user_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_last_modifying_user_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_last_modifying_user_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_last_modifying_user_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_last_modifying_user_init(&mut request);
                        request.published = Some(arg_from_str(value.unwrap_or("false"), err, "published", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.revisions().patch(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _revisions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Revision::default();
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
            fn request_last_modifying_user_init(request: &mut api::Revision) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::Revision) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "pinned" => {
                        request.pinned = Some(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "publish-auto" => {
                        request.publish_auto = Some(arg_from_str(value.unwrap_or("false"), err, "publish-auto", "boolean"));
                    },
                "published-outside-domain" => {
                        request.published_outside_domain = Some(arg_from_str(value.unwrap_or("false"), err, "published-outside-domain", "boolean"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "published-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.published_link = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_last_modifying_user_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_last_modifying_user_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_last_modifying_user_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_last_modifying_user_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_last_modifying_user_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_last_modifying_user_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_last_modifying_user_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_last_modifying_user_init(&mut request);
                        request.published = Some(arg_from_str(value.unwrap_or("false"), err, "published", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.revisions().update(request, opt.value_of("file-id").unwrap_or(""), opt.value_of("revision-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
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
                                                    connector: hyper::net::HttpConnector(None) 
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
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Drive::new(client, auth),
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
        ("about", "methods: 'get'", vec![
            ("get",  Some("Gets the information about the current user along with Drive API settings"), 
                  vec![
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("apps", "methods: 'get' and 'list'", vec![
            ("get",  Some("Gets a specific app."), 
                  vec![
                    (Some("app-id"),
                     None,
                     Some("The ID of the app."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a user's installed apps."), 
                  vec![
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("changes", "methods: 'get', 'list' and 'watch'", vec![
            ("get",  Some("Gets a specific change."), 
                  vec![
                    (Some("change-id"),
                     None,
                     Some("The ID of the change."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists the changes for a user."), 
                  vec![
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("watch",  Some("Subscribe to changes for a user."), 
                  vec![
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("channels", "methods: 'stop'", vec![
            ("stop",  Some("Stop watching resources through this channel"), 
                  vec![
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
        ("children", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",  Some("Removes a child from a folder."), 
                  vec![
                    (Some("folder-id"),
                     None,
                     Some("The ID of the folder."),
                     Some(true),
                     Some(false)),
        
                    (Some("child-id"),
                     None,
                     Some("The ID of the child."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a specific child reference."), 
                  vec![
                    (Some("folder-id"),
                     None,
                     Some("The ID of the folder."),
                     Some(true),
                     Some(false)),
        
                    (Some("child-id"),
                     None,
                     Some("The ID of the child."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Inserts a file into a folder."), 
                  vec![
                    (Some("folder-id"),
                     None,
                     Some("The ID of the folder."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a folder's children."), 
                  vec![
                    (Some("folder-id"),
                     None,
                     Some("The ID of the folder."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("comments", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  Some("Deletes a comment."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a comment by ID."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Creates a new comment on the given file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a file's comments."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",  Some("Updates an existing comment. This method supports patch semantics."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Updates an existing comment."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("files", "methods: 'copy', 'delete', 'empty-trash', 'get', 'insert', 'list', 'patch', 'touch', 'trash', 'untrash', 'update' and 'watch'", vec![
            ("copy",  Some("Creates a copy of the specified file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to copy."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("delete",  Some("Permanently deletes a file by ID. Skips the trash. The currently authenticated user must own the file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to delete."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("empty-trash",  Some("Permanently deletes all of the user's trashed files."), 
                  vec![
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a file's metadata by ID."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file in question."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Insert a new file."), 
                  vec![
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("mode"),
                     Some("u"),
                     Some("Specify the upload protocol (simple|resumable) and the file to upload"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists the user's files."), 
                  vec![
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",  Some("Updates file metadata and/or content. This method supports patch semantics."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to update."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("touch",  Some("Set the file's updated time to the current server time."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to update."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("trash",  Some("Moves a file to the trash."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to trash."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("untrash",  Some("Restores a file from the trash."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to untrash."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Updates file metadata and/or content."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file to update."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("mode"),
                     Some("u"),
                     Some("Specify the upload protocol (simple|resumable) and the file to upload"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("watch",  Some("Subscribe to changes on a file"), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file in question."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("parents", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",  Some("Removes a parent from a file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("parent-id"),
                     None,
                     Some("The ID of the parent."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a specific parent reference."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("parent-id"),
                     None,
                     Some("The ID of the parent."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Adds a parent folder for a file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a file's parents."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("permissions", "methods: 'delete', 'get', 'get-id-for-email', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  Some("Deletes a permission from a file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("permission-id"),
                     None,
                     Some("The ID for the permission."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a permission by ID."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("permission-id"),
                     None,
                     Some("The ID for the permission."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("get-id-for-email",  Some("Returns the permission ID for an email address."), 
                  vec![
                    (Some("email"),
                     None,
                     Some("The email address for which to return a permission ID"),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Inserts a permission for a file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a file's permissions."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",  Some("Updates a permission. This method supports patch semantics."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("permission-id"),
                     None,
                     Some("The ID for the permission."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Updates a permission."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("permission-id"),
                     None,
                     Some("The ID for the permission."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("properties", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  Some("Deletes a property."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("property-key"),
                     None,
                     Some("The key of the property."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a property by its key."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("property-key"),
                     None,
                     Some("The key of the property."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Adds a property to a file."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a file's properties."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",  Some("Updates a property. This method supports patch semantics."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("property-key"),
                     None,
                     Some("The key of the property."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Updates a property."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("property-key"),
                     None,
                     Some("The key of the property."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("realtime", "methods: 'get' and 'update'", vec![
            ("get",  Some("Exports the contents of the Realtime API data model associated with this file as JSON."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file that the Realtime API data model is associated with."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Overwrites the Realtime API data model associated with this file with the provided JSON data model."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file that the Realtime API data model is associated with."),
                     Some(true),
                     Some(false)),
        
                    (Some("mode"),
                     Some("u"),
                     Some("Specify the upload protocol (simple|resumable) and the file to upload"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
        ("replies", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  Some("Deletes a reply."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("reply-id"),
                     None,
                     Some("The ID of the reply."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a reply."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("reply-id"),
                     None,
                     Some("The ID of the reply."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",  Some("Creates a new reply to the given comment."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists all of the replies to a comment."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",  Some("Updates an existing reply. This method supports patch semantics."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("reply-id"),
                     None,
                     Some("The ID of the reply."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Updates an existing reply."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("comment-id"),
                     None,
                     Some("The ID of the comment."),
                     Some(true),
                     Some(false)),
        
                    (Some("reply-id"),
                     None,
                     Some("The ID of the reply."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("revisions", "methods: 'delete', 'get', 'list', 'patch' and 'update'", vec![
            ("delete",  Some("Removes a revision."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("revision-id"),
                     None,
                     Some("The ID of the revision."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  Some("Gets a specific revision."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("revision-id"),
                     None,
                     Some("The ID of the revision."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",  Some("Lists a file's revisions."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID of the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",  Some("Updates a revision. This method supports patch semantics."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("revision-id"),
                     None,
                     Some("The ID for the revision."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",  Some("Updates a revision."), 
                  vec![
                    (Some("file-id"),
                     None,
                     Some("The ID for the file."),
                     Some(true),
                     Some(false)),
        
                    (Some("revision-id"),
                     None,
                     Some("The ID for the revision."),
                     Some(true),
                     Some(false)),
        
                    (Some("kv"),
                     Some("r"),
                     Some("Set various fields of the request structure"),
                     Some(true),
                     Some(true)),
        
                    (Some("v"),
                     Some("p"),
                     Some("Set various fields of the request structure"),
                     Some(false),
                     Some(true)),
        
                    (Some("out"),
                     Some("o"),
                     Some("Specify the file into which to write the programs output"),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("drive2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150326")
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
           
           for &(main_command_name, ref about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::new(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, ref args) in subcommands {
                   let mut scmd = SubCommand::new(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
           
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