// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_ml1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CloudMachineLearningEngine<S>,
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
    async fn _projects_explain(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "http-body.content-type" => Some(("httpBody.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "http-body.data" => Some(("httpBody.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data", "http-body"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__ExplainRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().explain(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_get_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().get_config(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudMlV1__CancelJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_cancel(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error-message" => Some(("errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-id" => Some(("jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-position" => Some(("jobPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "prediction-input.batch-size" => Some(("predictionInput.batchSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.data-format" => Some(("predictionInput.dataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.input-paths" => Some(("predictionInput.inputPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "prediction-input.max-worker-count" => Some(("predictionInput.maxWorkerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "prediction-input.model-name" => Some(("predictionInput.modelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.output-data-format" => Some(("predictionInput.outputDataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.output-path" => Some(("predictionInput.outputPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.region" => Some(("predictionInput.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.runtime-version" => Some(("predictionInput.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.signature-name" => Some(("predictionInput.signatureName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.uri" => Some(("predictionInput.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.version-name" => Some(("predictionInput.versionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-output.error-count" => Some(("predictionOutput.errorCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "prediction-output.node-hours" => Some(("predictionOutput.nodeHours", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "prediction-output.output-path" => Some(("predictionOutput.outputPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-output.prediction-count" => Some(("predictionOutput.predictionCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.args" => Some(("trainingInput.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.enable-web-access" => Some(("trainingInput.enableWebAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-input.encryption-config.kms-key-name" => Some(("trainingInput.encryptionConfig.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.accelerator-config.count" => Some(("trainingInput.evaluatorConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.accelerator-config.type" => Some(("trainingInput.evaluatorConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.container-args" => Some(("trainingInput.evaluatorConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.evaluator-config.container-command" => Some(("trainingInput.evaluatorConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.evaluator-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.evaluatorConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.disk-config.boot-disk-type" => Some(("trainingInput.evaluatorConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.image-uri" => Some(("trainingInput.evaluatorConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.tpu-tf-version" => Some(("trainingInput.evaluatorConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-count" => Some(("trainingInput.evaluatorCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.evaluator-type" => Some(("trainingInput.evaluatorType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.algorithm" => Some(("trainingInput.hyperparameters.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.enable-trial-early-stopping" => Some(("trainingInput.hyperparameters.enableTrialEarlyStopping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.goal" => Some(("trainingInput.hyperparameters.goal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.hyperparameter-metric-tag" => Some(("trainingInput.hyperparameters.hyperparameterMetricTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.max-failed-trials" => Some(("trainingInput.hyperparameters.maxFailedTrials", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.max-parallel-trials" => Some(("trainingInput.hyperparameters.maxParallelTrials", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.max-trials" => Some(("trainingInput.hyperparameters.maxTrials", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.resume-previous-job-id" => Some(("trainingInput.hyperparameters.resumePreviousJobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.job-dir" => Some(("trainingInput.jobDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.accelerator-config.count" => Some(("trainingInput.masterConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.accelerator-config.type" => Some(("trainingInput.masterConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.container-args" => Some(("trainingInput.masterConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.master-config.container-command" => Some(("trainingInput.masterConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.master-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.master-config.disk-config.boot-disk-type" => Some(("trainingInput.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.image-uri" => Some(("trainingInput.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.tpu-tf-version" => Some(("trainingInput.masterConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-type" => Some(("trainingInput.masterType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.network" => Some(("trainingInput.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.package-uris" => Some(("trainingInput.packageUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.parameter-server-config.accelerator-config.count" => Some(("trainingInput.parameterServerConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.accelerator-config.type" => Some(("trainingInput.parameterServerConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.container-args" => Some(("trainingInput.parameterServerConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.parameter-server-config.container-command" => Some(("trainingInput.parameterServerConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.parameter-server-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.parameterServerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.disk-config.boot-disk-type" => Some(("trainingInput.parameterServerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.image-uri" => Some(("trainingInput.parameterServerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.tpu-tf-version" => Some(("trainingInput.parameterServerConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-count" => Some(("trainingInput.parameterServerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-type" => Some(("trainingInput.parameterServerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.python-module" => Some(("trainingInput.pythonModule", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.python-version" => Some(("trainingInput.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.region" => Some(("trainingInput.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.runtime-version" => Some(("trainingInput.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scale-tier" => Some(("trainingInput.scaleTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scheduling.max-running-time" => Some(("trainingInput.scheduling.maxRunningTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scheduling.max-wait-time" => Some(("trainingInput.scheduling.maxWaitTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scheduling.priority" => Some(("trainingInput.scheduling.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.service-account" => Some(("trainingInput.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.use-chief-in-tf-config" => Some(("trainingInput.useChiefInTfConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-input.worker-config.accelerator-config.count" => Some(("trainingInput.workerConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.accelerator-config.type" => Some(("trainingInput.workerConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.container-args" => Some(("trainingInput.workerConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.worker-config.container-command" => Some(("trainingInput.workerConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.worker-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.worker-config.disk-config.boot-disk-type" => Some(("trainingInput.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.image-uri" => Some(("trainingInput.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.tpu-tf-version" => Some(("trainingInput.workerConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-count" => Some(("trainingInput.workerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.worker-type" => Some(("trainingInput.workerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.framework" => Some(("trainingOutput.builtInAlgorithmOutput.framework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.model-path" => Some(("trainingOutput.builtInAlgorithmOutput.modelPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.python-version" => Some(("trainingOutput.builtInAlgorithmOutput.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.runtime-version" => Some(("trainingOutput.builtInAlgorithmOutput.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.completed-trial-count" => Some(("trainingOutput.completedTrialCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-output.consumed-ml-units" => Some(("trainingOutput.consumedMLUnits", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "training-output.hyperparameter-metric-tag" => Some(("trainingOutput.hyperparameterMetricTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.is-built-in-algorithm-job" => Some(("trainingOutput.isBuiltInAlgorithmJob", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-output.is-hyperparameter-tuning-job" => Some(("trainingOutput.isHyperparameterTuningJob", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-output.web-access-uris" => Some(("trainingOutput.webAccessUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-config", "algorithm", "args", "batch-size", "boot-disk-size-gb", "boot-disk-type", "built-in-algorithm-output", "completed-trial-count", "consumed-ml-units", "container-args", "container-command", "count", "create-time", "data-format", "disk-config", "enable-trial-early-stopping", "enable-web-access", "encryption-config", "end-time", "error-count", "error-message", "etag", "evaluator-config", "evaluator-count", "evaluator-type", "framework", "goal", "hyperparameter-metric-tag", "hyperparameters", "image-uri", "input-paths", "is-built-in-algorithm-job", "is-hyperparameter-tuning-job", "job-dir", "job-id", "job-position", "kms-key-name", "labels", "master-config", "master-type", "max-failed-trials", "max-parallel-trials", "max-running-time", "max-trials", "max-wait-time", "max-worker-count", "model-name", "model-path", "network", "node-hours", "output-data-format", "output-path", "package-uris", "parameter-server-config", "parameter-server-count", "parameter-server-type", "prediction-count", "prediction-input", "prediction-output", "priority", "python-module", "python-version", "region", "resume-previous-job-id", "runtime-version", "scale-tier", "scheduling", "service-account", "signature-name", "start-time", "state", "tpu-tf-version", "training-input", "training-output", "type", "uri", "use-chief-in-tf-config", "version-name", "web-access-uris", "worker-config", "worker-count", "worker-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Job = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().jobs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_jobs_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().jobs_get_iam_policy(opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().jobs_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_jobs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error-message" => Some(("errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-id" => Some(("jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-position" => Some(("jobPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "prediction-input.batch-size" => Some(("predictionInput.batchSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.data-format" => Some(("predictionInput.dataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.input-paths" => Some(("predictionInput.inputPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "prediction-input.max-worker-count" => Some(("predictionInput.maxWorkerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "prediction-input.model-name" => Some(("predictionInput.modelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.output-data-format" => Some(("predictionInput.outputDataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.output-path" => Some(("predictionInput.outputPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.region" => Some(("predictionInput.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.runtime-version" => Some(("predictionInput.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.signature-name" => Some(("predictionInput.signatureName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.uri" => Some(("predictionInput.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-input.version-name" => Some(("predictionInput.versionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-output.error-count" => Some(("predictionOutput.errorCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "prediction-output.node-hours" => Some(("predictionOutput.nodeHours", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "prediction-output.output-path" => Some(("predictionOutput.outputPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prediction-output.prediction-count" => Some(("predictionOutput.predictionCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.args" => Some(("trainingInput.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.enable-web-access" => Some(("trainingInput.enableWebAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-input.encryption-config.kms-key-name" => Some(("trainingInput.encryptionConfig.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.accelerator-config.count" => Some(("trainingInput.evaluatorConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.accelerator-config.type" => Some(("trainingInput.evaluatorConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.container-args" => Some(("trainingInput.evaluatorConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.evaluator-config.container-command" => Some(("trainingInput.evaluatorConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.evaluator-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.evaluatorConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.disk-config.boot-disk-type" => Some(("trainingInput.evaluatorConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.image-uri" => Some(("trainingInput.evaluatorConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-config.tpu-tf-version" => Some(("trainingInput.evaluatorConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.evaluator-count" => Some(("trainingInput.evaluatorCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.evaluator-type" => Some(("trainingInput.evaluatorType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.algorithm" => Some(("trainingInput.hyperparameters.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.enable-trial-early-stopping" => Some(("trainingInput.hyperparameters.enableTrialEarlyStopping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.goal" => Some(("trainingInput.hyperparameters.goal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.hyperparameter-metric-tag" => Some(("trainingInput.hyperparameters.hyperparameterMetricTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.max-failed-trials" => Some(("trainingInput.hyperparameters.maxFailedTrials", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.max-parallel-trials" => Some(("trainingInput.hyperparameters.maxParallelTrials", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.max-trials" => Some(("trainingInput.hyperparameters.maxTrials", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.hyperparameters.resume-previous-job-id" => Some(("trainingInput.hyperparameters.resumePreviousJobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.job-dir" => Some(("trainingInput.jobDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.accelerator-config.count" => Some(("trainingInput.masterConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.accelerator-config.type" => Some(("trainingInput.masterConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.container-args" => Some(("trainingInput.masterConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.master-config.container-command" => Some(("trainingInput.masterConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.master-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.master-config.disk-config.boot-disk-type" => Some(("trainingInput.masterConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.image-uri" => Some(("trainingInput.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-config.tpu-tf-version" => Some(("trainingInput.masterConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.master-type" => Some(("trainingInput.masterType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.network" => Some(("trainingInput.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.package-uris" => Some(("trainingInput.packageUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.parameter-server-config.accelerator-config.count" => Some(("trainingInput.parameterServerConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.accelerator-config.type" => Some(("trainingInput.parameterServerConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.container-args" => Some(("trainingInput.parameterServerConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.parameter-server-config.container-command" => Some(("trainingInput.parameterServerConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.parameter-server-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.parameterServerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.disk-config.boot-disk-type" => Some(("trainingInput.parameterServerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.image-uri" => Some(("trainingInput.parameterServerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-config.tpu-tf-version" => Some(("trainingInput.parameterServerConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-count" => Some(("trainingInput.parameterServerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.parameter-server-type" => Some(("trainingInput.parameterServerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.python-module" => Some(("trainingInput.pythonModule", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.python-version" => Some(("trainingInput.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.region" => Some(("trainingInput.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.runtime-version" => Some(("trainingInput.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scale-tier" => Some(("trainingInput.scaleTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scheduling.max-running-time" => Some(("trainingInput.scheduling.maxRunningTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scheduling.max-wait-time" => Some(("trainingInput.scheduling.maxWaitTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.scheduling.priority" => Some(("trainingInput.scheduling.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.service-account" => Some(("trainingInput.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.use-chief-in-tf-config" => Some(("trainingInput.useChiefInTfConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-input.worker-config.accelerator-config.count" => Some(("trainingInput.workerConfig.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.accelerator-config.type" => Some(("trainingInput.workerConfig.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.container-args" => Some(("trainingInput.workerConfig.containerArgs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.worker-config.container-command" => Some(("trainingInput.workerConfig.containerCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "training-input.worker-config.disk-config.boot-disk-size-gb" => Some(("trainingInput.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.worker-config.disk-config.boot-disk-type" => Some(("trainingInput.workerConfig.diskConfig.bootDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.image-uri" => Some(("trainingInput.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-config.tpu-tf-version" => Some(("trainingInput.workerConfig.tpuTfVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-input.worker-count" => Some(("trainingInput.workerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-input.worker-type" => Some(("trainingInput.workerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.framework" => Some(("trainingOutput.builtInAlgorithmOutput.framework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.model-path" => Some(("trainingOutput.builtInAlgorithmOutput.modelPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.python-version" => Some(("trainingOutput.builtInAlgorithmOutput.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.built-in-algorithm-output.runtime-version" => Some(("trainingOutput.builtInAlgorithmOutput.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.completed-trial-count" => Some(("trainingOutput.completedTrialCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-output.consumed-ml-units" => Some(("trainingOutput.consumedMLUnits", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "training-output.hyperparameter-metric-tag" => Some(("trainingOutput.hyperparameterMetricTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-output.is-built-in-algorithm-job" => Some(("trainingOutput.isBuiltInAlgorithmJob", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-output.is-hyperparameter-tuning-job" => Some(("trainingOutput.isHyperparameterTuningJob", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "training-output.web-access-uris" => Some(("trainingOutput.webAccessUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-config", "algorithm", "args", "batch-size", "boot-disk-size-gb", "boot-disk-type", "built-in-algorithm-output", "completed-trial-count", "consumed-ml-units", "container-args", "container-command", "count", "create-time", "data-format", "disk-config", "enable-trial-early-stopping", "enable-web-access", "encryption-config", "end-time", "error-count", "error-message", "etag", "evaluator-config", "evaluator-count", "evaluator-type", "framework", "goal", "hyperparameter-metric-tag", "hyperparameters", "image-uri", "input-paths", "is-built-in-algorithm-job", "is-hyperparameter-tuning-job", "job-dir", "job-id", "job-position", "kms-key-name", "labels", "master-config", "master-type", "max-failed-trials", "max-parallel-trials", "max-running-time", "max-trials", "max-wait-time", "max-worker-count", "model-name", "model-path", "network", "node-hours", "output-data-format", "output-path", "package-uris", "parameter-server-config", "parameter-server-count", "parameter-server-type", "prediction-count", "prediction-input", "prediction-output", "priority", "python-module", "python-version", "region", "resume-previous-job-id", "runtime-version", "scale-tier", "scheduling", "service-account", "signature-name", "start-time", "state", "tpu-tf-version", "training-input", "training-output", "type", "uri", "use-chief-in-tf-config", "version-name", "web-access-uris", "worker-config", "worker-count", "worker-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Job = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_jobs_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleIamV1__SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_jobs_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleIamV1__TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_get(opt.value_of("name").unwrap_or(""));
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
        let mut call = self.hub.projects().locations_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_studies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inactive-reason" => Some(("inactiveReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "study-config.algorithm" => Some(("studyConfig.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "study-config.automated-stopping-config.decay-curve-stopping-config.use-elapsed-time" => Some(("studyConfig.automatedStoppingConfig.decayCurveStoppingConfig.useElapsedTime", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "study-config.automated-stopping-config.median-automated-stopping-config.use-elapsed-time" => Some(("studyConfig.automatedStoppingConfig.medianAutomatedStoppingConfig.useElapsedTime", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["algorithm", "automated-stopping-config", "create-time", "decay-curve-stopping-config", "inactive-reason", "median-automated-stopping-config", "name", "state", "study-config", "use-elapsed-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Study = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "study-id" => {
                    call = call.study_id(value.unwrap_or(""));
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
                                                                           v.extend(["study-id"].iter().map(|v|*v));
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

    async fn _projects_locations_studies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_studies_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_studies_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_studies_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_add_measurement(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "measurement.elapsed-time" => Some(("measurement.elapsedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "measurement.step-count" => Some(("measurement.stepCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["elapsed-time", "measurement", "step-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__AddTrialMeasurementRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_add_measurement(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_check_early_stopping_state(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudMlV1__CheckTrialEarlyStoppingStateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_check_early_stopping_state(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_complete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "final-measurement.elapsed-time" => Some(("finalMeasurement.elapsedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-measurement.step-count" => Some(("finalMeasurement.stepCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "infeasible-reason" => Some(("infeasibleReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trial-infeasible" => Some(("trialInfeasible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["elapsed-time", "final-measurement", "infeasible-reason", "step-count", "trial-infeasible"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__CompleteTrialRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_complete(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-id" => Some(("clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-measurement.elapsed-time" => Some(("finalMeasurement.elapsedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-measurement.step-count" => Some(("finalMeasurement.stepCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "infeasible-reason" => Some(("infeasibleReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trial-infeasible" => Some(("trialInfeasible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["client-id", "elapsed-time", "end-time", "final-measurement", "infeasible-reason", "name", "start-time", "state", "step-count", "trial-infeasible"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Trial = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_studies_trials_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_studies_trials_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_studies_trials_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_list_optimal_trials(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudMlV1__ListOptimalTrialsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_list_optimal_trials(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudMlV1__StopTrialRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_stop(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_studies_trials_suggest(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-id" => Some(("clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestion-count" => Some(("suggestionCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["client-id", "suggestion-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__SuggestTrialsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_studies_trials_suggest(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_models_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "default-version.accelerator-config.count" => Some(("defaultVersion.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.accelerator-config.type" => Some(("defaultVersion.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.auto-scaling.max-nodes" => Some(("defaultVersion.autoScaling.maxNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.auto-scaling.min-nodes" => Some(("defaultVersion.autoScaling.minNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.container.args" => Some(("defaultVersion.container.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "default-version.container.command" => Some(("defaultVersion.container.command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "default-version.container.image" => Some(("defaultVersion.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.create-time" => Some(("defaultVersion.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.deployment-uri" => Some(("defaultVersion.deploymentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.description" => Some(("defaultVersion.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.error-message" => Some(("defaultVersion.errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.etag" => Some(("defaultVersion.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.explanation-config.integrated-gradients-attribution.num-integral-steps" => Some(("defaultVersion.explanationConfig.integratedGradientsAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.explanation-config.sampled-shapley-attribution.num-paths" => Some(("defaultVersion.explanationConfig.sampledShapleyAttribution.numPaths", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.explanation-config.xrai-attribution.num-integral-steps" => Some(("defaultVersion.explanationConfig.xraiAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.framework" => Some(("defaultVersion.framework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.is-default" => Some(("defaultVersion.isDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-version.labels" => Some(("defaultVersion.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "default-version.last-migration-model-id" => Some(("defaultVersion.lastMigrationModelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.last-migration-time" => Some(("defaultVersion.lastMigrationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.last-use-time" => Some(("defaultVersion.lastUseTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.machine-type" => Some(("defaultVersion.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.manual-scaling.nodes" => Some(("defaultVersion.manualScaling.nodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.name" => Some(("defaultVersion.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.package-uris" => Some(("defaultVersion.packageUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "default-version.prediction-class" => Some(("defaultVersion.predictionClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.python-version" => Some(("defaultVersion.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.request-logging-config.bigquery-table-name" => Some(("defaultVersion.requestLoggingConfig.bigqueryTableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.request-logging-config.sampling-percentage" => Some(("defaultVersion.requestLoggingConfig.samplingPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "default-version.routes.health" => Some(("defaultVersion.routes.health", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.routes.predict" => Some(("defaultVersion.routes.predict", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.runtime-version" => Some(("defaultVersion.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.service-account" => Some(("defaultVersion.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.state" => Some(("defaultVersion.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "online-prediction-console-logging" => Some(("onlinePredictionConsoleLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "online-prediction-logging" => Some(("onlinePredictionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "regions" => Some(("regions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-config", "args", "auto-scaling", "bigquery-table-name", "command", "container", "count", "create-time", "default-version", "deployment-uri", "description", "error-message", "etag", "explanation-config", "framework", "health", "image", "integrated-gradients-attribution", "is-default", "labels", "last-migration-model-id", "last-migration-time", "last-use-time", "machine-type", "manual-scaling", "max-nodes", "min-nodes", "name", "nodes", "num-integral-steps", "num-paths", "online-prediction-console-logging", "online-prediction-logging", "package-uris", "predict", "prediction-class", "python-version", "regions", "request-logging-config", "routes", "runtime-version", "sampled-shapley-attribution", "sampling-percentage", "service-account", "state", "type", "xrai-attribution"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Model = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_models_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_models_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_models_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_get_iam_policy(opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_models_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_models_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "default-version.accelerator-config.count" => Some(("defaultVersion.acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.accelerator-config.type" => Some(("defaultVersion.acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.auto-scaling.max-nodes" => Some(("defaultVersion.autoScaling.maxNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.auto-scaling.min-nodes" => Some(("defaultVersion.autoScaling.minNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.container.args" => Some(("defaultVersion.container.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "default-version.container.command" => Some(("defaultVersion.container.command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "default-version.container.image" => Some(("defaultVersion.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.create-time" => Some(("defaultVersion.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.deployment-uri" => Some(("defaultVersion.deploymentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.description" => Some(("defaultVersion.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.error-message" => Some(("defaultVersion.errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.etag" => Some(("defaultVersion.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.explanation-config.integrated-gradients-attribution.num-integral-steps" => Some(("defaultVersion.explanationConfig.integratedGradientsAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.explanation-config.sampled-shapley-attribution.num-paths" => Some(("defaultVersion.explanationConfig.sampledShapleyAttribution.numPaths", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.explanation-config.xrai-attribution.num-integral-steps" => Some(("defaultVersion.explanationConfig.xraiAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.framework" => Some(("defaultVersion.framework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.is-default" => Some(("defaultVersion.isDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-version.labels" => Some(("defaultVersion.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "default-version.last-migration-model-id" => Some(("defaultVersion.lastMigrationModelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.last-migration-time" => Some(("defaultVersion.lastMigrationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.last-use-time" => Some(("defaultVersion.lastUseTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.machine-type" => Some(("defaultVersion.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.manual-scaling.nodes" => Some(("defaultVersion.manualScaling.nodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "default-version.name" => Some(("defaultVersion.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.package-uris" => Some(("defaultVersion.packageUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "default-version.prediction-class" => Some(("defaultVersion.predictionClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.python-version" => Some(("defaultVersion.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.request-logging-config.bigquery-table-name" => Some(("defaultVersion.requestLoggingConfig.bigqueryTableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.request-logging-config.sampling-percentage" => Some(("defaultVersion.requestLoggingConfig.samplingPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "default-version.routes.health" => Some(("defaultVersion.routes.health", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.routes.predict" => Some(("defaultVersion.routes.predict", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.runtime-version" => Some(("defaultVersion.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.service-account" => Some(("defaultVersion.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-version.state" => Some(("defaultVersion.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "online-prediction-console-logging" => Some(("onlinePredictionConsoleLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "online-prediction-logging" => Some(("onlinePredictionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "regions" => Some(("regions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-config", "args", "auto-scaling", "bigquery-table-name", "command", "container", "count", "create-time", "default-version", "deployment-uri", "description", "error-message", "etag", "explanation-config", "framework", "health", "image", "integrated-gradients-attribution", "is-default", "labels", "last-migration-model-id", "last-migration-time", "last-use-time", "machine-type", "manual-scaling", "max-nodes", "min-nodes", "name", "nodes", "num-integral-steps", "num-paths", "online-prediction-console-logging", "online-prediction-logging", "package-uris", "predict", "prediction-class", "python-version", "regions", "request-logging-config", "routes", "runtime-version", "sampled-shapley-attribution", "sampling-percentage", "service-account", "state", "type", "xrai-attribution"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Model = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_models_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleIamV1__SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_models_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleIamV1__TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_models_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "accelerator-config.count" => Some(("acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "accelerator-config.type" => Some(("acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auto-scaling.max-nodes" => Some(("autoScaling.maxNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "auto-scaling.min-nodes" => Some(("autoScaling.minNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "container.args" => Some(("container.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.command" => Some(("container.command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.image" => Some(("container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment-uri" => Some(("deploymentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error-message" => Some(("errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explanation-config.integrated-gradients-attribution.num-integral-steps" => Some(("explanationConfig.integratedGradientsAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "explanation-config.sampled-shapley-attribution.num-paths" => Some(("explanationConfig.sampledShapleyAttribution.numPaths", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "explanation-config.xrai-attribution.num-integral-steps" => Some(("explanationConfig.xraiAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "framework" => Some(("framework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-default" => Some(("isDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-migration-model-id" => Some(("lastMigrationModelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-migration-time" => Some(("lastMigrationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-use-time" => Some(("lastUseTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "machine-type" => Some(("machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manual-scaling.nodes" => Some(("manualScaling.nodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package-uris" => Some(("packageUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "prediction-class" => Some(("predictionClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "python-version" => Some(("pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-logging-config.bigquery-table-name" => Some(("requestLoggingConfig.bigqueryTableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-logging-config.sampling-percentage" => Some(("requestLoggingConfig.samplingPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "routes.health" => Some(("routes.health", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routes.predict" => Some(("routes.predict", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-version" => Some(("runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-config", "args", "auto-scaling", "bigquery-table-name", "command", "container", "count", "create-time", "deployment-uri", "description", "error-message", "etag", "explanation-config", "framework", "health", "image", "integrated-gradients-attribution", "is-default", "labels", "last-migration-model-id", "last-migration-time", "last-use-time", "machine-type", "manual-scaling", "max-nodes", "min-nodes", "name", "nodes", "num-integral-steps", "num-paths", "package-uris", "predict", "prediction-class", "python-version", "request-logging-config", "routes", "runtime-version", "sampled-shapley-attribution", "sampling-percentage", "service-account", "state", "type", "xrai-attribution"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_versions_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_models_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_versions_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_models_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_versions_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_models_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().models_versions_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_models_versions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "accelerator-config.count" => Some(("acceleratorConfig.count", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "accelerator-config.type" => Some(("acceleratorConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auto-scaling.max-nodes" => Some(("autoScaling.maxNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "auto-scaling.min-nodes" => Some(("autoScaling.minNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "container.args" => Some(("container.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.command" => Some(("container.command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.image" => Some(("container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment-uri" => Some(("deploymentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error-message" => Some(("errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "explanation-config.integrated-gradients-attribution.num-integral-steps" => Some(("explanationConfig.integratedGradientsAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "explanation-config.sampled-shapley-attribution.num-paths" => Some(("explanationConfig.sampledShapleyAttribution.numPaths", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "explanation-config.xrai-attribution.num-integral-steps" => Some(("explanationConfig.xraiAttribution.numIntegralSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "framework" => Some(("framework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-default" => Some(("isDefault", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-migration-model-id" => Some(("lastMigrationModelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-migration-time" => Some(("lastMigrationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-use-time" => Some(("lastUseTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "machine-type" => Some(("machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manual-scaling.nodes" => Some(("manualScaling.nodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package-uris" => Some(("packageUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "prediction-class" => Some(("predictionClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "python-version" => Some(("pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-logging-config.bigquery-table-name" => Some(("requestLoggingConfig.bigqueryTableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-logging-config.sampling-percentage" => Some(("requestLoggingConfig.samplingPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "routes.health" => Some(("routes.health", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routes.predict" => Some(("routes.predict", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-version" => Some(("runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accelerator-config", "args", "auto-scaling", "bigquery-table-name", "command", "container", "count", "create-time", "deployment-uri", "description", "error-message", "etag", "explanation-config", "framework", "health", "image", "integrated-gradients-attribution", "is-default", "labels", "last-migration-model-id", "last-migration-time", "last-use-time", "machine-type", "manual-scaling", "max-nodes", "min-nodes", "name", "nodes", "num-integral-steps", "num-paths", "package-uris", "predict", "prediction-class", "python-version", "request-logging-config", "routes", "runtime-version", "sampled-shapley-attribution", "sampling-percentage", "service-account", "state", "type", "xrai-attribution"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_versions_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_models_versions_set_default(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudMlV1__SetDefaultVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().models_versions_set_default(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().operations_cancel(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().operations_list(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_predict(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "http-body.content-type" => Some(("httpBody.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "http-body.data" => Some(("httpBody.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data", "http-body"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudMlV1__PredictRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().predict(request, opt.value_of("name").unwrap_or(""));
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
                    ("explain", Some(opt)) => {
                        call_result = self._projects_explain(opt, dry_run, &mut err).await;
                    },
                    ("get-config", Some(opt)) => {
                        call_result = self._projects_get_config(opt, dry_run, &mut err).await;
                    },
                    ("jobs-cancel", Some(opt)) => {
                        call_result = self._projects_jobs_cancel(opt, dry_run, &mut err).await;
                    },
                    ("jobs-create", Some(opt)) => {
                        call_result = self._projects_jobs_create(opt, dry_run, &mut err).await;
                    },
                    ("jobs-get", Some(opt)) => {
                        call_result = self._projects_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("jobs-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_jobs_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("jobs-list", Some(opt)) => {
                        call_result = self._projects_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("jobs-patch", Some(opt)) => {
                        call_result = self._projects_jobs_patch(opt, dry_run, &mut err).await;
                    },
                    ("jobs-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_jobs_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("jobs-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_jobs_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._projects_locations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._projects_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-create", Some(opt)) => {
                        call_result = self._projects_locations_studies_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-delete", Some(opt)) => {
                        call_result = self._projects_locations_studies_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-get", Some(opt)) => {
                        call_result = self._projects_locations_studies_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-list", Some(opt)) => {
                        call_result = self._projects_locations_studies_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-add-measurement", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_add_measurement(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-check-early-stopping-state", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_check_early_stopping_state(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-complete", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_complete(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-create", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-delete", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-get", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-list", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-list-optimal-trials", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_list_optimal_trials(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-stop", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_stop(opt, dry_run, &mut err).await;
                    },
                    ("locations-studies-trials-suggest", Some(opt)) => {
                        call_result = self._projects_locations_studies_trials_suggest(opt, dry_run, &mut err).await;
                    },
                    ("models-create", Some(opt)) => {
                        call_result = self._projects_models_create(opt, dry_run, &mut err).await;
                    },
                    ("models-delete", Some(opt)) => {
                        call_result = self._projects_models_delete(opt, dry_run, &mut err).await;
                    },
                    ("models-get", Some(opt)) => {
                        call_result = self._projects_models_get(opt, dry_run, &mut err).await;
                    },
                    ("models-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_models_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("models-list", Some(opt)) => {
                        call_result = self._projects_models_list(opt, dry_run, &mut err).await;
                    },
                    ("models-patch", Some(opt)) => {
                        call_result = self._projects_models_patch(opt, dry_run, &mut err).await;
                    },
                    ("models-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_models_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("models-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_models_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("models-versions-create", Some(opt)) => {
                        call_result = self._projects_models_versions_create(opt, dry_run, &mut err).await;
                    },
                    ("models-versions-delete", Some(opt)) => {
                        call_result = self._projects_models_versions_delete(opt, dry_run, &mut err).await;
                    },
                    ("models-versions-get", Some(opt)) => {
                        call_result = self._projects_models_versions_get(opt, dry_run, &mut err).await;
                    },
                    ("models-versions-list", Some(opt)) => {
                        call_result = self._projects_models_versions_list(opt, dry_run, &mut err).await;
                    },
                    ("models-versions-patch", Some(opt)) => {
                        call_result = self._projects_models_versions_patch(opt, dry_run, &mut err).await;
                    },
                    ("models-versions-set-default", Some(opt)) => {
                        call_result = self._projects_models_versions_set_default(opt, dry_run, &mut err).await;
                    },
                    ("operations-cancel", Some(opt)) => {
                        call_result = self._projects_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._projects_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("operations-list", Some(opt)) => {
                        call_result = self._projects_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("predict", Some(opt)) => {
                        call_result = self._projects_predict(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "ml1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/ml1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CloudMachineLearningEngine::new(client, auth),
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
        ("projects", "methods: 'explain', 'get-config', 'jobs-cancel', 'jobs-create', 'jobs-get', 'jobs-get-iam-policy', 'jobs-list', 'jobs-patch', 'jobs-set-iam-policy', 'jobs-test-iam-permissions', 'locations-get', 'locations-list', 'locations-operations-cancel', 'locations-operations-get', 'locations-studies-create', 'locations-studies-delete', 'locations-studies-get', 'locations-studies-list', 'locations-studies-trials-add-measurement', 'locations-studies-trials-check-early-stopping-state', 'locations-studies-trials-complete', 'locations-studies-trials-create', 'locations-studies-trials-delete', 'locations-studies-trials-get', 'locations-studies-trials-list', 'locations-studies-trials-list-optimal-trials', 'locations-studies-trials-stop', 'locations-studies-trials-suggest', 'models-create', 'models-delete', 'models-get', 'models-get-iam-policy', 'models-list', 'models-patch', 'models-set-iam-policy', 'models-test-iam-permissions', 'models-versions-create', 'models-versions-delete', 'models-versions-get', 'models-versions-list', 'models-versions-patch', 'models-versions-set-default', 'operations-cancel', 'operations-get', 'operations-list' and 'predict'", vec![
            ("explain",
                    Some(r##"Performs explanation on the data in the request. {% dynamic include "/ai-platform/includes/___explain-request" %} "##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_explain",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of a model or a version. Authorization: requires the `predict` permission on the specified resource."##),
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
            ("get-config",
                    Some(r##"Get the service account information associated with your project. You need this information in order to grant the service account permissions for the Google Cloud Storage location where you put your model training code for training the model with Google Cloud Machine Learning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_get-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The project name."##),
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
            ("jobs-cancel",
                    Some(r##"Cancels a running job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the job to cancel."##),
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
            ("jobs-create",
                    Some(r##"Creates a training or a batch prediction job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project name."##),
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
            ("jobs-get",
                    Some(r##"Describes a job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the job to get the description of."##),
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
            ("jobs-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-get-iam-policy",
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
            ("jobs-list",
                    Some(r##"Lists the jobs in the project. If there are no jobs that match the request parameters, the list request returns an empty response body: {}."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project for which to list jobs."##),
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
            ("jobs-patch",
                    Some(r##"Updates a specific job resource. Currently the only supported fields to update are `labels`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The job name."##),
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
            ("jobs-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-set-iam-policy",
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
            ("jobs-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_jobs-test-iam-permissions",
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
            ("locations-get",
                    Some(r##"Get the complete list of CMLE capabilities in a location, along with their location-specific properties."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the location."##),
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
                    Some(r##"List all locations that provides at least one type of CMLE capability."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project for which available locations are to be listed (since some locations might be whitelisted for specific projects)."##),
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
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-operations-cancel",
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-operations-get",
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
            ("locations-studies-create",
                    Some(r##"Creates a study."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project and location that the study belongs to. Format: projects/{project}/locations/{location}"##),
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
            ("locations-studies-delete",
                    Some(r##"Deletes a study."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The study name."##),
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
            ("locations-studies-get",
                    Some(r##"Gets a study."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The study name."##),
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
            ("locations-studies-list",
                    Some(r##"Lists all the studies in a region for an associated project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project and location that the study belongs to. Format: projects/{project}/locations/{location}"##),
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
            ("locations-studies-trials-add-measurement",
                    Some(r##"Adds a measurement of the objective metrics to a trial. This measurement is assumed to have been taken before the trial is complete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-add-measurement",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The trial name."##),
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
            ("locations-studies-trials-check-early-stopping-state",
                    Some(r##"Checks whether a trial should stop or not. Returns a long-running operation. When the operation is successful, it will contain a CheckTrialEarlyStoppingStateResponse."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-check-early-stopping-state",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The trial name."##),
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
            ("locations-studies-trials-complete",
                    Some(r##"Marks a trial as complete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-complete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The trial name.metat"##),
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
            ("locations-studies-trials-create",
                    Some(r##"Adds a user provided trial to a study."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the study that the trial belongs to."##),
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
            ("locations-studies-trials-delete",
                    Some(r##"Deletes a trial."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The trial name."##),
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
            ("locations-studies-trials-get",
                    Some(r##"Gets a trial."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The trial name."##),
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
            ("locations-studies-trials-list",
                    Some(r##"Lists the trials associated with a study."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the study that the trial belongs to."##),
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
            ("locations-studies-trials-list-optimal-trials",
                    Some(r##"Lists the pareto-optimal trials for multi-objective study or the optimal trials for single-objective study. The definition of pareto-optimal can be checked in wiki page. https://en.wikipedia.org/wiki/Pareto_efficiency"##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-list-optimal-trials",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the study that the pareto-optimal trial belongs to."##),
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
            ("locations-studies-trials-stop",
                    Some(r##"Stops a trial."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-stop",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The trial name."##),
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
            ("locations-studies-trials-suggest",
                    Some(r##"Adds one or more trials to a study, with parameter values suggested by AI Platform Vizier. Returns a long-running operation associated with the generation of trial suggestions. When this long-running operation succeeds, it will contain a SuggestTrialsResponse."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_locations-studies-trials-suggest",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the study that the trial belongs to."##),
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
            ("models-create",
                    Some(r##"Creates a model which will later contain one or more versions. You must add at least one version before you can request predictions from the model. Add versions by calling projects.models.versions.create."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project name."##),
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
            ("models-delete",
                    Some(r##"Deletes a model. You can only delete a model if there are no versions in it. You can delete versions by calling projects.models.versions.delete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the model."##),
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
            ("models-get",
                    Some(r##"Gets information about a model, including its name, the description (if set), and the default version (if at least one version of the model has been deployed)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the model."##),
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
            ("models-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-get-iam-policy",
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
            ("models-list",
                    Some(r##"Lists the models in a project. Each project can contain multiple models, and each model can have multiple versions. If there are no models that match the request parameters, the list request returns an empty response body: {}."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project whose models are to be listed."##),
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
            ("models-patch",
                    Some(r##"Updates a specific model resource. Currently the only supported fields to update are `description` and `default_version.name`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The project name."##),
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
            ("models-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-set-iam-policy",
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
            ("models-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-test-iam-permissions",
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
            ("models-versions-create",
                    Some(r##"Creates a new version of a model from a trained TensorFlow model. If the version created in the cloud by this call is the first deployed version of the specified model, it will be made the default version of the model. When you add a version to a model that already has one or more versions, the default version does not automatically change. If you want a new version to be the default, you must call projects.models.versions.setDefault."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-versions-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the model."##),
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
            ("models-versions-delete",
                    Some(r##"Deletes a model version. Each model can have multiple versions deployed and in use at any given time. Use this method to remove a single version. Note: You cannot delete the version that is set as the default version of the model unless it is the only remaining version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-versions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the version. You can get the names of all the versions of a model by calling projects.models.versions.list."##),
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
            ("models-versions-get",
                    Some(r##"Gets information about a model version. Models can have multiple versions. You can call projects.models.versions.list to get the same information that this method returns for all of the versions of a model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-versions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the version."##),
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
            ("models-versions-list",
                    Some(r##"Gets basic information about all the versions of a model. If you expect that a model has many versions, or if you need to handle only a limited number of results at a time, you can request that the list be retrieved in batches (called pages). If there are no versions that match the request parameters, the list request returns an empty response body: {}."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-versions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the model for which to list the version."##),
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
            ("models-versions-patch",
                    Some(r##"Updates the specified Version resource. Currently the only update-able fields are `description`, `requestLoggingConfig`, `autoScaling.minNodes`, and `manualScaling.nodes`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-versions-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the model."##),
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
            ("models-versions-set-default",
                    Some(r##"Designates a version to be the default for the model. The default version is used for prediction requests made against the model that don't specify a version. The first version to be created for a model is automatically set as the default. You must make any subsequent changes to the default version setting manually using this method."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_models-versions-set-default",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the version to make the default for the model. You can get the names of all the versions of a model by calling projects.models.versions.list."##),
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
            ("operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_operations-cancel",
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
            ("operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_operations-get",
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
            ("operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_operations-list",
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
            ("predict",
                    Some(r##"Performs online prediction on the data in the request. {% dynamic include "/ai-platform/includes/___predict-request" %} "##),
                    "Details at http://byron.github.io/google-apis-rs/google_ml1_cli/projects_predict",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of a model or a version. Authorization: requires the `predict` permission on the specified resource."##),
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
    
    let mut app = App::new("ml1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240607")
           .about("An API to enable creating and using machine learning models.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_ml1_cli")
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
