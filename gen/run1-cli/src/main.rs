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
extern crate google_run1 as api;

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
    hub: api::CloudRun<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _api_v1_namespaces_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.api().v1_namespaces_get(opt.value_of("name").unwrap_or(""));
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

    fn _api_v1_namespaces_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.phase" => Some(("status.phase", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.finalizers" => Some(("spec.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "cluster-name", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "labels", "metadata", "name", "namespace", "phase", "resource-version", "self-link", "spec", "status", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Namespace = json::value::from_value(object).unwrap();
        let mut call = self.hub.api().v1_namespaces_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    fn _api_v1_namespaces_secrets_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "string-data" => Some(("stringData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "cluster-name", "creation-timestamp", "data", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "labels", "metadata", "name", "namespace", "resource-version", "self-link", "string-data", "type", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Secret = json::value::from_value(object).unwrap();
        let mut call = self.hub.api().v1_namespaces_secrets_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _api_v1_namespaces_secrets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.api().v1_namespaces_secrets_get(opt.value_of("name").unwrap_or(""));
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

    fn _api_v1_namespaces_secrets_replace_secret(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "string-data" => Some(("stringData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "cluster-name", "creation-timestamp", "data", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "labels", "metadata", "name", "namespace", "resource-version", "self-link", "string-data", "type", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Secret = json::value::from_value(object).unwrap();
        let mut call = self.hub.api().v1_namespaces_secrets_replace_secret(request, opt.value_of("name").unwrap_or(""));
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

    fn _namespaces_authorizeddomains_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().authorizeddomains_list(opt.value_of("parent").unwrap_or(""));
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

    fn _namespaces_configurations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().configurations_get(opt.value_of("name").unwrap_or(""));
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

    fn _namespaces_configurations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().configurations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _namespaces_domainmappings_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.mapped-route-name" => Some(("status.mappedRouteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.route-name" => Some(("spec.routeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.force-override" => Some(("spec.forceOverride", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.certificate-mode" => Some(("spec.certificateMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "api-version", "certificate-mode", "cluster-name", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "force-override", "generate-name", "generation", "kind", "labels", "mapped-route-name", "metadata", "name", "namespace", "observed-generation", "resource-version", "route-name", "self-link", "spec", "status", "uid", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DomainMapping = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().domainmappings_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _namespaces_domainmappings_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().domainmappings_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "propagation-policy" => {
                    call = call.propagation_policy(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.kind(value.unwrap_or(""));
                },
                "api-version" => {
                    call = call.api_version(value.unwrap_or(""));
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
                                                                           v.extend(["propagation-policy", "kind", "api-version"].iter().map(|v|*v));
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

    fn _namespaces_domainmappings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().domainmappings_get(opt.value_of("name").unwrap_or(""));
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

    fn _namespaces_domainmappings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().domainmappings_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _namespaces_revisions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().revisions_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "propagation-policy" => {
                    call = call.propagation_policy(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.kind(value.unwrap_or(""));
                },
                "api-version" => {
                    call = call.api_version(value.unwrap_or(""));
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
                                                                           v.extend(["propagation-policy", "kind", "api-version"].iter().map(|v|*v));
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

    fn _namespaces_revisions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().revisions_get(opt.value_of("name").unwrap_or(""));
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

    fn _namespaces_revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().revisions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _namespaces_routes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().routes_get(opt.value_of("name").unwrap_or(""));
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

    fn _namespaces_routes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().routes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _namespaces_services_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "observed-generation", "resource-version", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Service = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().services_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _namespaces_services_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().services_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "propagation-policy" => {
                    call = call.propagation_policy(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.kind(value.unwrap_or(""));
                },
                "api-version" => {
                    call = call.api_version(value.unwrap_or(""));
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
                                                                           v.extend(["propagation-policy", "kind", "api-version"].iter().map(|v|*v));
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

    fn _namespaces_services_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().services_get(opt.value_of("name").unwrap_or(""));
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

    fn _namespaces_services_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().services_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _namespaces_services_replace_service(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "observed-generation", "resource-version", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Service = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().services_replace_service(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_authorizeddomains_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_authorizeddomains_list(opt.value_of("parent").unwrap_or(""));
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

    fn _projects_locations_configurations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_configurations_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_configurations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_configurations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _projects_locations_domainmappings_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.mapped-route-name" => Some(("status.mappedRouteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.route-name" => Some(("spec.routeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.force-override" => Some(("spec.forceOverride", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.certificate-mode" => Some(("spec.certificateMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "api-version", "certificate-mode", "cluster-name", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "force-override", "generate-name", "generation", "kind", "labels", "mapped-route-name", "metadata", "name", "namespace", "observed-generation", "resource-version", "route-name", "self-link", "spec", "status", "uid", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DomainMapping = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_domainmappings_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_locations_domainmappings_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_domainmappings_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "propagation-policy" => {
                    call = call.propagation_policy(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.kind(value.unwrap_or(""));
                },
                "api-version" => {
                    call = call.api_version(value.unwrap_or(""));
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
                                                                           v.extend(["propagation-policy", "kind", "api-version"].iter().map(|v|*v));
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

    fn _projects_locations_domainmappings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_domainmappings_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_domainmappings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_domainmappings_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _projects_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_locations_namespaces_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_namespaces_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_namespaces_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.phase" => Some(("status.phase", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.finalizers" => Some(("spec.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "cluster-name", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "labels", "metadata", "name", "namespace", "phase", "resource-version", "self-link", "spec", "status", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Namespace = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_namespaces_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    fn _projects_locations_revisions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_revisions_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "propagation-policy" => {
                    call = call.propagation_policy(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.kind(value.unwrap_or(""));
                },
                "api-version" => {
                    call = call.api_version(value.unwrap_or(""));
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
                                                                           v.extend(["propagation-policy", "kind", "api-version"].iter().map(|v|*v));
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

    fn _projects_locations_revisions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_revisions_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_revisions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _projects_locations_routes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_routes_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_routes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_routes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _projects_locations_secrets_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "string-data" => Some(("stringData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "cluster-name", "creation-timestamp", "data", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "labels", "metadata", "name", "namespace", "resource-version", "self-link", "string-data", "type", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Secret = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_secrets_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_locations_secrets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_secrets_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_secrets_replace_secret(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "string-data" => Some(("stringData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "cluster-name", "creation-timestamp", "data", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "labels", "metadata", "name", "namespace", "resource-version", "self-link", "string-data", "type", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Secret = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_secrets_replace_secret(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_services_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "observed-generation", "resource-version", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Service = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_services_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_locations_services_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_services_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "propagation-policy" => {
                    call = call.propagation_policy(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.kind(value.unwrap_or(""));
                },
                "api-version" => {
                    call = call.api_version(value.unwrap_or(""));
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
                                                                           v.extend(["propagation-policy", "kind", "api-version"].iter().map(|v|*v));
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

    fn _projects_locations_services_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_services_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_services_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_services_get_iam_policy(opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(arg_from_str(value.unwrap_or("-0"), err, "options-requested-policy-version", "integer"));
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
                                                                           v.extend(["options-requested-policy-version"].iter().map(|v|*v));
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

    fn _projects_locations_services_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_services_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(arg_from_str(value.unwrap_or("false"), err, "watch", "boolean"));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(arg_from_str(value.unwrap_or("false"), err, "include-uninitialized", "boolean"));
                },
                "field-selector" => {
                    call = call.field_selector(value.unwrap_or(""));
                },
                "continue" => {
                    call = call.continue_(value.unwrap_or(""));
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
                                                                           v.extend(["label-selector", "watch", "include-uninitialized", "continue", "limit", "resource-version", "field-selector"].iter().map(|v|*v));
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

    fn _projects_locations_services_replace_service(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "observed-generation", "resource-version", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Service = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_services_replace_service(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_services_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy.version" => Some(("policy.version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "update-mask", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_services_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    fn _projects_locations_services_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_services_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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
            ("api", Some(opt)) => {
                match opt.subcommand() {
                    ("v1-namespaces-get", Some(opt)) => {
                        call_result = self._api_v1_namespaces_get(opt, dry_run, &mut err);
                    },
                    ("v1-namespaces-patch", Some(opt)) => {
                        call_result = self._api_v1_namespaces_patch(opt, dry_run, &mut err);
                    },
                    ("v1-namespaces-secrets-create", Some(opt)) => {
                        call_result = self._api_v1_namespaces_secrets_create(opt, dry_run, &mut err);
                    },
                    ("v1-namespaces-secrets-get", Some(opt)) => {
                        call_result = self._api_v1_namespaces_secrets_get(opt, dry_run, &mut err);
                    },
                    ("v1-namespaces-secrets-replace-secret", Some(opt)) => {
                        call_result = self._api_v1_namespaces_secrets_replace_secret(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("api".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("namespaces", Some(opt)) => {
                match opt.subcommand() {
                    ("authorizeddomains-list", Some(opt)) => {
                        call_result = self._namespaces_authorizeddomains_list(opt, dry_run, &mut err);
                    },
                    ("configurations-get", Some(opt)) => {
                        call_result = self._namespaces_configurations_get(opt, dry_run, &mut err);
                    },
                    ("configurations-list", Some(opt)) => {
                        call_result = self._namespaces_configurations_list(opt, dry_run, &mut err);
                    },
                    ("domainmappings-create", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_create(opt, dry_run, &mut err);
                    },
                    ("domainmappings-delete", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_delete(opt, dry_run, &mut err);
                    },
                    ("domainmappings-get", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_get(opt, dry_run, &mut err);
                    },
                    ("domainmappings-list", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_list(opt, dry_run, &mut err);
                    },
                    ("revisions-delete", Some(opt)) => {
                        call_result = self._namespaces_revisions_delete(opt, dry_run, &mut err);
                    },
                    ("revisions-get", Some(opt)) => {
                        call_result = self._namespaces_revisions_get(opt, dry_run, &mut err);
                    },
                    ("revisions-list", Some(opt)) => {
                        call_result = self._namespaces_revisions_list(opt, dry_run, &mut err);
                    },
                    ("routes-get", Some(opt)) => {
                        call_result = self._namespaces_routes_get(opt, dry_run, &mut err);
                    },
                    ("routes-list", Some(opt)) => {
                        call_result = self._namespaces_routes_list(opt, dry_run, &mut err);
                    },
                    ("services-create", Some(opt)) => {
                        call_result = self._namespaces_services_create(opt, dry_run, &mut err);
                    },
                    ("services-delete", Some(opt)) => {
                        call_result = self._namespaces_services_delete(opt, dry_run, &mut err);
                    },
                    ("services-get", Some(opt)) => {
                        call_result = self._namespaces_services_get(opt, dry_run, &mut err);
                    },
                    ("services-list", Some(opt)) => {
                        call_result = self._namespaces_services_list(opt, dry_run, &mut err);
                    },
                    ("services-replace-service", Some(opt)) => {
                        call_result = self._namespaces_services_replace_service(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("namespaces".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-authorizeddomains-list", Some(opt)) => {
                        call_result = self._projects_locations_authorizeddomains_list(opt, dry_run, &mut err);
                    },
                    ("locations-configurations-get", Some(opt)) => {
                        call_result = self._projects_locations_configurations_get(opt, dry_run, &mut err);
                    },
                    ("locations-configurations-list", Some(opt)) => {
                        call_result = self._projects_locations_configurations_list(opt, dry_run, &mut err);
                    },
                    ("locations-domainmappings-create", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_create(opt, dry_run, &mut err);
                    },
                    ("locations-domainmappings-delete", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_delete(opt, dry_run, &mut err);
                    },
                    ("locations-domainmappings-get", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_get(opt, dry_run, &mut err);
                    },
                    ("locations-domainmappings-list", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_list(opt, dry_run, &mut err);
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._projects_locations_list(opt, dry_run, &mut err);
                    },
                    ("locations-namespaces-get", Some(opt)) => {
                        call_result = self._projects_locations_namespaces_get(opt, dry_run, &mut err);
                    },
                    ("locations-namespaces-patch", Some(opt)) => {
                        call_result = self._projects_locations_namespaces_patch(opt, dry_run, &mut err);
                    },
                    ("locations-revisions-delete", Some(opt)) => {
                        call_result = self._projects_locations_revisions_delete(opt, dry_run, &mut err);
                    },
                    ("locations-revisions-get", Some(opt)) => {
                        call_result = self._projects_locations_revisions_get(opt, dry_run, &mut err);
                    },
                    ("locations-revisions-list", Some(opt)) => {
                        call_result = self._projects_locations_revisions_list(opt, dry_run, &mut err);
                    },
                    ("locations-routes-get", Some(opt)) => {
                        call_result = self._projects_locations_routes_get(opt, dry_run, &mut err);
                    },
                    ("locations-routes-list", Some(opt)) => {
                        call_result = self._projects_locations_routes_list(opt, dry_run, &mut err);
                    },
                    ("locations-secrets-create", Some(opt)) => {
                        call_result = self._projects_locations_secrets_create(opt, dry_run, &mut err);
                    },
                    ("locations-secrets-get", Some(opt)) => {
                        call_result = self._projects_locations_secrets_get(opt, dry_run, &mut err);
                    },
                    ("locations-secrets-replace-secret", Some(opt)) => {
                        call_result = self._projects_locations_secrets_replace_secret(opt, dry_run, &mut err);
                    },
                    ("locations-services-create", Some(opt)) => {
                        call_result = self._projects_locations_services_create(opt, dry_run, &mut err);
                    },
                    ("locations-services-delete", Some(opt)) => {
                        call_result = self._projects_locations_services_delete(opt, dry_run, &mut err);
                    },
                    ("locations-services-get", Some(opt)) => {
                        call_result = self._projects_locations_services_get(opt, dry_run, &mut err);
                    },
                    ("locations-services-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_services_get_iam_policy(opt, dry_run, &mut err);
                    },
                    ("locations-services-list", Some(opt)) => {
                        call_result = self._projects_locations_services_list(opt, dry_run, &mut err);
                    },
                    ("locations-services-replace-service", Some(opt)) => {
                        call_result = self._projects_locations_services_replace_service(opt, dry_run, &mut err);
                    },
                    ("locations-services-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_services_set_iam_policy(opt, dry_run, &mut err);
                    },
                    ("locations-services-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_services_test_iam_permissions(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "run1-secret.json",
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
                                          program_name: "run1",
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
            hub: api::CloudRun::new(client, auth),
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
        ("api", "methods: 'v1-namespaces-get', 'v1-namespaces-patch', 'v1-namespaces-secrets-create', 'v1-namespaces-secrets-get' and 'v1-namespaces-secrets-replace-secret'", vec![
            ("v1-namespaces-get",
                    Some(r##"Rpc to get information about a namespace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/api_v1-namespaces-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the namespace being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("v1-namespaces-patch",
                    Some(r##"Rpc to update a namespace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/api_v1-namespaces-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the namespace being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("v1-namespaces-secrets-create",
                    Some(r##"Creates a new secret."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/api_v1-namespaces-secrets-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project ID or project number in which this secret should
        be created."##),
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
            ("v1-namespaces-secrets-get",
                    Some(r##"Rpc to get information about a secret."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/api_v1-namespaces-secrets-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the secret being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("v1-namespaces-secrets-replace-secret",
                    Some(r##"Rpc to replace a secret.
        
        Only the spec and metadata labels and annotations are modifiable. After
        the Update request, Cloud Run will work to make the 'status'
        match the requested 'spec'.
        
        May provide metadata.resourceVersion to enforce update from last read for
        optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/api_v1-namespaces-secrets-replace-secret",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the secret being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
        
        ("namespaces", "methods: 'authorizeddomains-list', 'configurations-get', 'configurations-list', 'domainmappings-create', 'domainmappings-delete', 'domainmappings-get', 'domainmappings-list', 'revisions-delete', 'revisions-get', 'revisions-list', 'routes-get', 'routes-list', 'services-create', 'services-delete', 'services-get', 'services-list' and 'services-replace-service'", vec![
            ("authorizeddomains-list",
                    Some(r##"List authorized domains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_authorizeddomains-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent Application resource. Example: `apps/myapp`."##),
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
            ("configurations-get",
                    Some(r##"Get information about a configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_configurations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the configuration to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("configurations-list",
                    Some(r##"List configurations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_configurations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the configurations should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("domainmappings-create",
                    Some(r##"Create a new domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_domainmappings-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace in which the domain mapping should be created.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("domainmappings-delete",
                    Some(r##"Delete a domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_domainmappings-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the domain mapping to delete.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("domainmappings-get",
                    Some(r##"Get information about a domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_domainmappings-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the domain mapping to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("domainmappings-list",
                    Some(r##"List domain mappings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_domainmappings-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the domain mappings should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("revisions-delete",
                    Some(r##"Delete a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_revisions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the revision to delete.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("revisions-get",
                    Some(r##"Get information about a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_revisions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the revision to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("revisions-list",
                    Some(r##"List revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_revisions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the revisions should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("routes-get",
                    Some(r##"Get information about a route."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_routes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the route to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("routes-list",
                    Some(r##"List routes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_routes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the routes should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("services-create",
                    Some(r##"Create a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace in which the service should be created.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("services-delete",
                    Some(r##"Delete a service.
        This will cause the Service to stop serving traffic and will delete the
        child entities like Routes, Configurations and Revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the service to delete.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("services-get",
                    Some(r##"Get information about a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the service to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("services-list",
                    Some(r##"List services."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the services should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("services-replace-service",
                    Some(r##"Replace a service.
        
        Only the spec and metadata labels and annotations are modifiable. After
        the Update request, Cloud Run will work to make the 'status'
        match the requested 'spec'.
        
        May provide metadata.resourceVersion to enforce update from last read for
        optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-replace-service",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the service being replaced.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
        
        ("projects", "methods: 'locations-authorizeddomains-list', 'locations-configurations-get', 'locations-configurations-list', 'locations-domainmappings-create', 'locations-domainmappings-delete', 'locations-domainmappings-get', 'locations-domainmappings-list', 'locations-list', 'locations-namespaces-get', 'locations-namespaces-patch', 'locations-revisions-delete', 'locations-revisions-get', 'locations-revisions-list', 'locations-routes-get', 'locations-routes-list', 'locations-secrets-create', 'locations-secrets-get', 'locations-secrets-replace-secret', 'locations-services-create', 'locations-services-delete', 'locations-services-get', 'locations-services-get-iam-policy', 'locations-services-list', 'locations-services-replace-service', 'locations-services-set-iam-policy' and 'locations-services-test-iam-permissions'", vec![
            ("locations-authorizeddomains-list",
                    Some(r##"List authorized domains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-authorizeddomains-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent Application resource. Example: `apps/myapp`."##),
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
            ("locations-configurations-get",
                    Some(r##"Get information about a configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-configurations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the configuration to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-configurations-list",
                    Some(r##"List configurations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-configurations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the configurations should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-domainmappings-create",
                    Some(r##"Create a new domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-domainmappings-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace in which the domain mapping should be created.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-domainmappings-delete",
                    Some(r##"Delete a domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-domainmappings-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the domain mapping to delete.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-domainmappings-get",
                    Some(r##"Get information about a domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-domainmappings-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the domain mapping to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-domainmappings-list",
                    Some(r##"List domain mappings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-domainmappings-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the domain mappings should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-list",
                    Some(r##"Lists information about the supported locations for this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource that owns the locations collection, if applicable."##),
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
            ("locations-namespaces-get",
                    Some(r##"Rpc to get information about a namespace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-namespaces-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the namespace being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("locations-namespaces-patch",
                    Some(r##"Rpc to update a namespace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-namespaces-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the namespace being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("locations-revisions-delete",
                    Some(r##"Delete a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-revisions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the revision to delete.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-revisions-get",
                    Some(r##"Get information about a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-revisions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the revision to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-revisions-list",
                    Some(r##"List revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-revisions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the revisions should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-routes-get",
                    Some(r##"Get information about a route."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-routes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the route to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-routes-list",
                    Some(r##"List routes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-routes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the routes should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-secrets-create",
                    Some(r##"Creates a new secret."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-secrets-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project ID or project number in which this secret should
        be created."##),
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
            ("locations-secrets-get",
                    Some(r##"Rpc to get information about a secret."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-secrets-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the secret being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("locations-secrets-replace-secret",
                    Some(r##"Rpc to replace a secret.
        
        Only the spec and metadata labels and annotations are modifiable. After
        the Update request, Cloud Run will work to make the 'status'
        match the requested 'spec'.
        
        May provide metadata.resourceVersion to enforce update from last read for
        optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-secrets-replace-secret",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the secret being retrieved. If needed, replace
        {namespace_id} with the project ID."##),
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
            ("locations-services-create",
                    Some(r##"Create a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace in which the service should be created.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-services-delete",
                    Some(r##"Delete a service.
        This will cause the Service to stop serving traffic and will delete the
        child entities like Routes, Configurations and Revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the service to delete.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-services-get",
                    Some(r##"Get information about a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the service to retrieve.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-services-get-iam-policy",
                    Some(r##"Get the IAM Access Control policy currently in effect for the given
        Cloud Run service. This result does not include any inherited policies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested.
        See the operation documentation for the appropriate value for this field."##),
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
            ("locations-services-list",
                    Some(r##"List services."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the services should be listed.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-services-replace-service",
                    Some(r##"Replace a service.
        
        Only the spec and metadata labels and annotations are modifiable. After
        the Update request, Cloud Run will work to make the 'status'
        match the requested 'spec'.
        
        May provide metadata.resourceVersion to enforce update from last read for
        optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-replace-service",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the service being replaced.
        For Cloud Run (fully managed), replace {namespace_id} with the project ID
        or number."##),
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
            ("locations-services-set-iam-policy",
                    Some(r##"Sets the IAM Access control policy for the specified Service. Overwrites
        any existing policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified.
        See the operation documentation for the appropriate value for this field."##),
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
            ("locations-services-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified Project.
        
        There are no permissions required for making this API call."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested.
        See the operation documentation for the appropriate value for this field."##),
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
    
    let mut app = App::new("run1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.14+20200622")
           .about("Deploy and manage user provided container images that scale automatically based on HTTP traffic.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_run1_cli")
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