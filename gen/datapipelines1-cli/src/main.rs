// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_datapipelines1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Datapipelines<S>,
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
    async fn _projects_locations_pipelines_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-count" => Some(("jobCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-update-time" => Some(("lastUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pipeline-sources" => Some(("pipelineSources", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "schedule-info.next-job-time" => Some(("scheduleInfo.nextJobTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-info.schedule" => Some(("scheduleInfo.schedule", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-info.time-zone" => Some(("scheduleInfo.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scheduler-service-account-email" => Some(("schedulerServiceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.container-spec-gcs-path" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.containerSpecGcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.additional-experiments" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.additionalExperiments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.additional-user-labels" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.additionalUserLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.enable-streaming-engine" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.enableStreamingEngine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.flexrs-goal" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.flexrsGoal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.ip-configuration" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.ipConfiguration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.kms-key-name" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.machine-type" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.max-workers" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.maxWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.network" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.num-workers" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.numWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.service-account-email" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.subnetwork" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.temp-location" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.tempLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.worker-region" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.workerRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.worker-zone" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.workerZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.zone" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.job-name" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.jobName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.launch-options" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.launchOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.parameters" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.transform-name-mappings" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.transformNameMappings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.update" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.update", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.location" => Some(("workload.dataflowFlexTemplateRequest.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.project-id" => Some(("workload.dataflowFlexTemplateRequest.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.validate-only" => Some(("workload.dataflowFlexTemplateRequest.validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.gcs-path" => Some(("workload.dataflowLaunchTemplateRequest.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.additional-experiments" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.additionalExperiments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.additional-user-labels" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.additionalUserLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.bypass-temp-dir-validation" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.bypassTempDirValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.enable-streaming-engine" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.enableStreamingEngine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.ip-configuration" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.ipConfiguration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.kms-key-name" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.machine-type" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.max-workers" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.maxWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.network" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.num-workers" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.numWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.service-account-email" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.subnetwork" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.temp-location" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.tempLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.worker-region" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.workerRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.worker-zone" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.workerZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.zone" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.job-name" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.jobName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.parameters" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-launch-template-request.launch-parameters.transform-name-mapping" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.transformNameMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-launch-template-request.launch-parameters.update" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.update", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.location" => Some(("workload.dataflowLaunchTemplateRequest.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.project-id" => Some(("workload.dataflowLaunchTemplateRequest.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.validate-only" => Some(("workload.dataflowLaunchTemplateRequest.validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-experiments", "additional-user-labels", "bypass-temp-dir-validation", "container-spec-gcs-path", "create-time", "dataflow-flex-template-request", "dataflow-launch-template-request", "display-name", "enable-streaming-engine", "environment", "flexrs-goal", "gcs-path", "ip-configuration", "job-count", "job-name", "kms-key-name", "last-update-time", "launch-options", "launch-parameter", "launch-parameters", "location", "machine-type", "max-workers", "name", "network", "next-job-time", "num-workers", "parameters", "pipeline-sources", "project-id", "schedule", "schedule-info", "scheduler-service-account-email", "service-account-email", "state", "subnetwork", "temp-location", "time-zone", "transform-name-mapping", "transform-name-mappings", "type", "update", "validate-only", "worker-region", "worker-zone", "workload", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDatapipelinesV1Pipeline = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_pipelines_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_pipelines_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_pipelines_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_pipelines_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_pipelines_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_pipelines_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_pipelines_jobs_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_pipelines_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_pipelines_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_pipelines_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-count" => Some(("jobCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "last-update-time" => Some(("lastUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pipeline-sources" => Some(("pipelineSources", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "schedule-info.next-job-time" => Some(("scheduleInfo.nextJobTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-info.schedule" => Some(("scheduleInfo.schedule", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-info.time-zone" => Some(("scheduleInfo.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scheduler-service-account-email" => Some(("schedulerServiceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.container-spec-gcs-path" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.containerSpecGcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.additional-experiments" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.additionalExperiments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.additional-user-labels" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.additionalUserLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.enable-streaming-engine" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.enableStreamingEngine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.flexrs-goal" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.flexrsGoal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.ip-configuration" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.ipConfiguration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.kms-key-name" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.machine-type" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.max-workers" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.maxWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.network" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.num-workers" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.numWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.service-account-email" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.subnetwork" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.temp-location" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.tempLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.worker-region" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.workerRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.worker-zone" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.workerZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.environment.zone" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.environment.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.job-name" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.jobName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.launch-parameter.launch-options" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.launchOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.parameters" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.transform-name-mappings" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.transformNameMappings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-flex-template-request.launch-parameter.update" => Some(("workload.dataflowFlexTemplateRequest.launchParameter.update", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.location" => Some(("workload.dataflowFlexTemplateRequest.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.project-id" => Some(("workload.dataflowFlexTemplateRequest.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-flex-template-request.validate-only" => Some(("workload.dataflowFlexTemplateRequest.validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.gcs-path" => Some(("workload.dataflowLaunchTemplateRequest.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.additional-experiments" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.additionalExperiments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.additional-user-labels" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.additionalUserLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.bypass-temp-dir-validation" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.bypassTempDirValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.enable-streaming-engine" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.enableStreamingEngine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.ip-configuration" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.ipConfiguration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.kms-key-name" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.machine-type" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.max-workers" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.maxWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.network" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.num-workers" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.numWorkers", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.service-account-email" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.subnetwork" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.temp-location" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.tempLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.worker-region" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.workerRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.worker-zone" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.workerZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.environment.zone" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.environment.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.job-name" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.jobName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.launch-parameters.parameters" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-launch-template-request.launch-parameters.transform-name-mapping" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.transformNameMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "workload.dataflow-launch-template-request.launch-parameters.update" => Some(("workload.dataflowLaunchTemplateRequest.launchParameters.update", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.location" => Some(("workload.dataflowLaunchTemplateRequest.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.project-id" => Some(("workload.dataflowLaunchTemplateRequest.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload.dataflow-launch-template-request.validate-only" => Some(("workload.dataflowLaunchTemplateRequest.validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-experiments", "additional-user-labels", "bypass-temp-dir-validation", "container-spec-gcs-path", "create-time", "dataflow-flex-template-request", "dataflow-launch-template-request", "display-name", "enable-streaming-engine", "environment", "flexrs-goal", "gcs-path", "ip-configuration", "job-count", "job-name", "kms-key-name", "last-update-time", "launch-options", "launch-parameter", "launch-parameters", "location", "machine-type", "max-workers", "name", "network", "next-job-time", "num-workers", "parameters", "pipeline-sources", "project-id", "schedule", "schedule-info", "scheduler-service-account-email", "service-account-email", "state", "subnetwork", "temp-location", "time-zone", "transform-name-mapping", "transform-name-mappings", "type", "update", "validate-only", "worker-region", "worker-zone", "workload", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDatapipelinesV1Pipeline = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_pipelines_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_pipelines_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDatapipelinesV1RunPipelineRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_pipelines_run(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_pipelines_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDatapipelinesV1StopPipelineRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_pipelines_stop(request, opt.value_of("name").unwrap_or(""));
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
                    ("locations-pipelines-create", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-delete", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-get", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-jobs-list", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-list", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-patch", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-run", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_run(opt, dry_run, &mut err).await;
                    },
                    ("locations-pipelines-stop", Some(opt)) => {
                        call_result = self._projects_locations_pipelines_stop(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "datapipelines1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/datapipelines1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Datapipelines::new(client, auth),
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
        ("projects", "methods: 'locations-pipelines-create', 'locations-pipelines-delete', 'locations-pipelines-get', 'locations-pipelines-jobs-list', 'locations-pipelines-list', 'locations-pipelines-patch', 'locations-pipelines-run' and 'locations-pipelines-stop'", vec![
            ("locations-pipelines-create",
                    Some(r##"Creates a pipeline. For a batch pipeline, you can pass scheduler information. Data Pipelines uses the scheduler information to create an internal scheduler that runs jobs periodically. If the internal scheduler is not configured, you can use RunPipeline to run jobs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`."##),
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
            ("locations-pipelines-delete",
                    Some(r##"Deletes a pipeline. If a scheduler job is attached to the pipeline, it will be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`."##),
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
            ("locations-pipelines-get",
                    Some(r##"Looks up a single pipeline. Returns a "NOT_FOUND" error if no such pipeline exists. Returns a "FORBIDDEN" error if the caller doesn't have permission to access it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`."##),
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
            ("locations-pipelines-jobs-list",
                    Some(r##"Lists jobs for a given pipeline. Throws a "FORBIDDEN" error if the caller doesn't have permission to access it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`."##),
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
            ("locations-pipelines-list",
                    Some(r##"Lists pipelines. Returns a "FORBIDDEN" error if the caller doesn't have permission to access it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`."##),
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
            ("locations-pipelines-patch",
                    Some(r##"Updates a pipeline. If successful, the updated Pipeline is returned. Returns `NOT_FOUND` if the pipeline doesn't exist. If UpdatePipeline does not return successfully, you can retry the UpdatePipeline request until you receive a successful response."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects). * `LOCATION_ID` is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling `google.cloud.location.Locations.ListLocations`. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in [App Engine regions](https://cloud.google.com/about/locations#region). * `PIPELINE_ID` is the ID of the pipeline. Must be unique for the selected project and location."##),
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
            ("locations-pipelines-run",
                    Some(r##"Creates a job for the specified pipeline directly. You can use this method when the internal scheduler is not configured and you want to trigger the job directly or through an external system. Returns a "NOT_FOUND" error if the pipeline doesn't exist. Returns a "FORBIDDEN" error if the user doesn't have permission to access the pipeline or run jobs for the pipeline."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-run",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`."##),
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
            ("locations-pipelines-stop",
                    Some(r##"Freezes pipeline execution permanently. If there's a corresponding scheduler entry, it's deleted, and the pipeline state is changed to "ARCHIVED". However, pipeline metadata is retained."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datapipelines1_cli/projects_locations-pipelines-stop",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`."##),
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
    
    let mut app = App::new("datapipelines1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240616")
           .about("Data Pipelines provides an interface for creating, updating, and managing recurring Data Analytics jobs.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_datapipelines1_cli")
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
