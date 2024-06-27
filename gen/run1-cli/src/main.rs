// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_run1::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::CloudRun<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _namespaces_authorizeddomains_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().authorizeddomains_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_configurations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_configurations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().configurations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_domainmappings_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.certificate-mode" => Some(("spec.certificateMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.force-override" => Some(("spec.forceOverride", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.route-name" => Some(("spec.routeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.mapped-route-name" => Some(("status.mappedRouteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_domainmappings_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["api-version", "dry-run", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_domainmappings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_domainmappings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().domainmappings_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_executions_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::CancelExecutionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().executions_cancel(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_executions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().executions_delete(opt.value_of("name").unwrap_or(""));
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
                                                                           v.extend(["api-version", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_executions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().executions_get(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_executions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().executions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.run-execution-token" => Some(("spec.runExecutionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.start-execution-token" => Some(("spec.startExecutionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.parallelism" => Some(("spec.template.spec.parallelism", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.task-count" => Some(("spec.template.spec.taskCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.template.spec.max-retries" => Some(("spec.template.spec.template.spec.maxRetries", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.template.spec.service-account-name" => Some(("spec.template.spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.template.spec.timeout-seconds" => Some(("spec.template.spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.execution-count" => Some(("status.executionCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.latest-created-execution.completion-timestamp" => Some(("status.latestCreatedExecution.completionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-execution.creation-timestamp" => Some(("status.latestCreatedExecution.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-execution.name" => Some(("status.latestCreatedExecution.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "api-version", "cluster-name", "completion-timestamp", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "execution-count", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-execution", "max-retries", "metadata", "name", "namespace", "observed-generation", "parallelism", "resource-version", "run-execution-token", "self-link", "service-account-name", "spec", "start-execution-token", "status", "task-count", "template", "timeout-seconds", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Job = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().jobs_create(request, opt.value_of("parent").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().jobs_delete(opt.value_of("name").unwrap_or(""));
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
                                                                           v.extend(["api-version", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().jobs_get(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().jobs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_jobs_replace_job(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.run-execution-token" => Some(("spec.runExecutionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.start-execution-token" => Some(("spec.startExecutionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.parallelism" => Some(("spec.template.spec.parallelism", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.task-count" => Some(("spec.template.spec.taskCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.template.spec.max-retries" => Some(("spec.template.spec.template.spec.maxRetries", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.template.spec.service-account-name" => Some(("spec.template.spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.template.spec.timeout-seconds" => Some(("spec.template.spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.execution-count" => Some(("status.executionCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.latest-created-execution.completion-timestamp" => Some(("status.latestCreatedExecution.completionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-execution.creation-timestamp" => Some(("status.latestCreatedExecution.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-execution.name" => Some(("status.latestCreatedExecution.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "api-version", "cluster-name", "completion-timestamp", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "execution-count", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-execution", "max-retries", "metadata", "name", "namespace", "observed-generation", "parallelism", "resource-version", "run-execution-token", "self-link", "service-account-name", "spec", "start-execution-token", "status", "task-count", "template", "timeout-seconds", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Job = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().jobs_replace_job(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_jobs_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "overrides.task-count" => Some(("overrides.taskCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "overrides.timeout-seconds" => Some(("overrides.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["overrides", "task-count", "timeout-seconds"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.namespaces().jobs_run(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_revisions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["api-version", "dry-run", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_revisions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().revisions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_routes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_routes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().routes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_services_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.enable-service-links" => Some(("spec.template.spec.enableServiceLinks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.template.spec.node-selector" => Some(("spec.template.spec.nodeSelector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.spec.runtime-class-name" => Some(("spec.template.spec.runtimeClassName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "enable-service-links", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "node-selector", "observed-generation", "resource-version", "runtime-class-name", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_services_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["api-version", "dry-run", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_services_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_services_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().services_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_services_replace_service(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.enable-service-links" => Some(("spec.template.spec.enableServiceLinks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.template.spec.node-selector" => Some(("spec.template.spec.nodeSelector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.spec.runtime-class-name" => Some(("spec.template.spec.runtimeClassName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "enable-service-links", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "node-selector", "observed-generation", "resource-version", "runtime-class-name", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_tasks_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().tasks_get(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _namespaces_tasks_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.namespaces().tasks_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_authorizeddomains_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().authorizeddomains_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_authorizeddomains_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_authorizeddomains_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_configurations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_configurations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_configurations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_domainmappings_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.certificate-mode" => Some(("spec.certificateMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.force-override" => Some(("spec.forceOverride", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.route-name" => Some(("spec.routeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.mapped-route-name" => Some(("status.mappedRouteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_domainmappings_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["api-version", "dry-run", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_domainmappings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_domainmappings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_domainmappings_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_jobs_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_jobs_get_iam_policy(opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(        value.map(|v| arg_from_str(v, err, "options-requested-policy-version", "int32")).unwrap_or(-0));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_jobs_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_jobs_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_jobs_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_jobs_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_delete(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_get(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_wait(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "timeout" => Some(("timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["timeout"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleLongrunningWaitOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_operations_wait(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_revisions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["api-version", "dry-run", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_revisions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_revisions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_revisions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_routes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_routes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_routes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.enable-service-links" => Some(("spec.template.spec.enableServiceLinks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.template.spec.node-selector" => Some(("spec.template.spec.nodeSelector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.spec.runtime-class-name" => Some(("spec.template.spec.runtimeClassName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "enable-service-links", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "node-selector", "observed-generation", "resource-version", "runtime-class-name", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["api-version", "dry-run", "kind", "propagation-policy"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_services_get_iam_policy(opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(        value.map(|v| arg_from_str(v, err, "options-requested-policy-version", "int32")).unwrap_or(-0));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_services_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "watch" => {
                    call = call.watch(        value.map(|v| arg_from_str(v, err, "watch", "boolean")).unwrap_or(false));
                },
                "resource-version" => {
                    call = call.resource_version(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(        value.map(|v| arg_from_str(v, err, "limit", "int32")).unwrap_or(-0));
                },
                "label-selector" => {
                    call = call.label_selector(value.unwrap_or(""));
                },
                "include-uninitialized" => {
                    call = call.include_uninitialized(        value.map(|v| arg_from_str(v, err, "include-uninitialized", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["continue", "field-selector", "include-uninitialized", "label-selector", "limit", "resource-version", "watch"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_replace_service(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-version" => Some(("apiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.annotations" => Some(("metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.cluster-name" => Some(("metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.creation-timestamp" => Some(("metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.deletion-grace-period-seconds" => Some(("metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.deletion-timestamp" => Some(("metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.finalizers" => Some(("metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metadata.generate-name" => Some(("metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.generation" => Some(("metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "metadata.labels" => Some(("metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metadata.name" => Some(("metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.namespace" => Some(("metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.resource-version" => Some(("metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.self-link" => Some(("metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.uid" => Some(("metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.annotations" => Some(("spec.template.metadata.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.cluster-name" => Some(("spec.template.metadata.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.creation-timestamp" => Some(("spec.template.metadata.creationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-grace-period-seconds" => Some(("spec.template.metadata.deletionGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.deletion-timestamp" => Some(("spec.template.metadata.deletionTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.finalizers" => Some(("spec.template.metadata.finalizers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spec.template.metadata.generate-name" => Some(("spec.template.metadata.generateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.generation" => Some(("spec.template.metadata.generation", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.metadata.labels" => Some(("spec.template.metadata.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.metadata.name" => Some(("spec.template.metadata.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.namespace" => Some(("spec.template.metadata.namespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.resource-version" => Some(("spec.template.metadata.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.self-link" => Some(("spec.template.metadata.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.metadata.uid" => Some(("spec.template.metadata.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.container-concurrency" => Some(("spec.template.spec.containerConcurrency", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spec.template.spec.enable-service-links" => Some(("spec.template.spec.enableServiceLinks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "spec.template.spec.node-selector" => Some(("spec.template.spec.nodeSelector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spec.template.spec.runtime-class-name" => Some(("spec.template.spec.runtimeClassName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.service-account-name" => Some(("spec.template.spec.serviceAccountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spec.template.spec.timeout-seconds" => Some(("spec.template.spec.timeoutSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.address.url" => Some(("status.address.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-created-revision-name" => Some(("status.latestCreatedRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.latest-ready-revision-name" => Some(("status.latestReadyRevisionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.observed-generation" => Some(("status.observedGeneration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.url" => Some(("status.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "annotations", "api-version", "cluster-name", "container-concurrency", "creation-timestamp", "deletion-grace-period-seconds", "deletion-timestamp", "enable-service-links", "finalizers", "generate-name", "generation", "kind", "labels", "latest-created-revision-name", "latest-ready-revision-name", "metadata", "name", "namespace", "node-selector", "observed-generation", "resource-version", "runtime-class-name", "self-link", "service-account-name", "spec", "status", "template", "timeout-seconds", "uid", "url"]);
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
                "dry-run" => {
                    call = call.dry_run(value.unwrap_or(""));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_services_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("namespaces", Some(opt)) => {
                match opt.subcommand() {
                    ("authorizeddomains-list", Some(opt)) => {
                        call_result = self._namespaces_authorizeddomains_list(opt, dry_run, &mut err).await;
                    },
                    ("configurations-get", Some(opt)) => {
                        call_result = self._namespaces_configurations_get(opt, dry_run, &mut err).await;
                    },
                    ("configurations-list", Some(opt)) => {
                        call_result = self._namespaces_configurations_list(opt, dry_run, &mut err).await;
                    },
                    ("domainmappings-create", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_create(opt, dry_run, &mut err).await;
                    },
                    ("domainmappings-delete", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_delete(opt, dry_run, &mut err).await;
                    },
                    ("domainmappings-get", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_get(opt, dry_run, &mut err).await;
                    },
                    ("domainmappings-list", Some(opt)) => {
                        call_result = self._namespaces_domainmappings_list(opt, dry_run, &mut err).await;
                    },
                    ("executions-cancel", Some(opt)) => {
                        call_result = self._namespaces_executions_cancel(opt, dry_run, &mut err).await;
                    },
                    ("executions-delete", Some(opt)) => {
                        call_result = self._namespaces_executions_delete(opt, dry_run, &mut err).await;
                    },
                    ("executions-get", Some(opt)) => {
                        call_result = self._namespaces_executions_get(opt, dry_run, &mut err).await;
                    },
                    ("executions-list", Some(opt)) => {
                        call_result = self._namespaces_executions_list(opt, dry_run, &mut err).await;
                    },
                    ("jobs-create", Some(opt)) => {
                        call_result = self._namespaces_jobs_create(opt, dry_run, &mut err).await;
                    },
                    ("jobs-delete", Some(opt)) => {
                        call_result = self._namespaces_jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("jobs-get", Some(opt)) => {
                        call_result = self._namespaces_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("jobs-list", Some(opt)) => {
                        call_result = self._namespaces_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("jobs-replace-job", Some(opt)) => {
                        call_result = self._namespaces_jobs_replace_job(opt, dry_run, &mut err).await;
                    },
                    ("jobs-run", Some(opt)) => {
                        call_result = self._namespaces_jobs_run(opt, dry_run, &mut err).await;
                    },
                    ("revisions-delete", Some(opt)) => {
                        call_result = self._namespaces_revisions_delete(opt, dry_run, &mut err).await;
                    },
                    ("revisions-get", Some(opt)) => {
                        call_result = self._namespaces_revisions_get(opt, dry_run, &mut err).await;
                    },
                    ("revisions-list", Some(opt)) => {
                        call_result = self._namespaces_revisions_list(opt, dry_run, &mut err).await;
                    },
                    ("routes-get", Some(opt)) => {
                        call_result = self._namespaces_routes_get(opt, dry_run, &mut err).await;
                    },
                    ("routes-list", Some(opt)) => {
                        call_result = self._namespaces_routes_list(opt, dry_run, &mut err).await;
                    },
                    ("services-create", Some(opt)) => {
                        call_result = self._namespaces_services_create(opt, dry_run, &mut err).await;
                    },
                    ("services-delete", Some(opt)) => {
                        call_result = self._namespaces_services_delete(opt, dry_run, &mut err).await;
                    },
                    ("services-get", Some(opt)) => {
                        call_result = self._namespaces_services_get(opt, dry_run, &mut err).await;
                    },
                    ("services-list", Some(opt)) => {
                        call_result = self._namespaces_services_list(opt, dry_run, &mut err).await;
                    },
                    ("services-replace-service", Some(opt)) => {
                        call_result = self._namespaces_services_replace_service(opt, dry_run, &mut err).await;
                    },
                    ("tasks-get", Some(opt)) => {
                        call_result = self._namespaces_tasks_get(opt, dry_run, &mut err).await;
                    },
                    ("tasks-list", Some(opt)) => {
                        call_result = self._namespaces_tasks_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("namespaces".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("authorizeddomains-list", Some(opt)) => {
                        call_result = self._projects_authorizeddomains_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-authorizeddomains-list", Some(opt)) => {
                        call_result = self._projects_locations_authorizeddomains_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-configurations-get", Some(opt)) => {
                        call_result = self._projects_locations_configurations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-configurations-list", Some(opt)) => {
                        call_result = self._projects_locations_configurations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-domainmappings-create", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-domainmappings-delete", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-domainmappings-get", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-domainmappings-list", Some(opt)) => {
                        call_result = self._projects_locations_domainmappings_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-jobs-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_jobs_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-jobs-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_jobs_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-jobs-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_jobs_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._projects_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-delete", Some(opt)) => {
                        call_result = self._projects_locations_operations_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-wait", Some(opt)) => {
                        call_result = self._projects_locations_operations_wait(opt, dry_run, &mut err).await;
                    },
                    ("locations-revisions-delete", Some(opt)) => {
                        call_result = self._projects_locations_revisions_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-revisions-get", Some(opt)) => {
                        call_result = self._projects_locations_revisions_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-revisions-list", Some(opt)) => {
                        call_result = self._projects_locations_revisions_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-routes-get", Some(opt)) => {
                        call_result = self._projects_locations_routes_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-routes-list", Some(opt)) => {
                        call_result = self._projects_locations_routes_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-create", Some(opt)) => {
                        call_result = self._projects_locations_services_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-delete", Some(opt)) => {
                        call_result = self._projects_locations_services_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-get", Some(opt)) => {
                        call_result = self._projects_locations_services_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_services_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-list", Some(opt)) => {
                        call_result = self._projects_locations_services_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-replace-service", Some(opt)) => {
                        call_result = self._projects_locations_services_replace_service(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_services_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-services-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_services_test_iam_permissions(opt, dry_run, &mut err).await;
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
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "run1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/run1", config_dir)).build().await.unwrap();

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

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("namespaces", "methods: 'authorizeddomains-list', 'configurations-get', 'configurations-list', 'domainmappings-create', 'domainmappings-delete', 'domainmappings-get', 'domainmappings-list', 'executions-cancel', 'executions-delete', 'executions-get', 'executions-list', 'jobs-create', 'jobs-delete', 'jobs-get', 'jobs-list', 'jobs-replace-job', 'jobs-run', 'revisions-delete', 'revisions-get', 'revisions-list', 'routes-get', 'routes-list', 'services-create', 'services-delete', 'services-get', 'services-list', 'services-replace-service', 'tasks-get' and 'tasks-list'", vec![
            ("authorizeddomains-list",
                    Some(r##"List authorized domains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_authorizeddomains-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent Project resource. Example: `projects/myproject`."##),
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
                     Some(r##"The name of the configuration to retrieve. For Cloud Run, replace {namespace_id} with the project ID or number."##),
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
                    Some(r##"List configurations. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_configurations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the configurations should be listed. For Cloud Run, replace {namespace_id} with the project ID or number."##),
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
                     Some(r##"Required. The namespace in which the domain mapping should be created. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"Required. The name of the domain mapping to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"Required. The name of the domain mapping to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"List all domain mappings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_domainmappings-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The namespace from which the domain mappings should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("executions-cancel",
                    Some(r##"Cancel an execution."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_executions-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the execution to cancel. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("executions-delete",
                    Some(r##"Delete an execution."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_executions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the execution to delete. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("executions-get",
                    Some(r##"Get information about an execution."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_executions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the execution to retrieve. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("executions-list",
                    Some(r##"List executions. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_executions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The namespace from which the executions should be listed. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("jobs-create",
                    Some(r##"Create a job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_jobs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The namespace in which the job should be created. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("jobs-delete",
                    Some(r##"Delete a job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_jobs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the job to delete. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("jobs-get",
                    Some(r##"Get information about a job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_jobs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the job to retrieve. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("jobs-list",
                    Some(r##"List jobs. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The namespace from which the jobs should be listed. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("jobs-replace-job",
                    Some(r##"Replace a job. Only the spec and metadata labels and annotations are modifiable. After the Replace request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_jobs-replace-job",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the job being replaced. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("jobs-run",
                    Some(r##"Trigger creation of a new execution of this job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_jobs-run",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the job to run. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("revisions-delete",
                    Some(r##"Delete a revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_revisions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the revision to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"The name of the revision to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"List revisions. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_revisions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the revisions should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"The name of the route to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"List routes. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_routes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the routes should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"Creates a new Service. Service creation will trigger a new deployment. Use GetService, and check service.status to determine if the Service is ready."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`"##),
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
                    Some(r##"Deletes the provided service. This will cause the Service to stop serving traffic and will delete all associated Revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the service to delete. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`"##),
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
                    Some(r##"Gets information about a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the service to retrieve. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`"##),
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
                    Some(r##"Lists services for the given project and region. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent from where the resources should be listed. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`"##),
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
                    Some(r##"Replaces a service. Only the spec and metadata labels and annotations are modifiable. After the Update request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_services-replace-service",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the service to replace. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`"##),
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
            ("tasks-get",
                    Some(r##"Get information about a task."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_tasks-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the task to retrieve. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("tasks-list",
                    Some(r##"List tasks."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/namespaces_tasks-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The namespace from which the tasks should be listed. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
        
        ("projects", "methods: 'authorizeddomains-list', 'locations-authorizeddomains-list', 'locations-configurations-get', 'locations-configurations-list', 'locations-domainmappings-create', 'locations-domainmappings-delete', 'locations-domainmappings-get', 'locations-domainmappings-list', 'locations-jobs-get-iam-policy', 'locations-jobs-set-iam-policy', 'locations-jobs-test-iam-permissions', 'locations-list', 'locations-operations-delete', 'locations-operations-get', 'locations-operations-list', 'locations-operations-wait', 'locations-revisions-delete', 'locations-revisions-get', 'locations-revisions-list', 'locations-routes-get', 'locations-routes-list', 'locations-services-create', 'locations-services-delete', 'locations-services-get', 'locations-services-get-iam-policy', 'locations-services-list', 'locations-services-replace-service', 'locations-services-set-iam-policy' and 'locations-services-test-iam-permissions'", vec![
            ("authorizeddomains-list",
                    Some(r##"List authorized domains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_authorizeddomains-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent Project resource. Example: `projects/myproject`."##),
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
            ("locations-authorizeddomains-list",
                    Some(r##"List authorized domains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-authorizeddomains-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent Project resource. Example: `projects/myproject`."##),
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
                     Some(r##"The name of the configuration to retrieve. For Cloud Run, replace {namespace_id} with the project ID or number."##),
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
                    Some(r##"List configurations. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-configurations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the configurations should be listed. For Cloud Run, replace {namespace_id} with the project ID or number."##),
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
                     Some(r##"Required. The namespace in which the domain mapping should be created. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"Required. The name of the domain mapping to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"Required. The name of the domain mapping to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"List all domain mappings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-domainmappings-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The namespace from which the domain mappings should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("locations-jobs-get-iam-policy",
                    Some(r##"Get the IAM Access Control policy currently in effect for the given job. This result does not include any inherited policies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-jobs-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-jobs-set-iam-policy",
                    Some(r##"Sets the IAM Access control policy for the specified job. Overwrites any existing policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-jobs-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-jobs-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified job. There are no permissions required for making this API call."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-jobs-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-operations-delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-operations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be deleted."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("locations-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. To query for all of the operations for a project."##),
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
            ("locations-operations-wait",
                    Some(r##"Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-operations-wait",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to wait on."##),
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
                     Some(r##"The name of the revision to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"The name of the revision to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"List revisions. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-revisions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the revisions should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                     Some(r##"The name of the route to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
                    Some(r##"List routes. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-routes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The namespace from which the routes should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID"##),
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
            ("locations-services-create",
                    Some(r##"Creates a new Service. Service creation will trigger a new deployment. Use GetService, and check service.status to determine if the Service is ready."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`"##),
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
                    Some(r##"Deletes the provided service. This will cause the Service to stop serving traffic and will delete all associated Revisions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the service to delete. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`"##),
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
                    Some(r##"Gets information about a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the service to retrieve. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`"##),
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
                    Some(r##"Gets the IAM Access Control policy currently in effect for the given Cloud Run service. This result does not include any inherited policies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
                    Some(r##"Lists services for the given project and region. Results are sorted by creation time, descending."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent from where the resources should be listed. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`"##),
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
                    Some(r##"Replaces a service. Only the spec and metadata labels and annotations are modifiable. After the Update request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-replace-service",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the service to replace. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`"##),
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
                    Some(r##"Sets the IAM Access control policy for the specified Service. Overwrites any existing policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
                    Some(r##"Returns permissions that a caller has on the specified Project. There are no permissions required for making this API call."##),
                    "Details at http://byron.github.io/google-apis-rs/google_run1_cli/projects_locations-services-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
           .version("5.0.5+20240621")
           .about("Deploy and manage user provided container images that scale automatically based on incoming requests. The Cloud Run Admin API v1 follows the Knative Serving API specification, while v2 is aligned with Google Cloud AIP-based API standards, as described in https://google.aip.dev/.")
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
                   .help("Debug print all errors")
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

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
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
