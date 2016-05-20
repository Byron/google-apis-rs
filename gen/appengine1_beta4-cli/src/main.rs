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
extern crate google_appengine1_beta4 as api;

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
    hub: api::Appengine<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().get(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ensure-resources-exist" => {
                    call = call.ensure_resources_exist(arg_from_str(value.unwrap_or("false"), err, "ensure-resources-exist", "boolean"));
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
                                                                           v.extend(["ensure-resources-exist"].iter().map(|v|*v));
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

    fn _apps_modules_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
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

    fn _apps_modules_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
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

    fn _apps_modules_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_list(opt.value_of("apps-id").unwrap_or(""));
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

    fn _apps_modules_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "split.shard-by" => Some(("split.shardBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "split.allocations" => Some(("split.allocations", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allocations", "id", "name", "shard-by", "split"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Module = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "migrate-traffic" => {
                    call = call.migrate_traffic(arg_from_str(value.unwrap_or("false"), err, "migrate-traffic", "boolean"));
                },
                "mask" => {
                    call = call.mask(value.unwrap_or(""));
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
                                                                           v.extend(["mask", "migrate-traffic"].iter().map(|v|*v));
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

    fn _apps_modules_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployer" => Some(("deployer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-sec" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "container", "cool-down-period", "cpu", "cpu-utilization", "creation-time", "default-expiration", "deployer", "deployment", "disable-health-check", "disk-gb", "disk-utilization", "env", "env-variables", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "instance-class", "instance-tag", "instances", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "request-utilization", "resources", "restart-threshold", "runtime", "script", "security-level", "serving-status", "target-concurrent-requests", "target-read-bytes-per-sec", "target-read-ops-per-sec", "target-received-bytes-per-sec", "target-received-packets-per-sec", "target-request-count-per-sec", "target-sent-bytes-per-sec", "target-sent-packets-per-sec", "target-utilization", "target-write-bytes-per-sec", "target-write-ops-per-sec", "threadsafe", "timeout", "unhealthy-threshold", "url", "vm"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_versions_create(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
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

    fn _apps_modules_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
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

    fn _apps_modules_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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
                                                                           v.extend(["view"].iter().map(|v|*v));
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

    fn _apps_modules_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
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
                                                                           v.extend(["page-token", "page-size", "view"].iter().map(|v|*v));
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

    fn _apps_modules_versions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployer" => Some(("deployer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-sec" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "container", "cool-down-period", "cpu", "cpu-utilization", "creation-time", "default-expiration", "deployer", "deployment", "disable-health-check", "disk-gb", "disk-utilization", "env", "env-variables", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "instance-class", "instance-tag", "instances", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "request-utilization", "resources", "restart-threshold", "runtime", "script", "security-level", "serving-status", "target-concurrent-requests", "target-read-bytes-per-sec", "target-read-ops-per-sec", "target-received-bytes-per-sec", "target-received-packets-per-sec", "target-request-count-per-sec", "target-sent-bytes-per-sec", "target-sent-packets-per-sec", "target-utilization", "target-write-bytes-per-sec", "target-write-ops-per-sec", "threadsafe", "timeout", "unhealthy-threshold", "url", "vm"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_versions_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "mask" => {
                    call = call.mask(value.unwrap_or(""));
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
                                                                           v.extend(["mask"].iter().map(|v|*v));
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

    fn _apps_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("operations-id").unwrap_or(""));
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

    fn _apps_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_list(opt.value_of("apps-id").unwrap_or(""));
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
            ("apps", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._apps_get(opt, dry_run, &mut err);
                    },
                    ("modules-delete", Some(opt)) => {
                        call_result = self._apps_modules_delete(opt, dry_run, &mut err);
                    },
                    ("modules-get", Some(opt)) => {
                        call_result = self._apps_modules_get(opt, dry_run, &mut err);
                    },
                    ("modules-list", Some(opt)) => {
                        call_result = self._apps_modules_list(opt, dry_run, &mut err);
                    },
                    ("modules-patch", Some(opt)) => {
                        call_result = self._apps_modules_patch(opt, dry_run, &mut err);
                    },
                    ("modules-versions-create", Some(opt)) => {
                        call_result = self._apps_modules_versions_create(opt, dry_run, &mut err);
                    },
                    ("modules-versions-delete", Some(opt)) => {
                        call_result = self._apps_modules_versions_delete(opt, dry_run, &mut err);
                    },
                    ("modules-versions-get", Some(opt)) => {
                        call_result = self._apps_modules_versions_get(opt, dry_run, &mut err);
                    },
                    ("modules-versions-list", Some(opt)) => {
                        call_result = self._apps_modules_versions_list(opt, dry_run, &mut err);
                    },
                    ("modules-versions-patch", Some(opt)) => {
                        call_result = self._apps_modules_versions_patch(opt, dry_run, &mut err);
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._apps_operations_get(opt, dry_run, &mut err);
                    },
                    ("operations-list", Some(opt)) => {
                        call_result = self._apps_operations_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("apps".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "appengine1-beta4-secret.json",
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
                                          program_name: "appengine1-beta4",
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
            hub: api::Appengine::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "bearer-token", "callback", "fields", "key", "oauth-token", "pp", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("bearer-token", "bearer_token"),
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
        ("apps", "methods: 'get', 'modules-delete', 'modules-get', 'modules-list', 'modules-patch', 'modules-versions-create', 'modules-versions-delete', 'modules-versions-get', 'modules-versions-list', 'modules-versions-patch', 'operations-get' and 'operations-list'", vec![
            ("get",
                    Some(r##"Gets information about an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the application to get. For example: "apps/myapp"."##),
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
            ("modules-delete",
                    Some(r##"Deletes a module and all enclosed versions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. For example: "apps/myapp/modules/default"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-get",
                    Some(r##"Gets the current configuration of the module."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. For example: "apps/myapp/modules/default"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-list",
                    Some(r##"Lists all the modules in the application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. For example: "apps/myapp"."##),
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
            ("modules-patch",
                    Some(r##"Updates the configuration of the specified module."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. For example: "apps/myapp/modules/default"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-create",
                    Some(r##"Deploys new code and resource files to a version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. For example: "apps/myapp/modules/default"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-delete",
                    Some(r##"Deletes an existing version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. For example: "apps/myapp/modules/default/versions/v1"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-get",
                    Some(r##"Gets application deployment information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. For example: "apps/myapp/modules/default/versions/v1"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-list",
                    Some(r##"Lists the versions of a module."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. For example: "apps/myapp/modules/default"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-patch",
                    Some(r##"Updates an existing version. Note: UNIMPLEMENTED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. For example: "apps/myapp/modules/default/versions/1"."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_operations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation resource."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding below allows API services to override the binding to use different resource name schemes, such as `users/*/operations`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_operations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation collection."##),
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
    
    let mut app = App::new("appengine1-beta4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.4+20160314")
           .about("Provisions and manages App Engine applications.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli")
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