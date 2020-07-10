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
extern crate google_cloudbuild1 as api;

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
    hub: api::CloudBuild<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::CancelOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.operations().cancel(request, opt.value_of("name").unwrap_or(""));
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

    fn _operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_builds_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::CancelBuildRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().builds_cancel(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _projects_builds_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "finish-time" => Some(("finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timeout" => Some(("timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "log-url" => Some(("logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.project-id" => Some(("sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.invert-regex" => Some(("sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.commit-sha" => Some(("sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.substitutions" => Some(("sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source-provenance.resolved-repo-source.repo-name" => Some(("sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.tag-name" => Some(("sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.branch-name" => Some(("sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.dir" => Some(("sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.generation" => Some(("sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.object" => Some(("sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.bucket" => Some(("sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.images" => Some(("artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.objects.timing.end-time" => Some(("artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.objects.timing.start-time" => Some(("artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.objects.paths" => Some(("artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.objects.location" => Some(("artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logs-bucket" => Some(("logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status-detail" => Some(("statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.build-step-outputs" => Some(("results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "results.artifact-manifest" => Some(("results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-timing.end-time" => Some(("results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-timing.start-time" => Some(("results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.build-step-images" => Some(("results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "results.num-artifacts" => Some(("results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "options.substitution-option" => Some(("options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.machine-type" => Some(("options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.worker-pool" => Some(("options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.source-provenance-hash" => Some(("options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.log-streaming-option" => Some(("options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.secret-env" => Some(("options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.disk-size-gb" => Some(("options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.dynamic-substitutions" => Some(("options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "options.env" => Some(("options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.requested-verify-option" => Some(("options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.logging" => Some(("options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.project-id" => Some(("source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.invert-regex" => Some(("source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source.repo-source.commit-sha" => Some(("source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.substitutions" => Some(("source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source.repo-source.repo-name" => Some(("source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.tag-name" => Some(("source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.branch-name" => Some(("source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.dir" => Some(("source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.generation" => Some(("source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.object" => Some(("source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.bucket" => Some(("source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "images" => Some(("images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "queue-ttl" => Some(("queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build-trigger-id" => Some(("buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["artifact-manifest", "artifact-timing", "artifacts", "branch-name", "bucket", "build-step-images", "build-step-outputs", "build-trigger-id", "commit-sha", "create-time", "dir", "disk-size-gb", "dynamic-substitutions", "end-time", "env", "finish-time", "generation", "id", "images", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "num-artifacts", "object", "objects", "options", "paths", "project-id", "queue-ttl", "repo-name", "repo-source", "requested-verify-option", "resolved-repo-source", "resolved-storage-source", "results", "secret-env", "source", "source-provenance", "source-provenance-hash", "start-time", "status", "status-detail", "storage-source", "substitution-option", "substitutions", "tag-name", "tags", "timeout", "timing", "worker-pool"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Build = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().builds_create(request, opt.value_of("project-id").unwrap_or(""));
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

    fn _projects_builds_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().builds_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _projects_builds_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().builds_list(opt.value_of("project-id").unwrap_or(""));
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

    fn _projects_builds_retry(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::RetryBuildRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().builds_retry(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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

    fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::CancelOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_operations_cancel(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_locations_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "github.owner" => Some(("github.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.installation-id" => Some(("github.installationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.invert-regex" => Some(("github.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.pull-request.comment-control" => Some(("github.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.branch" => Some(("github.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.name" => Some(("github.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.invert-regex" => Some(("github.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.tag" => Some(("github.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.branch" => Some(("github.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ignored-files" => Some(("ignoredFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trigger-template.project-id" => Some(("triggerTemplate.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.invert-regex" => Some(("triggerTemplate.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger-template.commit-sha" => Some(("triggerTemplate.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.substitutions" => Some(("triggerTemplate.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trigger-template.repo-name" => Some(("triggerTemplate.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.tag-name" => Some(("triggerTemplate.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.branch-name" => Some(("triggerTemplate.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.dir" => Some(("triggerTemplate.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.status" => Some(("build.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.finish-time" => Some(("build.finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.timeout" => Some(("build.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.log-url" => Some(("build.logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.project-id" => Some(("build.sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.invert-regex" => Some(("build.sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.commit-sha" => Some(("build.sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.substitutions" => Some(("build.sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source-provenance.resolved-repo-source.repo-name" => Some(("build.sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.tag-name" => Some(("build.sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.branch-name" => Some(("build.sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.dir" => Some(("build.sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.generation" => Some(("build.sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.object" => Some(("build.sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.bucket" => Some(("build.sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.tags" => Some(("build.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.images" => Some(("build.artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.timing.end-time" => Some(("build.artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.timing.start-time" => Some(("build.artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.paths" => Some(("build.artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.location" => Some(("build.artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.logs-bucket" => Some(("build.logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.id" => Some(("build.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status-detail" => Some(("build.statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-outputs" => Some(("build.results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.artifact-manifest" => Some(("build.results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.end-time" => Some(("build.results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.start-time" => Some(("build.results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-images" => Some(("build.results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.num-artifacts" => Some(("build.results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.substitutions" => Some(("build.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.options.substitution-option" => Some(("build.options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.machine-type" => Some(("build.options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.worker-pool" => Some(("build.options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.source-provenance-hash" => Some(("build.options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.log-streaming-option" => Some(("build.options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.secret-env" => Some(("build.options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.disk-size-gb" => Some(("build.options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.dynamic-substitutions" => Some(("build.options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.env" => Some(("build.options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.requested-verify-option" => Some(("build.options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.logging" => Some(("build.options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.project-id" => Some(("build.source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.invert-regex" => Some(("build.source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source.repo-source.commit-sha" => Some(("build.source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.substitutions" => Some(("build.source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source.repo-source.repo-name" => Some(("build.source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.tag-name" => Some(("build.source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.branch-name" => Some(("build.source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.dir" => Some(("build.source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.generation" => Some(("build.source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.object" => Some(("build.source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.bucket" => Some(("build.source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.start-time" => Some(("build.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.project-id" => Some(("build.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.images" => Some(("build.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.queue-ttl" => Some(("build.queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.create-time" => Some(("build.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.build-trigger-id" => Some(("build.buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-files" => Some(("includedFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["artifact-manifest", "artifact-timing", "artifacts", "branch", "branch-name", "bucket", "build", "build-step-images", "build-step-outputs", "build-trigger-id", "comment-control", "commit-sha", "create-time", "description", "dir", "disabled", "disk-size-gb", "dynamic-substitutions", "end-time", "env", "filename", "finish-time", "generation", "github", "id", "ignored-files", "images", "included-files", "installation-id", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "owner", "paths", "project-id", "pull-request", "push", "queue-ttl", "repo-name", "repo-source", "requested-verify-option", "resolved-repo-source", "resolved-storage-source", "results", "secret-env", "source", "source-provenance", "source-provenance-hash", "start-time", "status", "status-detail", "storage-source", "substitution-option", "substitutions", "tag", "tag-name", "tags", "timeout", "timing", "trigger-template", "worker-pool"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BuildTrigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().triggers_create(request, opt.value_of("project-id").unwrap_or(""));
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

    fn _projects_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().triggers_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    fn _projects_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().triggers_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    fn _projects_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().triggers_list(opt.value_of("project-id").unwrap_or(""));
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

    fn _projects_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "github.owner" => Some(("github.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.installation-id" => Some(("github.installationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.invert-regex" => Some(("github.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.pull-request.comment-control" => Some(("github.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.branch" => Some(("github.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.name" => Some(("github.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.invert-regex" => Some(("github.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.tag" => Some(("github.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.branch" => Some(("github.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ignored-files" => Some(("ignoredFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trigger-template.project-id" => Some(("triggerTemplate.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.invert-regex" => Some(("triggerTemplate.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger-template.commit-sha" => Some(("triggerTemplate.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.substitutions" => Some(("triggerTemplate.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trigger-template.repo-name" => Some(("triggerTemplate.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.tag-name" => Some(("triggerTemplate.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.branch-name" => Some(("triggerTemplate.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.dir" => Some(("triggerTemplate.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.status" => Some(("build.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.finish-time" => Some(("build.finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.timeout" => Some(("build.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.log-url" => Some(("build.logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.project-id" => Some(("build.sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.invert-regex" => Some(("build.sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.commit-sha" => Some(("build.sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.substitutions" => Some(("build.sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source-provenance.resolved-repo-source.repo-name" => Some(("build.sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.tag-name" => Some(("build.sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.branch-name" => Some(("build.sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.dir" => Some(("build.sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.generation" => Some(("build.sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.object" => Some(("build.sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.bucket" => Some(("build.sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.tags" => Some(("build.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.images" => Some(("build.artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.timing.end-time" => Some(("build.artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.timing.start-time" => Some(("build.artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.paths" => Some(("build.artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.location" => Some(("build.artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.logs-bucket" => Some(("build.logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.id" => Some(("build.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status-detail" => Some(("build.statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-outputs" => Some(("build.results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.artifact-manifest" => Some(("build.results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.end-time" => Some(("build.results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.start-time" => Some(("build.results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-images" => Some(("build.results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.num-artifacts" => Some(("build.results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.substitutions" => Some(("build.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.options.substitution-option" => Some(("build.options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.machine-type" => Some(("build.options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.worker-pool" => Some(("build.options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.source-provenance-hash" => Some(("build.options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.log-streaming-option" => Some(("build.options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.secret-env" => Some(("build.options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.disk-size-gb" => Some(("build.options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.dynamic-substitutions" => Some(("build.options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.env" => Some(("build.options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.requested-verify-option" => Some(("build.options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.logging" => Some(("build.options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.project-id" => Some(("build.source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.invert-regex" => Some(("build.source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source.repo-source.commit-sha" => Some(("build.source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.substitutions" => Some(("build.source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source.repo-source.repo-name" => Some(("build.source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.tag-name" => Some(("build.source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.branch-name" => Some(("build.source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.dir" => Some(("build.source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.generation" => Some(("build.source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.object" => Some(("build.source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.bucket" => Some(("build.source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.start-time" => Some(("build.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.project-id" => Some(("build.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.images" => Some(("build.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.queue-ttl" => Some(("build.queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.create-time" => Some(("build.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.build-trigger-id" => Some(("build.buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-files" => Some(("includedFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["artifact-manifest", "artifact-timing", "artifacts", "branch", "branch-name", "bucket", "build", "build-step-images", "build-step-outputs", "build-trigger-id", "comment-control", "commit-sha", "create-time", "description", "dir", "disabled", "disk-size-gb", "dynamic-substitutions", "end-time", "env", "filename", "finish-time", "generation", "github", "id", "ignored-files", "images", "included-files", "installation-id", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "owner", "paths", "project-id", "pull-request", "push", "queue-ttl", "repo-name", "repo-source", "requested-verify-option", "resolved-repo-source", "resolved-storage-source", "results", "secret-env", "source", "source-provenance", "source-provenance-hash", "start-time", "status", "status-detail", "storage-source", "substitution-option", "substitutions", "tag", "tag-name", "tags", "timeout", "timing", "trigger-template", "worker-pool"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BuildTrigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().triggers_patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    fn _projects_triggers_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invert-regex" => Some(("invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "commit-sha" => Some(("commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "repo-name" => Some(("repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-name" => Some(("tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branch-name" => Some(("branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dir" => Some(("dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["branch-name", "commit-sha", "dir", "invert-regex", "project-id", "repo-name", "substitutions", "tag-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RepoSource = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().triggers_run(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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
            ("operations", Some(opt)) => {
                match opt.subcommand() {
                    ("cancel", Some(opt)) => {
                        call_result = self._operations_cancel(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._operations_get(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("builds-cancel", Some(opt)) => {
                        call_result = self._projects_builds_cancel(opt, dry_run, &mut err);
                    },
                    ("builds-create", Some(opt)) => {
                        call_result = self._projects_builds_create(opt, dry_run, &mut err);
                    },
                    ("builds-get", Some(opt)) => {
                        call_result = self._projects_builds_get(opt, dry_run, &mut err);
                    },
                    ("builds-list", Some(opt)) => {
                        call_result = self._projects_builds_list(opt, dry_run, &mut err);
                    },
                    ("builds-retry", Some(opt)) => {
                        call_result = self._projects_builds_retry(opt, dry_run, &mut err);
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err);
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err);
                    },
                    ("triggers-create", Some(opt)) => {
                        call_result = self._projects_triggers_create(opt, dry_run, &mut err);
                    },
                    ("triggers-delete", Some(opt)) => {
                        call_result = self._projects_triggers_delete(opt, dry_run, &mut err);
                    },
                    ("triggers-get", Some(opt)) => {
                        call_result = self._projects_triggers_get(opt, dry_run, &mut err);
                    },
                    ("triggers-list", Some(opt)) => {
                        call_result = self._projects_triggers_list(opt, dry_run, &mut err);
                    },
                    ("triggers-patch", Some(opt)) => {
                        call_result = self._projects_triggers_patch(opt, dry_run, &mut err);
                    },
                    ("triggers-run", Some(opt)) => {
                        call_result = self._projects_triggers_run(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "cloudbuild1-secret.json",
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
                                          program_name: "cloudbuild1",
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
            hub: api::CloudBuild::new(client, auth),
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
        ("operations", "methods: 'cancel' and 'get'", vec![
            ("cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation.  The server
        makes a best effort to cancel the operation, but success is not
        guaranteed.  If the server doesn't support this method, it returns
        `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
        Operations.GetOperation or
        other methods to check whether the cancellation succeeded or whether the
        operation completed despite cancellation. On successful cancellation,
        the operation is not deleted; instead, it becomes an operation with
        an Operation.error value with a google.rpc.Status.code of 1,
        corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/operations_cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("get",
                    Some(r##"Gets the latest state of a long-running operation.  Clients can use this
        method to poll the operation result at intervals as recommended by the API
        service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/operations_get",
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
            ]),
        
        ("projects", "methods: 'builds-cancel', 'builds-create', 'builds-get', 'builds-list', 'builds-retry', 'locations-operations-cancel', 'locations-operations-get', 'triggers-create', 'triggers-delete', 'triggers-get', 'triggers-list', 'triggers-patch' and 'triggers-run'", vec![
            ("builds-cancel",
                    Some(r##"Cancels a build in progress."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_builds-cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"Required. ID of the build."##),
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
            ("builds-create",
                    Some(r##"Starts a build with the specified configuration.
        
        This method returns a long-running `Operation`, which includes the build
        ID. Pass the build ID to `GetBuild` to determine the build status (such as
        `SUCCESS` or `FAILURE`)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_builds-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project."##),
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
            ("builds-get",
                    Some(r##"Returns information about a previously requested build.
        
        The `Build` that is returned includes its status (such as `SUCCESS`,
        `FAILURE`, or `WORKING`), and timing information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_builds-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"Required. ID of the build."##),
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
            ("builds-list",
                    Some(r##"Lists previously requested builds.
        
        Previously requested builds may still be in-progress, or may have finished
        successfully or unsuccessfully."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_builds-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project."##),
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
            ("builds-retry",
                    Some(r##"Creates a new build based on the specified build.
        
        This method creates a new build using the original build request, which may
        or may not result in an identical build.
        
        For triggered builds:
        
        * Triggered builds resolve to a precise revision; therefore a retry of a
        triggered build will result in a build that uses the same revision.
        
        For non-triggered builds that specify `RepoSource`:
        
        * If the original build built from the tip of a branch, the retried build
        will build from the tip of that branch, which may not be the same revision
        as the original build.
        * If the original build specified a commit sha or revision ID, the retried
        build will use the identical source.
        
        For builds that specify `StorageSource`:
        
        * If the original build pulled source from Google Cloud Storage without
        specifying the generation of the object, the new build will use the current
        object, which may be different from the original build source.
        * If the original build pulled source from Cloud Storage and specified the
        generation of the object, the new build will attempt to use the same
        object, which may or may not be available depending on the bucket's
        lifecycle management settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_builds-retry",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"Required. Build ID of the original build."##),
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation.  The server
        makes a best effort to cancel the operation, but success is not
        guaranteed.  If the server doesn't support this method, it returns
        `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
        Operations.GetOperation or
        other methods to check whether the cancellation succeeded or whether the
        operation completed despite cancellation. On successful cancellation,
        the operation is not deleted; instead, it becomes an operation with
        an Operation.error value with a google.rpc.Status.code of 1,
        corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation.  Clients can use this
        method to poll the operation result at intervals as recommended by the API
        service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-operations-get",
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
            ("triggers-create",
                    Some(r##"Creates a new `BuildTrigger`.
        
        This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project for which to configure automatic builds."##),
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
            ("triggers-delete",
                    Some(r##"Deletes a `BuildTrigger` by its project ID and trigger ID.
        
        This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project that owns the trigger."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. ID of the `BuildTrigger` to delete."##),
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
            ("triggers-get",
                    Some(r##"Returns information about a `BuildTrigger`.
        
        This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project that owns the trigger."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. Identifier (`id` or `name`) of the `BuildTrigger` to get."##),
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
            ("triggers-list",
                    Some(r##"Lists existing `BuildTrigger`s.
        
        This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project for which to list BuildTriggers."##),
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
            ("triggers-patch",
                    Some(r##"Updates a `BuildTrigger` by its project ID and trigger ID.
        
        This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project that owns the trigger."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. ID of the `BuildTrigger` to update."##),
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
            ("triggers-run",
                    Some(r##"Runs a `BuildTrigger` at a particular source revision."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-run",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. ID of the project."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. ID of the trigger."##),
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
    
    let mut app = App::new("cloudbuild1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.14+20200704")
           .about("Creates and manages builds on Google Cloud Platform.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli")
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