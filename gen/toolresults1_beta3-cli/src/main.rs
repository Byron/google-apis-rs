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
extern crate google_toolresults1_beta3 as api;

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
    hub: api::ToolResults<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _projects_get_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().get_settings(opt.value_of("project-id").unwrap_or(""));
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

    fn _projects_histories_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "history-id" => Some(("historyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "history-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::History = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().histories_create(request, opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
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

    fn _projects_histories_executions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "test-execution-matrix-id" => Some(("testExecutionMatrixId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time.nanos" => Some(("creationTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creation-time.seconds" => Some(("creationTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-id" => Some(("executionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "completion-time.nanos" => Some(("completionTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "completion-time.seconds" => Some(("completionTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.infrastructure-failure" => Some(("outcome.inconclusiveDetail.infrastructureFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.native-crash" => Some(("outcome.inconclusiveDetail.nativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.aborted-by-user" => Some(("outcome.inconclusiveDetail.abortedByUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-app-version" => Some(("outcome.skippedDetail.incompatibleAppVersion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-architecture" => Some(("outcome.skippedDetail.incompatibleArchitecture", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-device" => Some(("outcome.skippedDetail.incompatibleDevice", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.success-detail.other-native-crash" => Some(("outcome.successDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.other-native-crash" => Some(("outcome.failureDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.crashed" => Some(("outcome.failureDetail.crashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.not-installed" => Some(("outcome.failureDetail.notInstalled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.timed-out" => Some(("outcome.failureDetail.timedOut", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.summary" => Some(("outcome.summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aborted-by-user", "completion-time", "crashed", "creation-time", "execution-id", "failure-detail", "incompatible-app-version", "incompatible-architecture", "incompatible-device", "inconclusive-detail", "infrastructure-failure", "nanos", "native-crash", "not-installed", "other-native-crash", "outcome", "seconds", "skipped-detail", "state", "success-detail", "summary", "test-execution-matrix-id", "timed-out"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Execution = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().histories_executions_create(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
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

    fn _projects_histories_executions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_executions_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""));
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

    fn _projects_histories_executions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_executions_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_histories_executions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "test-execution-matrix-id" => Some(("testExecutionMatrixId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time.nanos" => Some(("creationTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creation-time.seconds" => Some(("creationTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-id" => Some(("executionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "completion-time.nanos" => Some(("completionTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "completion-time.seconds" => Some(("completionTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.infrastructure-failure" => Some(("outcome.inconclusiveDetail.infrastructureFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.native-crash" => Some(("outcome.inconclusiveDetail.nativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.aborted-by-user" => Some(("outcome.inconclusiveDetail.abortedByUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-app-version" => Some(("outcome.skippedDetail.incompatibleAppVersion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-architecture" => Some(("outcome.skippedDetail.incompatibleArchitecture", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-device" => Some(("outcome.skippedDetail.incompatibleDevice", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.success-detail.other-native-crash" => Some(("outcome.successDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.other-native-crash" => Some(("outcome.failureDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.crashed" => Some(("outcome.failureDetail.crashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.not-installed" => Some(("outcome.failureDetail.notInstalled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.timed-out" => Some(("outcome.failureDetail.timedOut", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.summary" => Some(("outcome.summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aborted-by-user", "completion-time", "crashed", "creation-time", "execution-id", "failure-detail", "incompatible-app-version", "incompatible-architecture", "incompatible-device", "inconclusive-detail", "infrastructure-failure", "nanos", "native-crash", "not-installed", "other-native-crash", "outcome", "seconds", "skipped-detail", "state", "success-detail", "summary", "test-execution-matrix-id", "timed-out"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Execution = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().histories_executions_patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
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

    fn _projects_histories_executions_steps_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "test-execution-step.test-timing.test-process-duration.nanos" => Some(("testExecutionStep.testTiming.testProcessDuration.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-execution-step.test-timing.test-process-duration.seconds" => Some(("testExecutionStep.testTiming.testProcessDuration.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-execution-step.tool-execution.exit-code.number" => Some(("testExecutionStep.toolExecution.exitCode.number", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-execution-step.tool-execution.command-line-arguments" => Some(("testExecutionStep.toolExecution.commandLineArguments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "step-id" => Some(("stepId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "run-duration.nanos" => Some(("runDuration.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "run-duration.seconds" => Some(("runDuration.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time.nanos" => Some(("creationTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creation-time.seconds" => Some(("creationTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tool-execution-step.tool-execution.exit-code.number" => Some(("toolExecutionStep.toolExecution.exitCode.number", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "tool-execution-step.tool-execution.command-line-arguments" => Some(("toolExecutionStep.toolExecution.commandLineArguments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "completion-time.nanos" => Some(("completionTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "completion-time.seconds" => Some(("completionTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.infrastructure-failure" => Some(("outcome.inconclusiveDetail.infrastructureFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.native-crash" => Some(("outcome.inconclusiveDetail.nativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.aborted-by-user" => Some(("outcome.inconclusiveDetail.abortedByUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-app-version" => Some(("outcome.skippedDetail.incompatibleAppVersion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-architecture" => Some(("outcome.skippedDetail.incompatibleArchitecture", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-device" => Some(("outcome.skippedDetail.incompatibleDevice", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.success-detail.other-native-crash" => Some(("outcome.successDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.other-native-crash" => Some(("outcome.failureDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.crashed" => Some(("outcome.failureDetail.crashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.not-installed" => Some(("outcome.failureDetail.notInstalled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.timed-out" => Some(("outcome.failureDetail.timedOut", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.summary" => Some(("outcome.summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-usage-duration.nanos" => Some(("deviceUsageDuration.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device-usage-duration.seconds" => Some(("deviceUsageDuration.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-images" => Some(("hasImages", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aborted-by-user", "command-line-arguments", "completion-time", "crashed", "creation-time", "description", "device-usage-duration", "exit-code", "failure-detail", "has-images", "incompatible-app-version", "incompatible-architecture", "incompatible-device", "inconclusive-detail", "infrastructure-failure", "name", "nanos", "native-crash", "not-installed", "number", "other-native-crash", "outcome", "run-duration", "seconds", "skipped-detail", "state", "step-id", "success-detail", "summary", "test-execution-step", "test-process-duration", "test-timing", "timed-out", "tool-execution", "tool-execution-step"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Step = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().histories_executions_steps_create(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
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

    fn _projects_histories_executions_steps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_executions_steps_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""), opt.value_of("step-id").unwrap_or(""));
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

    fn _projects_histories_executions_steps_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_executions_steps_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_histories_executions_steps_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "test-execution-step.test-timing.test-process-duration.nanos" => Some(("testExecutionStep.testTiming.testProcessDuration.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-execution-step.test-timing.test-process-duration.seconds" => Some(("testExecutionStep.testTiming.testProcessDuration.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-execution-step.tool-execution.exit-code.number" => Some(("testExecutionStep.toolExecution.exitCode.number", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-execution-step.tool-execution.command-line-arguments" => Some(("testExecutionStep.toolExecution.commandLineArguments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "step-id" => Some(("stepId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "run-duration.nanos" => Some(("runDuration.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "run-duration.seconds" => Some(("runDuration.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time.nanos" => Some(("creationTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creation-time.seconds" => Some(("creationTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tool-execution-step.tool-execution.exit-code.number" => Some(("toolExecutionStep.toolExecution.exitCode.number", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "tool-execution-step.tool-execution.command-line-arguments" => Some(("toolExecutionStep.toolExecution.commandLineArguments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "completion-time.nanos" => Some(("completionTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "completion-time.seconds" => Some(("completionTime.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.infrastructure-failure" => Some(("outcome.inconclusiveDetail.infrastructureFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.native-crash" => Some(("outcome.inconclusiveDetail.nativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.inconclusive-detail.aborted-by-user" => Some(("outcome.inconclusiveDetail.abortedByUser", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-app-version" => Some(("outcome.skippedDetail.incompatibleAppVersion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-architecture" => Some(("outcome.skippedDetail.incompatibleArchitecture", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.skipped-detail.incompatible-device" => Some(("outcome.skippedDetail.incompatibleDevice", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.success-detail.other-native-crash" => Some(("outcome.successDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.other-native-crash" => Some(("outcome.failureDetail.otherNativeCrash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.crashed" => Some(("outcome.failureDetail.crashed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.not-installed" => Some(("outcome.failureDetail.notInstalled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.failure-detail.timed-out" => Some(("outcome.failureDetail.timedOut", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "outcome.summary" => Some(("outcome.summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-usage-duration.nanos" => Some(("deviceUsageDuration.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device-usage-duration.seconds" => Some(("deviceUsageDuration.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-images" => Some(("hasImages", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aborted-by-user", "command-line-arguments", "completion-time", "crashed", "creation-time", "description", "device-usage-duration", "exit-code", "failure-detail", "has-images", "incompatible-app-version", "incompatible-architecture", "incompatible-device", "inconclusive-detail", "infrastructure-failure", "name", "nanos", "native-crash", "not-installed", "number", "other-native-crash", "outcome", "run-duration", "seconds", "skipped-detail", "state", "step-id", "success-detail", "summary", "test-execution-step", "test-process-duration", "test-timing", "timed-out", "tool-execution", "tool-execution-step"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Step = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().histories_executions_steps_patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""), opt.value_of("step-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
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

    fn _projects_histories_executions_steps_publish_xunit_xml_files(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PublishXunitXmlFilesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().histories_executions_steps_publish_xunit_xml_files(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""), opt.value_of("step-id").unwrap_or(""));
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

    fn _projects_histories_executions_steps_thumbnails_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_executions_steps_thumbnails_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""), opt.value_of("execution-id").unwrap_or(""), opt.value_of("step-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_histories_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("history-id").unwrap_or(""));
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

    fn _projects_histories_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().histories_list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter-by-name" => {
                    call = call.filter_by_name(value.unwrap_or(""));
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
                                                                           v.extend(["page-token", "filter-by-name", "page-size"].iter().map(|v|*v));
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

    fn _projects_initialize_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().initialize_settings(opt.value_of("project-id").unwrap_or(""));
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
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("get-settings", Some(opt)) => {
                        call_result = self._projects_get_settings(opt, dry_run, &mut err);
                    },
                    ("histories-create", Some(opt)) => {
                        call_result = self._projects_histories_create(opt, dry_run, &mut err);
                    },
                    ("histories-executions-create", Some(opt)) => {
                        call_result = self._projects_histories_executions_create(opt, dry_run, &mut err);
                    },
                    ("histories-executions-get", Some(opt)) => {
                        call_result = self._projects_histories_executions_get(opt, dry_run, &mut err);
                    },
                    ("histories-executions-list", Some(opt)) => {
                        call_result = self._projects_histories_executions_list(opt, dry_run, &mut err);
                    },
                    ("histories-executions-patch", Some(opt)) => {
                        call_result = self._projects_histories_executions_patch(opt, dry_run, &mut err);
                    },
                    ("histories-executions-steps-create", Some(opt)) => {
                        call_result = self._projects_histories_executions_steps_create(opt, dry_run, &mut err);
                    },
                    ("histories-executions-steps-get", Some(opt)) => {
                        call_result = self._projects_histories_executions_steps_get(opt, dry_run, &mut err);
                    },
                    ("histories-executions-steps-list", Some(opt)) => {
                        call_result = self._projects_histories_executions_steps_list(opt, dry_run, &mut err);
                    },
                    ("histories-executions-steps-patch", Some(opt)) => {
                        call_result = self._projects_histories_executions_steps_patch(opt, dry_run, &mut err);
                    },
                    ("histories-executions-steps-publish-xunit-xml-files", Some(opt)) => {
                        call_result = self._projects_histories_executions_steps_publish_xunit_xml_files(opt, dry_run, &mut err);
                    },
                    ("histories-executions-steps-thumbnails-list", Some(opt)) => {
                        call_result = self._projects_histories_executions_steps_thumbnails_list(opt, dry_run, &mut err);
                    },
                    ("histories-get", Some(opt)) => {
                        call_result = self._projects_histories_get(opt, dry_run, &mut err);
                    },
                    ("histories-list", Some(opt)) => {
                        call_result = self._projects_histories_list(opt, dry_run, &mut err);
                    },
                    ("initialize-settings", Some(opt)) => {
                        call_result = self._projects_initialize_settings(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "toolresults1-beta3-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
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
                                          program_name: "toolresults1-beta3",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledInteractive));

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
            hub: api::ToolResults::new(client, auth),
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
    let arg_data = [
        ("projects", "methods: 'get-settings', 'histories-create', 'histories-executions-create', 'histories-executions-get', 'histories-executions-list', 'histories-executions-patch', 'histories-executions-steps-create', 'histories-executions-steps-get', 'histories-executions-steps-list', 'histories-executions-steps-patch', 'histories-executions-steps-publish-xunit-xml-files', 'histories-executions-steps-thumbnails-list', 'histories-get', 'histories-list' and 'initialize-settings'", vec![
            ("get-settings",
                    Some(r##"Gets the Tool Results settings for a project.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to read from project"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_get-settings",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
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
            ("histories-create",
                    Some(r##"Creates a History.
        
        The returned History will have the id set.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing project does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
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
            ("histories-executions-create",
                    Some(r##"Creates an Execution.
        
        The returned Execution will have the id set.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
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
            ("histories-executions-get",
                    Some(r##"Gets an Execution.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Execution does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"An Execution id.
        
        Required."##),
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
            ("histories-executions-list",
                    Some(r##"Lists Histories for a given Project.
        
        The executions are sorted by creation_time in descending order. The execution_id key will be used to order the executions with the same creation_time.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
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
            ("histories-executions-patch",
                    Some(r##"Updates an existing Execution with the supplied partial entity.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal - NOT_FOUND - if the containing History does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id. Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"Required."##),
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
            ("histories-executions-steps-create",
                    Some(r##"Creates a Step.
        
        The returned Step will have the id set.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-steps-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"A Execution id.
        
        Required."##),
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
            ("histories-executions-steps-get",
                    Some(r##"Gets a Step.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Step does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-steps-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"A Execution id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"step-id"##),
                     None,
                     Some(r##"A Step id.
        
        Required."##),
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
            ("histories-executions-steps-list",
                    Some(r##"Lists Steps for a given Execution.
        
        The steps are sorted by creation_time in descending order. The step_id key will be used to order the steps with the same creation_time.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if an argument in the request happens to be invalid; e.g. if an attempt is made to list the children of a nonexistent Step - NOT_FOUND - if the containing Execution does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-steps-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"A Execution id.
        
        Required."##),
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
            ("histories-executions-steps-patch",
                    Some(r##"Updates an existing Step with the supplied partial entity.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal (e.g try to upload a duplicate xml file), if the updated step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-steps-patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"A Execution id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"step-id"##),
                     None,
                     Some(r##"A Step id.
        
        Required."##),
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
            ("histories-executions-steps-publish-xunit-xml-files",
                    Some(r##"Publish xml files to an existing Step.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to write project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal, e.g try to upload a duplicate xml file or a file too large. - NOT_FOUND - if the containing Execution does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-steps-publish-xunit-xml-files",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"A Execution id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"step-id"##),
                     None,
                     Some(r##"A Step id. Note: This step must include a TestExecutionStep.
        
        Required."##),
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
            ("histories-executions-steps-thumbnails-list",
                    Some(r##"Lists thumbnails of images attached to a step.
        
        May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read from the project, or from any of the images - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the step does not exist, or if any of the images do not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-executions-steps-thumbnails-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"execution-id"##),
                     None,
                     Some(r##"An Execution id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"step-id"##),
                     None,
                     Some(r##"A Step id.
        
        Required."##),
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
            ("histories-get",
                    Some(r##"Gets a History.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the History does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"history-id"##),
                     None,
                     Some(r##"A History id.
        
        Required."##),
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
            ("histories-list",
                    Some(r##"Lists Histories for a given Project.
        
        The histories are sorted by modification time in descending order. The history_id key will be used to order the history with the same modification time.
        
        May return any of the following canonical error codes:
        
        - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the History does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_histories-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
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
            ("initialize-settings",
                    Some(r##"Creates resources for settings which have not yet been set.
        
        Currently, this creates a single resource: a Google Cloud Storage bucket, to be used as the default bucket for this project. The bucket is created in the name of the user calling. Except in rare cases, calling this method in parallel from multiple clients will only create a single bucket. In order to avoid unnecessary storage charges, the bucket is configured to automatically delete objects older than 90 days.
        
        The bucket is created with the project-private ACL: All project team members are given permissions to the bucket and objects created within it according to their roles. Project owners have owners rights, and so on. The default ACL on objects created in the bucket is project-private as well. See Google Cloud Storage documentation for more details.
        
        If there is already a default bucket set and the project can access the bucket, this call does nothing. However, if the project doesn't have the permission to access the bucket or the bucket is deteleted, a new bucket will be created.
        
        May return any canonical error codes, including the following:
        
        - PERMISSION_DENIED - if the user is not authorized to write to project - Any error code raised by Google Cloud Storage"##),
                    "Details at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli/projects_initialize-settings",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"A Project id.
        
        Required."##),
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
    
    let mut app = App::new("toolresults1-beta3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.4+20160408")
           .about("Read and publish results from Cloud Test Lab.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_toolresults1_beta3_cli")
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