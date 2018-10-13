// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_clouddebugger2 as api;

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
    hub: api::CloudDebugger<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _controller_debuggees_breakpoints_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.controller().debuggees_breakpoints_list(opt.value_of("debuggee-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "wait-token" => {
                    call = call.wait_token(value.unwrap_or(""));
                },
                "success-on-timeout" => {
                    call = call.success_on_timeout(arg_from_str(value.unwrap_or("false"), err, "success-on-timeout", "boolean"));
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
                                                                           v.extend(["wait-token", "success-on-timeout"].iter().map(|v|*v));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _controller_debuggees_breakpoints_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "breakpoint.status.is-error" => Some(("breakpoint.status.isError", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "breakpoint.status.refers-to" => Some(("breakpoint.status.refersTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.status.description.parameters" => Some(("breakpoint.status.description.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "breakpoint.status.description.format" => Some(("breakpoint.status.description.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.final-time" => Some(("breakpoint.finalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.log-message-format" => Some(("breakpoint.logMessageFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.log-level" => Some(("breakpoint.logLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.labels" => Some(("breakpoint.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "breakpoint.user-email" => Some(("breakpoint.userEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.id" => Some(("breakpoint.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.location.column" => Some(("breakpoint.location.column", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "breakpoint.location.path" => Some(("breakpoint.location.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.location.line" => Some(("breakpoint.location.line", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "breakpoint.action" => Some(("breakpoint.action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.expressions" => Some(("breakpoint.expressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "breakpoint.is-final-state" => Some(("breakpoint.isFinalState", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "breakpoint.create-time" => Some(("breakpoint.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "breakpoint.condition" => Some(("breakpoint.condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "breakpoint", "column", "condition", "create-time", "description", "expressions", "final-time", "format", "id", "is-error", "is-final-state", "labels", "line", "location", "log-level", "log-message-format", "parameters", "path", "refers-to", "status", "user-email"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateActiveBreakpointRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.controller().debuggees_breakpoints_update(request, opt.value_of("debuggee-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _controller_debuggees_register(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "debuggee.status.is-error" => Some(("debuggee.status.isError", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "debuggee.status.refers-to" => Some(("debuggee.status.refersTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "debuggee.status.description.parameters" => Some(("debuggee.status.description.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "debuggee.status.description.format" => Some(("debuggee.status.description.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "debuggee.description" => Some(("debuggee.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "debuggee.is-disabled" => Some(("debuggee.isDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "debuggee.labels" => Some(("debuggee.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "debuggee.uniquifier" => Some(("debuggee.uniquifier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "debuggee.project" => Some(("debuggee.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "debuggee.agent-version" => Some(("debuggee.agentVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "debuggee.is-inactive" => Some(("debuggee.isInactive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "debuggee.id" => Some(("debuggee.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-version", "debuggee", "description", "format", "id", "is-disabled", "is-error", "is-inactive", "labels", "parameters", "project", "refers-to", "status", "uniquifier"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RegisterDebuggeeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.controller().debuggees_register(request);
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _debugger_debuggees_breakpoints_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.debugger().debuggees_breakpoints_delete(opt.value_of("debuggee-id").unwrap_or(""), opt.value_of("breakpoint-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "client-version" => {
                    call = call.client_version(value.unwrap_or(""));
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
                                                                           v.extend(["client-version"].iter().map(|v|*v));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _debugger_debuggees_breakpoints_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.debugger().debuggees_breakpoints_get(opt.value_of("debuggee-id").unwrap_or(""), opt.value_of("breakpoint-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "client-version" => {
                    call = call.client_version(value.unwrap_or(""));
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
                                                                           v.extend(["client-version"].iter().map(|v|*v));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _debugger_debuggees_breakpoints_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.debugger().debuggees_breakpoints_list(opt.value_of("debuggee-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "wait-token" => {
                    call = call.wait_token(value.unwrap_or(""));
                },
                "strip-results" => {
                    call = call.strip_results(arg_from_str(value.unwrap_or("false"), err, "strip-results", "boolean"));
                },
                "include-inactive" => {
                    call = call.include_inactive(arg_from_str(value.unwrap_or("false"), err, "include-inactive", "boolean"));
                },
                "include-all-users" => {
                    call = call.include_all_users(arg_from_str(value.unwrap_or("false"), err, "include-all-users", "boolean"));
                },
                "client-version" => {
                    call = call.client_version(value.unwrap_or(""));
                },
                "action-value" => {
                    call = call.action_value(value.unwrap_or(""));
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
                                                                           v.extend(["strip-results", "include-all-users", "client-version", "include-inactive", "wait-token", "action-value"].iter().map(|v|*v));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _debugger_debuggees_breakpoints_set(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.is-error" => Some(("status.isError", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.refers-to" => Some(("status.refersTo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.description.parameters" => Some(("status.description.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "status.description.format" => Some(("status.description.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-time" => Some(("finalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "log-message-format" => Some(("logMessageFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "log-level" => Some(("logLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "user-email" => Some(("userEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.column" => Some(("location.column", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.path" => Some(("location.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.line" => Some(("location.line", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expressions" => Some(("expressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-final-state" => Some(("isFinalState", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "condition" => Some(("condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "column", "condition", "create-time", "description", "expressions", "final-time", "format", "id", "is-error", "is-final-state", "labels", "line", "location", "log-level", "log-message-format", "parameters", "path", "refers-to", "status", "user-email"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Breakpoint = json::value::from_value(object).unwrap();
        let mut call = self.hub.debugger().debuggees_breakpoints_set(request, opt.value_of("debuggee-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "client-version" => {
                    call = call.client_version(value.unwrap_or(""));
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
                                                                           v.extend(["client-version"].iter().map(|v|*v));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _debugger_debuggees_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.debugger().debuggees_list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project" => {
                    call = call.project(value.unwrap_or(""));
                },
                "include-inactive" => {
                    call = call.include_inactive(arg_from_str(value.unwrap_or("false"), err, "include-inactive", "boolean"));
                },
                "client-version" => {
                    call = call.client_version(value.unwrap_or(""));
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
                                                                           v.extend(["project", "include-inactive", "client-version"].iter().map(|v|*v));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
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
            ("controller", Some(opt)) => {
                match opt.subcommand() {
                    ("debuggees-breakpoints-list", Some(opt)) => {
                        call_result = self._controller_debuggees_breakpoints_list(opt, dry_run, &mut err);
                    },
                    ("debuggees-breakpoints-update", Some(opt)) => {
                        call_result = self._controller_debuggees_breakpoints_update(opt, dry_run, &mut err);
                    },
                    ("debuggees-register", Some(opt)) => {
                        call_result = self._controller_debuggees_register(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("controller".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("debugger", Some(opt)) => {
                match opt.subcommand() {
                    ("debuggees-breakpoints-delete", Some(opt)) => {
                        call_result = self._debugger_debuggees_breakpoints_delete(opt, dry_run, &mut err);
                    },
                    ("debuggees-breakpoints-get", Some(opt)) => {
                        call_result = self._debugger_debuggees_breakpoints_get(opt, dry_run, &mut err);
                    },
                    ("debuggees-breakpoints-list", Some(opt)) => {
                        call_result = self._debugger_debuggees_breakpoints_list(opt, dry_run, &mut err);
                    },
                    ("debuggees-breakpoints-set", Some(opt)) => {
                        call_result = self._debugger_debuggees_breakpoints_set(opt, dry_run, &mut err);
                    },
                    ("debuggees-list", Some(opt)) => {
                        call_result = self._debugger_debuggees_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("debugger".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "clouddebugger2-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "clouddebugger2",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::CloudDebugger::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
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
    let arg_data = [
        ("controller", "methods: 'debuggees-breakpoints-list', 'debuggees-breakpoints-update' and 'debuggees-register'", vec![
            ("debuggees-breakpoints-list",
                    Some(r##"Returns the list of all active breakpoints for the debuggee.
        
        The breakpoint specification (`location`, `condition`, and `expressions`
        fields) is semantically immutable, although the field values may
        change. For example, an agent may update the location line number
        to reflect the actual line where the breakpoint was set, but this
        doesn't change the breakpoint semantics.
        
        This means that an agent does not need to check if a breakpoint has changed
        when it encounters the same breakpoint on a successive call.
        Moreover, an agent should remember the breakpoints that are completed
        until the controller removes them from the active list to avoid
        setting those breakpoints again."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/controller_debuggees-breakpoints-list",
                  vec![
                    (Some(r##"debuggee-id"##),
                     None,
                     Some(r##"Identifies the debuggee."##),
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
            ("debuggees-breakpoints-update",
                    Some(r##"Updates the breakpoint state or mutable fields.
        The entire Breakpoint message must be sent back to the controller service.
        
        Updates to active breakpoint fields are only allowed if the new value
        does not change the breakpoint specification. Updates to the `location`,
        `condition` and `expressions` fields should not alter the breakpoint
        semantics. These may only make changes such as canonicalizing a value
        or snapping the location to the correct line of code."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/controller_debuggees-breakpoints-update",
                  vec![
                    (Some(r##"debuggee-id"##),
                     None,
                     Some(r##"Identifies the debuggee being debugged."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"Breakpoint identifier, unique in the scope of the debuggee."##),
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
            ("debuggees-register",
                    Some(r##"Registers the debuggee with the controller service.
        
        All agents attached to the same application must call this method with
        exactly the same request content to get back the same stable `debuggee_id`.
        Agents should call this method again whenever `google.rpc.Code.NOT_FOUND`
        is returned from any controller method.
        
        This protocol allows the controller service to disable debuggees, recover
        from data loss, or change the `debuggee_id` format. Agents must handle
        `debuggee_id` value changing upon re-registration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/controller_debuggees-register",
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
        
        ("debugger", "methods: 'debuggees-breakpoints-delete', 'debuggees-breakpoints-get', 'debuggees-breakpoints-list', 'debuggees-breakpoints-set' and 'debuggees-list'", vec![
            ("debuggees-breakpoints-delete",
                    Some(r##"Deletes the breakpoint from the debuggee."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/debugger_debuggees-breakpoints-delete",
                  vec![
                    (Some(r##"debuggee-id"##),
                     None,
                     Some(r##"ID of the debuggee whose breakpoint to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"breakpoint-id"##),
                     None,
                     Some(r##"ID of the breakpoint to delete."##),
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
            ("debuggees-breakpoints-get",
                    Some(r##"Gets breakpoint information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/debugger_debuggees-breakpoints-get",
                  vec![
                    (Some(r##"debuggee-id"##),
                     None,
                     Some(r##"ID of the debuggee whose breakpoint to get."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"breakpoint-id"##),
                     None,
                     Some(r##"ID of the breakpoint to get."##),
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
            ("debuggees-breakpoints-list",
                    Some(r##"Lists all breakpoints for the debuggee."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/debugger_debuggees-breakpoints-list",
                  vec![
                    (Some(r##"debuggee-id"##),
                     None,
                     Some(r##"ID of the debuggee whose breakpoints to list."##),
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
            ("debuggees-breakpoints-set",
                    Some(r##"Sets the breakpoint to the debuggee."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/debugger_debuggees-breakpoints-set",
                  vec![
                    (Some(r##"debuggee-id"##),
                     None,
                     Some(r##"ID of the debuggee where the breakpoint is to be set."##),
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
            ("debuggees-list",
                    Some(r##"Lists all the debuggees that the user has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli/debugger_debuggees-list",
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
    
    let mut app = App::new("clouddebugger2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.7+20180925")
           .about("Examines the call stack and variables of a running application without stopping or slowing it down.
           ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_clouddebugger2_cli")
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