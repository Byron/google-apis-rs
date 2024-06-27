// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_dataproc1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Dataproc<S>,
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
    async fn _projects_locations_autoscaling_policies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-algorithm.cooldown-period" => Some(("basicAlgorithm.cooldownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.graceful-decommission-timeout" => Some(("basicAlgorithm.sparkStandaloneConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.remove-only-idle-workers" => Some(("basicAlgorithm.sparkStandaloneConfig.removeOnlyIdleWorkers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.graceful-decommission-timeout" => Some(("basicAlgorithm.yarnConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-factor" => Some(("basicAlgorithm.yarnConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-factor" => Some(("basicAlgorithm.yarnConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secondary-worker-config.max-instances" => Some(("secondaryWorkerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.min-instances" => Some(("secondaryWorkerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.weight" => Some(("secondaryWorkerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.max-instances" => Some(("workerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.min-instances" => Some(("workerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.weight" => Some(("workerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["basic-algorithm", "cooldown-period", "graceful-decommission-timeout", "id", "labels", "max-instances", "min-instances", "name", "remove-only-idle-workers", "scale-down-factor", "scale-down-min-worker-fraction", "scale-up-factor", "scale-up-min-worker-fraction", "secondary-worker-config", "spark-standalone-config", "weight", "worker-config", "yarn-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AutoscalingPolicy = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_autoscaling_policies_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_autoscaling_policies_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_autoscaling_policies_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_autoscaling_policies_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_autoscaling_policies_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_autoscaling_policies_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_autoscaling_policies_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_autoscaling_policies_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-algorithm.cooldown-period" => Some(("basicAlgorithm.cooldownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.graceful-decommission-timeout" => Some(("basicAlgorithm.sparkStandaloneConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.remove-only-idle-workers" => Some(("basicAlgorithm.sparkStandaloneConfig.removeOnlyIdleWorkers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.graceful-decommission-timeout" => Some(("basicAlgorithm.yarnConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-factor" => Some(("basicAlgorithm.yarnConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-factor" => Some(("basicAlgorithm.yarnConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secondary-worker-config.max-instances" => Some(("secondaryWorkerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.min-instances" => Some(("secondaryWorkerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.weight" => Some(("secondaryWorkerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.max-instances" => Some(("workerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.min-instances" => Some(("workerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.weight" => Some(("workerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["basic-algorithm", "cooldown-period", "graceful-decommission-timeout", "id", "labels", "max-instances", "min-instances", "name", "remove-only-idle-workers", "scale-down-factor", "scale-down-min-worker-fraction", "scale-up-factor", "scale-up-min-worker-fraction", "secondary-worker-config", "spark-standalone-config", "weight", "worker-config", "yarn-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AutoscalingPolicy = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_autoscaling_policies_update(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_batches_analyze(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AnalyzeBatchRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_batches_analyze(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_batches_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator" => Some(("creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.idle-ttl" => Some(("environmentConfig.executionConfig.idleTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.kms-key" => Some(("environmentConfig.executionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.network-tags" => Some(("environmentConfig.executionConfig.networkTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-config.execution-config.network-uri" => Some(("environmentConfig.executionConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.service-account" => Some(("environmentConfig.executionConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.staging-bucket" => Some(("environmentConfig.executionConfig.stagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.subnetwork-uri" => Some(("environmentConfig.executionConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.ttl" => Some(("environmentConfig.executionConfig.ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.metastore-service" => Some(("environmentConfig.peripheralsConfig.metastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.spark-history-server-config.dataproc-cluster" => Some(("environmentConfig.peripheralsConfig.sparkHistoryServerConfig.dataprocCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operation" => Some(("operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pyspark-batch.archive-uris" => Some(("pysparkBatch.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-batch.args" => Some(("pysparkBatch.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-batch.file-uris" => Some(("pysparkBatch.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-batch.jar-file-uris" => Some(("pysparkBatch.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-batch.main-python-file-uri" => Some(("pysparkBatch.mainPythonFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pyspark-batch.python-file-uris" => Some(("pysparkBatch.pythonFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-config.autotuning-config.scenarios" => Some(("runtimeConfig.autotuningConfig.scenarios", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-config.cohort" => Some(("runtimeConfig.cohort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.container-image" => Some(("runtimeConfig.containerImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.properties" => Some(("runtimeConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "runtime-config.repository-config.pypi-repository-config.pypi-repository" => Some(("runtimeConfig.repositoryConfig.pypiRepositoryConfig.pypiRepository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.version" => Some(("runtimeConfig.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.accelerator-type" => Some(("runtimeInfo.approximateUsage.acceleratorType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.milli-accelerator-seconds" => Some(("runtimeInfo.approximateUsage.milliAcceleratorSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.milli-dcu-seconds" => Some(("runtimeInfo.approximateUsage.milliDcuSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.shuffle-storage-gb-seconds" => Some(("runtimeInfo.approximateUsage.shuffleStorageGbSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.accelerator-type" => Some(("runtimeInfo.currentUsage.acceleratorType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.milli-accelerator" => Some(("runtimeInfo.currentUsage.milliAccelerator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.milli-dcu" => Some(("runtimeInfo.currentUsage.milliDcu", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.milli-dcu-premium" => Some(("runtimeInfo.currentUsage.milliDcuPremium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.shuffle-storage-gb" => Some(("runtimeInfo.currentUsage.shuffleStorageGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.shuffle-storage-gb-premium" => Some(("runtimeInfo.currentUsage.shuffleStorageGbPremium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.snapshot-time" => Some(("runtimeInfo.currentUsage.snapshotTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.diagnostic-output-uri" => Some(("runtimeInfo.diagnosticOutputUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.endpoints" => Some(("runtimeInfo.endpoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "runtime-info.output-uri" => Some(("runtimeInfo.outputUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-batch.archive-uris" => Some(("sparkBatch.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-batch.args" => Some(("sparkBatch.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-batch.file-uris" => Some(("sparkBatch.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-batch.jar-file-uris" => Some(("sparkBatch.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-batch.main-class" => Some(("sparkBatch.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-batch.main-jar-file-uri" => Some(("sparkBatch.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-r-batch.archive-uris" => Some(("sparkRBatch.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-r-batch.args" => Some(("sparkRBatch.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-r-batch.file-uris" => Some(("sparkRBatch.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-r-batch.main-r-file-uri" => Some(("sparkRBatch.mainRFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-sql-batch.jar-file-uris" => Some(("sparkSqlBatch.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-sql-batch.query-file-uri" => Some(("sparkSqlBatch.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-sql-batch.query-variables" => Some(("sparkSqlBatch.queryVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-message" => Some(("stateMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-time" => Some(("stateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uuid" => Some(("uuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-type", "approximate-usage", "archive-uris", "args", "autotuning-config", "cohort", "container-image", "create-time", "creator", "current-usage", "dataproc-cluster", "diagnostic-output-uri", "endpoints", "environment-config", "execution-config", "file-uris", "idle-ttl", "jar-file-uris", "kms-key", "labels", "main-class", "main-jar-file-uri", "main-python-file-uri", "main-r-file-uri", "metastore-service", "milli-accelerator", "milli-accelerator-seconds", "milli-dcu", "milli-dcu-premium", "milli-dcu-seconds", "name", "network-tags", "network-uri", "operation", "output-uri", "peripherals-config", "properties", "pypi-repository", "pypi-repository-config", "pyspark-batch", "python-file-uris", "query-file-uri", "query-variables", "repository-config", "runtime-config", "runtime-info", "scenarios", "service-account", "shuffle-storage-gb", "shuffle-storage-gb-premium", "shuffle-storage-gb-seconds", "snapshot-time", "spark-batch", "spark-history-server-config", "spark-r-batch", "spark-sql-batch", "staging-bucket", "state", "state-message", "state-time", "subnetwork-uri", "ttl", "uuid", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Batch = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_batches_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "batch-id" => {
                    call = call.batch_id(value.unwrap_or(""));
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
                                                                           v.extend(["batch-id", "request-id"].iter().map(|v|*v));
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

    async fn _projects_locations_batches_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_batches_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_batches_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_batches_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_batches_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_batches_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_cancel(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_session_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator" => Some(("creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.idle-ttl" => Some(("environmentConfig.executionConfig.idleTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.kms-key" => Some(("environmentConfig.executionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.network-tags" => Some(("environmentConfig.executionConfig.networkTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-config.execution-config.network-uri" => Some(("environmentConfig.executionConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.service-account" => Some(("environmentConfig.executionConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.staging-bucket" => Some(("environmentConfig.executionConfig.stagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.subnetwork-uri" => Some(("environmentConfig.executionConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.ttl" => Some(("environmentConfig.executionConfig.ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.metastore-service" => Some(("environmentConfig.peripheralsConfig.metastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.spark-history-server-config.dataproc-cluster" => Some(("environmentConfig.peripheralsConfig.sparkHistoryServerConfig.dataprocCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jupyter-session.display-name" => Some(("jupyterSession.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jupyter-session.kernel" => Some(("jupyterSession.kernel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.autotuning-config.scenarios" => Some(("runtimeConfig.autotuningConfig.scenarios", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-config.cohort" => Some(("runtimeConfig.cohort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.container-image" => Some(("runtimeConfig.containerImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.properties" => Some(("runtimeConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "runtime-config.repository-config.pypi-repository-config.pypi-repository" => Some(("runtimeConfig.repositoryConfig.pypiRepositoryConfig.pypiRepository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.version" => Some(("runtimeConfig.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uuid" => Some(("uuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["autotuning-config", "cohort", "container-image", "create-time", "creator", "dataproc-cluster", "description", "display-name", "environment-config", "execution-config", "idle-ttl", "jupyter-session", "kernel", "kms-key", "labels", "metastore-service", "name", "network-tags", "network-uri", "peripherals-config", "properties", "pypi-repository", "pypi-repository-config", "repository-config", "runtime-config", "scenarios", "service-account", "spark-history-server-config", "staging-bucket", "subnetwork-uri", "ttl", "update-time", "uuid", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SessionTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_session_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_session_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_session_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_session_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_session_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_session_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_session_templates_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_session_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator" => Some(("creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.idle-ttl" => Some(("environmentConfig.executionConfig.idleTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.kms-key" => Some(("environmentConfig.executionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.network-tags" => Some(("environmentConfig.executionConfig.networkTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-config.execution-config.network-uri" => Some(("environmentConfig.executionConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.service-account" => Some(("environmentConfig.executionConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.staging-bucket" => Some(("environmentConfig.executionConfig.stagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.subnetwork-uri" => Some(("environmentConfig.executionConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.ttl" => Some(("environmentConfig.executionConfig.ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.metastore-service" => Some(("environmentConfig.peripheralsConfig.metastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.spark-history-server-config.dataproc-cluster" => Some(("environmentConfig.peripheralsConfig.sparkHistoryServerConfig.dataprocCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jupyter-session.display-name" => Some(("jupyterSession.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jupyter-session.kernel" => Some(("jupyterSession.kernel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.autotuning-config.scenarios" => Some(("runtimeConfig.autotuningConfig.scenarios", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-config.cohort" => Some(("runtimeConfig.cohort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.container-image" => Some(("runtimeConfig.containerImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.properties" => Some(("runtimeConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "runtime-config.repository-config.pypi-repository-config.pypi-repository" => Some(("runtimeConfig.repositoryConfig.pypiRepositoryConfig.pypiRepository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.version" => Some(("runtimeConfig.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uuid" => Some(("uuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["autotuning-config", "cohort", "container-image", "create-time", "creator", "dataproc-cluster", "description", "display-name", "environment-config", "execution-config", "idle-ttl", "jupyter-session", "kernel", "kms-key", "labels", "metastore-service", "name", "network-tags", "network-uri", "peripherals-config", "properties", "pypi-repository", "pypi-repository-config", "repository-config", "runtime-config", "scenarios", "service-account", "spark-history-server-config", "staging-bucket", "subnetwork-uri", "ttl", "update-time", "uuid", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SessionTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_session_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_sessions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator" => Some(("creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.idle-ttl" => Some(("environmentConfig.executionConfig.idleTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.kms-key" => Some(("environmentConfig.executionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.network-tags" => Some(("environmentConfig.executionConfig.networkTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-config.execution-config.network-uri" => Some(("environmentConfig.executionConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.service-account" => Some(("environmentConfig.executionConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.staging-bucket" => Some(("environmentConfig.executionConfig.stagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.subnetwork-uri" => Some(("environmentConfig.executionConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.execution-config.ttl" => Some(("environmentConfig.executionConfig.ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.metastore-service" => Some(("environmentConfig.peripheralsConfig.metastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-config.peripherals-config.spark-history-server-config.dataproc-cluster" => Some(("environmentConfig.peripheralsConfig.sparkHistoryServerConfig.dataprocCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jupyter-session.display-name" => Some(("jupyterSession.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jupyter-session.kernel" => Some(("jupyterSession.kernel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.autotuning-config.scenarios" => Some(("runtimeConfig.autotuningConfig.scenarios", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-config.cohort" => Some(("runtimeConfig.cohort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.container-image" => Some(("runtimeConfig.containerImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.properties" => Some(("runtimeConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "runtime-config.repository-config.pypi-repository-config.pypi-repository" => Some(("runtimeConfig.repositoryConfig.pypiRepositoryConfig.pypiRepository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-config.version" => Some(("runtimeConfig.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.accelerator-type" => Some(("runtimeInfo.approximateUsage.acceleratorType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.milli-accelerator-seconds" => Some(("runtimeInfo.approximateUsage.milliAcceleratorSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.milli-dcu-seconds" => Some(("runtimeInfo.approximateUsage.milliDcuSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.approximate-usage.shuffle-storage-gb-seconds" => Some(("runtimeInfo.approximateUsage.shuffleStorageGbSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.accelerator-type" => Some(("runtimeInfo.currentUsage.acceleratorType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.milli-accelerator" => Some(("runtimeInfo.currentUsage.milliAccelerator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.milli-dcu" => Some(("runtimeInfo.currentUsage.milliDcu", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.milli-dcu-premium" => Some(("runtimeInfo.currentUsage.milliDcuPremium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.shuffle-storage-gb" => Some(("runtimeInfo.currentUsage.shuffleStorageGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.shuffle-storage-gb-premium" => Some(("runtimeInfo.currentUsage.shuffleStorageGbPremium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.current-usage.snapshot-time" => Some(("runtimeInfo.currentUsage.snapshotTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.diagnostic-output-uri" => Some(("runtimeInfo.diagnosticOutputUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-info.endpoints" => Some(("runtimeInfo.endpoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "runtime-info.output-uri" => Some(("runtimeInfo.outputUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "session-template" => Some(("sessionTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-message" => Some(("stateMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-time" => Some(("stateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user" => Some(("user", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uuid" => Some(("uuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-type", "approximate-usage", "autotuning-config", "cohort", "container-image", "create-time", "creator", "current-usage", "dataproc-cluster", "diagnostic-output-uri", "display-name", "endpoints", "environment-config", "execution-config", "idle-ttl", "jupyter-session", "kernel", "kms-key", "labels", "metastore-service", "milli-accelerator", "milli-accelerator-seconds", "milli-dcu", "milli-dcu-premium", "milli-dcu-seconds", "name", "network-tags", "network-uri", "output-uri", "peripherals-config", "properties", "pypi-repository", "pypi-repository-config", "repository-config", "runtime-config", "runtime-info", "scenarios", "service-account", "session-template", "shuffle-storage-gb", "shuffle-storage-gb-premium", "shuffle-storage-gb-seconds", "snapshot-time", "spark-history-server-config", "staging-bucket", "state", "state-message", "state-time", "subnetwork-uri", "ttl", "user", "uuid", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Session = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_sessions_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "session-id" => {
                    call = call.session_id(value.unwrap_or(""));
                },
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
                                                                           v.extend(["request-id", "session-id"].iter().map(|v|*v));
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

    async fn _projects_locations_sessions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_sessions_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_sessions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_sessions_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_sessions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_sessions_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_sessions_terminate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TerminateSessionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_sessions_terminate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dag-timeout" => Some(("dagTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-config.kms-key" => Some(("encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-selector.cluster-labels" => Some(("placement.clusterSelector.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-selector.zone" => Some(("placement.clusterSelector.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.cluster-name" => Some(("placement.managedCluster.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.autoscaling-config.policy-uri" => Some(("placement.managedCluster.config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.config-bucket" => Some(("placement.managedCluster.config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.gce-pd-kms-key-name" => Some(("placement.managedCluster.config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.kms-key" => Some(("placement.managedCluster.config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.enable-http-port-access" => Some(("placement.managedCluster.config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.http-ports" => Some(("placement.managedCluster.config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("placement.managedCluster.config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.internal-ip-only" => Some(("placement.managedCluster.config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.metadata" => Some(("placement.managedCluster.config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.network-uri" => Some(("placement.managedCluster.config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("placement.managedCluster.config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.private-ipv6-google-access" => Some(("placement.managedCluster.config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.key" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.values" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account-scopes" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.subnetwork-uri" => Some(("placement.managedCluster.config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.tags" => Some(("placement.managedCluster.config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.zone-uri" => Some(("placement.managedCluster.config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.gke-cluster-target" => Some(("placement.managedCluster.config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-time" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-start-time" => Some(("placement.managedCluster.config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.image-uri" => Some(("placement.managedCluster.config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.instance-names" => Some(("placement.managedCluster.config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.master-config.is-preemptible" => Some(("placement.managedCluster.config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.machine-type-uri" => Some(("placement.managedCluster.config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-cpu-platform" => Some(("placement.managedCluster.config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-num-instances" => Some(("placement.managedCluster.config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.num-instances" => Some(("placement.managedCluster.config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.preemptibility" => Some(("placement.managedCluster.config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.metastore-config.dataproc-metastore-service" => Some(("placement.managedCluster.config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.image-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.instance-names" => Some(("placement.managedCluster.config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.secondary-worker-config.is-preemptible" => Some(("placement.managedCluster.config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.machine-type-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.preemptibility" => Some(("placement.managedCluster.config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.identity-config.user-service-account-mapping" => Some(("placement.managedCluster.config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.enable-kerberos" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kdc-db-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.key-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kms-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.root-principal-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.image-version" => Some(("placement.managedCluster.config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.optional-components" => Some(("placement.managedCluster.config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.software-config.properties" => Some(("placement.managedCluster.config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.temp-bucket" => Some(("placement.managedCluster.config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.image-uri" => Some(("placement.managedCluster.config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.instance-names" => Some(("placement.managedCluster.config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.worker-config.is-preemptible" => Some(("placement.managedCluster.config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.machine-type-uri" => Some(("placement.managedCluster.config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-num-instances" => Some(("placement.managedCluster.config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.num-instances" => Some(("placement.managedCluster.config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.preemptibility" => Some(("placement.managedCluster.config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.labels" => Some(("placement.managedCluster.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-labels", "cluster-name", "cluster-namespace", "cluster-selector", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "create-time", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dag-timeout", "dataproc-metastore-service", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "http-ports", "id", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-cluster", "managed-group-config", "master-config", "metadata", "metastore-config", "min-cpu-platform", "min-num-instances", "name", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "placement", "policy-uri", "preemptibility", "private-ipv6-google-access", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "startup-config", "subnetwork-uri", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "update-time", "user-service-account-mapping", "values", "version", "worker-config", "zone", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkflowTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_workflow_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_workflow_templates_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(        value.map(|v| arg_from_str(v, err, "version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["version"].iter().map(|v|*v));
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

    async fn _projects_locations_workflow_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_workflow_templates_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(        value.map(|v| arg_from_str(v, err, "version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["version"].iter().map(|v|*v));
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

    async fn _projects_locations_workflow_templates_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_workflow_templates_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_instantiate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "parameters" => Some(("parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["parameters", "request-id", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InstantiateWorkflowTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_workflow_templates_instantiate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_instantiate_inline(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dag-timeout" => Some(("dagTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-config.kms-key" => Some(("encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-selector.cluster-labels" => Some(("placement.clusterSelector.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-selector.zone" => Some(("placement.clusterSelector.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.cluster-name" => Some(("placement.managedCluster.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.autoscaling-config.policy-uri" => Some(("placement.managedCluster.config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.config-bucket" => Some(("placement.managedCluster.config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.gce-pd-kms-key-name" => Some(("placement.managedCluster.config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.kms-key" => Some(("placement.managedCluster.config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.enable-http-port-access" => Some(("placement.managedCluster.config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.http-ports" => Some(("placement.managedCluster.config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("placement.managedCluster.config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.internal-ip-only" => Some(("placement.managedCluster.config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.metadata" => Some(("placement.managedCluster.config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.network-uri" => Some(("placement.managedCluster.config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("placement.managedCluster.config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.private-ipv6-google-access" => Some(("placement.managedCluster.config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.key" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.values" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account-scopes" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.subnetwork-uri" => Some(("placement.managedCluster.config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.tags" => Some(("placement.managedCluster.config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.zone-uri" => Some(("placement.managedCluster.config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.gke-cluster-target" => Some(("placement.managedCluster.config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-time" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-start-time" => Some(("placement.managedCluster.config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.image-uri" => Some(("placement.managedCluster.config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.instance-names" => Some(("placement.managedCluster.config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.master-config.is-preemptible" => Some(("placement.managedCluster.config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.machine-type-uri" => Some(("placement.managedCluster.config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-cpu-platform" => Some(("placement.managedCluster.config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-num-instances" => Some(("placement.managedCluster.config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.num-instances" => Some(("placement.managedCluster.config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.preemptibility" => Some(("placement.managedCluster.config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.metastore-config.dataproc-metastore-service" => Some(("placement.managedCluster.config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.image-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.instance-names" => Some(("placement.managedCluster.config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.secondary-worker-config.is-preemptible" => Some(("placement.managedCluster.config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.machine-type-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.preemptibility" => Some(("placement.managedCluster.config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.identity-config.user-service-account-mapping" => Some(("placement.managedCluster.config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.enable-kerberos" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kdc-db-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.key-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kms-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.root-principal-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.image-version" => Some(("placement.managedCluster.config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.optional-components" => Some(("placement.managedCluster.config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.software-config.properties" => Some(("placement.managedCluster.config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.temp-bucket" => Some(("placement.managedCluster.config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.image-uri" => Some(("placement.managedCluster.config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.instance-names" => Some(("placement.managedCluster.config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.worker-config.is-preemptible" => Some(("placement.managedCluster.config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.machine-type-uri" => Some(("placement.managedCluster.config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-num-instances" => Some(("placement.managedCluster.config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.num-instances" => Some(("placement.managedCluster.config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.preemptibility" => Some(("placement.managedCluster.config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.labels" => Some(("placement.managedCluster.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-labels", "cluster-name", "cluster-namespace", "cluster-selector", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "create-time", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dag-timeout", "dataproc-metastore-service", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "http-ports", "id", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-cluster", "managed-group-config", "master-config", "metadata", "metastore-config", "min-cpu-platform", "min-num-instances", "name", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "placement", "policy-uri", "preemptibility", "private-ipv6-google-access", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "startup-config", "subnetwork-uri", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "update-time", "user-service-account-mapping", "values", "version", "worker-config", "zone", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkflowTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_workflow_templates_instantiate_inline(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_workflow_templates_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_workflow_templates_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_workflow_templates_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_workflow_templates_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dag-timeout" => Some(("dagTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-config.kms-key" => Some(("encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-selector.cluster-labels" => Some(("placement.clusterSelector.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-selector.zone" => Some(("placement.clusterSelector.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.cluster-name" => Some(("placement.managedCluster.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.autoscaling-config.policy-uri" => Some(("placement.managedCluster.config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.config-bucket" => Some(("placement.managedCluster.config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.gce-pd-kms-key-name" => Some(("placement.managedCluster.config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.kms-key" => Some(("placement.managedCluster.config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.enable-http-port-access" => Some(("placement.managedCluster.config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.http-ports" => Some(("placement.managedCluster.config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("placement.managedCluster.config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.internal-ip-only" => Some(("placement.managedCluster.config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.metadata" => Some(("placement.managedCluster.config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.network-uri" => Some(("placement.managedCluster.config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("placement.managedCluster.config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.private-ipv6-google-access" => Some(("placement.managedCluster.config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.key" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.values" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account-scopes" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.subnetwork-uri" => Some(("placement.managedCluster.config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.tags" => Some(("placement.managedCluster.config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.zone-uri" => Some(("placement.managedCluster.config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.gke-cluster-target" => Some(("placement.managedCluster.config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-time" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-start-time" => Some(("placement.managedCluster.config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.image-uri" => Some(("placement.managedCluster.config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.instance-names" => Some(("placement.managedCluster.config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.master-config.is-preemptible" => Some(("placement.managedCluster.config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.machine-type-uri" => Some(("placement.managedCluster.config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-cpu-platform" => Some(("placement.managedCluster.config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-num-instances" => Some(("placement.managedCluster.config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.num-instances" => Some(("placement.managedCluster.config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.preemptibility" => Some(("placement.managedCluster.config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.metastore-config.dataproc-metastore-service" => Some(("placement.managedCluster.config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.image-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.instance-names" => Some(("placement.managedCluster.config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.secondary-worker-config.is-preemptible" => Some(("placement.managedCluster.config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.machine-type-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.preemptibility" => Some(("placement.managedCluster.config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.identity-config.user-service-account-mapping" => Some(("placement.managedCluster.config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.enable-kerberos" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kdc-db-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.key-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kms-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.root-principal-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.image-version" => Some(("placement.managedCluster.config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.optional-components" => Some(("placement.managedCluster.config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.software-config.properties" => Some(("placement.managedCluster.config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.temp-bucket" => Some(("placement.managedCluster.config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.image-uri" => Some(("placement.managedCluster.config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.instance-names" => Some(("placement.managedCluster.config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.worker-config.is-preemptible" => Some(("placement.managedCluster.config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.machine-type-uri" => Some(("placement.managedCluster.config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-num-instances" => Some(("placement.managedCluster.config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.num-instances" => Some(("placement.managedCluster.config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.preemptibility" => Some(("placement.managedCluster.config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.labels" => Some(("placement.managedCluster.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-labels", "cluster-name", "cluster-namespace", "cluster-selector", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "create-time", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dag-timeout", "dataproc-metastore-service", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "http-ports", "id", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-cluster", "managed-group-config", "master-config", "metadata", "metastore-config", "min-cpu-platform", "min-num-instances", "name", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "placement", "policy-uri", "preemptibility", "private-ipv6-google-access", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "startup-config", "subnetwork-uri", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "update-time", "user-service-account-mapping", "values", "version", "worker-config", "zone", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkflowTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_workflow_templates_update(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-algorithm.cooldown-period" => Some(("basicAlgorithm.cooldownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.graceful-decommission-timeout" => Some(("basicAlgorithm.sparkStandaloneConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.remove-only-idle-workers" => Some(("basicAlgorithm.sparkStandaloneConfig.removeOnlyIdleWorkers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.graceful-decommission-timeout" => Some(("basicAlgorithm.yarnConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-factor" => Some(("basicAlgorithm.yarnConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-factor" => Some(("basicAlgorithm.yarnConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secondary-worker-config.max-instances" => Some(("secondaryWorkerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.min-instances" => Some(("secondaryWorkerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.weight" => Some(("secondaryWorkerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.max-instances" => Some(("workerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.min-instances" => Some(("workerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.weight" => Some(("workerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["basic-algorithm", "cooldown-period", "graceful-decommission-timeout", "id", "labels", "max-instances", "min-instances", "name", "remove-only-idle-workers", "scale-down-factor", "scale-down-min-worker-fraction", "scale-up-factor", "scale-up-min-worker-fraction", "secondary-worker-config", "spark-standalone-config", "weight", "worker-config", "yarn-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AutoscalingPolicy = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_autoscaling_policies_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_autoscaling_policies_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_autoscaling_policies_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_autoscaling_policies_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_autoscaling_policies_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_autoscaling_policies_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().regions_autoscaling_policies_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_autoscaling_policies_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-algorithm.cooldown-period" => Some(("basicAlgorithm.cooldownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.graceful-decommission-timeout" => Some(("basicAlgorithm.sparkStandaloneConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.remove-only-idle-workers" => Some(("basicAlgorithm.sparkStandaloneConfig.removeOnlyIdleWorkers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-factor" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.spark-standalone-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.sparkStandaloneConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.graceful-decommission-timeout" => Some(("basicAlgorithm.yarnConfig.gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-factor" => Some(("basicAlgorithm.yarnConfig.scaleDownFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-down-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleDownMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-factor" => Some(("basicAlgorithm.yarnConfig.scaleUpFactor", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "basic-algorithm.yarn-config.scale-up-min-worker-fraction" => Some(("basicAlgorithm.yarnConfig.scaleUpMinWorkerFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secondary-worker-config.max-instances" => Some(("secondaryWorkerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.min-instances" => Some(("secondaryWorkerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "secondary-worker-config.weight" => Some(("secondaryWorkerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.max-instances" => Some(("workerConfig.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.min-instances" => Some(("workerConfig.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "worker-config.weight" => Some(("workerConfig.weight", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["basic-algorithm", "cooldown-period", "graceful-decommission-timeout", "id", "labels", "max-instances", "min-instances", "name", "remove-only-idle-workers", "scale-down-factor", "scale-down-min-worker-fraction", "scale-up-factor", "scale-up-min-worker-fraction", "secondary-worker-config", "spark-standalone-config", "weight", "worker-config", "yarn-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AutoscalingPolicy = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_autoscaling_policies_update(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_clusters_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-name" => Some(("clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.autoscaling-config.policy-uri" => Some(("config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.config-bucket" => Some(("config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.encryption-config.gce-pd-kms-key-name" => Some(("config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.encryption-config.kms-key" => Some(("config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.endpoint-config.enable-http-port-access" => Some(("config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.endpoint-config.http-ports" => Some(("config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.internal-ip-only" => Some(("config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.metadata" => Some(("config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.gce-cluster-config.network-uri" => Some(("config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.private-ipv6-google-access" => Some(("config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.reservation-affinity.key" => Some(("config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.reservation-affinity.values" => Some(("config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.service-account" => Some(("config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.service-account-scopes" => Some(("config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.subnetwork-uri" => Some(("config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.tags" => Some(("config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.zone-uri" => Some(("config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster-config.gke-cluster-target" => Some(("config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.auto-delete-time" => Some(("config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.auto-delete-ttl" => Some(("config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.idle-delete-ttl" => Some(("config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.idle-start-time" => Some(("config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-size-gb" => Some(("config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-type" => Some(("config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.local-ssd-interface" => Some(("config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.num-local-ssds" => Some(("config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.image-uri" => Some(("config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.instance-names" => Some(("config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.master-config.is-preemptible" => Some(("config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.master-config.machine-type-uri" => Some(("config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-group-manager-name" => Some(("config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-group-manager-uri" => Some(("config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-template-name" => Some(("config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.min-cpu-platform" => Some(("config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.min-num-instances" => Some(("config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.num-instances" => Some(("config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.preemptibility" => Some(("config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.startup-config.required-registration-fraction" => Some(("config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.metastore-config.dataproc-metastore-service" => Some(("config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-type" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.num-local-ssds" => Some(("config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.image-uri" => Some(("config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.instance-names" => Some(("config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.secondary-worker-config.is-preemptible" => Some(("config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.machine-type-uri" => Some(("config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.min-cpu-platform" => Some(("config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.min-num-instances" => Some(("config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.num-instances" => Some(("config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.preemptibility" => Some(("config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.security-config.identity-config.user-service-account-mapping" => Some(("config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.enable-kerberos" => Some(("config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.kdc-db-key-uri" => Some(("config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.key-password-uri" => Some(("config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.keystore-password-uri" => Some(("config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.keystore-uri" => Some(("config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.kms-key-uri" => Some(("config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.realm" => Some(("config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.root-principal-password-uri" => Some(("config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.truststore-password-uri" => Some(("config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.truststore-uri" => Some(("config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.image-version" => Some(("config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.optional-components" => Some(("config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.software-config.properties" => Some(("config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.temp-bucket" => Some(("config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-size-gb" => Some(("config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-type" => Some(("config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.local-ssd-interface" => Some(("config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.num-local-ssds" => Some(("config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.image-uri" => Some(("config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.instance-names" => Some(("config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.worker-config.is-preemptible" => Some(("config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.worker-config.machine-type-uri" => Some(("config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-group-manager-name" => Some(("config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-template-name" => Some(("config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.min-cpu-platform" => Some(("config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.min-num-instances" => Some(("config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.num-instances" => Some(("config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.preemptibility" => Some(("config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.startup-config.required-registration-fraction" => Some(("config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.hdfs-metrics" => Some(("metrics.hdfsMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.yarn-metrics" => Some(("metrics.yarnMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.detail" => Some(("status.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state-start-time" => Some(("status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.substate" => Some(("status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.auxiliary-services-config.metastore-config.dataproc-metastore-service" => Some(("virtualClusterConfig.auxiliaryServicesConfig.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.auxiliary-services-config.spark-history-server-config.dataproc-cluster" => Some(("virtualClusterConfig.auxiliaryServicesConfig.sparkHistoryServerConfig.dataprocCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.gke-cluster-config.gke-cluster-target" => Some(("virtualClusterConfig.kubernetesClusterConfig.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("virtualClusterConfig.kubernetesClusterConfig.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("virtualClusterConfig.kubernetesClusterConfig.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.kubernetes-namespace" => Some(("virtualClusterConfig.kubernetesClusterConfig.kubernetesNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.kubernetes-software-config.component-version" => Some(("virtualClusterConfig.kubernetesClusterConfig.kubernetesSoftwareConfig.componentVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "virtual-cluster-config.kubernetes-cluster-config.kubernetes-software-config.properties" => Some(("virtualClusterConfig.kubernetesClusterConfig.kubernetesSoftwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "virtual-cluster-config.staging-bucket" => Some(("virtualClusterConfig.stagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "auxiliary-services-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-name", "cluster-namespace", "cluster-uuid", "component-version", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dataproc-cluster", "dataproc-metastore-service", "detail", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "hdfs-metrics", "http-ports", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "kubernetes-cluster-config", "kubernetes-namespace", "kubernetes-software-config", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-group-config", "master-config", "metadata", "metastore-config", "metrics", "min-cpu-platform", "min-num-instances", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "policy-uri", "preemptibility", "private-ipv6-google-access", "project-id", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "spark-history-server-config", "staging-bucket", "startup-config", "state", "state-start-time", "status", "subnetwork-uri", "substate", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "user-service-account-mapping", "values", "virtual-cluster-config", "worker-config", "yarn-metrics", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Cluster = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_create(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "action-on-failed-primary-workers" => {
                    call = call.action_on_failed_primary_workers(value.unwrap_or(""));
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
                                                                           v.extend(["action-on-failed-primary-workers", "request-id"].iter().map(|v|*v));
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

    async fn _projects_regions_clusters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_clusters_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "graceful-termination-timeout" => {
                    call = call.graceful_termination_timeout(        value.map(|v| arg_from_str(v, err, "graceful-termination-timeout", "google-duration")).unwrap_or(chrono::Duration::seconds(0)));
                },
                "cluster-uuid" => {
                    call = call.cluster_uuid(value.unwrap_or(""));
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
                                                                           v.extend(["cluster-uuid", "graceful-termination-timeout", "request-id"].iter().map(|v|*v));
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

    async fn _projects_regions_clusters_diagnose(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "diagnosis-interval.end-time" => Some(("diagnosisInterval.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "diagnosis-interval.start-time" => Some(("diagnosisInterval.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job" => Some(("job", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "jobs" => Some(("jobs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tarball-access" => Some(("tarballAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tarball-gcs-dir" => Some(("tarballGcsDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "yarn-application-id" => Some(("yarnApplicationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "yarn-application-ids" => Some(("yarnApplicationIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["diagnosis-interval", "end-time", "job", "jobs", "start-time", "tarball-access", "tarball-gcs-dir", "yarn-application-id", "yarn-application-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DiagnoseClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_diagnose(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
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

    async fn _projects_regions_clusters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_clusters_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
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

    async fn _projects_regions_clusters_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_clusters_inject_credentials(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "credentials-ciphertext" => Some(("credentialsCiphertext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-uuid", "credentials-ciphertext"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InjectCredentialsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_inject_credentials(request, opt.value_of("project").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster").unwrap_or(""));
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

    async fn _projects_regions_clusters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_clusters_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
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

    async fn _projects_regions_clusters_node_groups_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.disk-config.boot-disk-provisioned-iops" => Some(("nodeGroupConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.disk-config.boot-disk-provisioned-throughput" => Some(("nodeGroupConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.disk-config.boot-disk-size-gb" => Some(("nodeGroupConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-group-config.disk-config.boot-disk-type" => Some(("nodeGroupConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.disk-config.local-ssd-interface" => Some(("nodeGroupConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.disk-config.num-local-ssds" => Some(("nodeGroupConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-group-config.image-uri" => Some(("nodeGroupConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.instance-names" => Some(("nodeGroupConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-group-config.is-preemptible" => Some(("nodeGroupConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-group-config.machine-type-uri" => Some(("nodeGroupConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.managed-group-config.instance-group-manager-name" => Some(("nodeGroupConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.managed-group-config.instance-group-manager-uri" => Some(("nodeGroupConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.managed-group-config.instance-template-name" => Some(("nodeGroupConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.min-cpu-platform" => Some(("nodeGroupConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.min-num-instances" => Some(("nodeGroupConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-group-config.num-instances" => Some(("nodeGroupConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-group-config.preemptibility" => Some(("nodeGroupConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-group-config.startup-config.required-registration-fraction" => Some(("nodeGroupConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "roles" => Some(("roles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "disk-config", "image-uri", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "is-preemptible", "labels", "local-ssd-interface", "machine-type-uri", "managed-group-config", "min-cpu-platform", "min-num-instances", "name", "node-group-config", "num-instances", "num-local-ssds", "preemptibility", "required-registration-fraction", "roles", "startup-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::NodeGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_node_groups_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "parent-operation-id" => {
                    call = call.parent_operation_id(value.unwrap_or(""));
                },
                "node-group-id" => {
                    call = call.node_group_id(value.unwrap_or(""));
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
                                                                           v.extend(["node-group-id", "parent-operation-id", "request-id"].iter().map(|v|*v));
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

    async fn _projects_regions_clusters_node_groups_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_clusters_node_groups_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_clusters_node_groups_repair(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "instance-names" => Some(("instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "repair-action" => Some(("repairAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["instance-names", "repair-action", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RepairNodeGroupRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_node_groups_repair(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_clusters_node_groups_resize(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "graceful-decommission-timeout" => Some(("gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-operation-id" => Some(("parentOperationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["graceful-decommission-timeout", "parent-operation-id", "request-id", "size"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ResizeNodeGroupRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_node_groups_resize(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_clusters_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-name" => Some(("clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.autoscaling-config.policy-uri" => Some(("config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.config-bucket" => Some(("config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.encryption-config.gce-pd-kms-key-name" => Some(("config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.encryption-config.kms-key" => Some(("config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.endpoint-config.enable-http-port-access" => Some(("config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.endpoint-config.http-ports" => Some(("config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.internal-ip-only" => Some(("config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.metadata" => Some(("config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.gce-cluster-config.network-uri" => Some(("config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.private-ipv6-google-access" => Some(("config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.reservation-affinity.key" => Some(("config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.reservation-affinity.values" => Some(("config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.service-account" => Some(("config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.service-account-scopes" => Some(("config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.subnetwork-uri" => Some(("config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.tags" => Some(("config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.zone-uri" => Some(("config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster-config.gke-cluster-target" => Some(("config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.auto-delete-time" => Some(("config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.auto-delete-ttl" => Some(("config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.idle-delete-ttl" => Some(("config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.lifecycle-config.idle-start-time" => Some(("config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-size-gb" => Some(("config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-type" => Some(("config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.local-ssd-interface" => Some(("config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.num-local-ssds" => Some(("config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.image-uri" => Some(("config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.instance-names" => Some(("config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.master-config.is-preemptible" => Some(("config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.master-config.machine-type-uri" => Some(("config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-group-manager-name" => Some(("config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-group-manager-uri" => Some(("config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-template-name" => Some(("config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.min-cpu-platform" => Some(("config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.min-num-instances" => Some(("config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.num-instances" => Some(("config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.preemptibility" => Some(("config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.startup-config.required-registration-fraction" => Some(("config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.metastore-config.dataproc-metastore-service" => Some(("config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-type" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.num-local-ssds" => Some(("config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.image-uri" => Some(("config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.instance-names" => Some(("config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.secondary-worker-config.is-preemptible" => Some(("config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.machine-type-uri" => Some(("config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.min-cpu-platform" => Some(("config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.min-num-instances" => Some(("config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.num-instances" => Some(("config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.preemptibility" => Some(("config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.security-config.identity-config.user-service-account-mapping" => Some(("config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.enable-kerberos" => Some(("config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.kdc-db-key-uri" => Some(("config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.key-password-uri" => Some(("config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.keystore-password-uri" => Some(("config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.keystore-uri" => Some(("config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.kms-key-uri" => Some(("config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.realm" => Some(("config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.root-principal-password-uri" => Some(("config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.truststore-password-uri" => Some(("config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.security-config.kerberos-config.truststore-uri" => Some(("config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.image-version" => Some(("config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.optional-components" => Some(("config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.software-config.properties" => Some(("config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.temp-bucket" => Some(("config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-size-gb" => Some(("config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-type" => Some(("config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.local-ssd-interface" => Some(("config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.num-local-ssds" => Some(("config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.image-uri" => Some(("config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.instance-names" => Some(("config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.worker-config.is-preemptible" => Some(("config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.worker-config.machine-type-uri" => Some(("config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-group-manager-name" => Some(("config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-template-name" => Some(("config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.min-cpu-platform" => Some(("config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.min-num-instances" => Some(("config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.num-instances" => Some(("config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.preemptibility" => Some(("config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.startup-config.required-registration-fraction" => Some(("config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.hdfs-metrics" => Some(("metrics.hdfsMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.yarn-metrics" => Some(("metrics.yarnMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.detail" => Some(("status.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state-start-time" => Some(("status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.substate" => Some(("status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.auxiliary-services-config.metastore-config.dataproc-metastore-service" => Some(("virtualClusterConfig.auxiliaryServicesConfig.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.auxiliary-services-config.spark-history-server-config.dataproc-cluster" => Some(("virtualClusterConfig.auxiliaryServicesConfig.sparkHistoryServerConfig.dataprocCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.gke-cluster-config.gke-cluster-target" => Some(("virtualClusterConfig.kubernetesClusterConfig.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("virtualClusterConfig.kubernetesClusterConfig.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("virtualClusterConfig.kubernetesClusterConfig.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.kubernetes-namespace" => Some(("virtualClusterConfig.kubernetesClusterConfig.kubernetesNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "virtual-cluster-config.kubernetes-cluster-config.kubernetes-software-config.component-version" => Some(("virtualClusterConfig.kubernetesClusterConfig.kubernetesSoftwareConfig.componentVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "virtual-cluster-config.kubernetes-cluster-config.kubernetes-software-config.properties" => Some(("virtualClusterConfig.kubernetesClusterConfig.kubernetesSoftwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "virtual-cluster-config.staging-bucket" => Some(("virtualClusterConfig.stagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "auxiliary-services-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-name", "cluster-namespace", "cluster-uuid", "component-version", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dataproc-cluster", "dataproc-metastore-service", "detail", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "hdfs-metrics", "http-ports", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "kubernetes-cluster-config", "kubernetes-namespace", "kubernetes-software-config", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-group-config", "master-config", "metadata", "metastore-config", "metrics", "min-cpu-platform", "min-num-instances", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "policy-uri", "preemptibility", "private-ipv6-google-access", "project-id", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "spark-history-server-config", "staging-bucket", "startup-config", "state", "state-start-time", "status", "subnetwork-uri", "substate", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "user-service-account-mapping", "values", "virtual-cluster-config", "worker-config", "yarn-metrics", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Cluster = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "graceful-decommission-timeout" => {
                    call = call.graceful_decommission_timeout(        value.map(|v| arg_from_str(v, err, "graceful-decommission-timeout", "google-duration")).unwrap_or(chrono::Duration::seconds(0)));
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
                                                                           v.extend(["graceful-decommission-timeout", "request-id", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_regions_clusters_repair(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "graceful-decommission-timeout" => Some(("gracefulDecommissionTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-operation-id" => Some(("parentOperationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-uuid", "graceful-decommission-timeout", "parent-operation-id", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RepairClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_repair(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
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

    async fn _projects_regions_clusters_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_clusters_start(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-uuid", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::StartClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_start(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
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

    async fn _projects_regions_clusters_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-uuid", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::StopClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_clusters_stop(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
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

    async fn _projects_regions_clusters_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().regions_clusters_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::CancelJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_jobs_cancel(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
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

    async fn _projects_regions_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_jobs_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
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

    async fn _projects_regions_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_jobs_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
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

    async fn _projects_regions_jobs_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_jobs_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_jobs_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "job-state-matcher" => {
                    call = call.job_state_matcher(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "cluster-name" => {
                    call = call.cluster_name(value.unwrap_or(""));
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
                                                                           v.extend(["cluster-name", "filter", "job-state-matcher", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_regions_jobs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "done" => Some(("done", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "driver-control-files-uri" => Some(("driverControlFilesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "driver-output-resource-uri" => Some(("driverOutputResourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "driver-scheduling-config.memory-mb" => Some(("driverSchedulingConfig.memoryMb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "driver-scheduling-config.vcores" => Some(("driverSchedulingConfig.vcores", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flink-job.args" => Some(("flinkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "flink-job.jar-file-uris" => Some(("flinkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "flink-job.logging-config.driver-log-levels" => Some(("flinkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "flink-job.main-class" => Some(("flinkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flink-job.main-jar-file-uri" => Some(("flinkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flink-job.properties" => Some(("flinkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "flink-job.savepoint-uri" => Some(("flinkJob.savepointUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hadoop-job.archive-uris" => Some(("hadoopJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.args" => Some(("hadoopJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.file-uris" => Some(("hadoopJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.jar-file-uris" => Some(("hadoopJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.logging-config.driver-log-levels" => Some(("hadoopJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hadoop-job.main-class" => Some(("hadoopJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hadoop-job.main-jar-file-uri" => Some(("hadoopJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hadoop-job.properties" => Some(("hadoopJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hive-job.continue-on-failure" => Some(("hiveJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "hive-job.jar-file-uris" => Some(("hiveJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hive-job.properties" => Some(("hiveJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hive-job.query-file-uri" => Some(("hiveJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hive-job.query-list.queries" => Some(("hiveJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hive-job.script-variables" => Some(("hiveJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-uuid" => Some(("jobUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pig-job.continue-on-failure" => Some(("pigJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pig-job.jar-file-uris" => Some(("pigJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pig-job.logging-config.driver-log-levels" => Some(("pigJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pig-job.properties" => Some(("pigJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pig-job.query-file-uri" => Some(("pigJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pig-job.query-list.queries" => Some(("pigJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pig-job.script-variables" => Some(("pigJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-labels" => Some(("placement.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-name" => Some(("placement.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-uuid" => Some(("placement.clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "presto-job.client-tags" => Some(("prestoJob.clientTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "presto-job.continue-on-failure" => Some(("prestoJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "presto-job.logging-config.driver-log-levels" => Some(("prestoJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "presto-job.output-format" => Some(("prestoJob.outputFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "presto-job.properties" => Some(("prestoJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "presto-job.query-file-uri" => Some(("prestoJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "presto-job.query-list.queries" => Some(("prestoJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.archive-uris" => Some(("pysparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.args" => Some(("pysparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.file-uris" => Some(("pysparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.jar-file-uris" => Some(("pysparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.logging-config.driver-log-levels" => Some(("pysparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pyspark-job.main-python-file-uri" => Some(("pysparkJob.mainPythonFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pyspark-job.properties" => Some(("pysparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pyspark-job.python-file-uris" => Some(("pysparkJob.pythonFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "reference.job-id" => Some(("reference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reference.project-id" => Some(("reference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scheduling.max-failures-per-hour" => Some(("scheduling.maxFailuresPerHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "scheduling.max-failures-total" => Some(("scheduling.maxFailuresTotal", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "spark-job.archive-uris" => Some(("sparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.args" => Some(("sparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.file-uris" => Some(("sparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.jar-file-uris" => Some(("sparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.logging-config.driver-log-levels" => Some(("sparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-job.main-class" => Some(("sparkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-job.main-jar-file-uri" => Some(("sparkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-job.properties" => Some(("sparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-r-job.archive-uris" => Some(("sparkRJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-r-job.args" => Some(("sparkRJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-r-job.file-uris" => Some(("sparkRJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-r-job.logging-config.driver-log-levels" => Some(("sparkRJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-r-job.main-r-file-uri" => Some(("sparkRJob.mainRFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-r-job.properties" => Some(("sparkRJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-sql-job.jar-file-uris" => Some(("sparkSqlJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-sql-job.logging-config.driver-log-levels" => Some(("sparkSqlJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-sql-job.properties" => Some(("sparkSqlJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-sql-job.query-file-uri" => Some(("sparkSqlJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-sql-job.query-list.queries" => Some(("sparkSqlJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-sql-job.script-variables" => Some(("sparkSqlJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "status.details" => Some(("status.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state-start-time" => Some(("status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.substate" => Some(("status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trino-job.client-tags" => Some(("trinoJob.clientTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trino-job.continue-on-failure" => Some(("trinoJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trino-job.logging-config.driver-log-levels" => Some(("trinoJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trino-job.output-format" => Some(("trinoJob.outputFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trino-job.properties" => Some(("trinoJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trino-job.query-file-uri" => Some(("trinoJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trino-job.query-list.queries" => Some(("trinoJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "args", "client-tags", "cluster-labels", "cluster-name", "cluster-uuid", "continue-on-failure", "details", "done", "driver-control-files-uri", "driver-log-levels", "driver-output-resource-uri", "driver-scheduling-config", "file-uris", "flink-job", "hadoop-job", "hive-job", "jar-file-uris", "job-id", "job-uuid", "labels", "logging-config", "main-class", "main-jar-file-uri", "main-python-file-uri", "main-r-file-uri", "max-failures-per-hour", "max-failures-total", "memory-mb", "output-format", "pig-job", "placement", "presto-job", "project-id", "properties", "pyspark-job", "python-file-uris", "queries", "query-file-uri", "query-list", "reference", "savepoint-uri", "scheduling", "script-variables", "spark-job", "spark-r-job", "spark-sql-job", "state", "state-start-time", "status", "substate", "trino-job", "vcores"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Job = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_jobs_patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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

    async fn _projects_regions_jobs_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_jobs_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_jobs_submit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job.done" => Some(("job.done", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.driver-control-files-uri" => Some(("job.driverControlFilesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.driver-output-resource-uri" => Some(("job.driverOutputResourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.driver-scheduling-config.memory-mb" => Some(("job.driverSchedulingConfig.memoryMb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.driver-scheduling-config.vcores" => Some(("job.driverSchedulingConfig.vcores", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.flink-job.args" => Some(("job.flinkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.flink-job.jar-file-uris" => Some(("job.flinkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.flink-job.logging-config.driver-log-levels" => Some(("job.flinkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.flink-job.main-class" => Some(("job.flinkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.flink-job.main-jar-file-uri" => Some(("job.flinkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.flink-job.properties" => Some(("job.flinkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.flink-job.savepoint-uri" => Some(("job.flinkJob.savepointUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.archive-uris" => Some(("job.hadoopJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.args" => Some(("job.hadoopJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.file-uris" => Some(("job.hadoopJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.jar-file-uris" => Some(("job.hadoopJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.logging-config.driver-log-levels" => Some(("job.hadoopJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hadoop-job.main-class" => Some(("job.hadoopJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.main-jar-file-uri" => Some(("job.hadoopJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.properties" => Some(("job.hadoopJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hive-job.continue-on-failure" => Some(("job.hiveJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.hive-job.jar-file-uris" => Some(("job.hiveJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hive-job.properties" => Some(("job.hiveJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hive-job.query-file-uri" => Some(("job.hiveJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hive-job.query-list.queries" => Some(("job.hiveJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hive-job.script-variables" => Some(("job.hiveJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.job-uuid" => Some(("job.jobUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.labels" => Some(("job.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.continue-on-failure" => Some(("job.pigJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.pig-job.jar-file-uris" => Some(("job.pigJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pig-job.logging-config.driver-log-levels" => Some(("job.pigJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.properties" => Some(("job.pigJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.query-file-uri" => Some(("job.pigJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.pig-job.query-list.queries" => Some(("job.pigJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pig-job.script-variables" => Some(("job.pigJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.placement.cluster-labels" => Some(("job.placement.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.placement.cluster-name" => Some(("job.placement.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.placement.cluster-uuid" => Some(("job.placement.clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.presto-job.client-tags" => Some(("job.prestoJob.clientTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.presto-job.continue-on-failure" => Some(("job.prestoJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.presto-job.logging-config.driver-log-levels" => Some(("job.prestoJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.presto-job.output-format" => Some(("job.prestoJob.outputFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.presto-job.properties" => Some(("job.prestoJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.presto-job.query-file-uri" => Some(("job.prestoJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.presto-job.query-list.queries" => Some(("job.prestoJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.archive-uris" => Some(("job.pysparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.args" => Some(("job.pysparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.file-uris" => Some(("job.pysparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.jar-file-uris" => Some(("job.pysparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.logging-config.driver-log-levels" => Some(("job.pysparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pyspark-job.main-python-file-uri" => Some(("job.pysparkJob.mainPythonFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.pyspark-job.properties" => Some(("job.pysparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pyspark-job.python-file-uris" => Some(("job.pysparkJob.pythonFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.reference.job-id" => Some(("job.reference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.reference.project-id" => Some(("job.reference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.scheduling.max-failures-per-hour" => Some(("job.scheduling.maxFailuresPerHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.scheduling.max-failures-total" => Some(("job.scheduling.maxFailuresTotal", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.spark-job.archive-uris" => Some(("job.sparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.args" => Some(("job.sparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.file-uris" => Some(("job.sparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.jar-file-uris" => Some(("job.sparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.logging-config.driver-log-levels" => Some(("job.sparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-job.main-class" => Some(("job.sparkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.main-jar-file-uri" => Some(("job.sparkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.properties" => Some(("job.sparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-r-job.archive-uris" => Some(("job.sparkRJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-r-job.args" => Some(("job.sparkRJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-r-job.file-uris" => Some(("job.sparkRJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-r-job.logging-config.driver-log-levels" => Some(("job.sparkRJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-r-job.main-r-file-uri" => Some(("job.sparkRJob.mainRFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-r-job.properties" => Some(("job.sparkRJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.jar-file-uris" => Some(("job.sparkSqlJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-sql-job.logging-config.driver-log-levels" => Some(("job.sparkSqlJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.properties" => Some(("job.sparkSqlJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.query-file-uri" => Some(("job.sparkSqlJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-sql-job.query-list.queries" => Some(("job.sparkSqlJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-sql-job.script-variables" => Some(("job.sparkSqlJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.status.details" => Some(("job.status.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.state" => Some(("job.status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.state-start-time" => Some(("job.status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.substate" => Some(("job.status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.trino-job.client-tags" => Some(("job.trinoJob.clientTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.trino-job.continue-on-failure" => Some(("job.trinoJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.trino-job.logging-config.driver-log-levels" => Some(("job.trinoJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.trino-job.output-format" => Some(("job.trinoJob.outputFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.trino-job.properties" => Some(("job.trinoJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.trino-job.query-file-uri" => Some(("job.trinoJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.trino-job.query-list.queries" => Some(("job.trinoJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "args", "client-tags", "cluster-labels", "cluster-name", "cluster-uuid", "continue-on-failure", "details", "done", "driver-control-files-uri", "driver-log-levels", "driver-output-resource-uri", "driver-scheduling-config", "file-uris", "flink-job", "hadoop-job", "hive-job", "jar-file-uris", "job", "job-id", "job-uuid", "labels", "logging-config", "main-class", "main-jar-file-uri", "main-python-file-uri", "main-r-file-uri", "max-failures-per-hour", "max-failures-total", "memory-mb", "output-format", "pig-job", "placement", "presto-job", "project-id", "properties", "pyspark-job", "python-file-uris", "queries", "query-file-uri", "query-list", "reference", "request-id", "savepoint-uri", "scheduling", "script-variables", "spark-job", "spark-r-job", "spark-sql-job", "state", "state-start-time", "status", "substate", "trino-job", "vcores"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SubmitJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_jobs_submit(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
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

    async fn _projects_regions_jobs_submit_as_operation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job.done" => Some(("job.done", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.driver-control-files-uri" => Some(("job.driverControlFilesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.driver-output-resource-uri" => Some(("job.driverOutputResourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.driver-scheduling-config.memory-mb" => Some(("job.driverSchedulingConfig.memoryMb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.driver-scheduling-config.vcores" => Some(("job.driverSchedulingConfig.vcores", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.flink-job.args" => Some(("job.flinkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.flink-job.jar-file-uris" => Some(("job.flinkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.flink-job.logging-config.driver-log-levels" => Some(("job.flinkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.flink-job.main-class" => Some(("job.flinkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.flink-job.main-jar-file-uri" => Some(("job.flinkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.flink-job.properties" => Some(("job.flinkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.flink-job.savepoint-uri" => Some(("job.flinkJob.savepointUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.archive-uris" => Some(("job.hadoopJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.args" => Some(("job.hadoopJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.file-uris" => Some(("job.hadoopJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.jar-file-uris" => Some(("job.hadoopJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.logging-config.driver-log-levels" => Some(("job.hadoopJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hadoop-job.main-class" => Some(("job.hadoopJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.main-jar-file-uri" => Some(("job.hadoopJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.properties" => Some(("job.hadoopJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hive-job.continue-on-failure" => Some(("job.hiveJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.hive-job.jar-file-uris" => Some(("job.hiveJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hive-job.properties" => Some(("job.hiveJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hive-job.query-file-uri" => Some(("job.hiveJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hive-job.query-list.queries" => Some(("job.hiveJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hive-job.script-variables" => Some(("job.hiveJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.job-uuid" => Some(("job.jobUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.labels" => Some(("job.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.continue-on-failure" => Some(("job.pigJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.pig-job.jar-file-uris" => Some(("job.pigJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pig-job.logging-config.driver-log-levels" => Some(("job.pigJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.properties" => Some(("job.pigJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.query-file-uri" => Some(("job.pigJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.pig-job.query-list.queries" => Some(("job.pigJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pig-job.script-variables" => Some(("job.pigJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.placement.cluster-labels" => Some(("job.placement.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.placement.cluster-name" => Some(("job.placement.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.placement.cluster-uuid" => Some(("job.placement.clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.presto-job.client-tags" => Some(("job.prestoJob.clientTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.presto-job.continue-on-failure" => Some(("job.prestoJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.presto-job.logging-config.driver-log-levels" => Some(("job.prestoJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.presto-job.output-format" => Some(("job.prestoJob.outputFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.presto-job.properties" => Some(("job.prestoJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.presto-job.query-file-uri" => Some(("job.prestoJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.presto-job.query-list.queries" => Some(("job.prestoJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.archive-uris" => Some(("job.pysparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.args" => Some(("job.pysparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.file-uris" => Some(("job.pysparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.jar-file-uris" => Some(("job.pysparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.logging-config.driver-log-levels" => Some(("job.pysparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pyspark-job.main-python-file-uri" => Some(("job.pysparkJob.mainPythonFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.pyspark-job.properties" => Some(("job.pysparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pyspark-job.python-file-uris" => Some(("job.pysparkJob.pythonFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.reference.job-id" => Some(("job.reference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.reference.project-id" => Some(("job.reference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.scheduling.max-failures-per-hour" => Some(("job.scheduling.maxFailuresPerHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.scheduling.max-failures-total" => Some(("job.scheduling.maxFailuresTotal", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.spark-job.archive-uris" => Some(("job.sparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.args" => Some(("job.sparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.file-uris" => Some(("job.sparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.jar-file-uris" => Some(("job.sparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.logging-config.driver-log-levels" => Some(("job.sparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-job.main-class" => Some(("job.sparkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.main-jar-file-uri" => Some(("job.sparkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.properties" => Some(("job.sparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-r-job.archive-uris" => Some(("job.sparkRJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-r-job.args" => Some(("job.sparkRJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-r-job.file-uris" => Some(("job.sparkRJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-r-job.logging-config.driver-log-levels" => Some(("job.sparkRJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-r-job.main-r-file-uri" => Some(("job.sparkRJob.mainRFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-r-job.properties" => Some(("job.sparkRJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.jar-file-uris" => Some(("job.sparkSqlJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-sql-job.logging-config.driver-log-levels" => Some(("job.sparkSqlJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.properties" => Some(("job.sparkSqlJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.query-file-uri" => Some(("job.sparkSqlJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-sql-job.query-list.queries" => Some(("job.sparkSqlJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-sql-job.script-variables" => Some(("job.sparkSqlJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.status.details" => Some(("job.status.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.state" => Some(("job.status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.state-start-time" => Some(("job.status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.substate" => Some(("job.status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.trino-job.client-tags" => Some(("job.trinoJob.clientTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.trino-job.continue-on-failure" => Some(("job.trinoJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.trino-job.logging-config.driver-log-levels" => Some(("job.trinoJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.trino-job.output-format" => Some(("job.trinoJob.outputFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.trino-job.properties" => Some(("job.trinoJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.trino-job.query-file-uri" => Some(("job.trinoJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.trino-job.query-list.queries" => Some(("job.trinoJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "args", "client-tags", "cluster-labels", "cluster-name", "cluster-uuid", "continue-on-failure", "details", "done", "driver-control-files-uri", "driver-log-levels", "driver-output-resource-uri", "driver-scheduling-config", "file-uris", "flink-job", "hadoop-job", "hive-job", "jar-file-uris", "job", "job-id", "job-uuid", "labels", "logging-config", "main-class", "main-jar-file-uri", "main-python-file-uri", "main-r-file-uri", "max-failures-per-hour", "max-failures-total", "memory-mb", "output-format", "pig-job", "placement", "presto-job", "project-id", "properties", "pyspark-job", "python-file-uris", "queries", "query-file-uri", "query-list", "reference", "request-id", "savepoint-uri", "scheduling", "script-variables", "spark-job", "spark-r-job", "spark-sql-job", "state", "state-start-time", "status", "substate", "trino-job", "vcores"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SubmitJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_jobs_submit_as_operation(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
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

    async fn _projects_regions_jobs_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().regions_jobs_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_operations_cancel(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_operations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_operations_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_operations_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_operations_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_operations_list(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_operations_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_operations_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_operations_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().regions_operations_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dag-timeout" => Some(("dagTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-config.kms-key" => Some(("encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-selector.cluster-labels" => Some(("placement.clusterSelector.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-selector.zone" => Some(("placement.clusterSelector.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.cluster-name" => Some(("placement.managedCluster.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.autoscaling-config.policy-uri" => Some(("placement.managedCluster.config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.config-bucket" => Some(("placement.managedCluster.config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.gce-pd-kms-key-name" => Some(("placement.managedCluster.config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.kms-key" => Some(("placement.managedCluster.config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.enable-http-port-access" => Some(("placement.managedCluster.config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.http-ports" => Some(("placement.managedCluster.config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("placement.managedCluster.config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.internal-ip-only" => Some(("placement.managedCluster.config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.metadata" => Some(("placement.managedCluster.config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.network-uri" => Some(("placement.managedCluster.config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("placement.managedCluster.config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.private-ipv6-google-access" => Some(("placement.managedCluster.config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.key" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.values" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account-scopes" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.subnetwork-uri" => Some(("placement.managedCluster.config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.tags" => Some(("placement.managedCluster.config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.zone-uri" => Some(("placement.managedCluster.config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.gke-cluster-target" => Some(("placement.managedCluster.config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-time" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-start-time" => Some(("placement.managedCluster.config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.image-uri" => Some(("placement.managedCluster.config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.instance-names" => Some(("placement.managedCluster.config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.master-config.is-preemptible" => Some(("placement.managedCluster.config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.machine-type-uri" => Some(("placement.managedCluster.config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-cpu-platform" => Some(("placement.managedCluster.config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-num-instances" => Some(("placement.managedCluster.config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.num-instances" => Some(("placement.managedCluster.config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.preemptibility" => Some(("placement.managedCluster.config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.metastore-config.dataproc-metastore-service" => Some(("placement.managedCluster.config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.image-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.instance-names" => Some(("placement.managedCluster.config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.secondary-worker-config.is-preemptible" => Some(("placement.managedCluster.config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.machine-type-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.preemptibility" => Some(("placement.managedCluster.config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.identity-config.user-service-account-mapping" => Some(("placement.managedCluster.config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.enable-kerberos" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kdc-db-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.key-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kms-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.root-principal-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.image-version" => Some(("placement.managedCluster.config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.optional-components" => Some(("placement.managedCluster.config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.software-config.properties" => Some(("placement.managedCluster.config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.temp-bucket" => Some(("placement.managedCluster.config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.image-uri" => Some(("placement.managedCluster.config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.instance-names" => Some(("placement.managedCluster.config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.worker-config.is-preemptible" => Some(("placement.managedCluster.config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.machine-type-uri" => Some(("placement.managedCluster.config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-num-instances" => Some(("placement.managedCluster.config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.num-instances" => Some(("placement.managedCluster.config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.preemptibility" => Some(("placement.managedCluster.config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.labels" => Some(("placement.managedCluster.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-labels", "cluster-name", "cluster-namespace", "cluster-selector", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "create-time", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dag-timeout", "dataproc-metastore-service", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "http-ports", "id", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-cluster", "managed-group-config", "master-config", "metadata", "metastore-config", "min-cpu-platform", "min-num-instances", "name", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "placement", "policy-uri", "preemptibility", "private-ipv6-google-access", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "startup-config", "subnetwork-uri", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "update-time", "user-service-account-mapping", "values", "version", "worker-config", "zone", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkflowTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_workflow_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_workflow_templates_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(        value.map(|v| arg_from_str(v, err, "version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["version"].iter().map(|v|*v));
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

    async fn _projects_regions_workflow_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_workflow_templates_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(        value.map(|v| arg_from_str(v, err, "version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["version"].iter().map(|v|*v));
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

    async fn _projects_regions_workflow_templates_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_workflow_templates_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_instantiate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "parameters" => Some(("parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["parameters", "request-id", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InstantiateWorkflowTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_workflow_templates_instantiate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_instantiate_inline(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dag-timeout" => Some(("dagTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-config.kms-key" => Some(("encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-selector.cluster-labels" => Some(("placement.clusterSelector.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-selector.zone" => Some(("placement.clusterSelector.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.cluster-name" => Some(("placement.managedCluster.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.autoscaling-config.policy-uri" => Some(("placement.managedCluster.config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.config-bucket" => Some(("placement.managedCluster.config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.gce-pd-kms-key-name" => Some(("placement.managedCluster.config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.kms-key" => Some(("placement.managedCluster.config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.enable-http-port-access" => Some(("placement.managedCluster.config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.http-ports" => Some(("placement.managedCluster.config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("placement.managedCluster.config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.internal-ip-only" => Some(("placement.managedCluster.config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.metadata" => Some(("placement.managedCluster.config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.network-uri" => Some(("placement.managedCluster.config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("placement.managedCluster.config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.private-ipv6-google-access" => Some(("placement.managedCluster.config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.key" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.values" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account-scopes" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.subnetwork-uri" => Some(("placement.managedCluster.config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.tags" => Some(("placement.managedCluster.config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.zone-uri" => Some(("placement.managedCluster.config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.gke-cluster-target" => Some(("placement.managedCluster.config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-time" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-start-time" => Some(("placement.managedCluster.config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.image-uri" => Some(("placement.managedCluster.config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.instance-names" => Some(("placement.managedCluster.config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.master-config.is-preemptible" => Some(("placement.managedCluster.config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.machine-type-uri" => Some(("placement.managedCluster.config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-cpu-platform" => Some(("placement.managedCluster.config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-num-instances" => Some(("placement.managedCluster.config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.num-instances" => Some(("placement.managedCluster.config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.preemptibility" => Some(("placement.managedCluster.config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.metastore-config.dataproc-metastore-service" => Some(("placement.managedCluster.config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.image-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.instance-names" => Some(("placement.managedCluster.config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.secondary-worker-config.is-preemptible" => Some(("placement.managedCluster.config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.machine-type-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.preemptibility" => Some(("placement.managedCluster.config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.identity-config.user-service-account-mapping" => Some(("placement.managedCluster.config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.enable-kerberos" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kdc-db-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.key-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kms-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.root-principal-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.image-version" => Some(("placement.managedCluster.config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.optional-components" => Some(("placement.managedCluster.config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.software-config.properties" => Some(("placement.managedCluster.config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.temp-bucket" => Some(("placement.managedCluster.config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.image-uri" => Some(("placement.managedCluster.config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.instance-names" => Some(("placement.managedCluster.config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.worker-config.is-preemptible" => Some(("placement.managedCluster.config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.machine-type-uri" => Some(("placement.managedCluster.config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-num-instances" => Some(("placement.managedCluster.config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.num-instances" => Some(("placement.managedCluster.config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.preemptibility" => Some(("placement.managedCluster.config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.labels" => Some(("placement.managedCluster.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-labels", "cluster-name", "cluster-namespace", "cluster-selector", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "create-time", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dag-timeout", "dataproc-metastore-service", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "http-ports", "id", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-cluster", "managed-group-config", "master-config", "metadata", "metastore-config", "min-cpu-platform", "min-num-instances", "name", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "placement", "policy-uri", "preemptibility", "private-ipv6-google-access", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "startup-config", "subnetwork-uri", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "update-time", "user-service-account-mapping", "values", "version", "worker-config", "zone", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkflowTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_workflow_templates_instantiate_inline(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_workflow_templates_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_workflow_templates_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().regions_workflow_templates_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_regions_workflow_templates_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dag-timeout" => Some(("dagTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-config.kms-key" => Some(("encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-selector.cluster-labels" => Some(("placement.clusterSelector.clusterLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-selector.zone" => Some(("placement.clusterSelector.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.cluster-name" => Some(("placement.managedCluster.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.autoscaling-config.policy-uri" => Some(("placement.managedCluster.config.autoscalingConfig.policyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.config-bucket" => Some(("placement.managedCluster.config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.gce-pd-kms-key-name" => Some(("placement.managedCluster.config.encryptionConfig.gcePdKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.encryption-config.kms-key" => Some(("placement.managedCluster.config.encryptionConfig.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.enable-http-port-access" => Some(("placement.managedCluster.config.endpointConfig.enableHttpPortAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.endpoint-config.http-ports" => Some(("placement.managedCluster.config.endpointConfig.httpPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.confidential-instance-config.enable-confidential-compute" => Some(("placement.managedCluster.config.gceClusterConfig.confidentialInstanceConfig.enableConfidentialCompute", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.internal-ip-only" => Some(("placement.managedCluster.config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.metadata" => Some(("placement.managedCluster.config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.gce-cluster-config.network-uri" => Some(("placement.managedCluster.config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.node-group-affinity.node-group-uri" => Some(("placement.managedCluster.config.gceClusterConfig.nodeGroupAffinity.nodeGroupUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.private-ipv6-google-access" => Some(("placement.managedCluster.config.gceClusterConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.consume-reservation-type" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.key" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.reservation-affinity.values" => Some(("placement.managedCluster.config.gceClusterConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.service-account-scopes" => Some(("placement.managedCluster.config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-integrity-monitoring" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-secure-boot" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.shielded-instance-config.enable-vtpm" => Some(("placement.managedCluster.config.gceClusterConfig.shieldedInstanceConfig.enableVtpm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.subnetwork-uri" => Some(("placement.managedCluster.config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gce-cluster-config.tags" => Some(("placement.managedCluster.config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.gce-cluster-config.zone-uri" => Some(("placement.managedCluster.config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.gke-cluster-target" => Some(("placement.managedCluster.config.gkeClusterConfig.gkeClusterTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.cluster-namespace" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.clusterNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.gke-cluster-config.namespaced-gke-deployment-target.target-gke-cluster" => Some(("placement.managedCluster.config.gkeClusterConfig.namespacedGkeDeploymentTarget.targetGkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-time" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.auto-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.autoDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-delete-ttl" => Some(("placement.managedCluster.config.lifecycleConfig.idleDeleteTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.lifecycle-config.idle-start-time" => Some(("placement.managedCluster.config.lifecycleConfig.idleStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.masterConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.image-uri" => Some(("placement.managedCluster.config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.instance-names" => Some(("placement.managedCluster.config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.master-config.is-preemptible" => Some(("placement.managedCluster.config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.machine-type-uri" => Some(("placement.managedCluster.config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-cpu-platform" => Some(("placement.managedCluster.config.masterConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.min-num-instances" => Some(("placement.managedCluster.config.masterConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.num-instances" => Some(("placement.managedCluster.config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.preemptibility" => Some(("placement.managedCluster.config.masterConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.master-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.masterConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.metastore-config.dataproc-metastore-service" => Some(("placement.managedCluster.config.metastoreConfig.dataprocMetastoreService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.image-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.instance-names" => Some(("placement.managedCluster.config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.secondary-worker-config.is-preemptible" => Some(("placement.managedCluster.config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.machine-type-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.min-num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.num-instances" => Some(("placement.managedCluster.config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.preemptibility" => Some(("placement.managedCluster.config.secondaryWorkerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.secondary-worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.secondaryWorkerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.identity-config.user-service-account-mapping" => Some(("placement.managedCluster.config.securityConfig.identityConfig.userServiceAccountMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-admin-server" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustAdminServer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-kdc" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustKdc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustRealm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.cross-realm-trust-shared-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.crossRealmTrustSharedPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.enable-kerberos" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.enableKerberos", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kdc-db-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kdcDbKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.key-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keyPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.keystore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.keystoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.kms-key-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.kmsKeyUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.realm" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.realm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.root-principal-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.rootPrincipalPasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.tgt-lifetime-hours" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.tgtLifetimeHours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-password-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststorePasswordUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.security-config.kerberos-config.truststore-uri" => Some(("placement.managedCluster.config.securityConfig.kerberosConfig.truststoreUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.image-version" => Some(("placement.managedCluster.config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.software-config.optional-components" => Some(("placement.managedCluster.config.softwareConfig.optionalComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.software-config.properties" => Some(("placement.managedCluster.config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.managed-cluster.config.temp-bucket" => Some(("placement.managedCluster.config.tempBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-iops" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-provisioned-throughput" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-size-gb" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.boot-disk-type" => Some(("placement.managedCluster.config.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.local-ssd-interface" => Some(("placement.managedCluster.config.workerConfig.diskConfig.localSsdInterface", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.disk-config.num-local-ssds" => Some(("placement.managedCluster.config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.image-uri" => Some(("placement.managedCluster.config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.instance-names" => Some(("placement.managedCluster.config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "placement.managed-cluster.config.worker-config.is-preemptible" => Some(("placement.managedCluster.config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.machine-type-uri" => Some(("placement.managedCluster.config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-group-manager-uri" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceGroupManagerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.managed-group-config.instance-template-name" => Some(("placement.managedCluster.config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-cpu-platform" => Some(("placement.managedCluster.config.workerConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.min-num-instances" => Some(("placement.managedCluster.config.workerConfig.minNumInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.num-instances" => Some(("placement.managedCluster.config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.preemptibility" => Some(("placement.managedCluster.config.workerConfig.preemptibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.config.worker-config.startup-config.required-registration-fraction" => Some(("placement.managedCluster.config.workerConfig.startupConfig.requiredRegistrationFraction", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "placement.managed-cluster.labels" => Some(("placement.managedCluster.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-delete-time", "auto-delete-ttl", "autoscaling-config", "boot-disk-provisioned-iops", "boot-disk-provisioned-throughput", "boot-disk-size-gb", "boot-disk-type", "cluster-labels", "cluster-name", "cluster-namespace", "cluster-selector", "confidential-instance-config", "config", "config-bucket", "consume-reservation-type", "create-time", "cross-realm-trust-admin-server", "cross-realm-trust-kdc", "cross-realm-trust-realm", "cross-realm-trust-shared-password-uri", "dag-timeout", "dataproc-metastore-service", "disk-config", "enable-confidential-compute", "enable-http-port-access", "enable-integrity-monitoring", "enable-kerberos", "enable-secure-boot", "enable-vtpm", "encryption-config", "endpoint-config", "gce-cluster-config", "gce-pd-kms-key-name", "gke-cluster-config", "gke-cluster-target", "http-ports", "id", "identity-config", "idle-delete-ttl", "idle-start-time", "image-uri", "image-version", "instance-group-manager-name", "instance-group-manager-uri", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "kdc-db-key-uri", "kerberos-config", "key", "key-password-uri", "keystore-password-uri", "keystore-uri", "kms-key", "kms-key-uri", "labels", "lifecycle-config", "local-ssd-interface", "machine-type-uri", "managed-cluster", "managed-group-config", "master-config", "metadata", "metastore-config", "min-cpu-platform", "min-num-instances", "name", "namespaced-gke-deployment-target", "network-uri", "node-group-affinity", "node-group-uri", "num-instances", "num-local-ssds", "optional-components", "placement", "policy-uri", "preemptibility", "private-ipv6-google-access", "properties", "realm", "required-registration-fraction", "reservation-affinity", "root-principal-password-uri", "secondary-worker-config", "security-config", "service-account", "service-account-scopes", "shielded-instance-config", "software-config", "startup-config", "subnetwork-uri", "tags", "target-gke-cluster", "temp-bucket", "tgt-lifetime-hours", "truststore-password-uri", "truststore-uri", "update-time", "user-service-account-mapping", "values", "version", "worker-config", "zone", "zone-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkflowTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().regions_workflow_templates_update(request, opt.value_of("name").unwrap_or(""));
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
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-autoscaling-policies-create", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-delete", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-get", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-list", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-autoscaling-policies-update", Some(opt)) => {
                        call_result = self._projects_locations_autoscaling_policies_update(opt, dry_run, &mut err).await;
                    },
                    ("locations-batches-analyze", Some(opt)) => {
                        call_result = self._projects_locations_batches_analyze(opt, dry_run, &mut err).await;
                    },
                    ("locations-batches-create", Some(opt)) => {
                        call_result = self._projects_locations_batches_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-batches-delete", Some(opt)) => {
                        call_result = self._projects_locations_batches_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-batches-get", Some(opt)) => {
                        call_result = self._projects_locations_batches_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-batches-list", Some(opt)) => {
                        call_result = self._projects_locations_batches_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err).await;
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
                    ("locations-session-templates-create", Some(opt)) => {
                        call_result = self._projects_locations_session_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-session-templates-delete", Some(opt)) => {
                        call_result = self._projects_locations_session_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-session-templates-get", Some(opt)) => {
                        call_result = self._projects_locations_session_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-session-templates-list", Some(opt)) => {
                        call_result = self._projects_locations_session_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-session-templates-patch", Some(opt)) => {
                        call_result = self._projects_locations_session_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-sessions-create", Some(opt)) => {
                        call_result = self._projects_locations_sessions_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-sessions-delete", Some(opt)) => {
                        call_result = self._projects_locations_sessions_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-sessions-get", Some(opt)) => {
                        call_result = self._projects_locations_sessions_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-sessions-list", Some(opt)) => {
                        call_result = self._projects_locations_sessions_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-sessions-terminate", Some(opt)) => {
                        call_result = self._projects_locations_sessions_terminate(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-create", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-delete", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-get", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-instantiate", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_instantiate(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-instantiate-inline", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_instantiate_inline(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-list", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-workflow-templates-update", Some(opt)) => {
                        call_result = self._projects_locations_workflow_templates_update(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-create", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_create(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-delete", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_delete(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-get", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_get(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-list", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_list(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("regions-autoscaling-policies-update", Some(opt)) => {
                        call_result = self._projects_regions_autoscaling_policies_update(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-create", Some(opt)) => {
                        call_result = self._projects_regions_clusters_create(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-delete", Some(opt)) => {
                        call_result = self._projects_regions_clusters_delete(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-diagnose", Some(opt)) => {
                        call_result = self._projects_regions_clusters_diagnose(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-get", Some(opt)) => {
                        call_result = self._projects_regions_clusters_get(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_clusters_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-inject-credentials", Some(opt)) => {
                        call_result = self._projects_regions_clusters_inject_credentials(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-list", Some(opt)) => {
                        call_result = self._projects_regions_clusters_list(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-node-groups-create", Some(opt)) => {
                        call_result = self._projects_regions_clusters_node_groups_create(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-node-groups-get", Some(opt)) => {
                        call_result = self._projects_regions_clusters_node_groups_get(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-node-groups-repair", Some(opt)) => {
                        call_result = self._projects_regions_clusters_node_groups_repair(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-node-groups-resize", Some(opt)) => {
                        call_result = self._projects_regions_clusters_node_groups_resize(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-patch", Some(opt)) => {
                        call_result = self._projects_regions_clusters_patch(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-repair", Some(opt)) => {
                        call_result = self._projects_regions_clusters_repair(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_clusters_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-start", Some(opt)) => {
                        call_result = self._projects_regions_clusters_start(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-stop", Some(opt)) => {
                        call_result = self._projects_regions_clusters_stop(opt, dry_run, &mut err).await;
                    },
                    ("regions-clusters-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_regions_clusters_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-cancel", Some(opt)) => {
                        call_result = self._projects_regions_jobs_cancel(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-delete", Some(opt)) => {
                        call_result = self._projects_regions_jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-get", Some(opt)) => {
                        call_result = self._projects_regions_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_jobs_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-list", Some(opt)) => {
                        call_result = self._projects_regions_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-patch", Some(opt)) => {
                        call_result = self._projects_regions_jobs_patch(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_jobs_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-submit", Some(opt)) => {
                        call_result = self._projects_regions_jobs_submit(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-submit-as-operation", Some(opt)) => {
                        call_result = self._projects_regions_jobs_submit_as_operation(opt, dry_run, &mut err).await;
                    },
                    ("regions-jobs-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_regions_jobs_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-cancel", Some(opt)) => {
                        call_result = self._projects_regions_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-delete", Some(opt)) => {
                        call_result = self._projects_regions_operations_delete(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-get", Some(opt)) => {
                        call_result = self._projects_regions_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_operations_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-list", Some(opt)) => {
                        call_result = self._projects_regions_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_operations_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-operations-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_regions_operations_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-create", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-delete", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-get", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-instantiate", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_instantiate(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-instantiate-inline", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_instantiate_inline(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-list", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("regions-workflow-templates-update", Some(opt)) => {
                        call_result = self._projects_regions_workflow_templates_update(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "dataproc1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/dataproc1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Dataproc::new(client, auth),
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
        ("projects", "methods: 'locations-autoscaling-policies-create', 'locations-autoscaling-policies-delete', 'locations-autoscaling-policies-get', 'locations-autoscaling-policies-get-iam-policy', 'locations-autoscaling-policies-list', 'locations-autoscaling-policies-set-iam-policy', 'locations-autoscaling-policies-test-iam-permissions', 'locations-autoscaling-policies-update', 'locations-batches-analyze', 'locations-batches-create', 'locations-batches-delete', 'locations-batches-get', 'locations-batches-list', 'locations-operations-cancel', 'locations-operations-delete', 'locations-operations-get', 'locations-operations-list', 'locations-session-templates-create', 'locations-session-templates-delete', 'locations-session-templates-get', 'locations-session-templates-list', 'locations-session-templates-patch', 'locations-sessions-create', 'locations-sessions-delete', 'locations-sessions-get', 'locations-sessions-list', 'locations-sessions-terminate', 'locations-workflow-templates-create', 'locations-workflow-templates-delete', 'locations-workflow-templates-get', 'locations-workflow-templates-get-iam-policy', 'locations-workflow-templates-instantiate', 'locations-workflow-templates-instantiate-inline', 'locations-workflow-templates-list', 'locations-workflow-templates-set-iam-policy', 'locations-workflow-templates-test-iam-permissions', 'locations-workflow-templates-update', 'regions-autoscaling-policies-create', 'regions-autoscaling-policies-delete', 'regions-autoscaling-policies-get', 'regions-autoscaling-policies-get-iam-policy', 'regions-autoscaling-policies-list', 'regions-autoscaling-policies-set-iam-policy', 'regions-autoscaling-policies-test-iam-permissions', 'regions-autoscaling-policies-update', 'regions-clusters-create', 'regions-clusters-delete', 'regions-clusters-diagnose', 'regions-clusters-get', 'regions-clusters-get-iam-policy', 'regions-clusters-inject-credentials', 'regions-clusters-list', 'regions-clusters-node-groups-create', 'regions-clusters-node-groups-get', 'regions-clusters-node-groups-repair', 'regions-clusters-node-groups-resize', 'regions-clusters-patch', 'regions-clusters-repair', 'regions-clusters-set-iam-policy', 'regions-clusters-start', 'regions-clusters-stop', 'regions-clusters-test-iam-permissions', 'regions-jobs-cancel', 'regions-jobs-delete', 'regions-jobs-get', 'regions-jobs-get-iam-policy', 'regions-jobs-list', 'regions-jobs-patch', 'regions-jobs-set-iam-policy', 'regions-jobs-submit', 'regions-jobs-submit-as-operation', 'regions-jobs-test-iam-permissions', 'regions-operations-cancel', 'regions-operations-delete', 'regions-operations-get', 'regions-operations-get-iam-policy', 'regions-operations-list', 'regions-operations-set-iam-policy', 'regions-operations-test-iam-permissions', 'regions-workflow-templates-create', 'regions-workflow-templates-delete', 'regions-workflow-templates-get', 'regions-workflow-templates-get-iam-policy', 'regions-workflow-templates-instantiate', 'regions-workflow-templates-instantiate-inline', 'regions-workflow-templates-list', 'regions-workflow-templates-set-iam-policy', 'regions-workflow-templates-test-iam-permissions' and 'regions-workflow-templates-update'", vec![
            ("locations-autoscaling-policies-create",
                    Some(r##"Creates new autoscaling policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("locations-autoscaling-policies-delete",
                    Some(r##"Deletes an autoscaling policy. It is an error to delete an autoscaling policy that is in use by one or more clusters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"##),
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
            ("locations-autoscaling-policies-get",
                    Some(r##"Retrieves autoscaling policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"##),
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
            ("locations-autoscaling-policies-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-autoscaling-policies-list",
                    Some(r##"Lists autoscaling policies in the project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("locations-autoscaling-policies-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-autoscaling-policies-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-autoscaling-policies-update",
                    Some(r##"Updates (replaces) autoscaling policy.Disabled check for update_mask, because all updates will be full replacements."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-autoscaling-policies-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"##),
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
            ("locations-batches-analyze",
                    Some(r##"Analyze a Batch for possible recommendations and insights."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-batches-analyze",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the batch to analyze in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/batches/BATCH_ID""##),
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
            ("locations-batches-create",
                    Some(r##"Creates a batch workload that executes asynchronously."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-batches-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this batch will be created."##),
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
            ("locations-batches-delete",
                    Some(r##"Deletes the batch workload resource. If the batch is not in a CANCELLED, SUCCEEDED or FAILED State, the delete operation fails and the response returns FAILED_PRECONDITION."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-batches-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the batch to retrieve in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/batches/BATCH_ID""##),
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
            ("locations-batches-get",
                    Some(r##"Gets the batch workload resource representation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-batches-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified name of the batch to retrieve in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/batches/BATCH_ID""##),
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
            ("locations-batches-list",
                    Some(r##"Lists batch workloads."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-batches-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent, which owns this collection of batches."##),
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-operations-delete",
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
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-operations-get",
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
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("locations-session-templates-create",
                    Some(r##"Create a session template synchronously."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-session-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this session template will be created."##),
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
            ("locations-session-templates-delete",
                    Some(r##"Deletes a session template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-session-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session template resource to delete."##),
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
            ("locations-session-templates-get",
                    Some(r##"Gets the resource representation for a session template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-session-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session template to retrieve."##),
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
            ("locations-session-templates-list",
                    Some(r##"Lists session templates."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-session-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent that owns this collection of session templates."##),
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
            ("locations-session-templates-patch",
                    Some(r##"Updates the session template synchronously."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-session-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the session template."##),
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
            ("locations-sessions-create",
                    Some(r##"Create an interactive session asynchronously."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-sessions-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this session will be created."##),
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
            ("locations-sessions-delete",
                    Some(r##"Deletes the interactive session resource. If the session is not in terminal state, it is terminated, and then deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-sessions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session resource to delete."##),
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
            ("locations-sessions-get",
                    Some(r##"Gets the resource representation for an interactive session."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-sessions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session to retrieve."##),
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
            ("locations-sessions-list",
                    Some(r##"Lists interactive sessions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-sessions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent, which owns this collection of sessions."##),
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
            ("locations-sessions-terminate",
                    Some(r##"Terminates the interactive session."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-sessions-terminate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session resource to terminate."##),
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
            ("locations-workflow-templates-create",
                    Some(r##"Creates new workflow template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("locations-workflow-templates-delete",
                    Some(r##"Deletes a workflow template. It does not cancel in-progress workflows."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.delete, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("locations-workflow-templates-get",
                    Some(r##"Retrieves the latest workflow template.Can retrieve previously instantiated template by specifying optional version parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("locations-workflow-templates-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-workflow-templates-instantiate",
                    Some(r##"Instantiates a template and begins execution.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-instantiate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("locations-workflow-templates-instantiate-inline",
                    Some(r##"Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-instantiate-inline",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,instantiateinline, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.instantiateinline, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("locations-workflow-templates-list",
                    Some(r##"Lists workflows that match the specified filter in the request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("locations-workflow-templates-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-workflow-templates-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-workflow-templates-update",
                    Some(r##"Updates (replaces) workflow template. The updated template must contain version that matches the current server version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_locations-workflow-templates-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("regions-autoscaling-policies-create",
                    Some(r##"Creates new autoscaling policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("regions-autoscaling-policies-delete",
                    Some(r##"Deletes an autoscaling policy. It is an error to delete an autoscaling policy that is in use by one or more clusters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"##),
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
            ("regions-autoscaling-policies-get",
                    Some(r##"Retrieves autoscaling policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"##),
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
            ("regions-autoscaling-policies-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-autoscaling-policies-list",
                    Some(r##"Lists autoscaling policies in the project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("regions-autoscaling-policies-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-autoscaling-policies-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-autoscaling-policies-update",
                    Some(r##"Updates (replaces) autoscaling policy.Disabled check for update_mask, because all updates will be full replacements."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-autoscaling-policies-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"##),
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
            ("regions-clusters-create",
                    Some(r##"Creates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
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
            ("regions-clusters-delete",
                    Some(r##"Deletes a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-diagnose",
                    Some(r##"Gets cluster diagnostic information. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata). After the operation completes, Operation.response contains DiagnoseClusterResults (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#diagnoseclusterresults)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-diagnose",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-get",
                    Some(r##"Gets the resource representation for a cluster in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-clusters-inject-credentials",
                    Some(r##"Inject encrypted credentials into all of the VMs in a cluster.The target cluster must be a personal auth cluster assigned to the user who is issuing the RPC."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-inject-credentials",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project the cluster belongs to, of the form projects/."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The region containing the cluster, of the form regions/."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster"##),
                     None,
                     Some(r##"Required. The cluster, in the form clusters/."##),
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
            ("regions-clusters-list",
                    Some(r##"Lists all regions/{region}/clusters in a project alphabetically."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
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
            ("regions-clusters-node-groups-create",
                    Some(r##"Creates a node group in a cluster. The returned Operation.metadata is NodeGroupOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#nodegroupoperationmetadata)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-node-groups-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this node group will be created. Format: projects/{project}/regions/{region}/clusters/{cluster}"##),
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
            ("regions-clusters-node-groups-get",
                    Some(r##"Gets the resource representation for a node group in a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-node-groups-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node group to retrieve. Format: projects/{project}/regions/{region}/clusters/{cluster}/nodeGroups/{nodeGroup}"##),
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
            ("regions-clusters-node-groups-repair",
                    Some(r##"Repair nodes in a node group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-node-groups-repair",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node group to resize. Format: projects/{project}/regions/{region}/clusters/{cluster}/nodeGroups/{nodeGroup}"##),
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
            ("regions-clusters-node-groups-resize",
                    Some(r##"Resizes a node group in a cluster. The returned Operation.metadata is NodeGroupOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#nodegroupoperationmetadata)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-node-groups-resize",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node group to resize. Format: projects/{project}/regions/{region}/clusters/{cluster}/nodeGroups/{nodeGroup}"##),
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
            ("regions-clusters-patch",
                    Some(r##"Updates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata). The cluster must be in a RUNNING state or an error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-repair",
                    Some(r##"Repairs a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-repair",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-clusters-start",
                    Some(r##"Starts a cluster in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-start",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-stop",
                    Some(r##"Stops a cluster in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-stop",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-name"##),
                     None,
                     Some(r##"Required. The cluster name."##),
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
            ("regions-clusters-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-jobs-cancel",
                    Some(r##"Starts a job cancellation request. To access the job resource after cancellation, call regions/{region}/jobs.list (https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/list) or regions/{region}/jobs.get (https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/get)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. The job ID."##),
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
            ("regions-jobs-delete",
                    Some(r##"Deletes the job from the project. If the job is active, the delete fails, and the response returns FAILED_PRECONDITION."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. The job ID."##),
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
            ("regions-jobs-get",
                    Some(r##"Gets the resource representation for a job in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. The job ID."##),
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
            ("regions-jobs-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-jobs-list",
                    Some(r##"Lists regions/{region}/jobs in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
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
            ("regions-jobs-patch",
                    Some(r##"Updates a job in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. The job ID."##),
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
            ("regions-jobs-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-jobs-submit",
                    Some(r##"Submits a job to a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-submit",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
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
            ("regions-jobs-submit-as-operation",
                    Some(r##"Submits job to a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-submit-as-operation",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Dataproc region in which to handle the request."##),
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
            ("regions-jobs-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("regions-operations-delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-delete",
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
            ("regions-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-get",
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
            ("regions-operations-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("regions-operations-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-operations-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-operations-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-workflow-templates-create",
                    Some(r##"Creates new workflow template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("regions-workflow-templates-delete",
                    Some(r##"Deletes a workflow template. It does not cancel in-progress workflows."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.delete, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("regions-workflow-templates-get",
                    Some(r##"Retrieves the latest workflow template.Can retrieve previously instantiated template by specifying optional version parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("regions-workflow-templates-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-workflow-templates-instantiate",
                    Some(r##"Instantiates a template and begins execution.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-instantiate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
            ("regions-workflow-templates-instantiate-inline",
                    Some(r##"Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-instantiate-inline",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,instantiateinline, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.instantiateinline, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("regions-workflow-templates-list",
                    Some(r##"Lists workflows that match the specified filter in the request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}"##),
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
            ("regions-workflow-templates-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-workflow-templates-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("regions-workflow-templates-update",
                    Some(r##"Updates (replaces) workflow template. The updated template must contain version that matches the current server version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-workflow-templates-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"##),
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
    
    let mut app = App::new("dataproc1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240617")
           .about("Manages Hadoop-based clusters and jobs on Google Cloud Platform.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_dataproc1_cli")
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
