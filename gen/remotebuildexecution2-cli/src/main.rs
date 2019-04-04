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
extern crate google_remotebuildexecution2 as api;

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
    hub: api::RemoteBuildExecution<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _action_results_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.action_results().get(opt.value_of("instance-name").unwrap_or(""), opt.value_of("hash").unwrap_or(""), opt.value_of("size-bytes").unwrap_or(""));
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

    fn _action_results_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "execution-metadata.output-upload-start-timestamp" => Some(("executionMetadata.outputUploadStartTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.worker-completed-timestamp" => Some(("executionMetadata.workerCompletedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.queued-timestamp" => Some(("executionMetadata.queuedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.worker" => Some(("executionMetadata.worker", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.execution-start-timestamp" => Some(("executionMetadata.executionStartTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.input-fetch-start-timestamp" => Some(("executionMetadata.inputFetchStartTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.worker-start-timestamp" => Some(("executionMetadata.workerStartTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.output-upload-completed-timestamp" => Some(("executionMetadata.outputUploadCompletedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.execution-completed-timestamp" => Some(("executionMetadata.executionCompletedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-metadata.input-fetch-completed-timestamp" => Some(("executionMetadata.inputFetchCompletedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stderr-digest.size-bytes" => Some(("stderrDigest.sizeBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stderr-digest.hash" => Some(("stderrDigest.hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stdout-raw" => Some(("stdoutRaw", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stderr-raw" => Some(("stderrRaw", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stdout-digest.size-bytes" => Some(("stdoutDigest.sizeBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stdout-digest.hash" => Some(("stdoutDigest.hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exit-code" => Some(("exitCode", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["execution-completed-timestamp", "execution-metadata", "execution-start-timestamp", "exit-code", "hash", "input-fetch-completed-timestamp", "input-fetch-start-timestamp", "output-upload-completed-timestamp", "output-upload-start-timestamp", "queued-timestamp", "size-bytes", "stderr-digest", "stderr-raw", "stdout-digest", "stdout-raw", "worker", "worker-completed-timestamp", "worker-start-timestamp"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BuildBazelRemoteExecutionV2ActionResult = json::value::from_value(object).unwrap();
        let mut call = self.hub.action_results().update(request, opt.value_of("instance-name").unwrap_or(""), opt.value_of("hash").unwrap_or(""), opt.value_of("size-bytes").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "results-cache-policy-priority" => {
                    call = call.results_cache_policy_priority(arg_from_str(value.unwrap_or("-0"), err, "results-cache-policy-priority", "integer"));
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
                                                                           v.extend(["results-cache-policy-priority"].iter().map(|v|*v));
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

    fn _actions_execute(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "results-cache-policy.priority" => Some(("resultsCachePolicy.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "skip-cache-lookup" => Some(("skipCacheLookup", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "action-digest.size-bytes" => Some(("actionDigest.sizeBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "action-digest.hash" => Some(("actionDigest.hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "execution-policy.priority" => Some(("executionPolicy.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action-digest", "execution-policy", "hash", "priority", "results-cache-policy", "size-bytes", "skip-cache-lookup"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BuildBazelRemoteExecutionV2ExecuteRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.actions().execute(request, opt.value_of("instance-name").unwrap_or(""));
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

    fn _blobs_batch_read(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BuildBazelRemoteExecutionV2BatchReadBlobsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.blobs().batch_read(request, opt.value_of("instance-name").unwrap_or(""));
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

    fn _blobs_batch_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.blobs().batch_update(request, opt.value_of("instance-name").unwrap_or(""));
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

    fn _blobs_find_missing(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BuildBazelRemoteExecutionV2FindMissingBlobsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.blobs().find_missing(request, opt.value_of("instance-name").unwrap_or(""));
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

    fn _blobs_get_tree(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.blobs().get_tree(opt.value_of("instance-name").unwrap_or(""), opt.value_of("hash").unwrap_or(""), opt.value_of("size-bytes").unwrap_or(""));
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _methods_get_capabilities(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.methods().get_capabilities(opt.value_of("instance-name").unwrap_or(""));
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

    fn _operations_wait_execution(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BuildBazelRemoteExecutionV2WaitExecutionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.operations().wait_execution(request, opt.value_of("name").unwrap_or(""));
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

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("action-results", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._action_results_get(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._action_results_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("action-results".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("actions", Some(opt)) => {
                match opt.subcommand() {
                    ("execute", Some(opt)) => {
                        call_result = self._actions_execute(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("actions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("blobs", Some(opt)) => {
                match opt.subcommand() {
                    ("batch-read", Some(opt)) => {
                        call_result = self._blobs_batch_read(opt, dry_run, &mut err);
                    },
                    ("batch-update", Some(opt)) => {
                        call_result = self._blobs_batch_update(opt, dry_run, &mut err);
                    },
                    ("find-missing", Some(opt)) => {
                        call_result = self._blobs_find_missing(opt, dry_run, &mut err);
                    },
                    ("get-tree", Some(opt)) => {
                        call_result = self._blobs_get_tree(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("blobs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("methods", Some(opt)) => {
                match opt.subcommand() {
                    ("get-capabilities", Some(opt)) => {
                        call_result = self._methods_get_capabilities(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("methods".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("operations", Some(opt)) => {
                match opt.subcommand() {
                    ("wait-execution", Some(opt)) => {
                        call_result = self._operations_wait_execution(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "remotebuildexecution2-secret.json",
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
                                          program_name: "remotebuildexecution2",
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
            hub: api::RemoteBuildExecution::new(client, auth),
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
        ("action-results", "methods: 'get' and 'update'", vec![
            ("get",
                    Some(r##"Retrieve a cached execution result.
        
        Errors:
        
        * `NOT_FOUND`: The requested `ActionResult` is not in the cache."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/action-results_get",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"hash"##),
                     None,
                     Some(r##"The hash. In the case of SHA-256, it will always be a lowercase hex string
        exactly 64 characters long."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"size-bytes"##),
                     None,
                     Some(r##"The size of the blob, in bytes."##),
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
                    Some(r##"Upload a new execution result.
        
        In order to allow the server to perform access control based on the type of
        action, and to assist with client debugging, the client MUST first upload
        the Action that produced the
        result, along with its
        Command, into the
        `ContentAddressableStorage`.
        
        Errors:
        
        * `INVALID_ARGUMENT`: One or more arguments are invalid.
        * `FAILED_PRECONDITION`: One or more errors occurred in updating the
          action result, such as a missing command or action.
        * `RESOURCE_EXHAUSTED`: There is insufficient storage space to add the
          entry to the cache."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/action-results_update",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"hash"##),
                     None,
                     Some(r##"The hash. In the case of SHA-256, it will always be a lowercase hex string
        exactly 64 characters long."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"size-bytes"##),
                     None,
                     Some(r##"The size of the blob, in bytes."##),
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
        
        ("actions", "methods: 'execute'", vec![
            ("execute",
                    Some(r##"Execute an action remotely.
        
        In order to execute an action, the client must first upload all of the
        inputs, the
        Command to run, and the
        Action into the
        ContentAddressableStorage.
        It then calls `Execute` with an `action_digest` referring to them. The
        server will run the action and eventually return the result.
        
        The input `Action`'s fields MUST meet the various canonicalization
        requirements specified in the documentation for their types so that it has
        the same digest as other logically equivalent `Action`s. The server MAY
        enforce the requirements and return errors if a non-canonical input is
        received. It MAY also proceed without verifying some or all of the
        requirements, such as for performance reasons. If the server does not
        verify the requirement, then it will treat the `Action` as distinct from
        another logically equivalent action if they hash differently.
        
        Returns a stream of
        google.longrunning.Operation messages
        describing the resulting execution, with eventual `response`
        ExecuteResponse. The
        `metadata` on the operation is of type
        ExecuteOperationMetadata.
        
        If the client remains connected after the first response is returned after
        the server, then updates are streamed as if the client had called
        WaitExecution
        until the execution completes or the request reaches an error. The
        operation can also be queried using Operations
        API.
        
        The server NEED NOT implement other methods or functionality of the
        Operations API.
        
        Errors discovered during creation of the `Operation` will be reported
        as gRPC Status errors, while errors that occurred while running the
        action will be reported in the `status` field of the `ExecuteResponse`. The
        server MUST NOT set the `error` field of the `Operation` proto.
        The possible errors include:
        
        * `INVALID_ARGUMENT`: One or more arguments are invalid.
        * `FAILED_PRECONDITION`: One or more errors occurred in setting up the
          action requested, such as a missing input or command or no worker being
          available. The client may be able to fix the errors and retry.
        * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run
          the action.
        * `UNAVAILABLE`: Due to a transient condition, such as all workers being
          occupied (and the server does not support a queue), the action could not
          be started. The client should retry.
        * `INTERNAL`: An internal error occurred in the execution engine or the
          worker.
        * `DEADLINE_EXCEEDED`: The execution timed out.
        * `CANCELLED`: The operation was cancelled by the client. This status is
          only possible if the server implements the Operations API CancelOperation
          method, and it was called for the current execution.
        
        In the case of a missing input or command, the server SHOULD additionally
        send a PreconditionFailure error detail
        where, for each requested blob not present in the CAS, there is a
        `Violation` with a `type` of `MISSING` and a `subject` of
        `"blobs/{hash}/{size}"` indicating the digest of the missing blob."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/actions_execute",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
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
        
        ("blobs", "methods: 'batch-read', 'batch-update', 'find-missing' and 'get-tree'", vec![
            ("batch-read",
                    Some(r##"Download many blobs at once.
        
        The server may enforce a limit of the combined total size of blobs
        to be downloaded using this API. This limit may be obtained using the
        Capabilities API.
        Requests exceeding the limit should either be split into smaller
        chunks or downloaded using the
        ByteStream API, as appropriate.
        
        This request is equivalent to calling a Bytestream `Read` request
        on each individual blob, in parallel. The requests may succeed or fail
        independently.
        
        Errors:
        
        * `INVALID_ARGUMENT`: The client attempted to read more than the
          server supported limit.
        
        Every error on individual read will be returned in the corresponding digest
        status."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/blobs_batch-read",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
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
            ("batch-update",
                    Some(r##"Upload many blobs at once.
        
        The server may enforce a limit of the combined total size of blobs
        to be uploaded using this API. This limit may be obtained using the
        Capabilities API.
        Requests exceeding the limit should either be split into smaller
        chunks or uploaded using the
        ByteStream API, as appropriate.
        
        This request is equivalent to calling a Bytestream `Write` request
        on each individual blob, in parallel. The requests may succeed or fail
        independently.
        
        Errors:
        
        * `INVALID_ARGUMENT`: The client attempted to upload more than the
          server supported limit.
        
        Individual requests may return the following errors, additionally:
        
        * `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob.
        * `INVALID_ARGUMENT`: The
        Digest does not match the
        provided data."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/blobs_batch-update",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
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
            ("find-missing",
                    Some(r##"Determine if blobs are present in the CAS.
        
        Clients can use this API before uploading blobs to determine which ones are
        already present in the CAS and do not need to be uploaded again.
        
        There are no method-specific errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/blobs_find-missing",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
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
            ("get-tree",
                    Some(r##"Fetch the entire directory tree rooted at a node.
        
        This request must be targeted at a
        Directory stored in the
        ContentAddressableStorage
        (CAS). The server will enumerate the `Directory` tree recursively and
        return every node descended from the root.
        
        The GetTreeRequest.page_token parameter can be used to skip ahead in
        the stream (e.g. when retrying a partially completed and aborted request),
        by setting it to a value taken from GetTreeResponse.next_page_token of the
        last successfully processed GetTreeResponse).
        
        The exact traversal order is unspecified and, unless retrieving subsequent
        pages from an earlier request, is not guaranteed to be stable across
        multiple invocations of `GetTree`.
        
        If part of the tree is missing from the CAS, the server will return the
        portion present and omit the rest.
        
        * `NOT_FOUND`: The requested tree root is not present in the CAS."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/blobs_get-tree",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"hash"##),
                     None,
                     Some(r##"The hash. In the case of SHA-256, it will always be a lowercase hex string
        exactly 64 characters long."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"size-bytes"##),
                     None,
                     Some(r##"The size of the blob, in bytes."##),
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
        
        ("methods", "methods: 'get-capabilities'", vec![
            ("get-capabilities",
                    Some(r##"GetCapabilities returns the server capabilities configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/methods_get-capabilities",
                  vec![
                    (Some(r##"instance-name"##),
                     None,
                     Some(r##"The instance of the execution system to operate against. A server may
        support multiple instances of the execution system (with their own workers,
        storage, caches, etc.). The server MAY require use of this field to select
        between them in an implementation-defined fashion, otherwise it can be
        omitted."##),
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
        
        ("operations", "methods: 'wait-execution'", vec![
            ("wait-execution",
                    Some(r##"Wait for an execution operation to complete. When the client initially
        makes the request, the server immediately responds with the current status
        of the execution. The server will leave the request stream open until the
        operation completes, and then respond with the completed operation. The
        server MAY choose to stream additional updates as execution progresses,
        such as to provide an update as to the state of the execution."##),
                    "Details at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli/operations_wait-execution",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the Operation
        returned by Execute."##),
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
    
    let mut app = App::new("remotebuildexecution2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.8+20190326")
           .about("Supplies a Remote Execution API service for tools such as bazel.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_remotebuildexecution2_cli")
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